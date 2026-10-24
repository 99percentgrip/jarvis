#!/usr/bin/env python3
"""Validate and install the exact nonpublishing candidate on its native CI runner.

This script does not assemble packages, publish, or authorize workflow dispatch.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import tempfile

from jarvis_package import (HELPERS, TARGETS, USER_FACING, expected_digest,
                            read_manifest, sha256_file, stage_archive)
from jarvis_release_candidate import TRIPLES, digest


def binary_identity(path: Path, target: str) -> bool:
    with path.open('rb') as handle:
        header = handle.read(64)
        if target.startswith('linux'):
            machine = 62 if target == 'linux-x86_64' else 183
            return (len(header) >= 20 and header[:5] == b'\x7fELF\x02'
                    and int.from_bytes(header[18:20], 'little') == machine)
        if target.startswith('darwin'):
            machine = 0x01000007 if target == 'darwin-x86_64' else 0x0100000c
            return (len(header) >= 8 and header[:4] == b'\xcf\xfa\xed\xfe'
                    and int.from_bytes(header[4:8], 'little') == machine)
        if header[:2] != b'MZ' or len(header) < 64:
            return False
        handle.seek(int.from_bytes(header[60:64], 'little'))
        pe = handle.read(6)
        return pe[:4] == b'PE\0\0' and pe[4:6] == b'\x64\x86'


def validate_candidate_archive(archive: Path, checksum: Path, provenance: Path, *,
                               target: str, commit: str, driver_run: str,
                               driver_arch: str, driver_dir: Path, source: Path,
                               check_native_binary: bool = True,
                               compiler: str | None = None) -> dict:
    if target not in TARGETS or archive.name != TARGETS[target]:
        raise ValueError('noncanonical target/archive identity')
    if not re.fullmatch(r'[0-9a-f]{40}', commit):
        raise ValueError('invalid exact source commit')
    if not checksum.is_file() or checksum.is_symlink() or not provenance.is_file() or provenance.is_symlink():
        raise ValueError('missing or redirected candidate sidecars')
    archive_hash = expected_digest(str(checksum), archive)
    receipt = json.loads(provenance.read_text(encoding='utf-8'))
    if not isinstance(receipt, dict) or set(receipt) != {
        'schema', 'status', 'source_commit', 'target', 'rust_target', 'compiler',
        'lockfile_sha256', 'archive', 'binaries', 'driver', 'archive_sha256'
    } or receipt['schema'] != 1 or receipt['status'] != 'nonpublishing-candidate':
        raise ValueError('invalid candidate provenance schema')
    if any((receipt['source_commit'] != commit, receipt['target'] != target,
            receipt['rust_target'] != TRIPLES[target], receipt['archive'] != archive.name,
            receipt['archive_sha256'] != archive_hash,
            receipt['lockfile_sha256'] != digest(source / 'Cargo.lock'),
            not isinstance(receipt['compiler'], str) or not receipt['compiler'].startswith('rustc '),
            compiler is not None and receipt['compiler'] != compiler)):
        raise ValueError('candidate source, target, lockfile or digest mismatch')
    driver = receipt['driver']
    name = f'jarvis-web-driver-{driver_arch}'
    tested = driver_dir / (name + '.tar.gz')
    tested_sha = driver_dir / (name + '.tar.gz.sha256')
    tested_id = driver_dir / (name + '.image-id')
    if driver_arch != ('linux-aarch64' if target in {'linux-aarch64', 'darwin-aarch64'} else 'linux-x86_64'):
        raise ValueError('tested browser architecture does not match target')
    if any(not file.is_file() or file.is_symlink() for file in (tested, tested_sha, tested_id)):
        raise ValueError('missing immutable tested driver inputs')
    driver_digest = digest(tested)
    if tested_sha.read_text(encoding='utf-8').split() != [driver_digest, tested.name]:
        raise ValueError('tested browser digest mismatch')
    image_id = tested_id.read_text(encoding='utf-8').strip()
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', image_id):
        raise ValueError('invalid browser image identity')
    if not isinstance(driver, dict) or driver != {
        'run_id': driver_run, 'arch': driver_arch,
        'sha256': driver_digest, 'image_id': image_id,
    }:
        raise ValueError('candidate browser provenance mismatch')
    staged = stage_archive(archive, str(checksum), target)
    try:
        manifest = read_manifest(staged, target)
        if manifest['source_commit'] != commit or manifest['archive'] != archive.name:
            raise ValueError('archive manifest source or filename mismatch')
        if manifest['version'] != _workspace_version(source):
            raise ValueError('archive workspace version mismatch')
        names = USER_FACING[target] + HELPERS[target]
        if not isinstance(receipt['binaries'], dict) or set(receipt['binaries']) != set(names):
            raise ValueError('shipping executable inventory mismatch')
        for name in names:
            executable = staged / name
            if not executable.is_file() or executable.is_symlink() or digest(executable) != receipt['binaries'][name]:
                raise ValueError(f'shipping binary mismatch: {name}')
            if check_native_binary and not binary_identity(executable, target):
                raise ValueError(f'wrong native executable identity: {name}')
        bundled = staged / 'web-driver'
        if (digest(bundled / 'image.tar.gz') != driver_digest
                or (bundled / 'image-id').read_text(encoding='utf-8').strip() != image_id
                or (bundled / 'image.sha256').read_bytes() != tested_sha.read_bytes()):
            raise ValueError('bundled driver differs from tested immutable image')
        return manifest
    finally:
        shutil.rmtree(staged)


def _workspace_version(source: Path) -> str:
    # Match the wrapper's checked-out workspace version without depending on
    # tomllib (system Python on native macOS runners can predate 3.11).
    text = (source / 'Cargo.toml').read_text(encoding='utf-8')
    section = re.search(r'(?ms)^\[workspace\.package\]\s*\n(.*?)(?=^\[|\Z)', text)
    version = re.search(r'(?m)^version\s*=\s*"([^"\n]+)"\s*$', section.group(1)) if section else None
    if version is None:
        raise ValueError('missing checked-out workspace package version')
    return version.group(1)


def native_target(target: str) -> bool:
    machine = platform.machine().lower()
    system = platform.system()
    return ((system == 'Linux' and target == ('linux-aarch64' if machine in ('aarch64', 'arm64') else 'linux-x86_64') and machine in ('aarch64', 'arm64', 'x86_64', 'amd64'))
            or (system == 'Darwin' and target == ('darwin-aarch64' if machine in ('aarch64', 'arm64') else 'darwin-x86_64') and machine in ('aarch64', 'arm64', 'x86_64'))
            or (system == 'Windows' and target == 'windows-x86_64' and machine in ('amd64', 'x86_64')))


def verify_signed_out_launch(binary: Path, source: Path, env: dict[str, str]) -> None:
    """Exercise private first-launch/provider Settings on the installed TUI."""
    subprocess.run(['python', str(source / 'apps/jarvis-tui/tests/first_launch_pty.py'),
                    str(binary)], check=True, env=env, timeout=180)


SENTINEL_BYTES = b'preserve\n'


def require_sentinel_bytes(path: Path, message: str) -> None:
    import stat
    try:
        if not stat.S_ISREG(path.lstat().st_mode) or path.read_bytes() != SENTINEL_BYTES:
            raise ValueError(message)
    except OSError as exc:
        raise ValueError(message) from exc


def run_native_install(archive: Path, checksum: Path, target: str, source: Path) -> None:
    if os.environ.get('GITHUB_ACTIONS') != 'true' or not os.environ.get('RUNNER_TEMP') or not native_target(target):
        raise ValueError('native candidate installation requires matching disposable CI runner')
    # The candidate archive and sidecars remain at their original paths; this
    # temporary root contains only installer outputs, state, and a foreign sentinel.
    with tempfile.TemporaryDirectory(prefix='candidate-install-', dir=os.environ['RUNNER_TEMP']) as raw:
        root = Path(raw)
        sentinel = root / 'unrelated-user-data.txt'
        sentinel.write_bytes(SENTINEL_BYTES)
        require_sentinel_bytes(sentinel, 'invalid initial sentinel fixture')
        env = {**os.environ, 'JARVIS_PACKAGE': str(archive),
               'JARVIS_PACKAGE_SHA256': str(checksum), 'JARVIS_TARGET': target,
               'JARVIS_HOME': str(root), 'HOME': str(root), 'USERPROFILE': str(root),
               'JARVIS_INSTALL_ROOT': str(root / 'installed'),
               'JARVIS_LAUNCHER_DIR': str(root / 'bin'),
               'JARVIS_MEMORY_ROOT': str(root / 'memory'),
               'JARVIS_SHELL_PROFILE': str(root / 'profile'),
               'XDG_CONFIG_HOME': str(root / 'config'), 'XDG_DATA_HOME': str(root / 'data'),
               'XDG_STATE_HOME': str(root / 'state'),
               'JARVIS_GLOBAL_COGNITION_ROOT': str(root / 'global-cognition'),
               'TMP': str(root), 'TEMP': str(root), 'TMPDIR': str(root)}
        if target == 'windows-x86_64':
            subprocess.run(['powershell.exe', '-NoProfile', '-ExecutionPolicy', 'Bypass',
                            '-File', str(source / 'scripts/test_jarvis_candidate_install.ps1')],
                           check=True, env=env)
        else:
            subprocess.run(['sh', str(source / 'scripts/install.sh')], check=True, env=env)
            generation = (root / 'installed/current').read_text(encoding='utf-8').strip()
            bundle = root / 'installed/generations' / generation
            for name in USER_FACING[target]:
                subprocess.run([str(bundle / name), '--version'], check=True, env=env, timeout=60)
            for name in USER_FACING[target]:
                subprocess.run([str(root / 'bin' / name), '--version'], check=True, env=env, timeout=60)
            verify_signed_out_launch(bundle / 'jarvis-tui', source, env)
            subprocess.run(['sh', str(source / 'scripts/uninstall.sh')], check=True, env=env)
            if (root / 'installed').exists() or any((root / 'bin').iterdir()):
                raise ValueError('uninstall left owned files')
        require_sentinel_bytes(sentinel, 'uninstall altered foreign sentinel')


def require_unchanged_sidecars(archive: Path, checksum: Path, provenance: Path,
                               before: dict[Path, str]) -> None:
    if any(sha256_file(path) != observed for path, observed in before.items()) or expected_digest(str(checksum), archive) != before[archive]:
        raise ValueError('candidate or sidecar bytes changed after native installation')


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('archive', 'checksum', 'provenance', 'driver-dir', 'source'):
        parser.add_argument('--' + name, required=True, type=Path)
    for name in ('target', 'source-commit', 'driver-run', 'driver-arch'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    archive = args.archive.resolve(strict=True)
    checksum = args.checksum.resolve(strict=True)
    provenance = args.provenance.resolve(strict=True)
    source = args.source.resolve(strict=True)
    observed = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    if observed != args.source_commit:
        raise ValueError('candidate checkout does not match approved source commit')
    before = {path: sha256_file(path) for path in (archive, checksum, provenance)}
    validate_candidate_archive(archive, checksum, provenance,
        target=args.target, commit=args.source_commit, driver_run=args.driver_run,
        driver_arch=args.driver_arch, driver_dir=args.driver_dir, source=source,
        compiler=subprocess.check_output(['rustc', '--version'], text=True).strip())
    run_native_install(archive, checksum, args.target, source)
    require_unchanged_sidecars(archive, checksum, provenance, before)
    accepted = archive.with_name(archive.name + '.accepted.json')
    if accepted.exists() or accepted.is_symlink():
        raise ValueError('candidate acceptance receipt already exists')
    accepted.write_text(json.dumps({'schema': 1, 'status': 'native-accepted',
        'source_commit': args.source_commit, 'target': args.target,
        'archive': archive.name, 'sha256': before[archive],
        'provenance_sha256': before[provenance]},  sort_keys=True) + '\n', encoding='utf-8')
    print(f'PASS native exact candidate acceptance: {archive.name} {before[archive]}')


if __name__ == '__main__':
    main()
