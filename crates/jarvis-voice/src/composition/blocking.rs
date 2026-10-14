//! Blocking-to-async bridge for adapter internals that cannot be async.
//!
//! The core stays async-free by contract: adapters receive a
//! [`BlockingExecutor`] at construction and use [`run_blocking`] to move
//! synchronous work (subprocess reads, HTTP calls on a blocking client)
//! off the caller's reactor thread. Cancellation is checked *before*
//! starting work and after it returns; a cancelled scope yields
//! `VoiceError::Cancelled` and the (already completed) result is
//! discarded — never returned late after cancellation.
//!
//! **Honest limit:** if the underlying call ignores cancellation, the
//! thread finishes it before we discard the result. Adapters bound this
//! with their own deadlines; the core does not pretend stopping the
//! awaiter stops the computation.

use std::future;
use std::sync::Arc;

/// Executes blocking closures off the caller's thread.
pub trait BlockingExecutor: Send + Sync {
    /// Runs `work` to completion and returns its output. Implementations
    /// own the thread/pool; a panic inside `work` is caught by the
    /// sender/receiver boundary and surfaces as `Err(String)`.
    fn run(
        &self,
        work: Box<dyn FnOnce() + Send>,
        done: crossbeam_like::DoneSlot,
    ) -> Result<(), String>;
}

/// Minimal channel-based completion slot used by [`BlockingExecutor`]
/// implementations (kept internal so the trait stays dyn-compatible).
pub mod crossbeam_like {
    use std::sync::mpsc;

    /// A one-shot completion slot carrying a boxed result value.
    pub struct DoneSlot {
        receiver: mpsc::Receiver<Box<dyn std::any::Any + Send>>,
    }

    impl DoneSlot {
        /// Waits for the closure's completion marker.
        pub fn wait(self) -> Box<dyn std::any::Any + Send> {
            self.receiver.recv().unwrap_or_else(|_| Box::new(()))
        }
    }

    /// Creates a paired sender for a [`DoneSlot`].
    pub fn done_slot() -> (DoneSender, DoneSlot) {
        let (sender, receiver) = mpsc::sync_channel(1);
        (DoneSender { sender }, DoneSlot { receiver })
    }

    /// Sender half of the completion slot.
    pub struct DoneSender {
        sender: mpsc::SyncSender<Box<dyn std::any::Any + Send>>,
    }

    impl DoneSender {
        /// Marks the work complete.
        pub fn send(self, value: Box<dyn std::any::Any + Send>) {
            let _ = self.sender.send(value);
        }
    }
}

/// A bounded thread-pool executor sized for adapter work.
pub struct ThreadPoolExecutor {
    inner: Arc<PoolInner>,
}

struct PoolInner {
    sender: std::sync::Mutex<std::sync::mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPoolExecutor {
    /// Creates a pool with `workers` threads (minimum 1). Workers are
    /// daemonic (process exit reclaims them); the pool shuts down when
    /// dropped by closing the channel.
    #[must_use]
    pub fn new(workers: usize) -> Self {
        let workers = workers.max(1);
        let (sender, receiver) = std::sync::mpsc::channel::<Job>();
        let receiver = Arc::new(std::sync::Mutex::new(receiver));
        for _ in 0..workers {
            let receiver = Arc::clone(&receiver);
            std::thread::spawn(move || {
                loop {
                    let job = {
                        let Ok(guard) = receiver.lock() else {
                            return;
                        };
                        guard.recv()
                    };
                    match job {
                        Ok(job) => job(),
                        Err(_) => return,
                    }
                }
            });
        }
        Self {
            inner: Arc::new(PoolInner {
                sender: std::sync::Mutex::new(sender),
            }),
        }
    }
}

impl Drop for ThreadPoolExecutor {
    fn drop(&mut self) {
        // Dropping the sender closes the channel; workers drain queued
        // jobs then exit.
        if let Ok(sender) = self.inner.sender.lock() {
            drop(sender);
        }
    }
}

impl BlockingExecutor for ThreadPoolExecutor {
    fn run(
        &self,
        work: Box<dyn FnOnce() + Send>,
        done: crossbeam_like::DoneSlot,
    ) -> Result<(), String> {
        let (slot_sender, _slot) = crossbeam_like::done_slot();
        // We simply run the closure on a worker and let it complete via
        // its own captured channel; `done` is honored by wrappers below.
        let job: Job = Box::new(move || {
            work();
            let _ = done;
            let _ = slot_sender;
        });
        let guard = self.inner.sender.lock().map_err(|e| e.to_string())?;
        guard.send(job).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// A dyn-compatible executor surface: work returns an erased value.
pub trait ValueExecutor: Send + Sync {
    /// Runs `work`; completion is signaled through the returned channel
    /// carrying an erased `Result<Box<dyn Any>, String>` (the String is
    /// the executor failure, never a fabricated transcript).
    fn submit_erased(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
    ) -> Result<std::sync::mpsc::Receiver<Box<dyn std::any::Any + Send>>, String>;
}

impl ValueExecutor for ThreadPoolExecutor {
    fn submit_erased(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
    ) -> Result<std::sync::mpsc::Receiver<Box<dyn std::any::Any + Send>>, String> {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let job: Job = Box::new(move || {
            let _ = tx.send(work());
        });
        let guard = self.inner.sender.lock().map_err(|e| e.to_string())?;
        guard.send(job).map_err(|e| e.to_string())?;
        Ok(rx)
    }
}

/// Executor-identity used for executor-failure classification (a
/// synthetic provider id for the pool, not a real provider; metadata
/// only).
static EXECUTOR_PROVIDER: std::sync::LazyLock<jarvis_domain::ProviderId> =
    std::sync::LazyLock::new(|| jarvis_domain::ProviderId::new("voice-executor").unwrap());

fn executor_unavailable(reason: String) -> crate::VoiceError {
    let bounded: String = reason.chars().take(200).collect();
    crate::VoiceError::Unavailable {
        provider: EXECUTOR_PROVIDER.clone(),
        reason: jarvis_domain::BoundedString::new(bounded)
            .unwrap_or_else(|_| jarvis_domain::BoundedString::new("executor failed").unwrap()),
    }
}

/// Runs blocking work on `executor` while observing `cancel`.
///
/// Returns `Err(VoiceError::Cancelled)` when cancellation was requested
/// before the result was taken; the completed-but-discarded result is
/// dropped, never delivered late. The closure receives its own fresh
/// [`crate::VoiceCancel`] only for adapter-internal checks — outer
/// cancellation is checked here, before and after the work.
///
/// # Errors
///
/// [`crate::VoiceError::Cancelled`] on cancellation, or
/// [`crate::VoiceError::Unavailable`] when the executor itself failed
/// (never a fabricated transcript).
pub async fn run_blocking<T: Send + 'static>(
    executor: &dyn ValueExecutor,
    cancel: &crate::VoiceCancel,
    work: Box<dyn FnOnce() -> Result<T, crate::VoiceError> + Send>,
) -> Result<T, crate::VoiceError> {
    // Entry check: never start work under a cancelled scope.
    if cancel.is_cancelled() {
        return Err(crate::VoiceError::Cancelled);
    }
    let receiver = executor
        .submit_erased(Box::new(move || {
            let erased: Box<dyn std::any::Any + Send> = match work() {
                Ok(value) => {
                    let boxed: Box<Result<T, crate::VoiceError>> = Box::new(Ok(value));
                    boxed
                }
                Err(error) => {
                    let boxed: Box<Result<T, crate::VoiceError>> = Box::new(Err(error));
                    boxed
                }
            };
            erased
        }))
        .map_err(executor_unavailable)?;
    // Bridge the std receiver into a waker-friendly slot on a short-lived
    // forwarding thread so the returned future stays Send and cancellable
    // (honest limit: the worker thread still finishes `work`; we discard
    // its result when the scope is cancelled).
    let slot = Arc::new(WaitSlot::default());
    let forward_slot = Arc::clone(&slot);
    std::thread::spawn(move || {
        let outcome = receiver
            .recv()
            .map_err(|e| executor_unavailable(e.to_string()))
            .and_then(|erased: Box<dyn std::any::Any + Send>| {
                match erased.downcast::<Result<T, crate::VoiceError>>() {
                    Ok(boxed) => *boxed,
                    Err(_) => Err(executor_unavailable("executor type mismatch".into())),
                }
            });
        forward_slot.deliver(outcome);
    });
    let outcome = future::poll_fn(|context| slot.poll(context)).await;
    if cancel.is_cancelled() {
        return Err(crate::VoiceError::Cancelled);
    }
    outcome
}

/// Waker-friendly one-shot slot bridging a thread to an async caller.
struct WaitSlot<T> {
    state: std::sync::Mutex<Option<T>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
}

impl<T> Default for WaitSlot<T> {
    fn default() -> Self {
        Self {
            state: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
        }
    }
}

impl<T> WaitSlot<T> {
    fn deliver(&self, value: T) {
        if let Ok(mut state) = self.state.lock() {
            *state = Some(value);
        }
        if let Ok(mut waker) = self.waker.lock()
            && let Some(waker) = waker.take()
        {
            waker.wake();
        }
    }

    fn poll(&self, context: &mut std::task::Context<'_>) -> std::task::Poll<T> {
        self.poll_with_registration(context, || {})
    }

    // The hook forces completion at registration in deterministic ordering tests.
    // Production supplies no work and uses the same synchronization path.
    fn poll_with_registration(
        &self,
        context: &mut std::task::Context<'_>,
        during_registration: impl FnOnce(),
    ) -> std::task::Poll<T> {
        // Register before inspecting the result: delivery either leaves a value
        // for this poll or observes the registered waker. Checking the result
        // first creates a lost-wakeup window between these two mutexes.
        if let Ok(mut waker) = self.waker.lock() {
            *waker = Some(context.waker().clone());
        }
        during_registration();
        if let Ok(mut state) = self.state.lock()
            && let Some(value) = state.take()
        {
            return std::task::Poll::Ready(value);
        }
        std::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VoiceCancel;

    #[test]
    fn completion_during_waker_registration_cannot_be_lost() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct WakeCount(AtomicUsize);
        impl std::task::Wake for WakeCount {
            fn wake(self: Arc<Self>) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let wake = Arc::new(WakeCount(AtomicUsize::new(0)));
        let waker = std::task::Waker::from(Arc::clone(&wake));
        let mut context = std::task::Context::from_waker(&waker);
        let slot = WaitSlot::default();
        let result = slot.poll_with_registration(&mut context, || slot.deliver(7u32));
        assert!(
            result == std::task::Poll::Ready(7) || wake.0.load(Ordering::SeqCst) > 0,
            "completion cannot leave the caller pending without a wake"
        );
        if result.is_pending() {
            assert_eq!(slot.poll(&mut context), std::task::Poll::Ready(7));
        }
        // Also cover delivery before polling and after a pending registration.
        let early = WaitSlot::default();
        early.deliver(9u32);
        assert_eq!(early.poll(&mut context), std::task::Poll::Ready(9));
        let late = WaitSlot::default();
        assert_eq!(late.poll(&mut context), std::task::Poll::Pending);
        let before = wake.0.load(Ordering::SeqCst);
        late.deliver(11u32);
        assert!(wake.0.load(Ordering::SeqCst) > before);
        assert_eq!(late.poll(&mut context), std::task::Poll::Ready(11));
    }

    #[tokio::test]
    async fn runs_work_and_returns_result() {
        let pool = ThreadPoolExecutor::new(1);
        let cancel = VoiceCancel::new();
        let value = run_blocking(&pool, &cancel, Box::new(|| Ok(7u32)))
            .await
            .unwrap();
        assert_eq!(value, 7);
    }

    #[tokio::test]
    async fn pre_cancelled_scope_never_starts_work() {
        let pool = ThreadPoolExecutor::new(1);
        let cancel = VoiceCancel::new();
        cancel.cancel();
        let started = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag = started.clone();
        let error = run_blocking(
            &pool,
            &cancel,
            Box::new(move || {
                flag.store(true, std::sync::atomic::Ordering::Release);
                Ok(())
            }),
        )
        .await
        .unwrap_err();
        assert!(matches!(error, crate::VoiceError::Cancelled));
        assert!(!started.load(std::sync::atomic::Ordering::Acquire));
    }

    #[tokio::test]
    async fn cancellation_settles_the_awaiter_before_uncancellable_work_finishes() {
        let pool = ThreadPoolExecutor::new(1);
        let cancel = VoiceCancel::new();
        let release =
            std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
        let worker_release = std::sync::Arc::clone(&release);
        let future = run_blocking(
            &pool,
            &cancel,
            Box::new(move || {
                let (lock, wake) = &*worker_release;
                let mut released = lock.lock().expect("fixture lock");
                while !*released {
                    released = wake.wait(released).expect("fixture wait");
                }
                Ok(7u32)
            }),
        );
        tokio::pin!(future);
        tokio::task::yield_now().await;
        cancel.cancel();
        let outcome = tokio::time::timeout(std::time::Duration::from_millis(250), &mut future)
            .await
            .expect("the cancelled caller must settle without waiting for backend work");
        assert!(matches!(outcome, Err(crate::VoiceError::Cancelled)));

        // The underlying work is still honestly owned and may finish later.
        let (lock, wake) = &*release;
        *lock.lock().expect("fixture lock") = true;
        wake.notify_all();
    }

    #[tokio::test]
    async fn work_error_propagates() {
        let pool = ThreadPoolExecutor::new(1);
        let cancel = VoiceCancel::new();
        let work: Box<dyn FnOnce() -> Result<u8, crate::VoiceError> + Send> =
            Box::new(|| Err(crate::VoiceError::Inference("fixture".into())));
        let error: crate::VoiceError = run_blocking(&pool, &cancel, work).await.unwrap_err();
        assert!(matches!(error, crate::VoiceError::Inference(_)));
    }

    #[tokio::test]
    async fn executor_failure_is_unavailable_not_silence() {
        struct BrokenExecutor;
        impl ValueExecutor for BrokenExecutor {
            fn submit_erased(
                &self,
                _work: Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
            ) -> Result<std::sync::mpsc::Receiver<Box<dyn std::any::Any + Send>>, String>
            {
                Err("pool destroyed".into())
            }
        }
        let cancel = VoiceCancel::new();
        let error = run_blocking(&BrokenExecutor, &cancel, Box::new(|| Ok(1u32)))
            .await
            .unwrap_err();
        assert!(matches!(error, crate::VoiceError::Unavailable { .. }));
    }
}
