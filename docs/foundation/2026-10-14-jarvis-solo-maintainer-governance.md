# Jarvis solo-maintainer governance execution

## Objective

Bootstrap mandatory, non-publishing exact-head Jarvis CI and replace the added
second-account requirement only after complete success. Stop before Rust migration.

## Methods and baseline

Authenticated Vesper rulesets and effective main rules are empty; classic branch
protection returns HTTP 404 Branch not protected. Vesper's established workflow
uses CI plus maintainer discipline and RRC exact-SHA release gates, rather than a
mandatory second account. Vesper was only queried, never modified.

Jarvis starts at 692985a1ceeb2740024c8a4b26770d5afb4ee889 with active main
ruleset 25049950 requiring one review and no bypass, and active tag freeze
25049955. GitHub default workflow permissions are read-only.

## Implementation files and review

Four verification workflows check out exact PR head and disable persistent Git
credentials. Existing tests, matrix targets, package identifiers, artifact naming,
installer logic and documentation filters are retained. Canonical CI adds the
new offline verifier refusal suite. `.github/verify_pr_matrix.py` validates the
four run paths, PR association, SHA, event, attempt, expected twelve jobs,
success-only conclusions, pagination and unchanged evidence. The PR workflow
`pr-verification.yml` has read-only contents/actions/PR access, no secret grants.
No release or registry mutation is called.

All eight inherited workflow definitions and called CI helpers were inspected.
Release explicitly grants contents:write and remains disabled. Controlled RRC
acceptance and disposable spike workflows remain disabled. Source package names
and isolated local test fixtures retain Vesper identities until approved migration.
CI tests execute on disposable native runners without user provider calls.

Owning root/.github/docs/foundation DOX, PRD and evidence index are updated with
this change. No application, installer, Cargo, dependency or release code changed.

## Verification

CI_PENDING. Offline verifier positive/refusal cases and existing release-gate
policy checks will execute before push. Hosted twelve-job matrix and aggregate
must finish for the exact test PR head before any approval-policy relaxation.
Complete job readback and branch enforcement receipts will be retained locally.

## Deviations, limitations and readiness

No fake reviews. No silent bypass. Prose-only PRs remain blocked by missing
filtered native checks under the conservative bootstrap, with no skipped check
represented as success. GitHub Actions app identity does not pin a workflow's
content; owner review of gate changes remains required. The readiness state is
pending complete CI. Publishing and release tags remain blocked.
