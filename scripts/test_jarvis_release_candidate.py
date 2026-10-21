"""Offline synthetic receipts for the nonpublishing five-target candidate wrapper."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from jarvis_release_candidate import validate_and_pack
from jarvis_package import HELPERS, TARGETS, USER_FACING


class CandidateTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / 'source'
        self.source.mkdir()
        (self.source / 'Cargo.lock').write_bytes(b'lock fixture\n')
        (self.source / 'Cargo.toml').write_text('[workspace.package]\nversion = "0.24.14"\n')
        self.driver = self.root / 'driver'
        self.driver.mkdir()
        self.resources = self.root / 'resources'
        (self.resources / 'web-driver').mkdir(parents=True)
        image = b'archive fixture'
        (self.driver / 'jarvis-web-driver-linux-x86_64.tar.gz').write_bytes(image)
        (self.driver / 'jarvis-web-driver-linux-x86_64.tar.gz.sha256').write_text(
            f'{hashlib.sha256(image).hexdigest()}  jarvis-web-driver-linux-x86_64.tar.gz\n'
        )
        (self.driver / 'jarvis-web-driver-linux-x86_64.image-id').write_text('sha256:' + 'a' * 64 + '\n')
        self.bins = self.root / 'bin'
        self.bins.mkdir()
        self.sha = 'a' * 40

    def pack(self, target='linux-x86_64', create_bins=True, **changes):
        if create_bins:
            for name in USER_FACING[target] + HELPERS[target]:
                (self.bins / name).write_bytes(b'fixture executable')
        arguments = dict(source=self.source, source_commit=self.sha, observed_commit=self.sha,
                         target=target, rust_target='x86_64-unknown-linux-gnu',
                         compiler='rustc 1.95.0 (fixture)', driver_run='123',
                         driver_arch='linux-x86_64', driver_dir=self.driver,
                         bin_dir=self.bins, resources=self.resources,
                         output=self.root / TARGETS[target], version='0.24.14')
        arguments.update(changes)
        return validate_and_pack(**arguments)

    def test_stage6_package_and_complete_provenance(self):
        receipt = self.pack()
        self.assertEqual(receipt['source_commit'], self.sha)
        self.assertEqual(receipt['driver']['run_id'], '123')
        self.assertEqual(receipt['lockfile_sha256'], hashlib.sha256(b'lock fixture\n').hexdigest())
        self.assertEqual(receipt['archive'], TARGETS['linux-x86_64'])
        self.assertEqual(len(receipt['binaries']), 4)
        self.assertTrue((self.root / (TARGETS['linux-x86_64'] + '.candidate.json')).is_file())

    def test_all_five_canonical_targets_reuse_stage6_archive_names(self):
        arm = 'jarvis-web-driver-linux-aarch64'
        for ext in ('tar.gz', 'tar.gz.sha256', 'image-id'):
            original = self.driver / f'jarvis-web-driver-linux-x86_64.{ext}'
            (self.driver / f'{arm}.{ext}').write_bytes(original.read_bytes().replace(
                b'jarvis-web-driver-linux-x86_64.tar.gz', f'{arm}.tar.gz'.encode()))
        triples = {
            'linux-x86_64': 'x86_64-unknown-linux-gnu',
            'linux-aarch64': 'aarch64-unknown-linux-gnu',
            'darwin-x86_64': 'x86_64-apple-darwin',
            'darwin-aarch64': 'aarch64-apple-darwin',
            'windows-x86_64': 'x86_64-pc-windows-msvc',
        }
        for target, triple in triples.items():
            with self.subTest(target=target):
                dest = self.root / target / TARGETS[target]
                arch = 'linux-aarch64' if target in {'linux-aarch64', 'darwin-aarch64'} else 'linux-x86_64'
                receipt = self.pack(target, rust_target=triple, driver_arch=arch, output=dest)
                self.assertEqual(receipt['archive'], TARGETS[target])
                self.assertEqual(receipt['driver']['arch'], arch)
                self.assertTrue(dest.is_file())

    def test_wrong_sha_target_helper_driver_or_archive_refuses_without_output(self):
        bad = [dict(observed_commit='b' * 40), dict(source_commit='abc'),
               dict(rust_target='aarch64-unknown-linux-gnu'),
               dict(driver_arch='linux-aarch64'),
               dict(output=self.root / 'wrong.tar.gz'), dict(version='0.24.13')]
        for kwargs in bad:
            with self.subTest(kwargs=kwargs):
                with self.assertRaises(ValueError):
                    self.pack(**kwargs)
                self.assertFalse((self.root / 'wrong.tar.gz').exists())
        self.pack()
        (self.bins / 'jarvis-web-fetch').unlink()
        with self.assertRaises(ValueError):
            self.pack(create_bins=False)
        (self.driver / 'jarvis-web-driver-linux-x86_64.tar.gz.sha256').write_text('0' * 64)
        with self.assertRaises(ValueError):
            self.pack()


if __name__ == '__main__':
    unittest.main()
