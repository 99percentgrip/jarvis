"""Offline negative and positive proof for the PR matrix admission contract."""
import copy
import unittest
from verify_pr_matrix import Refused, successful_jobs


class MatrixTests(unittest.TestCase):
    def setUp(self):
        self.sha = "a" * 40
        self.run = dict(id=1, head_sha=self.sha, event="pull_request",
                        path=".github/workflows/ci.yml", status="completed",
                        conclusion="success")
        self.jobs = [dict(name=n, head_sha=self.sha, run_id=1,
                          status="completed", conclusion="success")
                     for n in ["quality", "supply-chain", "windows-installer"]]

    def check(self, run=None, jobs=None):
        successful_jobs(run or self.run, self.jobs if jobs is None else jobs,
                        {"quality", "supply-chain", "windows-installer"},
                        self.sha, "ci.yml")

    def test_complete_success(self):
        self.check()

    def test_missing_duplicate_unexpected_and_empty_jobs(self):
        for jobs in [[], self.jobs[:-1], self.jobs + [self.jobs[0]],
                     self.jobs[:-1] + [dict(self.jobs[-1], name="other")]]:
            with self.subTest(jobs=jobs), self.assertRaises(Refused):
                self.check(jobs=jobs)

    def test_every_non_success_conclusion_refuses(self):
        for conclusion in [None, "skipped", "cancelled", "failure", "neutral",
                           "timed_out", "action_required", "stale"]:
            jobs = copy.deepcopy(self.jobs)
            jobs[0]["conclusion"] = conclusion
            with self.subTest(conclusion=conclusion), self.assertRaises(Refused):
                self.check(jobs=jobs)

    def test_wrong_job_identity_and_incomplete_jobs_refuse(self):
        for field, value in [("head_sha", "b" * 40), ("run_id", 2),
                             ("status", "in_progress")]:
            jobs = copy.deepcopy(self.jobs)
            jobs[0][field] = value
            with self.subTest(field=field), self.assertRaises(Refused):
                self.check(jobs=jobs)

    def test_wrong_workflow_identity_and_status_refuse(self):
        for field, value in [("head_sha", "b" * 40), ("event", "push"),
                             ("path", ".github/workflows/release.yml"),
                             ("status", "in_progress"), ("conclusion", "skipped")]:
            with self.subTest(field=field), self.assertRaises(Refused):
                self.check(run=dict(self.run, **{field: value}))


if __name__ == "__main__":
    unittest.main()
