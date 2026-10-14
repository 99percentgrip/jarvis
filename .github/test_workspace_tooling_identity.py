"""Validate executed helper selectors against actual workspace metadata, without native side effects."""
import ast
import json
from pathlib import Path
import re
import subprocess
import unittest

ROOT = Path(__file__).resolve().parent.parent


class ToolingIdentity(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        metadata = json.loads(subprocess.check_output([
            'cargo', 'metadata', '--locked', '--offline', '--no-deps',
            '--format-version', '1'], cwd=ROOT, text=True))
        cls.packages = {p['name']: p for p in metadata['packages']
                        if p['id'] in metadata['workspace_members']}

    def test_windows_governor_selects_actual_package_and_library_artifact(self):
        script = ROOT / 'crates/jarvis-harness/tests/windows_resource_job.py'
        tree = ast.parse(script.read_text())
        commands = [ast.literal_eval(n.args[0]) for n in ast.walk(tree)
                    if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute)
                    and n.func.attr == 'run' and n.args and isinstance(n.args[0], ast.List)]
        cargo = [cmd for cmd in commands if cmd[:2] == ['cargo', 'test']]
        self.assertEqual(len(cargo), 1)
        command = cargo[0]
        package = self.packages[command[command.index('-p') + 1]]
        self.assertIn('--no-run', command)
        self.assertIn('--all-features', command)
        libraries = {t['name'] for t in package['targets'] if 'lib' in t['kind']}
        selectors = [n.comparators[0].value for n in ast.walk(tree)
                     if isinstance(n, ast.Compare) and len(n.comparators) == 1
                     and isinstance(n.comparators[0], ast.Constant)
                     and isinstance(n.comparators[0].value, str)
                     and ast.unparse(n.left) == "row['target']['name']"]
        self.assertEqual(set(selectors), libraries)

    def test_lens_launchers_select_actual_example_and_existing_review_fixture(self):
        for filename in ['vesper_lens_browser.mjs', 'vesper_lens_interview_browser.mjs']:
            with self.subTest(filename=filename):
                script = ROOT / 'crates/jarvis-agent/tests' / filename
                source = script.read_text()
                launch = re.search(r'spawn\("cargo", \[([^\]]+)\]', source)
                self.assertIsNotNone(launch)
                command = re.findall(r'"([^"\n]+)"', launch[1])
                package = self.packages[command[command.index('-p') + 1]]
                example = command[command.index('--example') + 1]
                self.assertIn(example, {t['name'] for t in package['targets'] if 'example' in t['kind']})
                fixture = re.search(r'const fixture = "([^"]+)"', source)
                if filename == 'vesper_lens_browser.mjs':
                    self.assertIsNotNone(fixture)
                    self.assertTrue((ROOT / fixture[1]).is_file())
                else:
                    self.assertIn('--interview', command)


if __name__ == '__main__':
    unittest.main()
