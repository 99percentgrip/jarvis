"""Fail closed when hosted Linux credential host tests need secret-tool."""
import importlib.util
from pathlib import Path
from unittest import TestCase, main
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('credential_prep', Path(__file__).with_name('prepare-credential-runner.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class NativePrerequisiteTests(TestCase):
    def test_native_runner_installs_secret_tool_if_other_native_tools_are_ready(self):
        allowed = {'GITHUB_ACTIONS': 'true', 'RUNNER_OS': 'Linux', 'RUNNER_ENVIRONMENT': 'github-hosted'}
        with patch.object(module.subprocess, 'run') as run, patch.object(module.shutil, 'which') as which:
            installed = [False]
            def execute(command, **kwargs):
                if command[:4] == ['sudo', 'apt-get', 'install', '-y']:
                    installed[0] = True
                return type('Result', (), {'returncode': 0})()
            run.side_effect = execute
            which.side_effect = lambda name: (None if name == 'secret-tool' and not installed[0]
                                             else '/usr/bin/' + name)
            module.prepare(allowed, native=True)
            commands = [call.args[0] for call in run.call_args_list]
            self.assertIn(['sudo', 'apt-get', 'install', '-y', 'libdbus-1-dev', 'pkg-config',
                           'gnome-keyring', 'dbus', 'dbus-x11', 'libsecret-tools'], commands)


if __name__ == '__main__':
    main()
