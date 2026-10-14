"""Offline regression evidence for the actual publishing workflow's gate."""
import copy
from pathlib import Path
import re
import unittest
from release_gate import GateError, WORKFLOWS, inventory, verify

SHA = "1" * 40


class GateTests(unittest.TestCase):
    def setUp(self):
        self.runs = {name: [dict(id=index + 10, run_attempt=2, head_sha=SHA,
                                head_branch="main", event="push", status="completed", conclusion="success",
                                path=f".github/workflows/{name}")]
                     for index, name in enumerate(WORKFLOWS)}
        self.job_change = lambda job: job
        self.refresh_change = lambda run: run

    def fetch(self, endpoint, **params):
        if endpoint.startswith("actions/workflows/"):
            rows = copy.deepcopy(self.runs[endpoint.split('/')[2]])
            return dict(workflow_runs=rows, total_count=len(rows))
        identity = int(endpoint.split('/')[2])
        run = next(row for rows in self.runs.values() for row in rows if row['id'] == identity)
        if endpoint.endswith('/jobs'):
            self.assertIn(f"/attempts/{run['run_attempt']}/", endpoint)
            job = self.job_change(dict(run_id=identity, status="completed", conclusion="success"))
            return dict(jobs=[job] if job else [], total_count=1 if job else 0)
        return self.refresh_change(copy.deepcopy(run))

    def test_green_exact_attempt_selects_driver_run(self):
        self.assertEqual(verify(self.fetch, SHA)['web-driver.yml'], 13)

    def test_filtered_documentation_commit_cannot_authorize_a_release(self):
        self.runs['ci.yml'] = []
        with self.assertRaises(GateError): verify(self.fetch, SHA)

    def test_earlier_success_cannot_hide_newer_failed_or_active_run(self):
        for status, conclusion in [('completed', 'failure'), ('in_progress', None)]:
            self.runs['ci.yml'].append(dict(self.runs['ci.yml'][0], id=99, run_attempt=1,
                                            status=status, conclusion=conclusion))
            with self.assertRaises(GateError): verify(self.fetch, SHA)
            self.runs['ci.yml'].pop()

    def test_wrong_sha_branch_event_or_workflow_cannot_open_gate(self):
        for field, value in [('head_sha', '2'*40), ('head_branch', 'candidate'),
                             ('event', 'workflow_dispatch'), ('path', '.github/workflows/other.yml')]:
            original = self.runs['ci.yml'][0][field]
            self.runs['ci.yml'][0][field] = value
            with self.assertRaises(GateError): verify(self.fetch, SHA)
            self.runs['ci.yml'][0][field] = original

    def test_skipped_failed_running_and_missing_jobs_refuse(self):
        for job in [None, dict(run_id=10, status='completed', conclusion='skipped'),
                    dict(run_id=10, status='completed', conclusion='failure'),
                    dict(run_id=10, status='in_progress', conclusion=None)]:
            self.job_change = lambda _, job=job: job
            with self.assertRaises(GateError): verify(self.fetch, SHA)

    def test_rerun_during_validation_refuses(self):
        self.refresh_change = lambda run: dict(run, run_attempt=3)
        with self.assertRaises(GateError): verify(self.fetch, SHA)

    def test_new_run_appearing_during_validation_refuses(self):
        original = self.fetch
        count = 0
        def fetch(endpoint, **params):
            nonlocal count
            if endpoint == 'actions/workflows/ci.yml/runs':
                count += 1
                if count == 2:
                    self.runs['ci.yml'].append(dict(self.runs['ci.yml'][0], id=99, run_attempt=1,
                                                    status='in_progress', conclusion=None))
            return original(endpoint, **params)
        with self.assertRaises(GateError): verify(fetch, SHA)

    def test_pagination_and_truncated_inventory(self):
        def fetch(_, **params):
            return dict(jobs=list(range(100 if params['page']==1 else 1)), total_count=101)
        self.assertEqual(len(inventory(fetch,'jobs','jobs')),101)
        with self.assertRaises(GateError):
            inventory(lambda *_args, **_kw: dict(jobs=[], total_count=1),'jobs','jobs')


class WorkflowTriggerTests(unittest.TestCase):
    """Required checks must execute for prose and executable changes alike."""

    @staticmethod
    def trigger_block(text, event):
        # Fail closed outside the deliberately simple trigger schema. Requiring
        # unfiltered events avoids approximating GitHub glob or diff semantics.
        header = re.search(r'^on:\n((?:^[ \t].*\n|^\n)*)',
                           text, re.MULTILINE)
        if not header:
            raise ValueError('missing on mapping')
        block = re.search(r'^  ' + event + r':([^\n]*)\n'
                          r'((?:^    .*\n|^\n)*)',
                          header.group(1), re.MULTILINE)
        if not block:
            raise ValueError('missing required event: ' + event)
        if block.group(1).strip() not in ['', '{}']:
            raise ValueError('unsupported event syntax')
        body = block.group(2).strip()
        allowed = '' if event == 'pull_request' else 'branches: [main]'
        if body != allowed:
            raise ValueError('required event filtered or unsupported: ' + event)
        return True

    def test_required_workflows_cover_readme_docs_and_source(self):
        root = Path(__file__).resolve().parents[1]
        for workflow in WORKFLOWS:
            text = (root / '.github/workflows' / workflow).read_text()
            for event in ['pull_request', 'push']:
                for changed in ['README.md', 'docs/installation.md',
                                'docs/foundation/repair-evidence.json',
                                'crates/vesper-agent/src/lib.rs']:
                    with self.subTest(workflow=workflow, event=event, changed=changed):
                        self.assertTrue(self.trigger_block(text, event))

    def test_filters_and_missing_events_cannot_hide_required_checks(self):
        good = 'on:\n  pull_request:\n  push:\n    branches: [main]\n\npermissions:\n  contents: read\n'
        for event in ['pull_request', 'push']:
            for restriction in ["paths-ignore:\n      - 'README.md'",
                                "paths-ignore:\n      - 'docs/**'",
                                "paths-ignore:\n      - 'crates/**'",
                                "paths:\n      - 'crates/**'",
                                'branches-ignore: [main]',
                                'types: [opened]']:
                mutated = good.replace('  ' + event + ':\n',
                                       '  ' + event + ':\n    ' + restriction + '\n')
                with self.subTest(event=event, restriction=restriction):
                    with self.assertRaises(ValueError):
                        self.trigger_block(mutated, event)
            with self.assertRaises(ValueError):
                self.trigger_block(good.replace('  ' + event + ':', '  other:'), event)
        with self.assertRaises(ValueError):
            self.trigger_block(good.replace('[main]', '[other]'), 'push')

    def test_all_thirteen_required_contexts_have_covered_producers(self):
        import sys
        root = Path(__file__).resolve().parents[1]
        sys.path.insert(0, str(root / '.github'))
        from verify_pr_matrix import EXPECTED
        self.assertEqual(set(EXPECTED), set(WORKFLOWS))
        self.assertEqual(set().union(*EXPECTED.values()), {
            'quality', 'windows-installer', 'supply-chain', 'rust-1-88',
            'linux-x86_64', 'linux-arm64', 'macos-intel',
            'macos-apple-silicon', 'windows-x86_64',
            'Contained browser linux-x86_64', 'Contained browser linux-aarch64',
            'Native namespace Hive'})
        aggregate = (root / '.github/workflows/pr-verification.yml').read_text()
        self.assertTrue(self.trigger_block(aggregate, 'pull_request'))
        self.assertIn('    name: Jarvis verification\n', aggregate)
        self.assertIn('python3 scripts/test_release_gate.py',
                      (root / '.github/workflows/ci.yml').read_text())


if __name__ == '__main__': unittest.main()
