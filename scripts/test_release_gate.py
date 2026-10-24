"""Offline regression evidence for the actual publishing workflow's gate."""
import copy
import json
from pathlib import Path
import re
import unittest
from release_gate import EXPECTED_JOBS, GateError, WORKFLOWS, inventory, verify

SHA = "1" * 40


class GateTests(unittest.TestCase):
    def setUp(self):
        self.runs = {name: [dict(id=index + 10, run_attempt=2, head_sha=SHA,
                                head_branch="main", event="push", status="completed", conclusion="success",
                                path=f".github/workflows/{name}")]
                     for index, name in enumerate(WORKFLOWS)}
        self.jobs = {
            name: [dict(id=1000 + index * 10 + offset, name=job_name,
                        run_id=index + 10, status="completed", conclusion="success")
                   for offset, job_name in enumerate(sorted(EXPECTED_JOBS[name]))]
            for index, name in enumerate(WORKFLOWS)
        }
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
            workflow = next(name for name, rows in self.runs.items() if run in rows)
            jobs = [self.job_change(copy.deepcopy(job)) for job in self.jobs[workflow]]
            jobs = [job for job in jobs if job is not None]
            return dict(jobs=jobs, total_count=len(jobs))
        return self.refresh_change(copy.deepcopy(run))

    def test_green_exact_attempt_selects_driver_run(self):
        self.assertEqual(verify(self.fetch, SHA)['web-driver.yml'], 13)

    def test_missing_required_job_refuses_even_when_remaining_jobs_succeed(self):
        self.jobs['ci.yml'].pop(1)
        with self.assertRaises(GateError):
            verify(self.fetch, SHA)

    def test_expected_names_are_independently_pinned_to_workflow_contract(self):
        self.assertEqual(EXPECTED_JOBS, {
            'ci.yml': {'quality', 'windows-installer', 'supply-chain'},
            'msrv.yml': {'rust-1-88'},
            'platform-foundation.yml': {'linux-x86_64', 'linux-arm64',
                                        'macos-intel', 'macos-apple-silicon',
                                        'windows-x86_64'},
            'web-driver.yml': {'Contained browser linux-x86_64',
                               'Contained browser linux-aarch64',
                               'Native namespace Hive'},
        })

    def test_complete_job_sets_and_each_required_job_are_verified(self):
        self.assertEqual([len(self.jobs[name]) for name in WORKFLOWS], [3, 1, 5, 3])
        self.assertEqual(sum(map(len, self.jobs.values())), 12)
        self.assertEqual(verify(self.fetch, SHA)['web-driver.yml'], 13)
        for workflow in WORKFLOWS:
            for index in range(len(self.jobs[workflow])):
                with self.subTest(workflow=workflow, index=index):
                    job = self.jobs[workflow].pop(index)
                    try:
                        with self.assertRaises(GateError): verify(self.fetch, SHA)
                    finally:
                        self.jobs[workflow].insert(index, job)

    def test_single_success_cannot_substitute_for_full_set(self):
        self.jobs['platform-foundation.yml'] = self.jobs['platform-foundation.yml'][:1]
        with self.assertRaises(GateError): verify(self.fetch, SHA)

    def test_duplicate_names_ids_and_unexpected_jobs_refuse(self):
        for change in ('duplicate_name', 'duplicate_id', 'unexpected'):
            with self.subTest(change=change):
                extra = copy.deepcopy(self.jobs['ci.yml'][0])
                if change == 'duplicate_name':
                    extra['id'] = 9999
                elif change == 'duplicate_id':
                    extra['name'] = 'unexpected'
                else:
                    extra['id'], extra['name'] = 9999, 'unexpected'
                self.jobs['ci.yml'].append(extra)
                try:
                    with self.assertRaises(GateError): verify(self.fetch, SHA)
                finally:
                    self.jobs['ci.yml'].pop()

    def test_invalid_job_identity_and_wrong_run_refuse(self):
        for field, values in {'name': (None, '', 15),
                              'id': (None, 0, -1, '1', True),
                              'run_id': (None, 999, '10', True)}.items():
            for value in values:
                with self.subTest(field=field, value=value):
                    job = self.jobs['ci.yml'][0]
                    original = job[field]
                    job[field] = value
                    try:
                        with self.assertRaises(GateError): verify(self.fetch, SHA)
                    finally:
                        job[field] = original
            job = self.jobs['ci.yml'][0]
            original = job.pop(field)
            try:
                with self.assertRaises(GateError): verify(self.fetch, SHA)
            finally:
                job[field] = original
        self.jobs['ci.yml'].append(27)
        with self.assertRaises(GateError): verify(self.fetch, SHA)

    def test_every_non_successful_required_job_refuses(self):
        for status, conclusion in [('completed', 'failure'), ('completed', 'skipped'),
                                   ('completed', 'cancelled'), ('queued', None),
                                   ('in_progress', None)]:
            for workflow in WORKFLOWS:
                for job in self.jobs[workflow]:
                    with self.subTest(workflow=workflow, job=job['name'], status=status):
                        original = job['status'], job['conclusion']
                        job['status'], job['conclusion'] = status, conclusion
                        try:
                            with self.assertRaises(GateError): verify(self.fetch, SHA)
                        finally:
                            job['status'], job['conclusion'] = original

    def test_incomplete_job_pagination_refuses(self):
        original = self.fetch
        def fetch(endpoint, **params):
            response = original(endpoint, **params)
            if endpoint.endswith('/jobs'):
                response['total_count'] += 1
            return response
        with self.assertRaises(GateError): verify(fetch, SHA)

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


class NonpublishingCandidateTests(unittest.TestCase):
    def setUp(self):
        self.root = Path(__file__).resolve().parents[1]
        self.workflow = (self.root / '.github/workflows/release.yml').read_text()

    def test_candidate_workflow_is_technically_nonpublishing(self):
        self.assertIn('  workflow_dispatch:', self.workflow)
        self.assertNotRegex(self.workflow, r'(?m)^  push:')
        self.assertIn('  contents: read', self.workflow)
        self.assertNotIn('contents: write', self.workflow)
        self.assertNotIn('gh release', self.workflow)
        self.assertNotIn('registry push', self.workflow.lower())
        self.assertNotIn('git push', self.workflow)
        self.assertNotRegex(self.workflow, r'(?m)^\s*tags:')
        self.assertIn('persist-credentials: false', self.workflow)

    def test_candidate_reuses_stage6_package_and_driver_contracts(self):
        self.assertIn('python3 scripts/jarvis_release_candidate.py', self.workflow)
        self.assertIn('--source-commit "$CANDIDATE_COMMIT"', self.workflow)
        self.assertIn('--driver-run "${{ needs.candidate-gate.outputs.driver-run }}"', self.workflow)
        self.assertIn('${{ matrix.archive }}.candidate.json', self.workflow)
        self.assertIn('scripts/windows_package_audit.py', self.workflow)
        self.assertIn('scripts/release_gate.py', self.workflow)
        self.assertIn('actions/download-artifact@v4', self.workflow)
        for archive in [
            'jarvis-linux-x86_64.tar.gz',
            'jarvis-linux-aarch64.tar.gz',
            'jarvis-darwin-x86_64.tar.gz',
            'jarvis-darwin-aarch64.tar.gz',
            'jarvis-windows-x86_64.zip',
        ]:
            self.assertIn(archive, self.workflow)

    def test_candidate_wrapper_reuses_stage6_packer_and_has_no_publication_path(self):
        wrapper = (self.root / 'scripts/jarvis_release_candidate.py').read_text()
        self.assertIn('from jarvis_package import HELPERS, TARGETS, USER_FACING, pack', wrapper)
        self.assertIn("'status': 'nonpublishing-candidate'", wrapper)
        self.assertNotIn('gh release', wrapper)
        self.assertNotIn('git push', wrapper)

    def test_independent_registration_identity_is_dormant(self):
        registration = json.loads(
            (self.root / 'registry/independent-registration.json').read_text()
        )
        self.assertEqual(registration['status'], 'dormant')
        self.assertFalse(registration['publication_authorized'])
        self.assertEqual(registration['id'], 'jarvis')
        self.assertEqual(registration['executable'], 'jarvis-acp')
        self.assertIsNone(registration['inherited_vesper_registry_pr'])
        self.assertEqual(
            set(registration['package_contract'].values()),
            {
                'jarvis-linux-x86_64.tar.gz',
                'jarvis-linux-aarch64.tar.gz',
                'jarvis-darwin-x86_64.tar.gz',
                'jarvis-darwin-aarch64.tar.gz',
                'jarvis-windows-x86_64.zip',
            },
        )

    def test_normal_build_keeps_release_mutation_root_unavailable(self):
        recovery = (self.root / 'crates/jarvis-harness/src/release_recovery.rs').read_text()
        executor = (self.root / 'crates/jarvis-harness/src/release_executor.rs').read_text()
        self.assertIn(
            'if !cfg!(test) && !crate::release_executor::controlled_process_fixture_enabled()',
            recovery,
        )
        self.assertIn('.name("jarvis-release-controller".into())', executor)
        self.assertIn('#[cfg(feature = "test-support")]', executor)


class DriverIdentityTests(unittest.TestCase):
    def test_candidate_and_tested_driver_use_one_independent_identity(self):
        root = Path(__file__).resolve().parents[1]
        producer = (root / '.github/workflows/web-driver.yml').read_text()
        candidate = (root / '.github/workflows/release.yml').read_text()
        runtime = (root / 'crates/jarvis-harness/src/web_settings.rs').read_text()
        self.assertIn('docker build -f crates/jarvis-web-fetch/Dockerfile -t jarvis-web-driver:validated', producer)
        self.assertIn('jarvis-web-driver-${{ matrix.arch }}.tar.gz', producer)
        self.assertIn('jarvis-web-driver-${{ matrix.driver }}.tar.gz', candidate)
        self.assertIn('"jarvis-web-driver:validated".into()', runtime)
        for source in (producer, candidate, runtime):
            self.assertNotIn('vesper-web-driver:', source)
            self.assertNotIn('vesper-web-driver-${{', source)


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
