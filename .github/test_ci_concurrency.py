#!/usr/bin/env python3
"""Workflow concurrency must cancel superseded pull requests only."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = [
    ".github/workflows/ci.yml",
    ".github/workflows/msrv.yml",
    ".github/workflows/platform-foundation.yml",
    ".github/workflows/web-driver.yml",
    ".github/workflows/pr-verification.yml",
]
GROUP = (
    "${{ github.workflow }}-${{ github.event_name == 'pull_request' "
    "&& format('pr-{0}', github.event.pull_request.number) "
    "|| format('run-{0}', github.run_id) }}"
)


def github_and(left, right):
    return right if left else left


def github_or(left, right):
    return left if left else right


def concurrency_suffix(event_name, pr_number, run_id):
    """Mirror the workflow expression using GitHub's value-returning && and ||."""
    return github_or(
        github_and(event_name == "pull_request", f"pr-{pr_number}"),
        f"run-{run_id}",
    )


class ConcurrencyTests(unittest.TestCase):
    def test_required_workflows_use_the_isolated_group(self):
        for relative in WORKFLOWS:
            text = (ROOT / relative).read_text(encoding="utf-8")
            self.assertIn(GROUP, text, relative)
            self.assertIn(
                "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
                text,
                relative,
            )
            self.assertNotIn("group: ci\n", text)
            self.assertEqual(text.count("\nconcurrency:\n"), 1, relative)

    def test_pull_request_groups_do_not_collide_or_cover_pushes(self):
        self.assertEqual(concurrency_suffix("pull_request", 8, 100), "pr-8")
        self.assertEqual(concurrency_suffix("pull_request", 9, 100), "pr-9")
        self.assertNotEqual(
            concurrency_suffix("pull_request", 8, 100),
            concurrency_suffix("pull_request", 9, 100),
        )
        first_push = concurrency_suffix("push", None, 100)
        second_push = concurrency_suffix("push", None, 101)
        self.assertEqual(first_push, "run-100")
        self.assertEqual(second_push, "run-101")
        self.assertNotEqual(first_push, second_push)
        self.assertNotEqual(first_push, concurrency_suffix("pull_request", 8, 100))
        # A repeated event for one PR shares the group, so the newer run can
        # supersede it. Admission still requires the current head.
        self.assertEqual(
            concurrency_suffix("pull_request", 8, 100),
            concurrency_suffix("pull_request", 8, 200),
        )

    def test_release_workflows_are_not_given_this_cancellation_group(self):
        for name in ("release.yml", "release-recovery-acceptance.yml", "release-recovery-lifecycle-acceptance.yml"):
            text = (ROOT / ".github/workflows" / name).read_text(encoding="utf-8")
            self.assertNotIn("format('pr-{0}'", text, name)


if __name__ == "__main__":
    unittest.main()
