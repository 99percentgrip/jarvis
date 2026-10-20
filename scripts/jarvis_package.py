#!/usr/bin/env python3
"""Offline Jarvis package assembly and POSIX install/uninstall.

Archive bytes can be reproduced from the same payload files. That does not
make native compiler output byte-identical across machines.
"""
from __future__ import annotations

import hashlib
import json
import os
import platform
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import zipfile
from pathlib import Path

SCHEMA = 1
LAYOUT = 1
APPLICATION = "jarvis"
MAX_FILE = 2 * 1024 * 1024 * 1024
MAX_TOTAL = 3 * 1024 * 1024 * 1024
MAX_ENTRIES = 10000
TARGETS = {
    "linux-x86_64": "jarvis-linux-x86_64.tar.gz",
    "linux-aarch64": "jarvis-linux-aarch64.tar.gz",
    "darwin-x86_64": "jarvis-darwin-x86_64.tar.gz",
    "darwin-aarch64": "jarvis-darwin-aarch64.tar.gz",
    "windows-x86_64": "jarvis-windows-x86_64.zip",
}
USER_FACING = {
    "linux-x86_64": ("jarvis-acp", "jarvis-tui"),
    "linux-aarch64": ("jarvis-acp", "jarvis-tui"),
    "darwin-x86_64": ("jarvis-acp", "jarvis-tui"),
    "darwin-aarch64": ("jarvis-acp", "jarvis-tui"),
    "windows-x86_64": ("jarvis-acp.exe", "jarvis-tui.exe"),
}
HELPERS = {
    "linux-x86_64": ("jarvis-web-fetch", "jarvis-sandbox-init"),
    "linux-aarch64": ("jarvis-web-fetch", "jarvis-sandbox-init"),
    "darwin-x86_64": ("jarvis-web-fetch", "jarvis-sandbox-init"),
    "darwin-aarch64": ("jarvis-web-fetch", "jarvis-sandbox-init"),
    "windows-x86_64": ("jarvis-web-fetch.exe", "jarvis-sandbox-init.exe"),
}
DRIVER_FILES = {"image.tar.gz", "image-id", "image.sha256"}
MANIFEST_KEYS = {
    "schema",
    "layout",
    "application",
    "target",
    "archive",
    "version",
    "source_commit",
    "payloads",
}
PAYLOAD_KEYS = {"path", "type", "role", "required", "sha256"}
INSTALLER_NAMES = {
    ".jarvis-installer-owned.json",
    ".install.lock",
    "transaction.json",
    "current",
    "generations",
}


class InstallError(Exception):
    pass


def fail(message: str) -> None:
    raise InstallError(f"jarvis installer: {message}")


def detect_target() -> str:
    override = os.environ.get("JARVIS_TARGET", "").strip()
    if override:
        if override not in TARGETS:
            fail(f"unsupported target: {override}")
        return override
    system = platform.system()
    machine = platform.machine().lower()
    arch = {
        "x86_64": "x86_64",
        "amd64": "x86_64",
        "arm64": "aarch64",
        "aarch64": "aarch64",
    }.get(machine)
    if system == "Linux" and arch:
        return f"linux-{arch}"
    if system == "Darwin" and arch:
        return f"darwin-{arch}"
    if system == "Windows" and arch == "x86_64":
        return "windows-x86_64"
    fail(f"unsupported operating system or architecture: {system} {machine}")
    return ""


def home_dir() -> Path:
    override = os.environ.get("JARVIS_HOME", "").strip()
    if override:
        return Path(override)
    return Path.home()


def default_paths(target: str) -> tuple[Path, Path, Path]:
    home = home_dir()
    if target.startswith("linux"):
        data = Path(os.environ.get("XDG_DATA_HOME", str(home / ".local" / "share")))
        install = data / "jarvis" / "install"
        launcher = Path(os.environ.get("XDG_BIN_HOME", str(home / ".local" / "bin")))
    elif target.startswith("darwin"):
        install = home / "Library" / "Application Support" / "Jarvis" / "Install"
        launcher = Path(os.environ.get("XDG_BIN_HOME", str(home / ".local" / "bin")))
    else:
        local = Path(os.environ.get("LOCALAPPDATA", str(home / "AppData" / "Local")))
        install = local / "Programs" / "Jarvis"
        launcher = install
    memory = Path(os.environ.get("JARVIS_MEMORY_ROOT", str(home / ".jarvis" / "memory")))
    if os.environ.get("JARVIS_INSTALL_ROOT", "").strip():
        install = Path(os.environ["JARVIS_INSTALL_ROOT"])
    if os.environ.get("JARVIS_LAUNCHER_DIR", "").strip():
        launcher = Path(os.environ["JARVIS_LAUNCHER_DIR"])
    return install, launcher, memory


def looks_foreign(path: Path) -> bool:
    text = str(path).replace("\\", "/").lower()
    return "agent-vesper" in text or "agentvesper" in text


def refuse_foreign(path: Path) -> None:
    if looks_foreign(path):
        fail(f"refusing Vesper-owned or ambiguous path: {path}")
    if path.is_symlink():
        fail(f"refusing symlink redirection: {path}")


def safe_name(path: str) -> str:
    if not path or path.startswith(("/", "\\")) or "\\" in path or ":" in path:
        fail(f"unsafe archive path: {path}")
    parts = path.split("/")
    if any(part in {"", ".", ".."} for part in parts):
        fail(f"unsafe archive path: {path}")
    return path


def allowed_file(target: str, rel: str) -> str:
    rel = safe_name(rel)
    names = set(USER_FACING[target] + HELPERS[target])
    if "/" not in rel:
        if rel not in names:
            fail(f"unexpected package file: {rel}")
        return "user-facing" if rel in USER_FACING[target] else "helper"
    prefix, _, rest = rel.partition("/")
    if prefix == "skills":
        safe_name(rest)
        return "resource"
    if prefix == "web-driver":
        if "/" in rest or rest not in DRIVER_FILES:
            fail(f"unexpected driver file: {rel}")
        return "resource"
    fail(f"unexpected package file: {rel}")
    return ""


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    total = 0
    with path.open("rb") as handle:
        while True:
            block = handle.read(1024 * 1024)
            if not block:
                break
            total += len(block)
            if total > MAX_FILE:
                fail(f"payload exceeds size limit: {path}")
            digest.update(block)
    return digest.hexdigest()


def expected_digest(value: str, archive: Path) -> str:
    text = value.strip()
    candidate = Path(text)
    if candidate.is_file():
        parts = candidate.read_text(encoding="utf-8").strip().split()
        if not parts:
            fail("invalid SHA-256 checksum file")
        text = parts[0]
        if len(parts) > 1 and parts[1].lstrip("*") not in {archive.name, str(archive)}:
            fail("checksum filename does not match the archive")
    if len(text) != 64 or any(c not in "0123456789abcdefABCDEF" for c in text):
        fail("invalid SHA-256 checksum file")
    actual = sha256_file(archive)
    if actual.lower() != text.lower():
        fail("SHA-256 verification failed")
    return actual.lower()


def read_manifest(root: Path, target: str) -> dict:
    path = root / "manifest.json"
    if not path.is_file() or path.is_symlink():
        fail("malformed package manifest")
    try:
        manifest = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        fail("malformed package manifest")
    if not isinstance(manifest, dict) or set(manifest) != MANIFEST_KEYS:
        fail("malformed package manifest")
    if manifest["schema"] != SCHEMA or manifest["layout"] != LAYOUT:
        fail("unsupported package schema")
    if manifest["application"] != APPLICATION:
        fail("package application is not jarvis")
    if manifest["target"] != target or manifest["archive"] != TARGETS[target]:
        fail(f"wrong target: manifest is {manifest.get('target')}")
    if not isinstance(manifest["version"], str) or not manifest["version"].strip():
        fail("malformed package manifest")
    if not isinstance(manifest["source_commit"], str):
        fail("malformed package manifest")
    payloads = manifest["payloads"]
    if not isinstance(payloads, list):
        fail("malformed package manifest")
    seen = {}
    lowered = {}
    for item in payloads:
        if not isinstance(item, dict) or set(item) != PAYLOAD_KEYS:
            fail("malformed package manifest")
        rel = item["path"]
        if item["type"] != "file":
            fail("malformed package manifest")
        role = allowed_file(target, rel)
        if item["role"] != role:
            fail(f"payload role mismatch: {rel}")
        required = item["required"] is True
        if role == "user-facing" and not required:
            fail(f"required executable omitted from contract: {rel}")
        if role != "user-facing" and required:
            fail(f"optional payload marked required: {rel}")
        digest = item["sha256"]
        if not isinstance(digest, str) or len(digest) != 64:
            fail("malformed package manifest")
        key = rel.lower()
        if key in lowered:
            fail(f"case-insensitive path collision: {rel}")
        lowered[key] = rel
        seen[rel] = digest.lower()
    for name in USER_FACING[target]:
        if name not in seen:
            fail(f"missing executable: {name}")
    on_disk = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [name for name in dirnames if name != "__pycache__"]
        for filename in filenames:
            path = Path(dirpath) / filename
            if path.is_symlink():
                fail(f"refusing symlink in package: {path}")
            rel = path.relative_to(root).as_posix()
            if rel == "manifest.json":
                continue
            on_disk.append(rel)
    if len(on_disk) > MAX_ENTRIES:
        fail("package has too many entries")
    if set(on_disk) != set(seen):
        fail("package contents do not match the manifest")
    total = 0
    for rel, digest in seen.items():
        path = root / rel
        if not path.is_file():
            fail(f"missing payload: {rel}")
        total += path.stat().st_size
        if total > MAX_TOTAL:
            fail("package exceeds size limit")
        if sha256_file(path) != digest:
            fail(f"payload checksum mismatch: {rel}")
    return manifest


def extract_tar(archive: Path, dest: Path, target: str) -> None:
    try:
        tar = tarfile.open(archive, "r:gz")
    except tarfile.TarError:
        fail("invalid archive")
    with tar:
        members = tar.getmembers()
        if len(members) > MAX_ENTRIES:
            fail("package has too many entries")
        seen = set()
        for member in members:
            name = member.name.replace("\\", "/")
            if member.issym() or member.islnk() or member.isdev() or member.isfifo():
                fail(f"refusing special archive entry: {name}")
            if not name.startswith("jarvis/") and name != "jarvis":
                fail(f"unexpected bundle root: {name}")
            rel = name[len("jarvis/") :]
            if name == "jarvis":
                continue
            if member.isdir():
                if rel.rstrip("/"):
                    safe_name(rel.rstrip("/"))
                continue
            if not member.isfile():
                fail(f"refusing special archive entry: {name}")
            rel = safe_name(rel)
            key = rel.lower()
            if key in seen:
                fail(f"case-insensitive path collision: {rel}")
            seen.add(key)
            if member.size > MAX_FILE:
                fail(f"payload exceeds size limit: {rel}")
            if rel != "manifest.json":
                allowed_file(target, rel)
        # Second pass extracts after every name is accepted.
        for member in members:
            name = member.name.replace("\\", "/")
            if name == "jarvis" or not member.isfile():
                continue
            rel = safe_name(name[len("jarvis/") :])
            if rel != "manifest.json":
                allowed_file(target, rel)
            source = tar.extractfile(member)
            if source is None:
                fail("invalid archive")
            output = dest / rel
            if output.exists() or output.is_symlink():
                fail(f"duplicate archive entry: {rel}")
            output.parent.mkdir(parents=True, exist_ok=True)
            with output.open("xb") as handle:
                remaining = member.size
                while remaining:
                    block = source.read(min(1024 * 1024, remaining))
                    if not block:
                        break
                    handle.write(block)
                    remaining -= len(block)
            os.chmod(output, 0o755 if "/" not in rel else 0o644)


def extract_zip(archive: Path, dest: Path, target: str) -> None:
    try:
        package = zipfile.ZipFile(archive)
    except zipfile.BadZipFile:
        fail("invalid archive")
    with package:
        seen = set()
        for info in package.infolist():
            name = info.filename.replace("\\", "/")
            if name.endswith("/"):
                safe_name(name[len("jarvis/") :].rstrip("/")) if name.startswith("jarvis/") else fail(
                    f"unexpected bundle root: {name}"
                )
                continue
            if not name.startswith("jarvis/"):
                fail(f"unexpected bundle root: {name}")
            rel = safe_name(name[len("jarvis/") :])
            mode = (info.external_attr >> 16) & 0o170000
            if mode in {stat.S_IFLNK, stat.S_IFCHR, stat.S_IFBLK, stat.S_IFIFO}:
                fail(f"refusing special archive entry: {name}")
            key = rel.lower()
            if key in seen:
                fail(f"case-insensitive path collision: {rel}")
            seen.add(key)
            if info.file_size > MAX_FILE:
                fail(f"payload exceeds size limit: {rel}")
            if rel != "manifest.json":
                allowed_file(target, rel)
            output = dest / rel
            output.parent.mkdir(parents=True, exist_ok=True)
            with package.open(info, "r") as source, output.open("xb") as handle:
                remaining = info.file_size
                while remaining:
                    block = source.read(min(1024 * 1024, remaining))
                    if not block:
                        break
                    handle.write(block)
                    remaining -= len(block)
            os.chmod(output, 0o755 if "/" not in rel else 0o644)


def stage_archive(archive: Path, checksum: str, target: str) -> Path:
    if not archive.is_file() or archive.is_symlink():
        fail("package archive is missing")
    expected_digest(checksum, archive)
    dest = Path(tempfile.mkdtemp(prefix="jarvis-stage-"))
    try:
        if target == "windows-x86_64":
            extract_zip(archive, dest, target)
        else:
            extract_tar(archive, dest, target)
        read_manifest(dest, target)
    except Exception:
        shutil.rmtree(dest, ignore_errors=True)
        raise
    return dest


def generation_id() -> str:
    return "g" + hashlib.sha256(os.urandom(32)).hexdigest()[:16]


def read_json(path: Path) -> dict | None:
    if not path.exists():
        return None
    if path.is_symlink():
        fail(f"refusing symlink redirection: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value: dict) -> None:
    if path.is_symlink():
        fail(f"refusing symlink redirection: {path}")
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(temporary, path)


def pid_alive(pid: int) -> bool:
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def acquire_lock(root: Path) -> None:
    root.mkdir(parents=True, exist_ok=True)
    if root.is_symlink():
        fail(f"refusing symlink redirection: {root}")
    lock = root / ".install.lock"
    while True:
        try:
            lock.mkdir()
            (lock / "pid").write_text(str(os.getpid()), encoding="utf-8")
            return
        except FileExistsError:
            pid_text = (lock / "pid").read_text(encoding="utf-8").strip() if (lock / "pid").is_file() else "0"
            try:
                pid = int(pid_text)
            except ValueError:
                pid = 0
            if pid_alive(pid) and pid != os.getpid():
                fail("concurrent installation holds this install root")
            shutil.rmtree(lock, ignore_errors=True)


def release_lock(root: Path) -> None:
    lock = root / ".install.lock"
    if lock.exists() and not lock.is_symlink():
        pid_file = lock / "pid"
        if pid_file.is_file() and pid_file.read_text(encoding="utf-8").strip() == str(os.getpid()):
            shutil.rmtree(lock, ignore_errors=True)


def assert_adoptable(root: Path) -> None:
    if not root.exists():
        return
    refuse_foreign(root)
    if root.is_symlink():
        fail(f"refusing symlink redirection: {root}")
    unknown = [entry.name for entry in root.iterdir() if entry.name not in INSTALLER_NAMES]
    ownership = root / ".jarvis-installer-owned.json"
    if unknown:
        fail("refusing foreign-owned or ambiguous install root")
    if ownership.exists():
        record = read_json(ownership)
        if not record or record.get("application") != APPLICATION or record.get("schema") != SCHEMA:
            fail("refusing foreign-owned or ambiguous install root")
        if Path(record.get("install_root", "")) != root:
            fail("installer ownership record does not match this root")


def current_generation(root: Path) -> str | None:
    path = root / "current"
    if not path.exists():
        return None
    if path.is_symlink():
        fail(f"refusing symlink redirection: {path}")
    text = path.read_text(encoding="utf-8").strip()
    if not text or any(part in text for part in ("/", "\\", "..")):
        fail("invalid current generation")
    return text


def switch_current(root: Path, generation: str) -> None:
    path = root / "current"
    temporary = root / "current.tmp"
    temporary.write_text(generation + "\n", encoding="utf-8")
    os.replace(temporary, path)


def smoke(root: Path, generation: str, target: str) -> str:
    bundle = root / "generations" / generation
    texts = []
    env = os.environ.copy()
    env["JARVIS_BUNDLE_DIR"] = str(bundle)
    for name in USER_FACING[target]:
        executable = bundle / name
        if not os.access(executable, os.X_OK):
            os.chmod(executable, 0o755)
        try:
            result = subprocess.run(
                [str(executable), "--version"],
                check=False,
                capture_output=True,
                timeout=60,
                env=env,
            )
        except OSError as error:
            fail(f"executable preflight failed for {name}: {error}")
        output = (result.stdout or result.stderr).decode("utf-8", "replace").strip()
        if result.returncode != 0 or not output:
            fail(
                f"executable preflight failed for {name} (exit {result.returncode}). Existing installation was preserved."
            )
        texts.append(output.splitlines()[0])
    return "; ".join(texts)


def write_launchers(root: Path, launcher_dir: Path, target: str) -> list[str]:
    launcher_dir.mkdir(parents=True, exist_ok=True)
    refuse_foreign(launcher_dir)
    if launcher_dir.is_symlink():
        fail(f"refusing symlink redirection: {launcher_dir}")
    written = []
    mapping = (
        (("jarvis-acp", "jarvis-acp"), ("jarvis-tui", "jarvis-tui"))
        if not target.startswith("windows")
        else ()
    )
    for launcher_name, executable in mapping:
        path = launcher_dir / launcher_name
        if path.is_symlink():
            path.unlink()
        if path.exists():
            first = path.read_text(encoding="utf-8", errors="replace").splitlines()[:2]
            if first != ["#!/bin/sh", "# jarvis-launcher"]:
                fail(f"refusing to replace unowned launcher: {path}")
        body = (
            "#!/bin/sh\n"
            "# jarvis-launcher\n"
            "set -eu\n"
            f'root="{root}"\n'
            'gen=$(cat "$root/current")\n'
            'export JARVIS_BUNDLE_DIR="$root/generations/$gen"\n'
            f'exec "$JARVIS_BUNDLE_DIR/{executable}" "$@"\n'
        )
        temporary = path.with_suffix(".tmp")
        temporary.write_text(body, encoding="utf-8")
        os.chmod(temporary, 0o755)
        os.replace(temporary, path)
        written.append(str(path))
    return written


def seed_skills(bundle: Path, memory: Path) -> int:
    seed_root = bundle / "skills" / "skills"
    if not seed_root.is_dir():
        return 0
    refuse_foreign(memory)
    (memory / "skills").mkdir(parents=True, exist_ok=True)
    (memory / "bundles").mkdir(parents=True, exist_ok=True)
    manifest = memory / ".seed-manifest"
    if not manifest.exists():
        manifest.write_text("", encoding="utf-8")
    listed = {line.strip() for line in manifest.read_text(encoding="utf-8").splitlines() if line.strip()}
    seeded = 0
    for seed in sorted(seed_root.glob("*.md")):
        slug = seed.stem
        if slug in listed or (memory / "skills" / seed.name).exists():
            continue
        shutil.copy2(seed, memory / "skills" / seed.name)
        resource = seed_root / slug
        destination = memory / "skills" / slug
        if resource.is_dir() and not destination.exists():
            shutil.copytree(resource, destination)
        with manifest.open("a", encoding="utf-8") as handle:
            handle.write(slug + "\n")
        seeded += 1
    bundle_dir = bundle / "skills" / "bundles"
    if bundle_dir.is_dir():
        for seed in sorted(bundle_dir.glob("*.json")):
            destination = memory / "bundles" / seed.name
            if not destination.exists():
                shutil.copy2(seed, destination)
    return seeded


def update_profile(launcher_dir: Path, ownership: dict) -> None:
    if ownership.get("profile_edited"):
        return
    path_text = os.environ.get("PATH", "")
    if str(launcher_dir) in path_text.split(":"):
        ownership["profile_edited"] = False
        return
    profile = os.environ.get("JARVIS_SHELL_PROFILE", "").strip()
    if not profile:
        shell = os.environ.get("SHELL", "")
        profile = str(home_dir() / (".zprofile" if shell.endswith("/zsh") else ".profile"))
    line = f'export PATH="{launcher_dir}:$PATH"'
    marker = "# Jarvis installer"
    path = Path(profile)
    refuse_foreign(path)
    existing = path.read_text(encoding="utf-8") if path.exists() else ""
    if line not in existing.splitlines():
        with path.open("a", encoding="utf-8") as handle:
            if existing and not existing.endswith("\n"):
                handle.write("\n")
            handle.write(f"{marker}\n{line}\n")
        ownership["profile_edited"] = True
        ownership["profile_path"] = str(path)
        ownership["profile_line"] = line
    else:
        ownership["profile_edited"] = False


def remove_profile(ownership: dict) -> None:
    if not ownership.get("profile_edited"):
        return
    path = Path(ownership.get("profile_path") or "")
    line = ownership.get("profile_line") or ""
    if not path.is_file() or path.is_symlink() or not line:
        return
    rows = path.read_text(encoding="utf-8").splitlines()
    kept = []
    index = 0
    while index < len(rows):
        if rows[index] == "# Jarvis installer" and index + 1 < len(rows) and rows[index + 1] == line:
            index += 2
            continue
        kept.append(rows[index])
        index += 1
    path.write_text(("\n".join(kept) + ("\n" if kept else "")), encoding="utf-8")


def finish(root: Path, launcher_dir: Path, memory: Path, target: str, generation: str, previous: str | None, versions: str) -> None:
    bundle = root / "generations" / generation
    launchers = write_launchers(root, launcher_dir, target)
    seeded = seed_skills(bundle, memory)
    ownership = read_json(root / ".jarvis-installer-owned.json") or {
        "schema": SCHEMA,
        "application": APPLICATION,
        "profile_edited": False,
    }
    ownership.update(
        {
            "schema": SCHEMA,
            "application": APPLICATION,
            "install_root": str(root),
            "current_generation": generation,
            "previous_generation": previous,
            "launchers": launchers,
            "memory_root_recorded_not_owned": str(memory),
        }
    )
    if not target.startswith("windows"):
        update_profile(launcher_dir, ownership)
    write_json(root / ".jarvis-installer-owned.json", ownership)
    write_json(root / "transaction.json", {"state": "complete", "generation": generation, "previous": previous})
    print(f"Installed Jarvis ({versions})")
    for launcher in launchers:
        print(f"  {launcher}")
    print(f"Seeded {seeded} skill(s) into {memory}")
    print("No public release was downloaded. Optional web-driver import is not run unless the package contains it.")
    driver = bundle / "web-driver" / "image.tar.gz"
    if driver.is_file():
        print("Bundled web driver is installed. Import it only from Settings or an explicit setup command.")
    else:
        print("This package has no bundled web driver. Installation succeeded without one.")


def delete_generation(root: Path, name: str | None) -> None:
    if not name:
        return
    safe_name(name)
    path = root / "generations" / name
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path)


def recover(root: Path, launcher_dir: Path, memory: Path, target: str) -> None:
    transaction = read_json(root / "transaction.json")
    if not transaction:
        return
    state = transaction.get("state")
    generation = transaction.get("generation")
    previous = transaction.get("previous")
    if state == "complete":
        return
    current = current_generation(root)
    if state == "staged" and current != generation:
        delete_generation(root, generation)
        transaction_path = root / "transaction.json"
        if transaction_path.exists():
            transaction_path.unlink()
        return
    if state in {"staged", "activated"} and current == generation:
        finish(root, launcher_dir, memory, target, generation, previous, "recovered")


def prune_generations(root: Path, keep: set[str]) -> None:
    directory = root / "generations"
    if not directory.is_dir():
        return
    for child in directory.iterdir():
        if child.name not in keep and child.is_dir() and not child.is_symlink():
            shutil.rmtree(child)


def install_package() -> None:
    archive_value = os.environ.get("JARVIS_PACKAGE", "").strip()
    checksum = os.environ.get("JARVIS_PACKAGE_SHA256", "").strip()
    if not archive_value or not checksum:
        fail(
            "no public package is available. Set JARVIS_PACKAGE to a local archive and JARVIS_PACKAGE_SHA256 to its expected digest."
        )
    target = detect_target()
    install_root, launcher_dir, memory = default_paths(target)
    for path in (install_root, launcher_dir, memory):
        refuse_foreign(path)
    archive = Path(archive_value)
    staged = stage_archive(archive, checksum, target)
    try:
        assert_adoptable(install_root)
        acquire_lock(install_root)
        try:
            recover(install_root, launcher_dir, memory, target)
            previous = current_generation(install_root)
            generation = generation_id()
            destination = install_root / "generations" / generation
            if destination.exists() or (install_root / "generations").is_symlink():
                fail("refusing symlink redirection in the install root")
            shutil.copytree(staged, destination, symlinks=False)
            write_json(
                install_root / "transaction.json",
                {"state": "staged", "generation": generation, "previous": previous},
            )
            if os.environ.get("JARVIS_INSTALL_FAIL_AFTER") == "staged":
                fail("injected failure after staging; installation was not activated")
            versions = smoke(install_root, generation, target)
            switch_current(install_root, generation)
            write_json(
                install_root / "transaction.json",
                {"state": "activated", "generation": generation, "previous": previous},
            )
            if os.environ.get("JARVIS_INSTALL_FAIL_AFTER") == "activated":
                fail("injected failure after activation; rerun to finish launchers")
            finish(install_root, launcher_dir, memory, target, generation, previous, versions)
            prune_generations(install_root, {generation, previous} if previous else {generation})
        finally:
            release_lock(install_root)
    finally:
        shutil.rmtree(staged, ignore_errors=True)


def rollback() -> None:
    target = detect_target()
    install_root, launcher_dir, _memory = default_paths(target)
    refuse_foreign(install_root)
    assert_adoptable(install_root)
    acquire_lock(install_root)
    try:
        transaction = read_json(install_root / "transaction.json")
        if transaction and transaction.get("state") != "complete":
            fail("installation transaction is incomplete; rerun install before rollback")
        ownership = read_json(install_root / ".jarvis-installer-owned.json")
        if not ownership or not ownership.get("previous_generation"):
            fail("no verified previous Jarvis package is available to restore")
        current = ownership["current_generation"]
        previous = ownership["previous_generation"]
        if not (install_root / "generations" / previous / "manifest.json").is_file():
            fail("previous package payloads are missing")
        switch_current(install_root, previous)
        ownership["current_generation"] = previous
        ownership["previous_generation"] = current
        write_json(install_root / ".jarvis-installer-owned.json", ownership)
        print(f"Restored local Jarvis package {previous}")
    finally:
        release_lock(install_root)


def uninstall() -> None:
    target = detect_target()
    install_root, _launcher_dir, _memory = default_paths(target)
    refuse_foreign(install_root)
    if not install_root.exists():
        print("Jarvis uninstall found no installer-owned root.")
        return
    if install_root.is_symlink():
        fail(f"refusing symlink redirection: {install_root}")
    acquire_lock(install_root)
    try:
        ownership = read_json(install_root / ".jarvis-installer-owned.json")
        if not ownership or ownership.get("application") != APPLICATION:
            fail("refusing to uninstall a root this installer does not own")
        for launcher in ownership.get("launchers") or []:
            path = Path(launcher)
            if looks_foreign(path):
                fail(f"refusing Vesper-owned or ambiguous path: {path}")
            if not path.exists() and not path.is_symlink():
                continue
            if path.is_symlink():
                path.unlink()
                continue
            first = path.read_text(encoding="utf-8", errors="replace").splitlines()[:2]
            if first != ["#!/bin/sh", "# jarvis-launcher"]:
                fail(f"refusing to remove unowned launcher: {path}")
            path.unlink()
        for name in (ownership.get("current_generation"), ownership.get("previous_generation")):
            delete_generation(install_root, name)
        remove_profile(ownership)
        for name in ("current", "transaction.json", ".jarvis-installer-owned.json"):
            path = install_root / name
            if path.is_symlink():
                fail(f"refusing symlink redirection: {path}")
            if path.exists():
                path.unlink()
        generations = install_root / "generations"
        if generations.is_dir() and not any(generations.iterdir()):
            generations.rmdir()
        print("Jarvis uninstall removed installer-owned payloads only.")
        print("Configuration, credentials, sessions, and memory were preserved.")
    finally:
        release_lock(install_root)
        if install_root.exists() and not install_root.is_symlink():
            remaining = [entry.name for entry in install_root.iterdir() if entry.name != ".install.lock"]
            if not remaining:
                release_lock(install_root)
                if install_root.exists() and not any(install_root.iterdir()):
                    install_root.rmdir()


def pack(bin_dir: Path, target: str, output: Path, version: str, commit: str, resources: Path | None) -> None:
    if target not in TARGETS:
        fail(f"unsupported target: {target}")
    staging = Path(tempfile.mkdtemp(prefix="jarvis-pack-"))
    try:
        root = staging / "jarvis"
        root.mkdir()
        payloads = []
        names = list(USER_FACING[target] + HELPERS[target])
        for name in names:
            source = bin_dir / name
            if name in HELPERS[target] and not source.is_file():
                continue
            if not source.is_file():
                fail(f"missing executable: {source}")
            shutil.copy2(source, root / name)
            os.chmod(root / name, 0o755)
        if resources and resources.exists():
            for rel in sorted(path.relative_to(resources).as_posix() for path in resources.rglob("*") if path.is_file()):
                allowed_file(target, rel)
                destination = root / rel
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(resources / rel, destination)
        files = sorted(path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file())
        for rel in files:
            role = allowed_file(target, rel)
            payloads.append(
                {
                    "path": rel,
                    "type": "file",
                    "role": role,
                    "required": role == "user-facing",
                    "sha256": sha256_file(root / rel),
                }
            )
        manifest = {
            "schema": SCHEMA,
            "layout": LAYOUT,
            "application": APPLICATION,
            "target": target,
            "archive": TARGETS[target],
            "version": version,
            "source_commit": commit,
            "payloads": payloads,
        }
        (root / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        output.parent.mkdir(parents=True, exist_ok=True)
        if target == "windows-x86_64":
            with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as package:
                for path in sorted(root.rglob("*")):
                    if path.is_file():
                        info = zipfile.ZipInfo(f"jarvis/{path.relative_to(root).as_posix()}")
                        info.date_time = (1980, 1, 1, 0, 0, 0)
                        info.compress_type = zipfile.ZIP_DEFLATED
                        info.external_attr = (0o644 if "/" in info.filename[len("jarvis/"):] else 0o755) << 16
                        package.writestr(info, path.read_bytes())
        else:
            raw = tempfile.NamedTemporaryFile(delete=False)
            raw.close()
            with tarfile.open(raw.name, "w", format=tarfile.PAX_FORMAT) as tar:
                for path in sorted(root.rglob("*")):
                    info = tarfile.TarInfo(f"jarvis/{path.relative_to(root).as_posix()}")
                    info.mtime = 0
                    info.uid = 0
                    info.gid = 0
                    if path.is_file():
                        info.size = path.stat().st_size
                        info.mode = 0o755 if "/" not in path.relative_to(root).as_posix() else 0o644
                        info.type = tarfile.REGTYPE
                        with path.open("rb") as handle:
                            tar.addfile(info, handle)
            import gzip
            with open(raw.name, "rb") as source, open(output, "wb") as destination:
                with gzip.GzipFile(filename="", mode="wb", fileobj=destination, mtime=0) as compressor:
                    shutil.copyfileobj(source, compressor)
            os.unlink(raw.name)
    finally:
        shutil.rmtree(staging, ignore_errors=True)
    digest = sha256_file(output)
    checksum = output.with_name(output.name + ".sha256")
    checksum.write_text(f"{digest}  {output.name}\n", encoding="utf-8")
    print(digest)


def main(argv: list[str]) -> int:
    try:
        command = argv[1] if len(argv) > 1 else "install"
        if command == "install":
            if "--rollback" in argv[2:]:
                rollback()
            else:
                install_package()
        elif command == "rollback":
            rollback()
        elif command == "uninstall":
            uninstall()
        elif command == "pack":
            import argparse
            parser = argparse.ArgumentParser()
            parser.add_argument("--bin-dir", required=True)
            parser.add_argument("--target", required=True)
            parser.add_argument("--output", required=True)
            parser.add_argument("--version", required=True)
            parser.add_argument("--commit", default="")
            parser.add_argument("--resources")
            args = parser.parse_args(argv[2:])
            pack(
                Path(args.bin_dir),
                args.target,
                Path(args.output),
                args.version,
                args.commit,
                Path(args.resources) if args.resources else None,
            )
        else:
            fail(f"unknown command: {command}")
    except InstallError as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
