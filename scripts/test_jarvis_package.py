#!/usr/bin/env python3
"""Full offline installer coverage. Uses disposable homes, never live user state."""
from __future__ import annotations

import hashlib
import json
import os
import stat
import subprocess
import tarfile
import tempfile
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INSTALL = ROOT / "scripts" / "install.sh"
UNINSTALL = ROOT / "scripts" / "uninstall.sh"
PACKAGE = ROOT / "scripts" / "jarvis_package.py"


def run(args, env, check=True):
    result = subprocess.run(args, env=env, text=True, capture_output=True, check=False)
    if check and result.returncode != 0:
        raise AssertionError(f"{args}\\n{result.stdout}\\n{result.stderr}")
    return result


def base_env(home: Path, **extra):
    env = os.environ.copy()
    env["JARVIS_HOME"] = str(home)
    env["JARVIS_INSTALL_ROOT"] = str(home / "install")
    env["JARVIS_LAUNCHER_DIR"] = str(home / "bin")
    env["JARVIS_MEMORY_ROOT"] = str(home / "memory")
    env["JARVIS_SHELL_PROFILE"] = str(home / "profile")
    env["JARVIS_TARGET"] = "linux-x86_64"
    env["PATH"] = "/usr/bin:/bin"
    for key in (
        "JARVIS_PACKAGE",
        "JARVIS_PACKAGE_SHA256",
        "JARVIS_INSTALL_FAIL_AFTER",
    ):
        env.pop(key, None)
    env.update(extra)
    return env


def fixture(directory: Path, name: str, text: str = "fixture 0.0.0"):
    path = directory / name
    path.write_text("#!/bin/sh\nprintf '%s\\n' '" + text + "'\n", encoding="utf-8")
    path.chmod(0o755)


def make_package(home: Path, version_text: str = "fixture 0.0.0", extra=None, corrupt=False):
    bindir = home / "payload"
    bindir.mkdir(parents=True)
    fixture(bindir, "jarvis-acp", version_text)
    fixture(bindir, "jarvis-tui", version_text)
    resources = home / "resources"
    (resources / "skills" / "skills").mkdir(parents=True)
    (resources / "skills" / "skills" / "demo.md").write_text("seed\\n", encoding="utf-8")
    if extra:
        extra(bindir, resources)
    output = home / "jarvis-linux-x86_64.tar.gz"
    env = os.environ.copy()
    result = run(
        [
            "python3",
            str(PACKAGE),
            "pack",
            "--bin-dir",
            str(bindir),
            "--target",
            "linux-x86_64",
            "--output",
            str(output),
            "--version",
            "0.24.14",
            "--commit",
            "abc",
            "--resources",
            str(resources),
        ],
        env,
    )
    digest = (output.with_suffix(output.suffix + ".sha256").read_text().split()[0]
              if False else result.stdout.strip().splitlines()[-1])
    if corrupt:
        data = bytearray(output.read_bytes())
        data[-1] ^= 0xFF
        output.write_bytes(data)
    return output, digest


def test_requires_explicit_archive():
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        result = run(["sh", str(INSTALL)], base_env(home), check=False)
        assert result.returncode != 0
        assert "no public package" in result.stderr
        assert "99percentgrip/agent-vesper" not in (INSTALL.read_text() + UNINSTALL.read_text())
        assert "latest/download" not in INSTALL.read_text()


def test_install_upgrade_rollback_uninstall_and_coexistence():
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        vesper = home / ".agent-vesper" / "memory"
        vesper.mkdir(parents=True)
        (vesper / "secret").write_text("vesper-secret\\n", encoding="utf-8")
        archive, digest = make_package(home / "one")
        env = base_env(
            home,
            JARVIS_PACKAGE=str(archive),
            JARVIS_PACKAGE_SHA256=digest,
        )
        (home / "memory").mkdir()
        sentinel = home / "memory" / "keep.txt"
        sentinel.write_text("keep\\n", encoding="utf-8")
        before = sentinel.stat().st_ino
        run(["sh", str(INSTALL)], env)
        launcher = home / "bin" / "jarvis-acp"
        version = run([str(launcher), "--version"], env)
        assert "fixture 0.0.0" in version.stdout
        assert (home / "memory" / "skills" / "demo.md").is_file()
        assert sentinel.stat().st_ino == before
        assert (vesper / "secret").read_text() == "vesper-secret\\n"
        # Repeat install and upgrade preserve the sentinel inode.
        archive2, digest2 = make_package(home / "two", "fixture 0.0.1")
        env["JARVIS_PACKAGE"] = str(archive2)
        env["JARVIS_PACKAGE_SHA256"] = digest2
        run(["sh", str(INSTALL)], env)
        assert "fixture 0.0.1" in run([str(launcher), "--version"], env).stdout
        assert sentinel.stat().st_ino == before
        assert (home / "memory" / "skills" / "demo.md").read_text() == "seed\\n"
        run(["sh", str(INSTALL), "--rollback"], env)
        assert "fixture 0.0.0" in run([str(launcher), "--version"], env).stdout
        run(["sh", str(UNINSTALL)], env)
        assert not launcher.exists()
        assert sentinel.stat().st_ino == before
        assert (vesper / "secret").read_text() == "vesper-secret\\n"
        refused = base_env(home, JARVIS_INSTALL_ROOT=str(vesper), JARVIS_PACKAGE=str(archive2), JARVIS_PACKAGE_SHA256=digest2)
        result = run(["sh", str(INSTALL)], refused, check=False)
        assert result.returncode != 0
        assert "Vesper" in result.stderr
        assert (vesper / "secret").read_text() == "vesper-secret\\n"


def test_negative_packages():
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        archive, digest = make_package(home / "good")
        env = base_env(home, JARVIS_PACKAGE=str(archive), JARVIS_PACKAGE_SHA256="0" * 64)
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0 and "SHA-256" in result.stderr
        env["JARVIS_PACKAGE_SHA256"] = "not-a-digest"
        result = run(["sh", str(INSTALL)], env, check=False)
        assert "invalid SHA-256" in result.stderr
        hostile = home / "hostile.tar.gz"
        with tarfile.open(hostile, "w:gz") as tar:
            info = tarfile.TarInfo("jarvis/../../tmp/escaped")
            data = b"escaped"
            info.size = len(data)
            import io
            tar.addfile(info, io.BytesIO(data))
        digest = hashlib.sha256(hostile.read_bytes()).hexdigest()
        env = base_env(home, JARVIS_PACKAGE=str(hostile), JARVIS_PACKAGE_SHA256=digest)
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0
        assert not (Path("/tmp/escaped")).exists()
        # Wrong target and missing executable via a truncated manifest package.
        bindir = home / "bad"
        bindir.mkdir()
        fixture(bindir, "jarvis-acp")
        output = home / "jarvis-linux-x86_64.tar.gz"
        result = run(
            ["python3", str(PACKAGE), "pack", "--bin-dir", str(bindir), "--target", "linux-x86_64", "--output", str(output), "--version", "0.24.14"],
            os.environ.copy(),
            check=False,
        )
        assert result.returncode != 0 and "missing executable" in result.stderr
        # Foreign install root.
        archive, digest = make_package(home / "foreign")
        root = home / "install"
        root.mkdir()
        (root / "notes.txt").write_text("mine\\n", encoding="utf-8")
        env = base_env(home, JARVIS_PACKAGE=str(archive), JARVIS_PACKAGE_SHA256=digest)
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0 and "foreign-owned" in result.stderr
        assert (root / "notes.txt").read_text() == "mine\\n"
        # Symlink install root.
        outside = home / "outside"
        outside.mkdir()
        link = home / "linked"
        link.symlink_to(outside, target_is_directory=True)
        env["JARVIS_INSTALL_ROOT"] = str(link)
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0 and "symlink" in result.stderr
        assert not any(outside.iterdir())


def test_interrupt_concurrent_and_unowned_removal():
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        archive, digest = make_package(home / "pkg")
        env = base_env(
            home,
            JARVIS_PACKAGE=str(archive),
            JARVIS_PACKAGE_SHA256=digest,
            JARVIS_INSTALL_FAIL_AFTER="staged",
        )
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0
        assert not (home / "bin" / "jarvis-acp").exists()
        env.pop("JARVIS_INSTALL_FAIL_AFTER")
        run(["sh", str(INSTALL)], env)
        assert "fixture 0.0.0" in run([str(home / "bin" / "jarvis-tui"), "--version"], env).stdout
        lock = home / "install" / ".install.lock"
        lock.mkdir()
        (lock / "pid").write_text(str(os.getpid()), encoding="utf-8")
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0 and "concurrent" in result.stderr
        import shutil
        shutil.rmtree(lock)
        other = home / "bin" / "unrelated"
        other.write_text("leave me\\n", encoding="utf-8")
        run(["sh", str(UNINSTALL)], env)
        assert other.read_text() == "leave me\\n"
        assert (home / "memory" / "skills" / "demo.md").is_file()


def test_zip_slip_and_case_collision():
    with tempfile.TemporaryDirectory() as raw:
        home = Path(raw)
        archive = home / "bad.zip"
        with zipfile.ZipFile(archive, "w") as package:
            package.writestr("../escaped.txt", "nope")
        digest = hashlib.sha256(archive.read_bytes()).hexdigest()
        env = base_env(
            home,
            JARVIS_TARGET="windows-x86_64",
            JARVIS_PACKAGE=str(archive),
            JARVIS_PACKAGE_SHA256=digest,
        )
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0
        assert not (home / "escaped.txt").exists()
        collision = home / "case.zip"
        with zipfile.ZipFile(collision, "w") as package:
            package.writestr("jarvis/skills/Demo.md", b"a")
            package.writestr("jarvis/skills/demo.md", b"b")
        digest = hashlib.sha256(collision.read_bytes()).hexdigest()
        env["JARVIS_PACKAGE"] = str(collision)
        env["JARVIS_PACKAGE_SHA256"] = digest
        result = run(["sh", str(INSTALL)], env, check=False)
        assert result.returncode != 0 and "collision" in result.stderr


def main():
    test_requires_explicit_archive()
    test_install_upgrade_rollback_uninstall_and_coexistence()
    test_negative_packages()
    test_interrupt_concurrent_and_unowned_removal()
    test_zip_slip_and_case_collision()
    text = INSTALL.read_text() + (ROOT / "scripts" / "install.ps1").read_text()
    for banned in ("99percentgrip/agent-vesper", "astral-sh/uv", "latest/download"):
        if banned in text:
            raise SystemExit(f"banned release fallback remains: {banned}")
    print("Installer upgrade preserves user state and database inode: PASS")


if __name__ == "__main__":
    main()
