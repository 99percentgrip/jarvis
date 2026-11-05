#!/usr/bin/env python3
"""Real TUI API-key lifecycle on an isolated private Linux Secret Service.

Run only under .github/linux_secret_service_fixture.py and a network namespace.
No caller credential manager, cloud traffic or user profile is accessible.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import uuid

from settings_pty import Host, fixture_owned_directory


def run_one(binary, provider):
    private = Path(os.environ['NATIVE_SECRET_SERVICE_FIXTURE_HOME'])
    assert os.environ['NATIVE_SECRET_SERVICE_FIXTURE'] == 'unlocked-private-bus'
    assert Path.home() == private and (private / '.native-credential-test-home').is_file()
    assert os.environ['DBUS_SESSION_BUS_ADDRESS'].startswith(f'unix:path={private}/run/')
    with tempfile.TemporaryDirectory(prefix='jarvis-phase5b-host-') as temporary:
        root = Path(temporary)
        fixture_owned_directory(root / 'config')
        key = 'synthetic-' + uuid.uuid4().hex
        rotated = 'synthetic-' + uuid.uuid4().hex
        env = {'JARVIS_HOME': str(root / 'state'), 'ZAI_API_KEY': '',
               'JARVIS_VRO_ENABLED': '0', 'DBUS_SESSION_BUS_ADDRESS': os.environ['DBUS_SESSION_BUS_ADDRESS'],
               'XDG_RUNTIME_DIR': str(private / 'run'),
               f'JARVIS_{provider.upper()}_CREDENTIALS_PATH': str(root / 'config' / f'native-{provider}.json')}

        def stored_key():
            result = subprocess.run(['secret-tool', 'lookup', 'service', '99percentgrip.jarvis',
                                     'username', f'{provider}:native-auth'], capture_output=True,
                                    text=True, timeout=10)
            if result.returncode == 1 and not result.stdout and not result.stderr:
                return None
            assert result.returncode == 0 and result.stdout, 'private native read failed'
            record = json.loads(result.stdout)
            if record['mode'] == 'signed-out':
                assert 'api_key' not in record
                return None
            assert record['mode'] == 'api-key'
            return record['api_key']

        def open_auth(host):
            host.wait('Start coding')
            host.key('s')
            host.choose('Providers')
            host.choose(provider)
            host.choose(f'Manage authentication · {provider}')
            host.wait('Authentication')
            host.choose('API key (usage-based billing)' if provider == 'openai'
                        else 'xAI API key (usage-based API billing)')

        def quit(host):
            host.key('\x1b')
            host.key('\x1b')
            host.key('\x1b')
            host.wait('Start coding')
            host.key('q')
            assert host.wait_exit(10) == 0

        first = Host(binary, root, env)
        try:
            open_auth(first)
            first.choose('Enter or replace API key')
            first.wait('masked')
            first.key(key)
            assert key not in first.text()
            first.choose('Save and use API key')
            first.wait('API key saved and selected immediately')
            assert stored_key() == key, 'TUI save differs from native value'
            assert key not in first.raw
            quit(first)
        finally:
            first.close()
        # A new host process must see the persisted method; the original process
        # has exited and cannot supply an in-memory credential or selection.
        second = Host(binary, root, env)
        try:
            open_auth(second)
            second.wait('Remove stored API key')
            assert stored_key() == key, 'new TUI process lost saved native value'
            assert key not in second.raw
            second.choose('Enter or replace API key')
            second.key(rotated)
            second.choose('Save and use API key')
            second.wait('API key saved and selected immediately')
            assert stored_key() == rotated, 'TUI rotation differs from native value'
            assert rotated not in second.raw
            quit(second)
        finally:
            second.close()
        third = Host(binary, root, env)
        try:
            open_auth(third)
            third.wait('Remove stored API key')
            assert stored_key() == rotated, 'new TUI process lost rotated native value'
            assert key not in third.raw and rotated not in third.raw
            third.choose('Remove stored API key')
            third.choose('Confirm')
            third.wait('removed')
            assert stored_key() is None, 'TUI logout left native API key readable'
            quit(third)
        finally:
            third.close()
        fourth = Host(binary, root, env)
        try:
            open_auth(fourth)
            assert 'Remove stored API key' not in fourth.text(), fourth.text()
            assert key not in fourth.raw and rotated not in fourth.raw
            quit(fourth)
        finally:
            fourth.close()
        assert not (root / 'config' / f'native-{provider}.json').exists(), 'native save wrote fallback vault'
    print(f'PASS: real TUI private-native {provider} API-key save, exact reload, rotate, remove, restart absence; no cloud requests')


def run(binary):
    for provider in ('openai', 'xai'):
        run_one(binary, provider)


if __name__ == '__main__':
    run(str(Path(sys.argv[1]).resolve()))
