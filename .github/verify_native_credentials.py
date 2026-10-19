"""Explicit hosted synthetic acceptance with nonzero exact Rust receipts."""

import os
import re
import subprocess
import sys


def require_hosted_fixture(environment):
    if (
        environment.get("GITHUB_ACTIONS") != "true"
        or environment.get("JARVIS_NATIVE_CREDENTIAL_ACCEPTANCE")
        != "isolated-hosted-fixture"
        or environment.get("RUNNER_OS") not in {"Windows", "macOS", "Linux"}
    ):
        raise RuntimeError("native credential acceptance requires an isolated hosted fixture")


def has_exact_receipt(output, case):
    output = output.replace("\r\n", "\n")
    named = re.search(r"^test " + re.escape(case) + r" \.\.\. ok$", output, re.M)
    summary = re.search(
        r"^test result: ok\. 1 passed; 0 failed; 0 ignored;[^\n]*$", output, re.M
    )
    return named is not None and summary is not None


def main():
    require_hosted_fixture(os.environ)
    if os.environ["RUNNER_OS"] == "Linux" and os.environ.get("NATIVE_SECRET_SERVICE_FIXTURE") != "unlocked-private-bus":
        from linux_secret_service_fixture import run_private
        run_private([sys.executable, __file__])
        return
    if os.environ["RUNNER_OS"] == "Linux":
        from pathlib import Path
        root = Path(os.environ["NATIVE_SECRET_SERVICE_FIXTURE_HOME"])
        if (Path.home() != root or not (root / ".native-credential-test-home").is_file()
                or not os.environ["DBUS_SESSION_BUS_ADDRESS"].startswith("unix:path=" + str(root / "run") + "/")):
            raise RuntimeError("native Linux acceptance refuses a desktop bus/home")
    sys.stdout.reconfigure(encoding="utf-8")
    cases = [
        (
            ["-p", "jarvis-auth", "--test", "native_persistence"],
            "hosted_native_credential_persistence",
        ),
        (
            ["-p", "jarvis-provider-openai", "--lib"],
            "auth::tests::hosted_device_flow_persists_large_subscription_record",
        ),
    ]
    if os.environ["RUNNER_OS"] == "Linux":
        cases.append((["-p", "jarvis-auth", "--test", "linux_secret_service"],
                      "linux_native_persistence_namespace_and_restart"))
    for selection, case in cases:
        command = [
            "cargo", "test", "--locked", *selection, case,
            "--", "--ignored", "--exact", "--test-threads=1", "--show-output",
        ]
        result = subprocess.run(
            command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            encoding="utf-8", errors="strict", timeout=240, check=False,
        )
        print(result.stdout, end="", flush=True)
        if result.returncode != 0 or not has_exact_receipt(result.stdout, case):
            raise RuntimeError(f"native credential acceptance lacks one passing exact case: {case}")
        print(f"native credential acceptance verified: {case}", flush=True)
    if os.environ["RUNNER_OS"] == "Linux":
        from pathlib import Path
        environment = dict(os.environ, NATIVE_SECRET_SERVICE_FIXTURE="missing-private-bus",
                           DBUS_SESSION_BUS_ADDRESS="unix:path=" + str(
                               Path(os.environ["NATIVE_SECRET_SERVICE_FIXTURE_HOME"]) / "absent-fixture-bus"))
        case = "missing_service_never_creates_a_plaintext_vault"
        result = subprocess.run(["cargo", "test", "--locked", "-p", "jarvis-auth", "--test",
                                 "linux_secret_service", case, "--", "--ignored", "--exact",
                                 "--show-output"], env=environment, capture_output=True,
                                text=True, timeout=60, check=False)
        print(result.stdout, end="", flush=True)
        if result.returncode or not has_exact_receipt(result.stdout, case):
            raise RuntimeError("missing Secret Service must fail without a plaintext vault")


if __name__ == "__main__":
    main()
