#!/usr/bin/env python3
"""Pack the local native build and install it into a disposable root.

This does not publish artifacts or contact a release endpoint.
"""
from __future__ import annotations

import os
import platform
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MAP = {
    "linux-x86_64": "linux-x86_64",
    "linux-arm64": "linux-aarch64",
    "linux-aarch64": "linux-aarch64",
    "macos-intel": "darwin-x86_64",
    "macos-apple-silicon": "darwin-aarch64",
    "windows-x86_64": "windows-x86_64",
}


def target_name() -> str:
    override = os.environ.get("JARVIS_CI_TARGET", "").strip()
    if override:
        if override not in MAP:
            raise SystemExit(f"unsupported CI target {override}")
        return MAP[override]
    system = platform.system()
    machine = platform.machine().lower()
    if system == "Linux" and machine in {"x86_64", "amd64"}:
        return "linux-x86_64"
    if system == "Linux" and machine in {"aarch64", "arm64"}:
        return "linux-aarch64"
    if system == "Darwin" and machine == "x86_64":
        return "darwin-x86_64"
    if system == "Darwin" and machine in {"arm64", "aarch64"}:
        return "darwin-aarch64"
    if system == "Windows":
        return "windows-x86_64"
    raise SystemExit(f"unsupported native target {system} {machine}")


def main() -> None:
    target = target_name()
    suffix = ".exe" if target.startswith("windows") else ""
    bindir = ROOT / "target" / "debug"
    required = [f"jarvis-acp{suffix}", f"jarvis-tui{suffix}", f"jarvis-web-fetch{suffix}", f"jarvis-sandbox-init{suffix}"]
    missing = [name for name in required if not (bindir / name).is_file()]
    if missing:
        raise SystemExit(f"native package inputs missing: {missing}")
    if target.startswith("windows"):
        subprocess.run(
            ["powershell.exe", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(ROOT / "scripts" / "test_install_windows_release.ps1"), "-BinaryDir", str(bindir)],
            check=True,
        )
        print("PASS native Windows offline package install")
        return
    version = "0.24.14"
    for line in (ROOT / "Cargo.toml").read_text(encoding="utf-8").splitlines():
        if line.startswith("version = "):
            version = line.split('"')[1]
            break
    commit = os.environ.get("GITHUB_SHA", "")
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        output = home / ("jarvis-windows-x86_64.zip" if target.startswith("windows") else {
            "linux-x86_64": "jarvis-linux-x86_64.tar.gz",
            "linux-aarch64": "jarvis-linux-aarch64.tar.gz",
            "darwin-x86_64": "jarvis-darwin-x86_64.tar.gz",
            "darwin-aarch64": "jarvis-darwin-aarch64.tar.gz",
        }[target])
        resources = home / "resources" / "web-driver"
        resources.mkdir(parents=True)
        image = resources / "image.tar.gz"
        image.write_bytes(b"offline-driver-fixture")
        import hashlib
        digest_image = hashlib.sha256(image.read_bytes()).hexdigest()
        (resources / "image.sha256").write_text(digest_image + "  image.tar.gz\n", encoding="utf-8")
        (resources / "image-id").write_text("sha256:" + "a" * 64 + "\n", encoding="utf-8")
        pack = subprocess.run(
            ["python3", str(ROOT / "scripts" / "jarvis_package.py"), "pack", "--bin-dir", str(bindir), "--target", target, "--output", str(output), "--version", version, "--commit", commit, "--resources", str(home / "resources")],
            check=True, text=True, capture_output=True,
        )
        digest = pack.stdout.strip().splitlines()[-1]
        env = os.environ.copy()
        env.update({
            "JARVIS_HOME": str(home),
            "JARVIS_INSTALL_ROOT": str(home / "install"),
            "JARVIS_LAUNCHER_DIR": str(home / "bin"),
            "JARVIS_MEMORY_ROOT": str(home / "memory"),
            "JARVIS_SHELL_PROFILE": str(home / "profile"),
            "JARVIS_TARGET": target,
            "JARVIS_PACKAGE": str(output),
            "JARVIS_PACKAGE_SHA256": digest,
        })
        (home / "memory").mkdir()
        (home / "memory" / "keep.txt").write_text("keep\n", encoding="utf-8")
        subprocess.run(["sh", str(ROOT / "scripts" / "install.sh")], check=True, env=env)
        for name in ("jarvis-acp", "jarvis-tui"):
            subprocess.run([str(home / "bin" / name), "--version"], check=True, env=env)
        current = (home / "install" / "current").read_text(encoding="utf-8").strip()
        driver = home / "install" / "generations" / current / "web-driver" / "image.tar.gz"
        if driver.read_bytes() != b"offline-driver-fixture":
            raise SystemExit("bundled driver resource was not installed")
        subprocess.run(["sh", str(ROOT / "scripts" / "uninstall.sh")], check=True, env=env)
        if (home / "memory" / "keep.txt").read_text(encoding="utf-8") != "keep\n":
            raise SystemExit("uninstall removed memory")
        if (home / "bin" / "jarvis-acp").exists():
            raise SystemExit("uninstall left a launcher")
    print(f"PASS native offline package install for {target}")


if __name__ == "__main__":
    main()
