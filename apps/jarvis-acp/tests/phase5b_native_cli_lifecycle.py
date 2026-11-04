#!/usr/bin/env python3
"""Real ACP executable --setup/--check-auth/--logout on a private Linux bus.

Must run under .github/linux_secret_service_fixture.py in a network namespace.
This tests the CLI host controls, not an in-memory provider test factory.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import uuid


def run(binary):
    fixture = Path(os.environ['NATIVE_SECRET_SERVICE_FIXTURE_HOME'])
    assert os.environ['NATIVE_SECRET_SERVICE_FIXTURE'] == 'unlocked-private-bus'
    assert Path.home() == fixture and (fixture / '.native-credential-test-home').is_file()
    assert os.environ['DBUS_SESSION_BUS_ADDRESS'].startswith(f'unix:path={fixture}/run/')
    with tempfile.TemporaryDirectory(prefix='jarvis-phase5b-acp-') as temporary:
        root = Path(temporary)
        auth = root / 'auth'
        auth.mkdir(mode=0o700)
        # Native OpenAI account is fixed by the real adapter; the private bus
        # isolates it instead of claiming UUID account isolation in production.
        path = auth / 'openai.json'
        foreign = auth / 'foreign.json'
        base = {'PATH': os.environ['PATH'], 'HOME': str(root), 'USERPROFILE': str(root),
                'XDG_CONFIG_HOME': str(root / 'config'), 'XDG_DATA_HOME': str(root / 'data'),
                'XDG_STATE_HOME': str(root / 'state'), 'XDG_CACHE_HOME': str(root / 'cache'),
                'XDG_RUNTIME_DIR': str(fixture / 'run'),
                'DBUS_SESSION_BUS_ADDRESS': os.environ['DBUS_SESSION_BUS_ADDRESS'],
                'JARVIS_OPENAI_CREDENTIALS_PATH': str(path),
                'JARVIS_XAI_CREDENTIALS_PATH': str(auth / 'xai.json'), 'JARVIS_PROVIDER': 'openai',
                'HTTP_PROXY': 'http://127.0.0.1:9', 'HTTPS_PROXY': 'http://127.0.0.1:9',
                'ALL_PROXY': 'http://127.0.0.1:9', 'NO_PROXY': ''}
        first, rotated = ('synthetic-' + uuid.uuid4().hex for _ in range(2))

        def call(flag, *, key=None, vault=None, provider='openai'):
            env = dict(base)
            if key is not None:
                env['OPENAI_API_KEY'] = key
            if vault is not None:
                env['JARVIS_OPENAI_CREDENTIALS_PATH'] = str(vault)
            result = subprocess.run([binary, '--provider', provider, flag], env=env,
                                    cwd=root, capture_output=True, text=True, timeout=15)
            for secret in (first, rotated):
                assert secret not in result.stderr + result.stdout, 'ACP emitted synthetic secret'
            assert not result.stdout, 'ACP protocol stdout was polluted'
            return result

        assert call('--check-auth').returncode != 0
        first_setup = call('--setup', key=first)
        assert first_setup.returncode == 0, first_setup.stderr
        assert call('--check-auth').returncode == 0
        assert call('--setup', key=rotated).returncode == 0
        assert call('--check-auth').returncode == 0
        assert call('--check-auth', provider='xai').returncode != 0, 'OpenAI state crossed into xAI'
        foreign.write_text(json.dumps({'application': '99percentgrip.agent-vesper', 'credentials': {}}))
        foreign.chmod(0o600)
        # Genuine production preflight failure, not injected native denial:
        # a foreign-owned fallback path must refuse a write before changing
        # the previously committed native entry.
        assert call('--setup', key=first, vault=foreign).returncode != 0
        assert call('--check-auth').returncode == 0
        assert call('--logout', vault=foreign).returncode != 0
        assert call('--check-auth').returncode == 0
        assert call('--logout').returncode == 0
        assert call('--check-auth').returncode != 0
        assert not path.exists(), 'production native write must not fall back to plaintext'
        assert json.loads(foreign.read_text())['application'] == '99percentgrip.agent-vesper'
    print('PASS: ACP native OpenAI CLI absent/save/restart/rotate/restart/failed foreign-path write+logout preservation/logout/restart absence')


if __name__ == '__main__':
    run(str(Path(sys.argv[1]).resolve()))
