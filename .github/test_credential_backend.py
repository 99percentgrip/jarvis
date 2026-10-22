"""Actual all-feature Cargo graph and fail-closed Linux fixture contracts."""

import importlib.util
import json
import os
import re
from pathlib import Path
import subprocess
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("credential_runner", ROOT / ".github/prepare-credential-runner.py")
RUNNER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNNER)


class CredentialGraphTests(unittest.TestCase):
    def test_actual_all_feature_resolution_has_only_explicit_native_backends(self):
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"],
            cwd=ROOT, text=True))
        packages = {p["id"]: p for p in metadata["packages"]}
        nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
        names = {packages[key]["name"] for key in nodes}
        self.assertFalse(names & {"keyring", "zbus", "secret-service", "zbus-secret-service-keyring-store",
                                  "zvariant", "zvariant_derive", "zbus_names"})
        auth = next(n for key, n in nodes.items() if packages[key]["name"] == "jarvis-auth")
        dependencies = {packages[d["pkg"]]["name"] for d in auth["deps"]}
        self.assertTrue({"keyring-core", "apple-native-keyring-store", "windows-native-keyring-store",
                         "dbus-secret-service-keyring-store"} <= dependencies)
        for name, version in [("keyring-core", "1.0.0"), ("apple-native-keyring-store", "1.0.1"),
                              ("windows-native-keyring-store", "1.1.0")]:
            self.assertEqual({p["version"] for p in packages.values() if p["name"] == name}, {version})
        for key, node in nodes.items():
            if packages[key]["name"] == "dbus-secret-service-keyring-store":
                self.assertIn("crypto-rust", node["features"])
                self.assertNotIn("crypto-openssl", node["features"])
            if packages[key]["name"] == "libdbus-sys":
                self.assertNotIn("vendored", node["features"])

    def test_candidate_linux_preparation_is_guarded_and_precedes_release_build(self):
        source = (ROOT / ".github/workflows/release.yml").read_text()
        package = source.split("\n  package:\n", 1)[1]
        preparation = package.index("- name: Prepare Linux native credential dependencies")
        before_build = package[preparation:package.index("- name: Build exact candidate binaries")]
        self.assertIn("if: runner.os == 'Linux'", before_build)
        self.assertIn("timeout-minutes: 5", before_build)
        self.assertIn("run: python3 .github/prepare-credential-runner.py --native", before_build)
        self.assertLess(preparation, package.index("- name: Install pinned Rust toolchain"))

    def test_every_linux_compiling_workflow_prepares_the_native_build_dependency(self):
        for name, jobs in [("ci.yml", ["quality"]), ("msrv.yml", ["rust-1-88"]),
                            ("platform-foundation.yml", ["foundation"]),
                            ("web-driver.yml", ["browser", "namespace"])]:
            source = (ROOT / ".github/workflows" / name).read_text()
            for job_name in jobs:
                job = source.split(f"\n  {job_name}:\n", 1)[1]
                job = re.split(r"\n  \S", job, maxsplit=1)[0]
                self.assertIn("run: python3 .github/prepare-credential-runner.py", job)
                self.assertLess(job.index("prepare-credential-runner.py"), job.index("cargo "))


class CredentialPreparationTests(unittest.TestCase):
    def test_refuses_local_and_non_linux_environments_before_any_command(self):
        allowed = {"GITHUB_ACTIONS": "true", "RUNNER_OS": "Linux", "RUNNER_ENVIRONMENT": "github-hosted"}
        with patch.object(RUNNER.subprocess, "run") as run:
            for omitted in allowed:
                with self.assertRaises(RuntimeError):
                    RUNNER.prepare({k: v for k, v in allowed.items() if k != omitted})
            with self.assertRaises(RuntimeError):
                RUNNER.prepare(dict(allowed, RUNNER_OS="Windows"))
            run.assert_not_called()

    def test_missing_development_package_is_installed_with_bounded_commands(self):
        allowed = {"GITHUB_ACTIONS": "true", "RUNNER_OS": "Linux", "RUNNER_ENVIRONMENT": "github-hosted"}
        with patch.object(RUNNER.subprocess, "run") as run:
            run.return_value.returncode = 1
            RUNNER.prepare(allowed)
            commands = [c.args[0] for c in run.call_args_list]
            self.assertIn(["sudo", "apt-get", "install", "-y", "libdbus-1-dev", "pkg-config"], commands)
            for call in run.call_args_list[1:]:
                self.assertLessEqual(call.kwargs["timeout"], 120)


if __name__ == "__main__":
    unittest.main()
