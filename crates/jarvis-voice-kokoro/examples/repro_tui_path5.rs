//! R3 repro #5: THE decisive experiment. speak_unit called from inside
//! a tokio::spawn'd task (the TUI's apply_agent_progress actually runs
//! inside tokio::main's event loop). If jarvis_voice's park-based
//! block_on or the Kokoro future hangs in that context, the TUI event
//! loop freezes — Alex's "not replying".

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::StreamExt as _;
use jarvis_voice::ports::VoiceTts as _;

fn main() {
    println!("repro #5: speak_unit path inside tokio::spawn + block_on");
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let handle = tokio::spawn(async move {
            let adapter = Arc::new(
                jarvis_voice_kokoro::KokoroTts::new(Arc::new(
                    jarvis_voice::composition::blocking::ThreadPoolExecutor::new(1),
                ))
                .expect("adapter"),
            );
            let profile = jarvis_voice::ports::VoiceProfile {
                voice_id: jarvis_domain::BoundedString::<128>::new("am_michael".to_owned())
                    .expect("fits"),
                label: jarvis_domain::BoundedString::<128>::new("Michael".to_owned())
                    .expect("fits"),
                sample_rate_hz: jarvis_voice::audio::CANONICAL_SAMPLE_RATE_HZ,
            };
            let started = Instant::now();
            println!("[task] synthesizing (open phase)…");
            let cancel = jarvis_voice::VoiceCancel::new();
            // jarvis_voice::test_util::block_on INSIDE a tokio task —
            // this is what a naive speak_unit port does and what the
            // suspected deadlock looks like.
            let opened = jarvis_voice::test_util::block_on(adapter.synthesize(
                "Understood, all checks passed.",
                &profile,
                &cancel,
            ));
            match opened {
                Ok(mut stream) => {
                    let mut audio = 0usize;
                    loop {
                        match jarvis_voice::test_util::block_on(stream.next()) {
                            Some(Ok(jarvis_voice::ports::TtsChunk::Audio(frame))) => {
                                audio += frame.sample_count();
                            }
                            Some(Ok(jarvis_voice::ports::TtsChunk::Finished)) => break,
                            _ => break,
                        }
                    }
                    println!("[task] DONE {audio} samples in {:?}", started.elapsed());
                }
                Err(error) => println!("[task] OPEN FAILED: {error}"),
            }
        });
        match tokio::time::timeout(Duration::from_secs(60), handle).await {
            Ok(Ok(())) => println!("repro #5 completed normally (no hang)"),
            Ok(Err(join)) => println!("task failed: {join}"),
            Err(_) => {
                println!("TIMEOUT 60 s: park-block_on inside tokio task WEDGED");
                std::process::exit(7);
            }
        }
    });
}
