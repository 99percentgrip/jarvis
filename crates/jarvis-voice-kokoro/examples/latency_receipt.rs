//! Explicit installed-pack latency measurement; no audio devices or downloads.
#[cfg(not(feature = "ort"))]
fn main() {
    eprintln!("requires --features ort");
}
#[cfg(feature = "ort")]
fn main() {
    use jarvis_voice_kokoro::{EngineState, StylePack};
    use std::time::Instant;
    let root = jarvis_voice_kokoro::pack_root();
    for i in 0..3 {
        let start = Instant::now();
        assert!(jarvis_voice_kokoro::assess_pack(&root).unwrap().is_none());
        println!(
            "assessment {i}: {:.3} ms",
            start.elapsed().as_secs_f64() * 1000.0
        );
    }
    let start = Instant::now();
    let EngineState::Ready(engine) = EngineState::load(&root).unwrap() else {
        panic!("not ready")
    };
    println!(
        "engine load: {:.3} ms",
        start.elapsed().as_secs_f64() * 1000.0
    );
    let voice = StylePack::load(&root, "am_michael").unwrap();
    let phonemizer = jarvis_voice_kokoro::resolve_phonemizer().unwrap();
    for i in 0..3 {
        let start = Instant::now();
        let output = engine
            .synthesize_blocking(
                "This is a preview of the selected voice.",
                &voice,
                &phonemizer,
            )
            .unwrap();
        println!(
            "synthesis {i}: {:.3} ms, {} samples",
            start.elapsed().as_secs_f64() * 1000.0,
            output
                .frames
                .iter()
                .map(|f| f.sample_count())
                .sum::<usize>()
        );
    }
}
