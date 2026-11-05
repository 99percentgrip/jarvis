"""Fail-closed mirror rotation regression for the real browser snapshot helper."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / '.github/snapshot-apt.sh'


class MirrorTests(unittest.TestCase):
    def run_case(self, failure):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            sources = root / 'sources.list'
            sources.write_text('deb [check-valid-until=no] http://snapshot.debian.org/archive/debian/20261005T000000Z bookworm main\n'
                               'deb [check-valid-until=no] http://snapshot.debian.org/archive/debian-security/20261005T000000Z bookworm-security main\n')
            mock = root / 'apt-get'
            mock.write_text('''#!/bin/sh
printf '%s | %s\\n' "$(head -n 1 "$SNAPSHOT_SOURCES")" "$*" >> "$MOCK_LOG"
case "$*" in *' update') phase=update;; *' install '*) phase=install;; *) exit 83;; esac
case "$MOCK_FAILURE" in
 timeout) case "$(cat "$SNAPSHOT_SOURCES")" in *snapshot-cloudflare*) exit 0;; esac; echo 'Ign:7 http://snapshot.debian.org/archive/debian/20261005T000000Z bookworm/main amd64 package' >&2; exit 124;;
 dpkg) case "$*" in *'--no-download'*) echo 'E: Sub-process /usr/bin/dpkg returned an error code (1)' >&2; exit 100;; esac;;
 unsigned) echo 'E: The repository is not signed' >&2; exit 100;;
 transient) case "$(cat "$SNAPSHOT_SOURCES")" in *snapshot-cloudflare*) exit 0;; esac; echo 'E: Failed to fetch http://snapshot.debian.org/file/blob/package.deb  503  No healthy backends' >&2; exit 100;;
 404) echo 'E: Failed to fetch http://snapshot.debian.org/file/blob/package.deb  404  Not Found' >&2; exit 100;;
 corrupt) echo 'E: Failed to fetch http://snapshot.debian.org/file/blob/package.deb  Hash Sum mismatch' >&2; exit 100;;
 always) echo 'E: Failed to fetch http://snapshot.debian.org/file/blob/package.deb  504  Gateway Timeout' >&2; exit 100;;
esac
exit 0
''')
            mock.chmod(0o755)
            sleep = root / 'sleep'
            sleep.write_text('#!/bin/sh\n: \n')
            sleep.chmod(0o755)
            env = dict(os.environ, PATH=f'{root}:{os.environ["PATH"]}',
                       SNAPSHOT_SOURCES=str(sources), MOCK_LOG=str(root / 'calls'),
                       MOCK_FAILURE=failure, HEADLESS_VERSION='154.0.8037.92-1~deb12u1')
            result = subprocess.run(['sh', str(SCRIPT)], env=env, text=True, capture_output=True, timeout=20)
            calls = (root / 'calls').read_text().splitlines()
            return result, calls, sources.read_text()

    def test_transport_failure_switches_only_to_same_timestamp_signed_snapshot(self):
        result, calls, sources = self.run_case('transient')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('snapshot-cloudflare.debian.org/archive/debian/20261005T000000Z', sources)
        self.assertTrue(any('snapshot-cloudflare.debian.org' in line and ' update' in line for line in calls))
        self.assertTrue(any('snapshot-cloudflare.debian.org' in line and ' install ' in line and
                            'chromium-headless-shell=154.0.8037.92-1~deb12u1' in line for line in calls))
        self.assertIn('snapshot-cloudflare.debian.org/archive/debian-security/20261005T000000Z', sources)
        self.assertTrue(all('APT::Update::Error-Mode=any' in line for line in calls[:-1]))
        self.assertIn('--no-download install', calls[-1])

    def test_bounded_download_timeout_can_switch_without_package_scripts(self):
        result, calls, sources = self.run_case('timeout')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('snapshot-cloudflare.debian.org', sources)
        self.assertIn('--no-download install', calls[-1])

    def test_missing_artifact_never_switches(self):
        result, calls, sources = self.run_case('404')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('snapshot-cloudflare.debian.org', sources)

    def test_integrity_error_never_switches(self):
        result, calls, sources = self.run_case('corrupt')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('snapshot-cloudflare.debian.org', sources)

    def test_unsigned_metadata_never_switches(self):
        result, calls, sources = self.run_case('unsigned')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('snapshot-cloudflare.debian.org', sources)

    def test_installation_failure_is_not_retried_or_switched(self):
        result, calls, sources = self.run_case('dpkg')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(calls), 3, calls)
        self.assertIn('--no-download install', calls[-1])
        self.assertNotIn('snapshot-cloudflare.debian.org', sources)

    def test_both_snapshot_frontends_unavailable_fails_bounded(self):
        result, calls, sources = self.run_case('always')
        self.assertNotEqual(result.returncode, 0)
        self.assertLessEqual(len(calls), 8)
        self.assertIn('snapshot-cloudflare.debian.org', sources)


if __name__ == '__main__':
    unittest.main()
