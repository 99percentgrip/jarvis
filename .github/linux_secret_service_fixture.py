"""Run explicit acceptance on an owned bus/home and foreground GNOME daemon."""

import os
from pathlib import Path
import secrets
import subprocess
import sys
import tempfile
import time
from xml.sax.saxutils import escape


def run_private(command):
    if sys.platform != "linux" or not command:
        raise RuntimeError("private Secret Service acceptance requires Linux and a command")
    with tempfile.TemporaryDirectory(prefix="jarvis-native-fixture-") as temporary:
        root = Path(temporary)
        root.chmod(0o700)
        (root / ".native-credential-test-home").write_text("synthetic-only\n")
        environment = dict(os.environ)
        environment["CARGO_HOME"] = os.environ.get("CARGO_HOME", str(Path.home() / ".cargo"))
        environment["RUSTUP_HOME"] = os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup"))
        for key, leaf in [("HOME", "."), ("XDG_CONFIG_HOME", "config"),
                          ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"),
                          ("XDG_STATE_HOME", "state"), ("XDG_RUNTIME_DIR", "run")]:
            directory = root / leaf
            directory.mkdir(mode=0o700, exist_ok=True)
            environment[key] = str(directory)
        for key in ["DBUS_SESSION_BUS_ADDRESS", "GNOME_KEYRING_CONTROL", "SSH_AUTH_SOCK"]:
            environment.pop(key, None)
        environment["NATIVE_SECRET_SERVICE_FIXTURE_HOME"] = str(root)
        config = root / "bus.conf"
        config.write_text('<busconfig><type>session</type><listen>unix:tmpdir=' +
                          escape(str(root / "run")) + '</listen><auth>EXTERNAL</auth>'
                          '<policy context="default"><allow send_destination="*"/>'
                          '<allow receive_sender="*"/><allow own="*"/>'
                          '<allow eavesdrop="true"/></policy></busconfig>')
        result = subprocess.run(["dbus-run-session", "--config-file", str(config), "--",
                                 sys.executable, str(Path(__file__).resolve()),
                                 "--inside-private-bus", *command], env=environment,
                                check=False, timeout=300)
        if result.returncode:
            raise RuntimeError("private native credential acceptance failed")


def inside_private_bus(command):
    root = Path(os.environ["NATIVE_SECRET_SERVICE_FIXTURE_HOME"])
    address = os.environ["DBUS_SESSION_BUS_ADDRESS"]
    if (Path.home() != root or not (root / ".native-credential-test-home").is_file()
            or not address.startswith("unix:path=" + str(root / "run") + "/")):
        raise RuntimeError("refusing a non-fixture Secret Service bus/home")
    environment = dict(os.environ, NATIVE_SECRET_SERVICE_FIXTURE="unlocked-private-bus",
                       NATIVE_SECRET_SERVICE_FIXTURE_BUS=address)
    with (root / "daemon.log").open("w") as log:
        daemon = subprocess.Popen(["gnome-keyring-daemon", "--foreground", "--unlock",
                                   "--components=secrets", "--control-directory", str(root / "control")],
                                  stdin=subprocess.PIPE, stdout=log, stderr=subprocess.STDOUT)
        monitor = None
        try:
            daemon.stdin.write(secrets.token_hex(24).encode())
            daemon.stdin.close()
            deadline = time.monotonic() + 20
            while True:
                result = subprocess.run(["dbus-send", "--session", "--print-reply",
                                         "--dest=org.freedesktop.DBus", "/org/freedesktop/DBus",
                                         "org.freedesktop.DBus.NameHasOwner", "string:org.freedesktop.secrets"],
                                        capture_output=True, text=True, timeout=5, check=True)
                if "boolean true" in result.stdout:
                    break
                if daemon.poll() is not None or time.monotonic() >= deadline:
                    raise RuntimeError("private Secret Service did not become ready")
                time.sleep(0.05)
            with (root / "session-algorithms.log").open("w+") as observation:
                # This filter records only session negotiation, never SetSecret/GetSecrets.
                monitor = subprocess.Popen(["dbus-monitor", "--session",
                    "type='method_call',interface='org.freedesktop.Secret.Service',member='OpenSession'"],
                    stdout=observation, stderr=subprocess.DEVNULL)
                deadline = time.monotonic() + 5
                while "NameAcquired" not in (root / "session-algorithms.log").read_text():
                    if monitor.poll() is not None or time.monotonic() >= deadline:
                        raise RuntimeError("session-negotiation observer did not become ready")
                    time.sleep(0.02)
                subprocess.run(command, env=environment, check=True, timeout=240)
                monitor.terminate()
                monitor.wait(timeout=5)
                observed = (root / "session-algorithms.log").read_text()
                if 'string "dh-ietf1024-sha256-aes128-cbc-pkcs7"' not in observed or 'string "plain"' in observed:
                    raise RuntimeError("native store did not exclusively negotiate encrypted sessions")
                print("real private Secret Service encrypted DH session negotiation verified", flush=True)
        finally:
            if monitor is not None and monitor.poll() is None:
                monitor.terminate()
                monitor.wait(timeout=5)
            if daemon.poll() is None:
                daemon.terminate()
                try:
                    daemon.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    daemon.kill()
                    daemon.wait(timeout=5)


if __name__ == "__main__":
    if sys.argv[1:2] == ["--inside-private-bus"]:
        inside_private_bus(sys.argv[2:])
    else:
        run_private(sys.argv[1:])
