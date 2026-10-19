//! R3 worker device check (evidence helper): drives the REAL
//! SpeechWorker with the REAL pack and the REAL aplay, exactly the
//! production wiring. Prints worker outcomes with timings so a silent
//! speaker path shows up as bytes=0 / never-Drained.
//!
//! NOTE: this PLAYS audio through the default device.

use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() {
    println!("worker device check: real pack + real aplay (audio will play)");
    let root = jarvis_tui::platform_voice::data_root()
        .expect("independent application data root")
        .join("voice-pack");
    jarvis_security::validate_private_path(&root).expect("private voice pack ownership");
    let state = match jarvis_voice_kokoro::engine::EngineState::load(&root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("ENGINE LOAD FAILED: {error}");
            std::process::exit(2);
        }
    };
    match state {
        jarvis_voice_kokoro::engine::EngineState::Ready(_) => println!("engine: Ready"),
        jarvis_voice_kokoro::engine::EngineState::NotReady(problem) => {
            eprintln!("NOT READY: {}", problem.description());
            std::process::exit(2);
        }
    }
    // Player path: same resolution the host uses.
    let player = std::env::var_os("PATH")
        .and_then(|path| {
            std::env::split_paths(&path)
                .map(|dir| dir.join("aplay"))
                .find(|candidate| candidate.is_file())
        })
        .unwrap_or_else(|| std::path::PathBuf::from("aplay"));
    println!("player: {}", player.display());
    let playback = Arc::new(jarvis_tui::voice_playback::PlaybackOwner::new(player, None));
    let worker = jarvis_tui::voice_speech_worker::SpeechWorker::spawn(
        jarvis_tui::voice_conversation::EngineSelection::Neural {
            voice_id: "am_michael".into(),
        },
        Arc::clone(&playback),
    );
    let started = Instant::now();
    worker.enqueue(jarvis_tui::voice_speech_worker::SpeechJob {
        segment: 1,
        text: "Understood. The voice worker is speaking through your speakers now.".into(),
    });
    let deadline = started + Duration::from_secs(120);
    let mut progress_bytes = 0u64;
    let mut spoke = false;
    while Instant::now() < deadline {
        for outcome in worker.drain() {
            match outcome {
                jarvis_tui::voice_speech_worker::SpeechOutcome::Progress {
                    segment: _,
                    through_bytes,
                } => {
                    if progress_bytes == 0 {
                        println!(
                            "[t+{:?}] FIRST bytes handed to player: {through_bytes}",
                            started.elapsed()
                        );
                    }
                    progress_bytes = through_bytes;
                }
                jarvis_tui::voice_speech_worker::SpeechOutcome::Spoke { samples, .. } => {
                    println!(
                        "[t+{:?}] SPOKE: {samples} samples ({progress_bytes} bytes total)",
                        started.elapsed()
                    );
                    spoke = true;
                }
                jarvis_tui::voice_speech_worker::SpeechOutcome::Stale { .. } => {
                    println!("[t+{:?}] STALE (unexpected)", started.elapsed());
                }
                jarvis_tui::voice_speech_worker::SpeechOutcome::Failed { error, .. } => {
                    println!("[t+{:?}] FAILED: {error}", started.elapsed());
                    std::process::exit(3);
                }
            }
        }
        if spoke {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    if !spoke {
        println!("NEVER SPOKE within 120 s — device path broken");
        std::process::exit(4);
    }
    worker.shutdown();
    println!("worker device check OK");
}
