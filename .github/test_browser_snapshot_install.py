"""Offline regression for the pinned browser image's bounded APT acquisition."""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / ".github" / "snapshot-apt.sh"
DOCKERFILE = ROOT / "crates" / ("jar" + "vis-web-fetch") / "Dockerfile"
VERSION = "154.0.8037.92-1~deb12u1"


class SnapshotInstallTests(unittest.TestCase):
    def invoke(self, failures):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root / "apt-get").write_text(
                "#!/bin/sh\n"
                "printf '%s\\n' \"$*\" >> \"$MOCK_LOG\"\n"
                "case \"$*\" in *' update') phase=update;; *' install '*) phase=install;; *) exit 83;; esac\n"
                "count_file=\"$MOCK_ROOT/$phase\"\n"
                "count=0; test ! -f \"$count_file\" || count=$(cat \"$count_file\")\n"
                "count=$((count + 1)); printf '%s' \"$count\" > \"$count_file\"\n"
                "case \"$MOCK_FAILURES\" in \"$phase:$count\"|\"$phase-all\"|always) exit 100;; esac\n"
                "exit 0\n"
            )
            (root / "apt-get").chmod(0o755)
            (root / "sleep").write_text("#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$MOCK_SLEEPS\"\n")
            (root / "sleep").chmod(0o755)
            env = dict(os.environ, PATH=f"{root}:{os.environ['PATH']}", HEADLESS_VERSION=VERSION,
                       MOCK_ROOT=str(root), MOCK_LOG=str(root / "calls"),
                       MOCK_SLEEPS=str(root / "sleeps"), MOCK_FAILURES=failures)
            result = subprocess.run(["sh", str(SCRIPT)], env=env, text=True, capture_output=True)
            calls = (root / "calls").read_text().splitlines() if (root / "calls").exists() else []
            sleeps = (root / "sleeps").read_text().splitlines() if (root / "sleeps").exists() else []
            return result, calls, sleeps

    def test_single_snapshot_fetch_failure_retries_same_pinned_install(self):
        result, calls, sleeps = self.invoke("install:1")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(calls), 3, calls)
        self.assertIn(" update", calls[0])
        self.assertEqual(calls[1], calls[2])
        self.assertIn("chromium-headless-shell=" + VERSION, calls[1])
        self.assertIn("ca-certificates", calls[1])
        self.assertIn("libcap-ng0 procps tini", calls[1])
        self.assertEqual(len(sleeps), 1)
        for call in calls:
            self.assertIn("Acquire::Retries=2", call)
            self.assertIn("Acquire::http::Timeout=30", call)
            self.assertIn("Acquire::https::Timeout=30", call)

    def test_transient_metadata_failure_retries_before_install(self):
        result, calls, sleeps = self.invoke("update:1")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(calls), 3, calls)
        self.assertEqual(calls[0], calls[1])
        self.assertIn("APT::Update::Error-Mode=any", calls[0])
        self.assertIn("chromium-headless-shell=" + VERSION, calls[2])
        self.assertEqual(len(sleeps), 1)

    def test_persistent_install_failure_refuses_unpinned_fallback(self):
        # The fake fails every install while update succeeds: no alternative
        # version, mirror or success receipt is ever admitted.
        result, calls, sleeps = self.invoke("install-all")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(calls), 3, calls)
        self.assertEqual(calls[1], calls[2])
        self.assertIn("chromium-headless-shell=" + VERSION, calls[2])
        self.assertEqual(len(sleeps), 1)

    def test_persistent_snapshot_failure_exits_without_unpinned_fallback(self):
        result, calls, sleeps = self.invoke("always")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(calls), 2, calls)
        self.assertTrue(all(" update" in call for call in calls))
        self.assertEqual(len(sleeps), 1)

    def test_dockerfile_keeps_exact_snapshot_and_image_pins(self):
        text = DOCKERFILE.read_text()
        self.assertIn("ARG DEBIAN_SNAPSHOT=20261005T000000Z", text)
        self.assertIn("ARG HEADLESS_VERSION=" + VERSION, text)
        self.assertIn("snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT}", text)
        self.assertIn("snapshot.debian.org/archive/debian-security/${DEBIAN_SNAPSHOT}", text)
        self.assertIn("@sha256:88200866dfff7ea7f5cbcb6ec7c8a701889efe6fe859fe64d6990e4b07ea4171", text)
        self.assertIn("COPY .github/snapshot-apt.sh /tmp/snapshot-apt.sh", text)
        self.assertIn("/bin/sh /tmp/snapshot-apt.sh", text)
        self.assertIn("python3 .github/test_browser_snapshot_install.py", (ROOT / ".github/workflows/ci.yml").read_text())


if __name__ == "__main__":
    unittest.main()
