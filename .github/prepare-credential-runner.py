"""Install distro-owned libdbus build/runtime prerequisites on disposable CI only."""

import os
import shutil
import subprocess
import sys


def prepare(environment, native=False):
    if not all(environment.get(k) == v for k, v in {
        "GITHUB_ACTIONS": "true", "RUNNER_OS": "Linux",
        "RUNNER_ENVIRONMENT": "github-hosted",
    }.items()):
        raise RuntimeError("credential preparation requires disposable hosted Linux")
    ready = subprocess.run(["pkg-config", "--exists", "dbus-1"], check=False).returncode == 0
    if not ready or (native and not shutil.which("gnome-keyring-daemon")):
        subprocess.run(["sudo", "--preserve-env=GITHUB_ACTIONS,RUNNER_OS", "python3",
                        ".github/prepare-linux-apt.py"], check=True, timeout=30)
        subprocess.run(["sudo", "apt-get", "update", "-qq"], check=True, timeout=120)
        packages = ["libdbus-1-dev", "pkg-config"]
        if native:
            packages += ["gnome-keyring", "dbus", "dbus-x11"]
        subprocess.run(["sudo", "apt-get", "install", "-y", *packages],
                       check=True, timeout=120)
    subprocess.run(["pkg-config", "--modversion", "dbus-1"], check=True, timeout=10)
    if native:
        for command in ["gnome-keyring-daemon", "dbus-run-session", "dbus-monitor", "dbus-send"]:
            if not shutil.which(command):
                raise RuntimeError(f"missing native credential fixture prerequisite: {command}")


if __name__ == "__main__":
    if sys.argv[1:] not in ([], ["--native"]):
        raise SystemExit("only --native is supported")
    prepare(os.environ, native=sys.argv[1:] == ["--native"])
