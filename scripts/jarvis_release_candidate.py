#!/usr/bin/env python3
"""Fail-closed, offline nonpublishing wrapper around the Stage 6 package builder."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

from jarvis_package import HELPERS, TARGETS, USER_FACING, pack

TRIPLES = {
    'linux-x86_64': 'x86_64-unknown-linux-gnu',
    'linux-aarch64': 'aarch64-unknown-linux-gnu',
    'darwin-x86_64': 'x86_64-apple-darwin',
    'darwin-aarch64': 'aarch64-apple-darwin',
    'windows-x86_64': 'x86_64-pc-windows-msvc',
}
DRIVERS = {'linux-x86_64', 'linux-aarch64'}


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(65536), b''):
            h.update(block)
    return h.hexdigest()


def validate_and_pack(*, source: Path, source_commit: str, observed_commit: str,
                      target: str, rust_target: str, compiler: str, driver_run: str,
                      driver_arch: str, driver_dir: Path, bin_dir: Path,
                      resources: Path, output: Path, version: str) -> dict:
    if not re.fullmatch('[0-9a-f]{40}', source_commit) or observed_commit != source_commit:
        raise ValueError('candidate commit is not the exact checked-out commit')
    if target not in TARGETS or rust_target != TRIPLES[target]:
        raise ValueError('candidate target does not match its native Rust triple')
    if output.name != TARGETS[target] or output.exists() or output.with_name(output.name + '.sha256').exists():
        raise ValueError('candidate output must be a new canonical archive')
    if not re.fullmatch(r'\d+', driver_run) or not driver_run.strip('0'):
        raise ValueError('tested driver run ID is required')
    if driver_arch not in DRIVERS or driver_arch != ('linux-aarch64' if target in {'linux-aarch64', 'darwin-aarch64'} else 'linux-x86_64'):
        raise ValueError('tested driver architecture does not match candidate')
    if not compiler.startswith('rustc ') or not version or not re.fullmatch(r'[0-9a-f]{64}', digest(source / 'Cargo.lock')):
        raise ValueError('compiler, version and lockfile provenance are required')
    manifest = (source / 'Cargo.toml').read_text(encoding='utf-8')
    package_section = re.search(r'(?ms)^\[workspace\.package\]\s*\n(.*?)(?=^\[|\Z)', manifest)
    declared_version = re.search(r'(?m)^version\s*=\s*"([^"\n]+)"\s*$', package_section.group(1)) if package_section else None
    if declared_version is None or declared_version.group(1) != version:
        raise ValueError('candidate version differs from checked-out workspace package')
    binaries = {}
    for name in USER_FACING[target] + HELPERS[target]:
        path = bin_dir / name
        if not path.is_file() or path.is_symlink() or path.stat().st_size == 0:
            raise ValueError(f'missing or redirected shipping executable: {name}')
        binaries[name] = digest(path)
    name = f'jarvis-web-driver-{driver_arch}'
    archive = driver_dir / (name + '.tar.gz')
    checksum = driver_dir / (name + '.tar.gz.sha256')
    image_id = driver_dir / (name + '.image-id')
    for path in (archive, checksum, image_id):
        if not path.is_file() or path.is_symlink():
            raise ValueError(f'missing tested driver input: {path.name}')
    expected = checksum.read_text(encoding='utf-8').split()
    if len(expected) != 2 or expected[1] != archive.name or expected[0] != digest(archive):
        raise ValueError('tested driver digest or filename mismatch')
    image = image_id.read_text(encoding='utf-8').strip()
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', image):
        raise ValueError('tested driver image identity missing or malformed')
    staged_driver = resources / 'web-driver'
    if not staged_driver.is_dir() or staged_driver.is_symlink():
        raise ValueError('driver staging directory missing or redirected')
    for leaf, original in [('image.tar.gz', archive), ('image.sha256', checksum), ('image-id', image_id)]:
        destination = staged_driver / leaf
        if destination.is_symlink() or destination.exists() and digest(destination) != digest(original):
            raise ValueError('staged driver does not match tested driver')
    lock = digest(source / 'Cargo.lock')
    receipt = {
        'schema': 1, 'status': 'nonpublishing-candidate', 'source_commit': source_commit,
        'target': target, 'rust_target': rust_target, 'compiler': compiler,
        'lockfile_sha256': lock, 'archive': output.name, 'binaries': binaries,
        'driver': {'run_id': driver_run, 'arch': driver_arch,
                   'sha256': expected[0], 'image_id': image},
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='candidate-', dir=output.parent) as temporary:
        stage = Path(temporary)
        staged_resources = stage / 'resources'
        shutil.copytree(resources, staged_resources)
        for leaf, original in [('image.tar.gz', archive), ('image.sha256', checksum), ('image-id', image_id)]:
            shutil.copy2(original, staged_resources / 'web-driver' / leaf)
        staged_archive = stage / output.name
        pack(bin_dir, target, staged_archive, version, source_commit, staged_resources)
        archive_hash = digest(staged_archive)
        receipt['archive_sha256'] = archive_hash
        receipt_path = output.with_name(output.name + '.candidate.json')
        if receipt_path.exists():
            raise ValueError('candidate provenance receipt already exists')
        (stage / receipt_path.name).write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n', encoding='utf-8')
        shutil.move(staged_archive, output)
        shutil.move(stage / (output.name + '.sha256'), output.with_name(output.name + '.sha256'))
        shutil.move(stage / receipt_path.name, receipt_path)
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('source', 'driver-dir', 'bin-dir', 'resources', 'output'):
        parser.add_argument('--' + name, type=Path, required=True)
    for name in ('source-commit', 'target', 'rust-target', 'driver-run', 'driver-arch', 'version'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    source = args.source.resolve()
    observed = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    compiler = subprocess.check_output(['rustc', '--version'], text=True).strip()
    receipt = validate_and_pack(source=source, source_commit=args.source_commit,
        observed_commit=observed, target=args.target, rust_target=args.rust_target,
        compiler=compiler, driver_run=args.driver_run, driver_arch=args.driver_arch,
        driver_dir=args.driver_dir, bin_dir=args.bin_dir, resources=args.resources,
        output=args.output, version=args.version)
    print(json.dumps({'archive': receipt['archive'], 'sha256': receipt['archive_sha256']}))


if __name__ == '__main__':
    main()
