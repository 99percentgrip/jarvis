//! VRO-17 R16: the host-owned persistent Silero VAD worker for the FLM
//! composition. Wraps the fixed `voice_vad.py` (same script, same
//! environment — never a second Python environment or VAD script) as
//! ONE lazily-started, reused child per process.
//!
//! Provenance discipline (binding PRD + the corrected launch-gate
//! evidence): only an explicit successful detector decision with **zero
//! speech intervals** yields the no-speech outcome. VAD errors,
//! malformed replies, missing assets and timeouts are errors — never
//! "confirmed silence", never a silent bypass to the backend.

use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Arc;

use jarvis_domain::{BoundedString, ProviderId};
use jarvis_voice::cancel::VoiceCancel;
use jarvis_voice::composition::blocking::{self, ValueExecutor};
use jarvis_voice::error::VoiceError;

/// The worker protocol's bounded line size (JSON replies are small).
const MAX_LINE_BYTES: usize = 8192;
/// One VAD filter request's deadline (the installed Silero model
/// filters a 120 s capture in well under a second; generous bound
/// without an unbounded wait).
const FILTER_DEADLINE: std::time::Duration = std::time::Duration::from_secs(60);

/// Provider identity used in VAD-stage errors.
fn provider() -> ProviderId {
    ProviderId::new("stt-flm-npu").expect("static id fits")
}

/// The persistent VAD worker child: stdin request lines in, one JSON
/// reply line per request, `shutdown` op at teardown.
struct VadChild {
    child: Child,
    stdin: ChildStdin,
    lines: std::sync::mpsc::Receiver<String>,
}

impl VadChild {
    /// Spawns the fixed worker through the harness voice venv's Python
    /// (the same interpreter dictation uses; never a second venv).
    fn spawn() -> Result<Self, VoiceError> {
        let python = venv_python()?;
        let mut command = Command::new(&python);
        command
            .args(["-u", "-c", include_str!("voice_vad.py")])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            // The worker resolves its own site-packages from the venv
            // interpreter; an inherited PYTHONPATH (fixture contexts set
            // one) can shadow `faster_whisper` with a stub and kill the
            // import. Remove it: the worker never needs it.
            .env_remove("PYTHONPATH");
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let runtime_provider = provider();
        let mut child = command.spawn().map_err(|error| VoiceError::Unavailable {
            provider: runtime_provider.clone(),
            reason: BoundedString::new(format!(
                "VAD worker unavailable ({}): press F5 once so Vesper prepares the voice backend",
                error.kind()
            ))
            .unwrap_or_else(|_| BoundedString::new("VAD worker unavailable").unwrap()),
        })?;
        let stdin = child.stdin.take().ok_or_else(|| unavailability("stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| unavailability("stdout"))?;
        let (sender, lines) = std::sync::mpsc::sync_channel(16);
        std::thread::spawn(move || {
            let mut reader = std::io::BufReader::new(stdout);
            let mut buffer = String::new();
            loop {
                buffer.clear();
                match reader.read_line(&mut buffer) {
                    Ok(0) | Err(_) => return,
                    Ok(_) => {
                        if buffer.len() > MAX_LINE_BYTES
                            || sender.send(buffer.trim_end().to_owned()).is_err()
                        {
                            return;
                        }
                    }
                }
            }
        });
        let mut worker = Self {
            child,
            stdin,
            lines,
        };
        // The worker emits readiness before accepting requests; a child
        // that cannot initialize its model must fail here, not at the
        // first utterance.
        let ready = worker.recv(FILTER_DEADLINE)?;
        let value: serde_json::Value =
            serde_json::from_str(&ready).map_err(|_| VoiceError::Unavailable {
                provider: runtime_provider.clone(),
                reason: BoundedString::new("VAD worker produced a malformed readiness line")
                    .expect("static fits"),
            })?;
        if value.get("ready").and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(VoiceError::Unavailable {
                provider: runtime_provider,
                reason: BoundedString::new("VAD worker did not report readiness")
                    .expect("static fits"),
            });
        }
        Ok(worker)
    }

    fn recv(&mut self, deadline: std::time::Duration) -> Result<String, VoiceError> {
        self.lines
            .recv_timeout(deadline)
            .map_err(|_| unavailability("reply (deadline exceeded)"))
    }

    fn exited(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(Some(_)) | Err(_))
    }
}

impl Drop for VadChild {
    fn drop(&mut self) {
        // Graceful shutdown first; the owned-group kill follows so a
        // wedged child still cannot outlive the process.
        let _ = writeln!(self.stdin, r#"{{"op":"shutdown"}}"#);
        let _ = self.stdin.flush();
        #[cfg(unix)]
        {
            let _ = Command::new("kill")
                .args(["-TERM", &format!("{}", self.child.id())])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn unavailability(what: &'static str) -> VoiceError {
    VoiceError::Unavailable {
        provider: provider(),
        reason: BoundedString::new(format!("VAD worker {what} unavailable")).unwrap(),
    }
}

/// The harness voice venv interpreter (same resolution as dictation).
fn venv_python() -> Result<PathBuf, VoiceError> {
    let candidate = crate::platform_voice::venv_python(&crate::voice_venv_root());
    if candidate.is_file() {
        return Ok(candidate);
    }
    Err(VoiceError::Unavailable {
        provider: provider(),
        reason: BoundedString::new(
            "the local VAD backend is not prepared: press F5 once so Vesper prepares it",
        )
        .expect("static fits"),
    })
}

/// Fresh private audio workspace; no prefix scan or foreign-directory reclamation.
fn private_vad_work() -> std::io::Result<jarvis_security::PrivateTemporaryDirectory> {
    jarvis_security::private_temporary_directory("jarvis-flm-vad-work-")
}

/// The shared worker owner: one lazily-started child per process,
/// reused across requests (VAD-only; never instantiates a CPU
/// recognizer).
pub struct VadWorker {
    child: std::sync::Mutex<Option<VadChild>>,
    executor: Arc<dyn ValueExecutor>,
}

impl VadWorker {
    /// Creates the owner (spawns nothing).
    #[must_use]
    pub fn new(executor: Arc<dyn ValueExecutor>) -> Self {
        Self {
            child: std::sync::Mutex::new(None),
            executor,
        }
    }

    /// Runs one filter request against the shared child. Returns
    /// `Ok(None)` for an explicit zero-speech decision, `Ok(Some(wav))`
    /// for a canonical speech-only WAV.
    fn filter(&self, wav_path: &str, out_path: &str) -> Result<Option<Vec<u8>>, VoiceError> {
        let mut guard = self
            .child
            .lock()
            .map_err(|_| unavailability("state (poisoned lock)"))?;
        let needs_spawn = match guard.as_mut() {
            Some(child) => child.exited(),
            None => true,
        };
        if needs_spawn {
            *guard = Some(VadChild::spawn()?);
        }
        let child = guard.as_mut().ok_or_else(|| unavailability("child"))?;
        let request = serde_json::json!({
            "op": "filter",
            "input_wav": wav_path,
            "output_wav": out_path,
            "id": 1,
        });
        let line = serde_json::to_string(&request)
            .map_err(|_| VoiceError::InvalidInput("VAD request serialization failed".into()))?;
        child
            .stdin
            .write_all(line.as_bytes())
            .and_then(|()| child.stdin.write_all(b"\n"))
            .and_then(|()| child.stdin.flush())
            .map_err(|_| unavailability("pipe (closed)"))?;
        let reply = child.recv(FILTER_DEADLINE)?;
        if child.exited() {
            return Err(unavailability("worker (exited)"));
        }
        let value: serde_json::Value = serde_json::from_str(&reply)
            .map_err(|_| VoiceError::InvalidInput("VAD reply malformed".into()))?;
        if value.get("error").is_some() {
            return Err(VoiceError::Inference(
                "local speech detection failed; audio retained for retry".into(),
            ));
        }
        match value.get("speech").and_then(serde_json::Value::as_bool) {
            Some(false) => Ok(None),
            Some(true) => {
                let bytes = std::fs::read(out_path)
                    .map_err(|_| unavailability("filtered audio (missing)"))?;
                if bytes.is_empty() {
                    return Err(VoiceError::Inference(
                        "local speech detection produced no audio".into(),
                    ));
                }
                Ok(Some(bytes))
            }
            None => Err(VoiceError::InvalidInput(
                "VAD reply missing its decision".into(),
            )),
        }
    }
}

/// One capture's VAD preprocessing: writes the canonical WAV to private
/// managed storage, runs the shared worker, returns the speech-only
/// WAV bytes (`None` = explicit zero-speech decision).
///
/// Errors are errors: VAD failure never becomes silence and never
/// bypasses to the recognizer.
pub async fn preprocess(pcm: &[u8], cancel: &VoiceCancel) -> Result<Option<Vec<u8>>, VoiceError> {
    if cancel.is_cancelled() {
        return Err(VoiceError::Cancelled);
    }
    // Private managed directory (removed on drop; audio never persists).
    let private_work = private_vad_work().map_err(|_| unavailability("work dir"))?;
    let dir = private_work.path().to_path_buf();
    let wav = dir.join("capture.wav");
    let write_result = write_canonical_wav(&wav, pcm);
    match write_result {
        Ok(()) => {
            let filtered = dir.join("speech.wav");
            let owner = shared_worker();
            let wav_path = wav.display().to_string();
            let out_path = filtered.display().to_string();
            let executor = Arc::clone(&owner.executor);
            blocking::run_blocking(
                executor.as_ref(),
                cancel,
                Box::new(move || {
                    // A cancelled caller must not remove audio while the owned
                    // blocking worker can still write it. Cleanup follows settlement.
                    let _private_work = private_work;
                    owner.filter(&wav_path, &out_path)
                }),
            )
            .await
        }
        Err(error) => Err(error),
    }
}

/// The one process-wide VAD worker owner.
fn shared_worker() -> &'static VadWorker {
    static WORKER: std::sync::OnceLock<VadWorker> = std::sync::OnceLock::new();
    WORKER.get_or_init(|| {
        let pool = jarvis_voice::composition::blocking::ThreadPoolExecutor::new(1);
        let erased: Arc<dyn ValueExecutor> = Arc::new(pool) as Arc<dyn ValueExecutor>;
        VadWorker::new(erased)
    })
}

/// Writes canonical PCM as the WAV the worker requires (44-byte header,
/// 16 kHz mono i16 — same shape as the sidecar's writer).
fn write_canonical_wav(path: &std::path::Path, pcm: &[u8]) -> Result<(), VoiceError> {
    let mut header = Vec::with_capacity(44);
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&((36 + pcm.len()) as u32).to_le_bytes());
    header.extend_from_slice(b"WAVE");
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes());
    header.extend_from_slice(&1u16.to_le_bytes());
    header.extend_from_slice(&1u16.to_le_bytes());
    header.extend_from_slice(&16_000u32.to_le_bytes());
    header.extend_from_slice(&32_000u32.to_le_bytes());
    header.extend_from_slice(&2u16.to_le_bytes());
    header.extend_from_slice(&16u16.to_le_bytes());
    header.extend_from_slice(b"data");
    header.extend_from_slice(&(pcm.len() as u32).to_le_bytes());
    header.extend_from_slice(pcm);
    jarvis_security::write_private_atomic(path, &header).map_err(|_| unavailability("capture data"))
}

#[cfg(test)]
mod ownership_tests {
    use super::*;

    #[test]
    fn vad_workspaces_are_unique_private_and_cleanup_only_their_own_audio() {
        let foreign = tempfile::Builder::new()
            .prefix("jarvis-flm-vad-foreign-")
            .tempdir()
            .unwrap();
        let canary = foreign.path().join("capture.wav");
        std::fs::write(&canary, b"foreign audio").unwrap();
        let first = private_vad_work().unwrap();
        let second = private_vad_work().unwrap();
        assert_ne!(first.path(), second.path());
        jarvis_security::validate_owned_directory(first.path()).unwrap();
        jarvis_security::validate_owned_directory(second.path()).unwrap();
        let audio = first.path().join("capture.wav");
        write_canonical_wav(&audio, &[0, 0, 1, 0]).unwrap();
        jarvis_security::validate_private_file(&audio).unwrap();
        assert_eq!(&std::fs::read(&audio).unwrap()[..4], b"RIFF");
        let first_path = first.path().to_path_buf();
        let second_path = second.path().to_path_buf();
        drop(first);
        drop(second);
        assert!(!first_path.exists());
        assert!(!second_path.exists());
        assert_eq!(std::fs::read(canary).unwrap(), b"foreign audio");
    }
}
