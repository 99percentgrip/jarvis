//! Bounded diagnostic audio for Alex's listening matrix (mission §8):
//! the six word cells + both full fixtures, both voices, fixed build.
//! ≤8 MiB/file, ≤32 MiB aggregate, owned diagnostic directory under the native application cache.
#![allow(missing_docs)]
fn main() {
    let espeak: std::path::PathBuf = std::env::var_os("PATH")
        .and_then(|p| {
            std::env::split_paths(&p)
                .map(|d| d.join("espeak-ng"))
                .find(|c| c.is_file())
        })
        .expect("espeak-ng on PATH");
    let pack_root = jarvis_voice_kokoro::pack_root();
    let output_root = jarvis_config::JarvisPaths::from_environment()
        .expect("native private roots")
        .cache
        .join("voice-diagnostics")
        .join("pron-ab");
    jarvis_security::prepare_owned_directory(&output_root).expect("owned diagnostic output");
    let engine =
        match jarvis_voice_kokoro::engine::EngineState::load(&pack_root).expect("pack loads") {
            jarvis_voice_kokoro::engine::EngineState::Ready(engine) => engine,
            _ => panic!("pack not ready"),
        };
    let cells = [
        (
            "reply",
            "Here is my one sentence reply to the short test, sent without using any tool.",
        ),
        (
            "ready",
            "Replying fast now. I'm here ready, and this instant answer completes your long test.",
        ),
    ];
    let mut total = 0u64;
    let mut peak = 0u64;
    for (voice, _) in [("am_michael", 0), ("af_heart", 0)] {
        let style =
            jarvis_voice_kokoro::engine::StylePack::load(&pack_root, voice).expect("voice style");
        for (tag, text) in cells.iter().chain(
            [
                ("reply", "Please reply now."),
                ("ready", "I am ready now."),
                ("now", "The reply is ready."),
            ]
            .iter(),
        ) {
            let output = engine
                .synthesize_blocking(text, &style, &espeak)
                .expect("synthesis");
            let name = format!("{voice}_{tag}_{}", text.len());
            let path = output_root.join(format!("{name}.raw"));
            jarvis_security::validate_private_path(&path).expect("private diagnostic file");
            let bytes: Vec<u8> = output
                .frames
                .iter()
                .flat_map(|f| f.bytes().iter().copied())
                .collect();
            let len = bytes.len() as u64;
            assert!(len <= 8 * 1024 * 1024, "per-file cap");
            total += len;
            peak = peak.max(total);
            assert!(total <= 32 * 1024 * 1024, "aggregate cap");
            std::fs::write(&path, &bytes).unwrap();
            println!(
                "{voice}/{tag} ({} chars): {} bytes 16k-s16 -> {:?}",
                text.chars().count(),
                len,
                path
            );
        }
    }
    println!(
        "peak {} bytes, aggregate {} bytes (caps 8MiB/32MiB)",
        peak, total
    );
}
