//! Non-shipping host for synthetic release-ledger protocol acceptance only.
#![forbid(unsafe_code)]
use std::path::PathBuf;

#[path = "../src/main.rs"]
mod host;

fn main() -> std::io::Result<()> {
    let gate = std::env::var("JARVIS_TEST_DISPATCH_GATE")
        .ok()
        .and_then(|value| value.parse::<std::net::SocketAddr>().ok());
    if !gate.is_some_and(|address| address.ip().is_loopback() && address.port() != 0) {
        return Err(std::io::Error::other(
            "explicit loopback fixture gate required",
        ));
    }
    let cwd = std::env::current_dir()?.canonicalize()?;
    let root = std::env::var_os("JARVIS_RELEASE_ROOT")
        .ok_or_else(|| std::io::Error::other("synthetic ledger root required"))?;
    if PathBuf::from(root).canonicalize()? != cwd.join("release-state") {
        return Err(std::io::Error::other(
            "fixture ledger must stay in its workspace",
        ));
    }
    let remote = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()?;
    if !remote.status.success()
        || String::from_utf8_lossy(&remote.stdout).trim() != "https://github.com/fixture/rrc-hosts"
    {
        return Err(std::io::Error::other(
            "synthetic repository identity required",
        ));
    }
    jarvis_harness::release_executor::enable_permissive_resource_governor_for_process_tests();
    host::main()
}
