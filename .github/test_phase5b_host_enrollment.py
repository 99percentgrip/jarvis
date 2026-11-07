"""Guard Linux production-host auth acceptance enrollment, not just test filenames."""
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class Phase5BHostEnrollment(unittest.TestCase):
    def test_linux_foundation_executes_both_real_hosts_under_private_offline_manager(self):
        workflow = (ROOT / '.github/workflows/platform-foundation.yml').read_text()
        step = workflow.split('name: Verify Phase 5B production host native credential lifecycle', 1)[1].split('\n      - name:', 1)[0]
        self.assertIn("matrix.platform_spike == 'linux'", step)
        self.assertIn('bwrap --unshare-net', step)
        self.assertIn('.github/linux_secret_service_fixture.py', step)
        for host, binary in [('jarvis-tui', 'jarvis-tui'), ('jarvis-acp', 'jarvis-acp')]:
            self.assertIn(f'apps/{host}/tests/phase5b_native_', step)
            self.assertIn(f'target/debug/{binary}', step)
        self.assertNotIn('continue-on-error:', step)
        self.assertIn('timeout-minutes:', step)


if __name__ == '__main__':
    unittest.main()
