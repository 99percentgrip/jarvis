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

    def test_sentinel_bytes_are_canonical_and_preserved_by_actual_native_path(self):
        import os
        from unittest.mock import patch
        from jarvis_candidate_acceptance import run_native_install
        seen = []
        def simulate_lifecycle(command, **kwargs):
            sentinel = Path(kwargs['env']['JARVIS_HOME']) / 'unrelated-user-data.txt'
            seen.append((command, sentinel.read_bytes()))
            return None
        with tempfile.TemporaryDirectory() as raw:
            with patch.dict(os.environ, {'GITHUB_ACTIONS': 'true', 'RUNNER_TEMP': raw}):
                with patch('jarvis_candidate_acceptance.native_target', return_value=True), \
                     patch('jarvis_candidate_acceptance.subprocess.run', side_effect=simulate_lifecycle):
                    run_native_install(Path(raw) / 'archive', Path(raw) / 'checksum',
                                       'windows-x86_64', Path(raw))
        self.assertEqual(len(seen), 1)
        self.assertEqual(seen[0][1], b'preserve\n')
        self.assertIn('test_jarvis_candidate_install.ps1', seen[0][0][-1])

    def test_sentinel_refuses_invalid_fixture_and_post_lifecycle_mutations(self):
        import os
        from unittest.mock import patch
        from jarvis_candidate_acceptance import run_native_install
        def run_with_mutation(value):
            def mutate(command, **kwargs):
                path = Path(kwargs['env']['JARVIS_HOME']) / 'unrelated-user-data.txt'
                if value is None:
                    path.unlink()
                else:
                    path.write_bytes(value)
            with tempfile.TemporaryDirectory() as raw:
                with patch.dict(os.environ, {'GITHUB_ACTIONS': 'true', 'RUNNER_TEMP': raw}):
                    with patch('jarvis_candidate_acceptance.native_target', return_value=True), \
                         patch('jarvis_candidate_acceptance.subprocess.run', side_effect=mutate):
                        run_native_install(Path(raw) / 'archive', Path(raw) / 'checksum',
                                           'windows-x86_64', Path(raw))
        for bad in (b'preserve\r\n', b'changed\n', b'preserve', b'preserve\nX', None):
            with self.subTest(bad=bad), self.assertRaises((ValueError, FileNotFoundError)):
                run_with_mutation(bad)

    def test_initial_fixture_and_redirected_path_fail_closed(self):
        from jarvis_candidate_acceptance import require_sentinel_bytes
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            path = root / 'sentinel'
            with self.assertRaisesRegex(ValueError, 'invalid initial sentinel fixture'):
                require_sentinel_bytes(path, 'invalid initial sentinel fixture')
            path.write_bytes(b'preserve\r\n')
            with self.assertRaisesRegex(ValueError, 'invalid initial sentinel fixture'):
                require_sentinel_bytes(path, 'invalid initial sentinel fixture')
            path.write_bytes(b'preserve\n')
            require_sentinel_bytes(path, 'invalid initial sentinel fixture')
            path.unlink()
            other = root / 'other'
            other.write_bytes(b'preserve\n')
            path.symlink_to(other)
            with self.assertRaisesRegex(ValueError, 'invalid initial sentinel fixture'):
                require_sentinel_bytes(path, 'invalid initial sentinel fixture')

    def test_windows_acceptance_reuses_native_installer_and_restores_user_path(self):
        script = (Path(__file__).resolve().parent / 'test_jarvis_candidate_install.ps1').read_text()
        for token in ("'install.ps1'", "'uninstall.ps1'", "$env:RUNNER_TEMP",
                      "[Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames",
                      "$pathKey.SetValue('Path', $originalUserPath, $originalPathKind)",
                      "$pathKey.DeleteValue('Path', $false)",
                      "$originalFailure = $_", "$env:Path = $originalProcessPath",
                      "'unrelated-user-data.txt'"):
            self.assertIn(token, script)
        self.assertIn('first_launch_pty.py', script)
        self.assertNotIn('ZipFile', script)
        guard = "Assert-JarvisCandidateSentinel -Path $sentinel"
        self.assertEqual(script.count(guard), 2)
        self.assertLess(script.index('invalid initial sentinel fixture'), script.index("'install.ps1'"))
        self.assertGreater(script.index('uninstall altered foreign sentinel'), script.index("'uninstall.ps1'"))
        self.assertIn(". (Join-Path $PSScriptRoot 'jarvis_candidate_sentinel.ps1')", script)
        native_test = (Path(__file__).resolve().parent / 'test_jarvis_candidate_sentinel.ps1').read_text()
        self.assertIn(". (Join-Path $PSScriptRoot 'jarvis_candidate_sentinel.ps1')", native_test)

    def test_windows_candidate_selects_pinned_python_before_hash_verified_install(self):
        workflow = (Path(__file__).resolve().parents[1] / '.github/workflows/release.yml').read_text()
        setup = '      - name: Select Windows candidate acceptance Python\n'
        prepare = '      - name: Prepare Windows native terminal acceptance\n'
        accept = '      - name: Verify and install exact candidate archive\n'
        upload = '      - name: Upload nonpublishing candidate artifact\n'
        self.assertEqual(workflow.count(setup), 1)
        self.assertLess(workflow.index(setup), workflow.index(prepare))
        self.assertLess(workflow.index(prepare), workflow.index(accept))
        self.assertLess(workflow.index(accept), workflow.index(upload))
        setup_step = workflow.split(setup, 1)[1].split('      - name:', 1)[0]
        prepare_step = workflow.split(prepare, 1)[1].split('      - name:', 1)[0]
        self.assertIn("if: matrix.package_target == 'windows-x86_64'", setup_step)
        self.assertIn('uses: actions/setup-python@v6', setup_step)
        self.assertIn("python-version: '3.13'", setup_step)
        self.assertIn("architecture: 'x64'", setup_step)
        self.assertNotIn('cache:', setup_step)
        self.assertIn("if: matrix.package_target == 'windows-x86_64'", prepare_step)
        assertion = 'python -c "import struct,sys; ok = sys.platform == \'win32\' and sys.implementation.name == \'cpython\' and sys.version_info[:2] == (3,13) and struct.calcsize(\'P\') == 8; sys.exit(0 if ok else \'Expected 64-bit CPython 3.13 for Windows candidate acceptance\')"'
        pip = 'python -m pip install --only-binary=:all: --require-hashes -r apps/jarvis-tui/tests/windows-terminal-requirements.txt'
        self.assertLess(prepare_step.index(assertion), prepare_step.index(pip))
        requirements = (Path(__file__).resolve().parents[1] / 'apps/jarvis-tui/tests/windows-terminal-requirements.txt').read_text()
        self.assertIn('pywinpty==3.0.5 --hash=sha256:48db1b0ad9d0a1b81dcaaa7163a99a7808deaceb0c1b2344716dc1fc090c3c4c', requirements)
        self.assertIn('${{ matrix.archive }}.accepted.json', workflow.split(upload, 1)[1])
        self.assertIn('permissions:\n  actions: read\n  contents: read', workflow)
        self.assertNotIn('continue-on-error: true', workflow)
        self.assertEqual(workflow.count('package_target:'), 5)

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
