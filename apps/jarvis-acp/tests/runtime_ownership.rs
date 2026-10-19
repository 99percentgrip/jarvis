//! Real ordinary-binary ownership boundaries; no provider or native secret calls.
#![allow(dead_code)]
mod support;
use serde_json::json;
use support::ProcessHarness;

#[test]
fn legacy_environment_and_legacy_read_knobs_never_adopt_private_sessions() {
    let fixture = tempfile::tempdir().unwrap();
    let old = fixture.path().join(".agent-vesper");
    std::fs::create_dir(&old).unwrap();
    let canary = old.join("private-session.json");
    let bytes = b"SYNTHETIC-VESPER-PRIVATE-SESSION-NOT-FOR-IMPORT";
    std::fs::write(&canary, bytes).unwrap();
    let current = fixture.path().join("current-sessions");
    jarvis_security::prepare_owned_directory(&current).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut process = ProcessHarness::spawn_with_environment(
        listener.local_addr().unwrap(),
        [
            ("AGENT_VESPER_HOME", old.display().to_string()),
            ("AGENT_VESPER_SESSION_ROOT", old.display().to_string()),
            ("AGENT_VESPER_ENABLE_LEGACY_SESSION_READS", "1".into()),
            ("JARVIS_ENABLE_SESSION_READS", "1".into()),
            ("JARVIS_SESSION_ROOT", current.display().to_string()),
            ("JARVIS_ENABLE_LEGACY_SESSION_READS", "1".into()),
            ("JARVIS_LEGACY_SESSION_ROOT", old.display().to_string()),
        ],
    );
    let session = process.initialize_and_new_session();
    process.send(json!({"jsonrpc":"2.0","id":12,"method":"session/list","params":{}}));
    let listed = process.response(12);
    assert!(listed.get("error").is_none(), "{listed}");
    let sessions = listed["result"]["sessions"].as_array().unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0]["sessionId"], session);
    assert!(
        !serde_json::to_string(process.transcript())
            .unwrap()
            .contains("SYNTHETIC-VESPER")
    );
    process.finish();
    assert_eq!(std::fs::read(&canary).unwrap(), bytes);
    assert_eq!(std::fs::read_dir(&old).unwrap().count(), 1);
    assert!(
        listener.accept().is_err(),
        "read-only discovery dispatched a provider"
    );
}

#[test]
fn explicit_private_root_does_not_authorize_unowned_state_adoption() {
    let fixture = tempfile::tempdir().unwrap();
    for name in [".agent-vesper", "unmarked-private-store"] {
        let old = fixture.path().join(name);
        std::fs::create_dir(&old).unwrap();
        let canary = old.join("private.json");
        std::fs::write(&canary, b"synthetic unowned state").unwrap();
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_jarvis-acp"));
        command
            .env_clear()
            .current_dir(fixture.path())
            .env("HOME", fixture.path())
            .env("USERPROFILE", fixture.path())
            .env("APPDATA", fixture.path().join("roaming"))
            .env("LOCALAPPDATA", fixture.path().join("local"))
            .env("XDG_CONFIG_HOME", fixture.path().join("config"))
            .env("JARVIS_SESSION_ROOT", &old);
        for key in ["SystemRoot", "WINDIR", "COMSPEC", "PATH"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        let output = command.output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("ownership refused"));
        assert_eq!(std::fs::read(&canary).unwrap(), b"synthetic unowned state");
        assert_eq!(std::fs::read_dir(&old).unwrap().count(), 1);
        assert!(!fixture.path().join("config").exists());
    }
}
