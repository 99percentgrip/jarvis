use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::{Arc, Condvar, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

use serde_json::{Value, json};

mod credentials;
#[allow(unused_imports)]
pub use credentials::set_signed_out_xai_credentials;

pub const CANARY: &str = "vesper-stage41-secret-canary";
// Per-operation timeout for receiving one JSON-RPC line from the spawned
// agent process. The agent normally responds in well under 1 second, but
// when `cargo test --workspace` runs every test binary in parallel on a
// shared CI runner, process startup and IPC can briefly exceed 10 seconds
// under contention. 30 seconds gives ~3x headroom for that contention
// while still failing fast enough that a genuinely-hung process is
// surfaced in a reasonable test cycle. (The dedicated serial
// `process_blockers -- --test-threads=1` step rarely approaches this
// ceiling because it has no parallel contention.)
const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct ReaderGate(Arc<(Mutex<bool>, Condvar)>);

impl ReaderGate {
    pub fn running() -> Self {
        Self(Arc::new((Mutex::new(true), Condvar::new())))
    }

    pub fn pause(&self) {
        *self.0.0.lock().unwrap() = false;
    }

    pub fn resume(&self) {
        *self.0.0.lock().unwrap() = true;
        self.0.1.notify_all();
    }

    fn wait(&self) {
        let (lock, ready) = &*self.0;
        let mut running = lock.lock().unwrap();
        while !*running {
            running = ready.wait(running).unwrap();
        }
    }
}

pub struct ProcessHarness {
    child: Child,
    stdin: Option<ChildStdin>,
    lines: mpsc::Receiver<String>,
    transcript: Vec<Value>,
    pending_responses: BTreeMap<String, Value>,
    expected_response: Option<u64>,
    reader_gate: ReaderGate,
    reader: Option<thread::JoinHandle<()>>,
    stderr: Option<thread::JoinHandle<String>>,
    temp: PathBuf,
}

impl ProcessHarness {
    pub fn spawn(address: std::net::SocketAddr) -> Self {
        Self::spawn_binary(
            env!("CARGO_BIN_EXE_jarvis-acp"),
            address,
            std::iter::empty::<(&str, String)>(),
        )
    }

    pub fn spawn_test_driver(address: std::net::SocketAddr, gate: std::net::SocketAddr) -> Self {
        Self::spawn_binary(
            env!("CARGO_BIN_EXE_jarvis-acp-test-driver"),
            address,
            [
                ("JARVIS_TEST_DISPATCH_GATE", gate.to_string()),
                ("JARVIS_FULL_HARNESS", "0".to_owned()),
            ],
        )
    }

    /// Starts the integration-only driver with the explicit permissive RRC
    /// resource policy. It uses deterministic test-only resource telemetry so
    /// shared-runner pressure cannot control process-lifecycle acceptance. The
    /// supplied loopback address is deliberately also the unused provider gate,
    /// so a release/status-only test can prove it made no provider connection.
    #[allow(dead_code)] // Used only by the separately compiled governor process test.
    pub fn spawn_resource_governor_test_driver(
        address: std::net::SocketAddr,
        extra: impl IntoIterator<Item = (&'static str, String)>,
    ) -> Self {
        Self::spawn_binary(
            env!("CARGO_BIN_EXE_jarvis-acp-test-driver"),
            address,
            [
                ("JARVIS_TEST_DISPATCH_GATE", address.to_string()),
                ("JARVIS_FULL_HARNESS", "1".to_owned()),
            ]
            .into_iter()
            .chain(extra),
        )
    }

    pub fn spawn_with_environment(
        address: std::net::SocketAddr,
        extra: impl IntoIterator<Item = (&'static str, String)>,
    ) -> Self {
        Self::spawn_binary(env!("CARGO_BIN_EXE_jarvis-acp"), address, extra)
    }

    pub fn spawn_legacy_format_fixture(
        address: std::net::SocketAddr,
        extra: impl IntoIterator<Item = (&'static str, String)>,
    ) -> Self {
        Self::spawn_binary(
            env!("CARGO_BIN_EXE_jarvis-acp-test-driver"),
            address,
            [
                ("JARVIS_TEST_DISPATCH_GATE", address.to_string()),
                ("JARVIS_FULL_HARNESS", "0".to_owned()),
            ]
            .into_iter()
            .chain(extra),
        )
    }

    fn spawn_binary(
        binary: &str,
        address: std::net::SocketAddr,
        extra: impl IntoIterator<Item = (&'static str, String)>,
    ) -> Self {
        let temp = std::env::temp_dir().join(format!(
            "agent-vesper-stage41-{}-{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&temp).unwrap();
        // HOME/env isolation does not isolate OS credential managers. An explicit
        // signed-out Vesper record prevents inactive OpenAI discovery from seeing
        // host credentials; native OpenAI fixtures use their synthetic loopback route.
        let openai_fixture = temp.join("openai-signed-out.json");
        fs::write(
            &openai_fixture,
            serde_json::to_vec(&serde_json::json!({
                "application": "99percentgrip.jarvis", "credentials": {
                    "openai": {"native-auth": "{\"mode\":\"signed-out\"}"},
                    "xai": {"native-auth": "{\"mode\":\"signed-out\"}"}
                }
            }))
            .unwrap(),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&openai_fixture, fs::Permissions::from_mode(0o600)).unwrap();
        }
        let mut command = Command::new(binary);
        command
            // The host resolves workspace settings from its cwd; running
            // inside the isolated temp root keeps that read hermetic (the
            // AT-01 lane previously depended on the *test runner's* cwd,
            // which made its enabled-settings assertion vacuous).
            .current_dir(&temp)
            .env_clear()
            .env("HOME", &temp)
            .env("JARVIS_OPENAI_CREDENTIALS_PATH", &openai_fixture)
            .env("JARVIS_XAI_CREDENTIALS_PATH", &openai_fixture)
            .env("XDG_CONFIG_HOME", temp.join("config"))
            .env("XDG_CACHE_HOME", temp.join("cache"))
            .env("XDG_DATA_HOME", temp.join("data"))
            .env("XDG_STATE_HOME", temp.join("state"))
            .env("ZAI_API_KEY", CANARY)
            .env("JARVIS_GLM_BASE_URL", format!("http://{address}/v4"))
            .env("JARVIS_ALLOW_INSECURE_LOOPBACK", "1")
            .env("JARVIS_FULL_HARNESS", "0")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // `env_clear()` nukes the parent environment for isolation, but the
        // spawned `agent-vesper-acp` binary still needs a small set of
        // platform-critical env vars to function. Without them:
        //   * Windows: Winsock (`WSAStartup`) and DLL loading require
        //     `SystemRoot` / `SystemDrive` / `windir`; stripping them makes
        //     the binary hang forever on its first TCP operation, which
        //     reproduces as the integration-test hang seen on windows-2025.
        //   * macOS: PATH and TMPDIR are needed for any subprocess the
        //     runtime spawns and for sane temp-dir resolution.
        //   * Linux: PATH is needed if the runtime shells out for anything
        //     during the test (defensive; Linux currently passes either way).
        // None of these expose the user's real `ZAI_API_KEY` or any
        // `AGENT_VESPER_*` configuration; isolation of those secrets is
        // preserved because they are set explicitly above after env_clear.
        restore_isolated_environment(&mut command);
        for (key, value) in extra {
            command.env(key, value);
        }
        let mut child = command.spawn().unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let child_stderr = child.stderr.take().unwrap();
        let gate = ReaderGate::running();
        let reader_gate = gate.clone();
        let (sender, lines) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                reader_gate.wait();
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let _ = sender.send(line.trim_end().to_owned());
                    }
                }
            }
        });
        let stderr = thread::spawn(move || {
            let mut output = String::new();
            let _ = BufReader::new(child_stderr).read_to_string(&mut output);
            output
        });
        Self {
            child,
            stdin: Some(stdin),
            lines,
            transcript: Vec::new(),
            pending_responses: BTreeMap::new(),
            expected_response: None,
            reader_gate: gate,
            reader: Some(reader),
            stderr: Some(stderr),
            temp,
        }
    }

    pub fn initialize_and_new_session(&mut self) -> String {
        self.send(
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}),
        );
        assert_eq!(self.response(1)["result"]["protocolVersion"], 1);
        self.send(json!({"jsonrpc":"2.0","id":2,"method":"session/new","params":{"cwd":self.temp.to_string_lossy(),"mcpServers":[]}}));
        self.response(2)["result"]["sessionId"]
            .as_str()
            .unwrap()
            .to_owned()
    }

    #[allow(dead_code)] // Used only by the separately compiled governor process test.
    pub fn initialize_and_new_session_in(&mut self, cwd: &std::path::Path) -> String {
        self.send(
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}),
        );
        assert_eq!(self.response(1)["result"]["protocolVersion"], 1);
        self.send(json!({
            "jsonrpc":"2.0","id":2,"method":"session/new",
            "params":{"cwd":cwd.to_string_lossy(),"mcpServers":[]}
        }));
        self.response(2)["result"]["sessionId"]
            .as_str()
            .unwrap()
            .to_owned()
    }

    pub fn send(&mut self, value: Value) {
        let stdin = self.stdin.as_mut().unwrap();
        serde_json::to_writer(&mut *stdin, &value).unwrap();
        stdin.write_all(b"\n").unwrap();
        stdin.flush().unwrap();
    }

    pub fn prompt(&mut self, id: u64, session: &str, text: &str, message_id: &str) {
        self.send(json!({
            "jsonrpc":"2.0","id":id,"method":"session/prompt",
            "params":{"sessionId":session,"prompt":[{"type":"text","text":text}],
            "_meta":{"userMessageId":message_id}}
        }));
    }

    pub fn response(&mut self, id: u64) -> Value {
        let key = id.to_string();
        if let Some(value) = self.pending_responses.remove(&key) {
            return value;
        }
        self.expected_response = Some(id);
        loop {
            let value = self.next();
            if value["id"] == id {
                self.expected_response = None;
                return value;
            }
            if !value["id"].is_null() {
                self.pending_responses
                    .insert(value["id"].to_string(), value);
            }
        }
    }

    pub fn next(&mut self) -> Value {
        let expected_response = self.expected_response;
        let line = self.lines.recv_timeout(TIMEOUT).unwrap_or_else(|error| {
            let status = self.child.try_wait().ok().flatten();
            panic!(
                "ACP receive failed: {error}; expected_response={expected_response:?}; child_status={status:?}; events={}; recent_shapes={:?}",
                self.transcript.len(), protocol_diagnostic(&self.transcript)
            )
        });
        assert!(!line.contains(CANARY), "secret reached ACP stdout");
        let value: Value = serde_json::from_str(&line).expect("stdout contained non-JSON text");
        self.transcript.push(value.clone());
        value
    }

    pub fn transcript(&self) -> &[Value] {
        &self.transcript
    }

    pub fn pause_stdout(&self) {
        self.reader_gate.pause();
    }

    pub fn resume_stdout(&self) {
        self.reader_gate.resume();
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn isolated_root(&self) -> &std::path::Path {
        &self.temp
    }

    pub fn finish(self) {
        let _ = self.finish_and_capture();
    }

    pub fn finish_and_capture(mut self) -> (Vec<Value>, String) {
        self.stdin.take();
        self.reader_gate.resume();
        wait_for_exit(&mut self.child);
        if let Some(reader) = self.reader.take() {
            reader.join().unwrap();
        }
        let stderr = self.stderr.take().unwrap().join().unwrap();
        assert!(!stderr.contains(CANARY), "secret reached ACP stderr");
        let _ = fs::remove_dir_all(&self.temp);
        (std::mem::take(&mut self.transcript), stderr)
    }
}

/// Diagnostic allowlist: never print prompt/tool content, credentials or arbitrary fields.
pub fn protocol_diagnostic(values: &[Value]) -> Vec<Value> {
    values
        .iter()
        .rev()
        .take(12)
        .map(|value| {
            let update = &value["params"]["update"];
            let kind = match update["sessionUpdate"].as_str() {
                Some("tool_call") => "tool_call",
                Some("tool_call_update") => "tool_call_update",
                Some("agent_message_chunk") => "agent_message_chunk",
                Some("agent_thought_chunk") => "agent_thought_chunk",
                Some("available_commands_update") => "available_commands_update",
                _ => "other",
            };
            let status = match update["status"].as_str() {
                Some("pending") => "pending",
                Some("in_progress") => "in_progress",
                Some("completed") => "completed",
                Some("failed") => "failed",
                _ => "other",
            };
            let method = match value["method"].as_str() {
                Some("session/update") => "session/update",
                Some("session/request_permission") => "session/request_permission",
                _ => "other",
            };
            json!({"response_id": value["id"].as_u64(), "has_error": value.get("error").is_some(),
               "rpc_method": method, "session_update": kind, "tool_status": status})
        })
        .collect()
}

impl Drop for ProcessHarness {
    fn drop(&mut self) {
        self.reader_gate.resume();
        self.stdin.take();
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(stderr) = self.stderr.take() {
            let _ = stderr.join();
        }
        let _ = fs::remove_dir_all(&self.temp);
    }
}

pub fn read_http_request(stream: &mut impl Read) -> String {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let read = stream.read(&mut buffer).unwrap();
        assert!(read > 0, "HTTP request ended before headers");
        bytes.extend_from_slice(&buffer[..read]);
        let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let header_end = header_end + 4;
        let headers = String::from_utf8_lossy(&bytes[..header_end]).to_lowercase();
        let content_length = headers
            .lines()
            .find_map(|line| line.strip_prefix("content-length: "))
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        if bytes.len() >= header_end + content_length {
            return String::from_utf8_lossy(&bytes[..header_end + content_length]).into_owned();
        }
    }
}

pub fn write_sse(stream: &mut impl Write, body: &str) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        body.len(),
        body
    )
    .unwrap();
    stream.flush().unwrap();
}

pub fn successful_body(content: &str) -> String {
    format!(
        "data: {{\"choices\":[{{\"delta\":{{\"content\":{}}},\"finish_reason\":\"stop\"}}],\"usage\":{{\"prompt_tokens\":2,\"completion_tokens\":3,\"total_tokens\":5}}}}\n\ndata: [DONE]\n\n",
        serde_json::to_string(content).unwrap()
    )
}

pub fn terminal_count(values: &[Value], id: u64) -> usize {
    values.iter().filter(|value| value["id"] == id).count()
}

pub fn update_texts(values: &[Value], kind: &str) -> Vec<String> {
    values
        .iter()
        .filter(|value| value["params"]["update"]["sessionUpdate"] == kind)
        .filter_map(|value| {
            value["params"]["update"]["content"]["text"]
                .as_str()
                .map(str::to_owned)
        })
        .collect()
}

#[cfg(target_os = "linux")]
pub fn rss_kib(pid: u32) -> u64 {
    fs::read_to_string(format!("/proc/{pid}/status"))
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix("VmRSS:"))
        .and_then(|value| value.split_whitespace().next())
        .unwrap()
        .parse()
        .unwrap()
}

fn wait_for_exit(child: &mut Child) {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(status.success());
            return;
        }
        assert!(Instant::now() < deadline, "ACP process did not exit on EOF");
        thread::sleep(Duration::from_millis(10));
    }
}

fn unique_suffix() -> u128 {
    // Combine wall-clock nanoseconds with a monotonic per-process counter.
    // On Apple Silicon the clock has nanosecond resolution, but two parallel
    // test threads can still observe the same nanosecond tick; the counter
    // guarantees every ProcessHarness temp dir is distinct even under that
    // race, which prevents cross-test isolation-root collisions when the
    // workspace test harness runs process_blockers in parallel.
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    nanos.wrapping_add(seq as u128)
}

/// Preserve OS execution prerequisites without inheriting real private runtime roots.
#[allow(dead_code)] // Shared across separately compiled integration test modules.
pub fn restore_isolated_environment(command: &mut Command) {
    let home = command
        .get_envs()
        .find(|(key, _)| *key == "HOME")
        .and_then(|(_, value)| value)
        .map(PathBuf::from)
        .or_else(|| command.get_current_dir().map(PathBuf::from))
        .expect("process fixture requires an explicit synthetic home or cwd");
    assert!(
        home.is_absolute(),
        "fixture home must be native and absolute"
    );
    for key in critical_environment_keys() {
        if let Ok(value) = std::env::var(key) {
            command.env(key, value);
        }
    }
    command
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("APPDATA", home.join("roaming"))
        .env("LOCALAPPDATA", home.join("local"));
}

/// OS execution inputs only; application-private roots are always synthetic.
pub fn critical_environment_keys() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        &[
            "SystemRoot",
            "SystemDrive",
            "windir",
            "TEMP",
            "TMP",
            "PATHEXT",
            "PATH",
            "PROGRAMDATA",
            "COMSPEC",
        ]
    } else if cfg!(target_os = "macos") {
        &[
            "PATH", "TMPDIR", "LANG", "LC_ALL", "LC_CTYPE", "USER", "LOGNAME", "SHELL", "TZ",
        ]
    } else {
        &[
            "PATH", "LANG", "LC_ALL", "LC_CTYPE", "USER", "LOGNAME", "SHELL", "TZ", "TMPDIR",
        ]
    }
}

/// Keeps native OS credential managers out of independent process fixtures too.
#[allow(dead_code)] // Consumed by separate process-test binaries sharing this module.
pub fn signed_out_openai_fixture() -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    serde_json::to_writer(
        &mut file,
        &serde_json::json!({
            "application": "99percentgrip.jarvis", "credentials": {"openai": {"native-auth": "{\"mode\":\"signed-out\"}"}}
        }),
    )
    .unwrap();
    file
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn timeout_diagnostic_bounds_shapes_and_excludes_all_content() {
        let secret = "synthetic-diagnostic-secret-must-not-appear";
        let mut events = vec![
            json!({"id": 9, "params": {"update": {
            "sessionUpdate": "tool_call_update", "status": "in_progress",
            "content": secret, "rawOutput": secret, "title": secret}}, "error": {"message": secret}});
            20
        ];
        events.push(json!({"id": secret, "params": {"update": {"sessionUpdate": secret, "status": secret}}}));
        let observed = protocol_diagnostic(&events);
        assert_eq!(observed.len(), 12);
        let encoded = serde_json::to_string(&observed).unwrap();
        assert!(!encoded.contains(secret));
        assert!(encoded.contains("in_progress"));
        assert_eq!(observed[0]["response_id"], Value::Null);
        assert_eq!(observed[0]["session_update"], "other");
    }
}
