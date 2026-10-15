"""Offline, filesystem-backed seed-helper isolation acceptance; no live credentials."""
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]


def load(name, script):
    spec = importlib.util.spec_from_file_location(name, script)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class RuntimeIsolation(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="seed isolation ")
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)
        self.legacy = self.home / ".agent-vesper"
        self.legacy.mkdir()
        self.canary = self.legacy / "google_token.json"
        self.canary.write_bytes(b"SYNTHETIC-PRIVATE-TOKEN-NOT-FOR-IMPORT")
        (self.legacy / ".env").write_text("GITHUB_TOKEN=SYNTHETIC-PRIVATE-TOKEN-NOT-FOR-IMPORT\n")
        self.before = {p.name: p.read_bytes() for p in self.legacy.iterdir()}
        env = {"HOME": str(self.home), "USERPROFILE": str(self.home),
               "XDG_CACHE_HOME": str(self.home / "cache"),
               "XDG_CONFIG_HOME": str(self.home / "config"),
               "APPDATA": str(self.home / "roaming"),
               "LOCALAPPDATA": str(self.home / "local"),
               "AGENT_VESPER_HOME": str(self.legacy),
               "AGENT_VESPER_CITATION_LEDGER": str(self.canary)}
        self.environ = patch.dict(os.environ, env)
        self.environ.start()
        self.addCleanup(self.environ.stop)
        self.addCleanup(self.assert_preserved)
        self.sources = load("sources_isolation", Path(__file__).with_name("sources.py"))

    def assert_preserved(self):
        self.assertEqual({p.name: p.read_bytes() for p in self.legacy.iterdir()}, self.before)

    def owned_cache(self):
        cache = self.sources._private_cache()
        cache.mkdir(mode=0o700, parents=True)
        if os.name == "nt":
            code = """$ErrorActionPreference='Stop'; $p=$env:FIXTURE_CACHE;
$user=[System.Security.Principal.WindowsIdentity]::GetCurrent().User;
$acl=Get-Acl -LiteralPath $p; $acl.SetAccessRuleProtection($true,$false);
foreach($sid in @($user.Value,'S-1-5-18','S-1-5-32-544')){
$r=[System.Security.AccessControl.FileSystemAccessRule]::new([System.Security.Principal.SecurityIdentifier]::new($sid),'FullControl','ContainerInherit,ObjectInherit','None','Allow'); $acl.AddAccessRule($r)};
Set-Acl -LiteralPath $p -AclObject $acl"""
            result = subprocess.run(["powershell.exe", "-NoProfile", "-NonInteractive", "-Command", code],
                                    env=dict(os.environ, FIXTURE_CACHE=str(cache)), capture_output=True)
            self.assertEqual(result.returncode, 0, "synthetic native ACL setup failed")
        (cache / ".jarvis-owner").write_bytes(b"99percentgrip.jarvis/runtime-v1\n")
        return cache

    def test_default_ledger_and_external_user_output_are_independent(self):
        with patch.dict(os.environ, {}, clear=False):
            os.environ.pop("JARVIS_CITATION_LEDGER", None)
            cache = self.owned_cache()
            path = self.sources.resolve_ledger_path()
            self.assertEqual(path, cache / "citations" / "ledger.json")
            self.sources.add_sources(path, ["https://example.invalid/first"])
            self.assertEqual(len(self.sources.load_ledger(path)["sources"]), 1)
            external = self.home / "user project" / "sources.json"
            self.sources.add_sources(external, ["https://example.invalid/user"])
            self.assertEqual(len(self.sources.load_ledger(external)["sources"]), 1)

    def test_unowned_or_legacy_ledger_is_refused_without_writes(self):
        cache = self.sources._private_cache()
        cache.mkdir(mode=0o700, parents=True)
        (cache / "unowned-record").write_bytes(b"untouched")
        with self.assertRaises(SystemExit):
            self.sources.add_sources(cache / "citations" / "ledger.json", ["https://example.invalid"])
        self.assertEqual((cache / "unowned-record").read_bytes(), b"untouched")
        with self.assertRaises(SystemExit):
            self.sources.resolve_ledger_path(str(self.canary))
        with self.assertRaises(SystemExit):
            self.sources.resolve_ledger_path(str(self.home / "user" / ".." / "ledger.json"))

    def test_locks_are_not_stolen_and_atomic_write_leaves_no_temporary_files(self):
        ledger = self.home / "user project" / "ledger.json"
        self.sources.add_sources(ledger, ["https://example.invalid"])
        with self.sources._LedgerLock(ledger):
            lock = ledger.with_suffix(".json.lock")
            with self.assertRaises(SystemExit):
                with self.sources._LedgerLock(ledger, timeout=0):
                    self.fail("unverifiable lock was stolen")
            self.assertTrue(lock.exists())
        self.assertFalse(lock.exists())
        self.assertFalse(list(ledger.parent.glob(".citation-*")))

    def test_google_helpers_refuse_before_token_read_refresh_or_revocation(self):
        scripts = ROOT / "google-workspace" / "scripts"
        api = load("google_isolation", scripts / "google_api.py")
        setup = load("google_setup_isolation", scripts / "setup.py")
        bridge = load("google_bridge_isolation", scripts / "gws_bridge.py")
        for call in [api.get_credentials, setup.main, bridge.get_valid_token]:
            with self.assertRaises(RuntimeError):
                call()
        for script in ["google_api.py", "setup.py", "gws_bridge.py"]:
            result = subprocess.run([sys.executable, str(scripts / script), "--check"], capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"independent authorization", result.stderr)
            self.assertNotIn(self.canary.read_bytes(), result.stderr + result.stdout)

    def test_posix_symlink_refuses_and_github_does_not_read_legacy_dotenv(self):
        if os.name != "nt":
            redirect = self.home / "redirect.json"
            redirect.symlink_to(self.canary)
            with self.assertRaises(SystemExit):
                self.sources.add_sources(redirect, ["https://example.invalid"])
            stub = self.home / "commands"
            stub.mkdir()
            for command in ["gh", "curl"]:
                file = stub / command
                file.write_text("#!/bin/sh\nexit 1\n")
                file.chmod(0o700)
            env = dict(os.environ, PATH=str(stub) + os.pathsep + os.environ.get("PATH", ""), GITHUB_TOKEN="")
            helper = ROOT / "github-auth" / "scripts" / "gh-env.sh"
            result = subprocess.run(["bash", "-c", 'source "$1"; test "$GH_AUTH_METHOD" = none', "fixture", str(helper)],
                                    cwd=self.home, env=env, capture_output=True)
            self.assertEqual(result.returncode, 0)
            self.assertNotIn(self.canary.read_bytes(), result.stdout + result.stderr)
        else:
            # Native junction refusal is also exercised by jarvis-security's
            # mandatory Windows filesystem test (no developer-mode privilege).
            self.assertEqual(self.sources._private_cache(), self.home / "local" / "Jarvis" / "Cache")


if __name__ == "__main__":
    unittest.main()
