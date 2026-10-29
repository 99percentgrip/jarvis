#!/usr/bin/env python3
"""Bounded, secret-safe diagnostics for an offline ACP child."""
import os
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
from release_hosts_pty import Acp


class AcpDiagnostics(unittest.TestCase):
    def test_early_exit_classifies_without_disclosing_private_stderr(self):
        with tempfile.TemporaryDirectory() as directory:
            script = Path(directory) / 'child.py'
            script.write_text("import sys\nsys.stderr.write('Private application storage ownership refused; SECRET_FIXTURE_TOKEN\\n')\nsys.exit(7)\n")
            child = Acp((sys.executable, script), directory, dict(os.environ, PYTHONPATH=''))
            try:
                with self.assertRaisesRegex(AssertionError, 'exit=7; stderr_class=application ownership') as caught:
                    child.rpc(1, 'initialize', dict(protocolVersion=1))
                self.assertNotIn('SECRET_FIXTURE_TOKEN', str(caught.exception))
            finally:
                child.close()

    def test_deadline_reaps_owned_child(self):
        with tempfile.TemporaryDirectory() as directory:
            script = Path(directory) / 'child.py'
            script.write_text('import time\ntime.sleep(120)\n')
            child = Acp((sys.executable, script), directory, dict(os.environ, PYTHONPATH=''))
            try:
                error = child.failure('response deadline exceeded')
                self.assertIn('response deadline exceeded', str(error))
                self.assertIsNotNone(child.child.poll())
            finally:
                child.close()


if __name__ == '__main__':
    unittest.main()
