#!/usr/bin/env python3
"""Native Settings fixture: POSIX PTY or pinned Windows ConPTY, private state."""
import json
import os
from pathlib import Path
import re
import select
import signal
import struct
import subprocess
import sys
import tempfile
import time
import codecs

# Receipts use the same UTF-8 contract as the native terminal, independent of
# the Windows redirected-output code page.
for stream in (sys.stdout, sys.stderr):
    if hasattr(stream, "reconfigure"):
        stream.reconfigure(encoding="utf-8")

if os.name != "nt":
    import fcntl
    import pty
    import termios


def private_config_root(root):
    if os.name == "nt":
        return root / "config" / "Jarvis"
    if sys.platform == "darwin":
        return root / "home" / "Library" / "Application Support" / "Jarvis" / "Config"
    return root / "config" / "jarvis"


def private_data_root(root):
    if os.name == "nt":
        return root / "data" / "Jarvis" / "Data"
    if sys.platform == "darwin":
        return root / "home" / "Library" / "Application Support" / "Jarvis" / "Data"
    return root / "data" / "jarvis"


def fixture_owned_directory(path):
    """Declare synthetic test data before a child loads it; never a runtime importer."""
    path.mkdir(mode=0o700, parents=True, exist_ok=True)
    if os.name == "nt":
        code = """$ErrorActionPreference='Stop'; $p=$env:FIXTURE_PRIVATE_PATH;
$user=[System.Security.Principal.WindowsIdentity]::GetCurrent().User;
$acl=Get-Acl -LiteralPath $p; $acl.SetAccessRuleProtection($true,$false);
foreach($sid in @($user.Value,'S-1-5-18','S-1-5-32-544')){
$r=[System.Security.AccessControl.FileSystemAccessRule]::new([System.Security.Principal.SecurityIdentifier]::new($sid),'FullControl','ContainerInherit,ObjectInherit','None','Allow'); $acl.AddAccessRule($r)};
Set-Acl -LiteralPath $p -AclObject $acl"""
        subprocess.run(["powershell.exe", "-NoProfile", "-NonInteractive", "-Command", code],
                       env=dict(os.environ, FIXTURE_PRIVATE_PATH=str(path)), check=True, capture_output=True, timeout=15)
    else:
        path.chmod(0o700)
    (path / ".jarvis-owner").write_bytes(b"99percentgrip.jarvis/runtime-v1\n")


class WindowsChild:
    """ConPTY child with the bounded subset of Popen used by these fixtures."""
    def __init__(self, binary, root, env):
        from winpty import PtyProcess
        self.terminal = PtyProcess.spawn([binary], cwd=str(root), env=env,
                                         dimensions=(40, 120), backend="0")
        self.pid = self.terminal.pid
        self.returncode = None

    def poll(self):
        if not self.terminal.pty.isalive():
            self.returncode = self.terminal.pty.get_exitstatus()
        return self.returncode

    def wait(self, timeout):
        deadline = time.monotonic() + timeout
        while self.poll() is None and time.monotonic() < deadline:
            time.sleep(.02)
        if self.poll() is None:
            raise subprocess.TimeoutExpired("ConPTY child", timeout)
        return self.returncode

    def close(self):
        if self.poll() is None:
            try:
                subprocess.run(["taskkill", "/PID", str(self.pid), "/T", "/F"],
                               check=True, capture_output=True, timeout=10)
            except subprocess.CalledProcessError as error:
                # The child can exit between poll() and taskkill. A nonzero
                # kill result is benign only after the owned child is dead.
                try:
                    self.wait(10)
                except subprocess.TimeoutExpired as timeout:
                    raise AssertionError(
                        "ConPTY child did not exit after failed taskkill "
                        f"(exit {error.returncode})"
                    ) from timeout
            else:
                self.wait(10)
        self.terminal.pty.cancel_io()
        self.terminal.fileobj.close()
        self.terminal._server.close()
        self.terminal._thread.join(timeout=5)
        assert not self.terminal._thread.is_alive(), "ConPTY reader did not settle"
        self.terminal.closed = True

class Host:
    def __init__(self, binary, root, extra_env=None):
        # Resolve in the caller workspace before the child enters its private cwd.
        binary = str(Path(binary).resolve())
        self.screen = [[' '] * 120 for _ in range(40)]
        self.x = self.y = 0
        self.pending = ''
        self.raw = ''
        self.decoder = codecs.getincrementaldecoder('utf-8')(errors='replace')
        home = root / 'home'
        home.mkdir(exist_ok=True)
        for directory in [root / "global-memory", root / "global-cognition", root / ".jarvis"]:
            if directory.exists():
                fixture_owned_directory(directory)
        fixture_credentials = root / "fixture-credentials"
        fixture_credentials.mkdir(mode=0o700, exist_ok=True)
        for provider in ["openai", "xai"]:
            path = fixture_credentials / f"{provider}.json"
            path.write_text(json.dumps({"application": "99percentgrip.jarvis", "credentials": {
                provider: {"native-auth": json.dumps({"mode": "signed-out"})}}}))
            path.chmod(0o600)
        # Inactive optional providers must fail before native credential lookup.
        blocker = fixture_credentials / "unavailable.json"
        blocker.write_text("{}")
        blocker.chmod(0o600)
        env = dict(PATH=os.environ['PATH'], HOME=str(home), USERPROFILE=str(home),
                   XDG_DATA_HOME=str(root/'data'), XDG_CONFIG_HOME=str(root/'config'),
                   XDG_STATE_HOME=str(root/'app-state'), XDG_CACHE_HOME=str(root/'cache'),
                   JARVIS_OPENAI_CREDENTIALS_PATH=str(fixture_credentials/'openai.json'),
                   JARVIS_XAI_CREDENTIALS_PATH=str(fixture_credentials/'xai.json'),
                   JARVIS_LMSTUDIO_CREDENTIALS_PATH=str(blocker), JARVIS_CREDENTIALS_PATH=str(blocker),
                   TERM='xterm-256color', LANG='C.UTF-8', ZAI_API_KEY='fixture-not-a-real-key',
                   JARVIS_PROVIDER='zai', JARVIS_SANDBOX='off',
                   HTTP_PROXY='http://127.0.0.1:9', HTTPS_PROXY='http://127.0.0.1:9',
                   ALL_PROXY='http://127.0.0.1:9', NO_PROXY='',
                   JARVIS_LMSTUDIO_ROOT=str(root/'lmstudio'),
                   JARVIS_GLOBAL_MEMORY_ROOT=str(root/'global-memory'),
                   JARVIS_GLOBAL_COGNITION_ROOT=str(root/'global-cognition'))
        if os.name == "nt":
            # OS console/process initialization needs these; no user settings or
            # credentials are inherited. Scratch directories remain private.
            for name in ("SystemRoot", "WINDIR", "COMSPEC"):
                if name in os.environ:
                    env[name] = os.environ[name]
            scratch = root / "tmp"
            scratch.mkdir(exist_ok=True)
            env.update(TEMP=str(scratch), TMP=str(scratch),
                       APPDATA=str(root / "config"), LOCALAPPDATA=str(root / "data"))
        if extra_env:
            env.update(extra_env)
        if os.name == "nt":
            self.child = WindowsChild(binary, root, env)
            self.master = self.child.terminal.fileobj
        else:
            self.master, slave = pty.openpty()
            fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack('HHHH', 40, 120, 0, 0))
            self.child = subprocess.Popen([binary], cwd=root, env=env, stdin=slave,
                                          stdout=slave, stderr=slave, start_new_session=True)
            os.close(slave)
    def write(self, data):
        if os.name == "nt":
            self.child.terminal.write(data.decode('utf-8'))
        else:
            # A small macOS PTY can fill while the child waits for its output
            # to drain. Feed partial writes and read output on the same owner.
            flags = fcntl.fcntl(self.master, fcntl.F_GETFL)
            deadline = time.monotonic() + 10
            remaining = memoryview(data)
            fcntl.fcntl(self.master, fcntl.F_SETFL, flags | os.O_NONBLOCK)
            try:
                while remaining:
                    if time.monotonic() >= deadline:
                        raise TimeoutError('terminal input did not drain within 10 seconds')
                    try:
                        count = os.write(self.master, remaining)
                    except BlockingIOError:
                        self.pump(.01)
                        continue
                    if count <= 0:
                        raise OSError('terminal input write made no progress')
                    remaining = remaining[count:]
            finally:
                fcntl.fcntl(self.master, fcntl.F_SETFL, flags)
    def feed(self, text):
        self.pending += text
        while self.pending:
            if self.pending.startswith('\x1b['):
                match = re.match(r'\x1b\[([0-9;?<>:]*)([@-~])', self.pending)
                if not match:
                    return
                params, final = match.groups()
                self.pending = self.pending[match.end():]
                if params.startswith(('?', '<', '>')):
                    continue
                values = [int(v or 0) for v in params.split(';')] if ':' not in params else []
                n = (values[0] or 1) if values else 1
                if final in 'Hf':
                    self.y = max(0, min(39, n-1))
                    self.x = max(0, min(119, (values[1] if len(values)>1 else 1)-1))
                elif final == 'G': self.x = max(0,min(119,n-1))
                elif final == 'A': self.y = max(0,self.y-n)
                elif final == 'B': self.y = min(39,self.y+n)
                elif final == 'C': self.x = min(119,self.x+n)
                elif final == 'D': self.x = max(0,self.x-n)
                elif final == 'J' and params in ('2','3'): self.screen = [[' ']*120 for _ in range(40)]
                elif final == 'K':
                    for x in range(self.x,120): self.screen[self.y][x]=' '
                elif final == 'n' and params == '6': self.write(b'\x1b[1;1R')
                continue
            c, self.pending = self.pending[0], self.pending[1:]
            if c == '\x1b':
                if not self.pending:
                    self.pending = c
                    return
                self.pending = self.pending[1:]
            elif c == '\r': self.x = 0
            elif c == '\n': self.y = min(39,self.y+1)
            elif c >= ' ':
                self.screen[self.y][self.x] = c
                self.x = min(119,self.x+1)
    def text(self): return '\n'.join(''.join(row) for row in self.screen)
    def pump(self, duration=.25):
        deadline = time.monotonic()+duration
        while time.monotonic()<deadline:
            ready,_,_ = select.select([self.master],[],[],max(0,deadline-time.monotonic()))
            if ready:
                try: data=self.master.recv(65536) if os.name == "nt" else os.read(self.master,65536)
                except OSError: break
                if not data: break
                text=self.decoder.decode(data)
                self.raw+=text
                self.feed(text)
    def wait(self, label, timeout=20):
        deadline=time.monotonic()+timeout
        while time.monotonic()<deadline:
            self.pump(.1)
            if label in self.text(): return
            if self.child.poll() is not None: break
        raise AssertionError(f'Missing {label!r}; exit={self.child.poll()}\n{self.text()}\nraw tail: {self.raw[-1200:]!r}')
    def wait_exit(self, timeout=10):
        # macOS can block terminal restoration/exit until the PTY reader
        # consumes pending output. Keep observing the terminal while waiting.
        deadline = time.monotonic() + timeout
        while self.child.poll() is None:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise AssertionError(
                    f'Terminal child did not exit within {timeout} seconds\n'
                    f'{self.text()}\nraw tail: {self.raw[-1200:]!r}')
            self.pump(min(.1, remaining))
        return self.child.poll()
    def key(self, value):
        self.write(value.encode())
        self.pump()
    def choose(self, label):
        self.wait(label)
        for _ in range(30):
            target = next((y for y, row in enumerate(self.screen)
                           if label in ''.join(row) and ''.join(row).strip().startswith('│')), None)
            focus = next((y for y, row in enumerate(self.screen)
                          if '›' in ''.join(row) and '│' in ''.join(row)), None)
            assert target is not None and focus is not None, self.text()
            if target == focus:
                self.key('\r')
                return
            self.key('\x1b[B' if target > focus else '\x1b[A')
        raise AssertionError(f'Could not select {label!r}\n{self.text()}')
    def click(self, label):
        if os.name == "nt":
            # ConPTY fixtures drive native key events; POSIX retains mouse proof.
            self.choose(label)
            return
        self.wait(label)
        for y,row in enumerate(self.screen):
            line=''.join(row)
            if label in line and line.strip().startswith("│"):
                x=line.index(label)+1
                self.key(f'\x1b[<0;{x+1};{y+1}M')
                return
        raise AssertionError(label)
    def close(self):
        if os.name == "nt":
            self.child.close()
        else:
            if self.child.poll() is None:
                os.killpg(self.child.pid,signal.SIGTERM)
                try:
                    self.child.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(self.child.pid, signal.SIGKILL)
                    self.child.wait(timeout=5)
            os.close(self.master)


def run(binary):
    with tempfile.TemporaryDirectory(prefix='vesper-settings-pty-') as temp:
        root=Path(temp)
        saved=private_config_root(root)/'ui/settings.json'
        host=Host(binary,root)
        try:
            host.wait('Start coding')
            host.key('s')
            host.click('Permissions')
            host.click('bypass')
            assert not saved.exists()
            host.key('\x1b')
            host.click('Discard changes')
            host.wait('Start coding')
            assert not saved.exists()
            host.key('s')
            host.wait('current Ask')
            host.click('Providers')
            host.click('openai')
            host.click('Cancel')
            assert not (root/'.jarvis/provider').exists()
            for category,value in [('Primary model','glm-5.2'),('Reasoning depth','high'),
                                   ('Auxiliary model','glm-4.7'),('Generation style','precise'),
                                   ('Mixture of Agents','enabled'),('Permissions','bypass'),
                                   ('Session mode','ask'),('Visual theme','dracula')]:
                host.click(category)
                host.click(value)
            host.click('Implementation acceptance')
            host.click('Enforced completion: OFF')
            host.wait('Enforced completion: ON')
            assert 'PRD path:' not in host.text()
            host.key('\x1b')
            host.click('Swarm')
            host.click('Swarm: OFF')
            host.wait('Swarm: ON')
            host.key('\x1b')
            host.click('Web tools')
            host.click('Web tools: OFF')
            host.wait('Web tools: ON')
            host.key('\x1b')
            host.key('\x1b')
            host.click('Keep editing')
            assert not saved.exists()
            host.key('\x1b')
            host.click('Save changes')
            host.wait('Start coding')
            data=json.loads(saved.read_text())
            assert data['common']['/permission']=='/permission bypass'
            assert data['common']['/mode']=='/mode ask'
            assert data['providers']['zai']['/model']=='/model glm-5.2'
            assert data['providers']['zai']['/thinking']=='/thinking high'
            assert data['providers']['zai']['/auxiliary']=='/auxiliary glm-4.7'
            assert data['providers']['zai']['/generation']=='/generation precise'
            assert data['providers']['zai']['/mixture']=='/mixture enabled'
            assert json.loads((root/'.jarvis/acceptance-settings.json').read_text())==dict(enabled=True,prd='')
            assert json.loads((root/'.jarvis/swarm-settings.json').read_text())['enabled']
            assert json.loads((root/'.jarvis/web-settings.json').read_text())['enabled']
            host.click('Start coding')
            host.key('/permission\r')
            host.wait('Bypass')
        finally: host.close()
        host=Host(binary,root)
        try:
            host.wait('Start coding')
            host.wait('glm-5.2')
            host.key('s')
            host.wait('current Bypass')
            host.wait('current Plan')
            host.click('Implementation acceptance')
            host.wait('Enforced completion: ON')
        finally: host.close()
        print('PASS: native Settings mouse navigation, discard, keep editing, grouped save, automatic PRD toggle, active permission and restart persistence; isolated HOME/workspace, no provider prompt.')

if __name__=='__main__':
    run(str(Path(sys.argv[1]).resolve()))
