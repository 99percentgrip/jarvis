"""Offline exact-archive candidate acceptance and workflow ordering regressions."""
import json
from pathlib import Path
import tempfile
import unittest

from jarvis_package import TARGETS
from test_jarvis_release_candidate import CandidateTests


class CandidateArchiveTests(CandidateTests):
    def test_exact_archive_receipt_validates_and_rejects_mutations(self):
        from jarvis_candidate_acceptance import validate_candidate_archive
        receipt = self.pack()
        archive = self.root / TARGETS['linux-x86_64']
        provenance = archive.with_name(archive.name + '.candidate.json')
        checksum = archive.with_name(archive.name + '.sha256')
        manifest = validate_candidate_archive(archive, checksum, provenance,
            target='linux-x86_64', commit=self.sha, driver_run='123',
            driver_arch='linux-x86_64', driver_dir=self.driver, source=self.source,
            check_native_binary=False)
        self.assertEqual(manifest['source_commit'], self.sha)
        for path, changed in [(provenance, lambda data: {**data, 'source_commit': 'b'*40}),
                              (provenance, lambda data: {**data, 'driver': {**data['driver'], 'image_id': 'sha256:'+'b'*64}})]:
            original = path.read_text()
            path.write_text(json.dumps(changed(json.loads(original))))
            with self.assertRaises(ValueError):
                validate_candidate_archive(archive, checksum, provenance,
                    target='linux-x86_64', commit=self.sha, driver_run='123',
                    driver_arch='linux-x86_64', driver_dir=self.driver, source=self.source,
                    check_native_binary=False)
            path.write_text(original)
        archive.write_bytes(archive.read_bytes() + b'tamper')
        with self.assertRaises(Exception):
            validate_candidate_archive(archive, checksum, provenance,
                target='linux-x86_64', commit=self.sha, driver_run='123',
                driver_arch='linux-x86_64', driver_dir=self.driver, source=self.source,
                check_native_binary=False)

    def test_acceptance_rejects_rewritten_checksum_and_provenance_bytes(self):
        from jarvis_candidate_acceptance import require_unchanged_sidecars
        from jarvis_package import sha256_file
        self.pack()
        archive = self.root / TARGETS['linux-x86_64']
        checksum = archive.with_name(archive.name + '.sha256')
        provenance = archive.with_name(archive.name + '.candidate.json')
        before = {path: sha256_file(path) for path in (archive, checksum, provenance)}
        require_unchanged_sidecars(archive, checksum, provenance, before)
        for sidecar in (checksum, provenance):
            original = sidecar.read_bytes()
            sidecar.write_bytes(original + b' ')
            with self.assertRaisesRegex(ValueError, 'sidecar bytes changed'):
                require_unchanged_sidecars(archive, checksum, provenance, before)
            sidecar.write_bytes(original)

    def test_native_binary_headers_and_local_install_refusal(self):
        import os
        from unittest.mock import patch
        from jarvis_candidate_acceptance import binary_identity, run_native_install
        binary = self.root / 'host'
        for target, machine in [('linux-x86_64', 62), ('linux-aarch64', 183)]:
            binary.write_bytes(b'\x7fELF\x02' + b'\0' * 13 + machine.to_bytes(2, 'little'))
            self.assertTrue(binary_identity(binary, target))
        binary.write_bytes(b'not an executable')
        self.assertFalse(binary_identity(binary, 'linux-x86_64'))
        with patch.dict(os.environ, {'GITHUB_ACTIONS': 'false', 'RUNNER_TEMP': str(self.root)}):
            with self.assertRaises(ValueError):
                run_native_install(binary, binary, 'linux-x86_64', self.source)

    def test_windows_acceptance_reuses_native_installer_and_restores_user_path(self):
        script = (Path(__file__).resolve().parent / 'test_jarvis_candidate_install.ps1').read_text()
        for token in ("'install.ps1'", "'uninstall.ps1'", "$env:RUNNER_TEMP",
                      "[Environment]::SetEnvironmentVariable('Path', $originalUserPath, 'User')",
                      "'unrelated-user-data.txt'"):
            self.assertIn(token, script)
        self.assertIn('first_launch_pty.py', script)
        self.assertNotIn('ZipFile', script)

    def test_workflow_installs_exact_archive_before_upload_and_refuses_failed_acceptance(self):
        workflow = (Path(__file__).resolve().parents[1] / '.github/workflows/release.yml').read_text()
        self.assertLess(workflow.index('Assemble with canonical Stage 6 package builder'),
                        workflow.index('Verify and install exact candidate archive'))
        self.assertLess(workflow.index('Verify and install exact candidate archive'),
                        workflow.index('Upload nonpublishing candidate artifact'))
        self.assertIn('scripts/jarvis_candidate_acceptance.py', workflow)
        self.assertIn('windows-terminal-requirements.txt', workflow)
        from jarvis_candidate_acceptance import run_native_install
        import inspect
        self.assertIn('verify_signed_out_launch(bundle', inspect.getsource(run_native_install))
        self.assertIn('${{ matrix.archive }}.accepted.json', workflow)
        self.assertNotIn('continue-on-error: true', workflow)


if __name__ == '__main__':
    unittest.main()
