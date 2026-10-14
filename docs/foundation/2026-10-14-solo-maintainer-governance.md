# J.A.R.V.I.S. solo-maintainer governance execution

## Objective

Bootstrap mandatory, non-publishing exact-head J.A.R.V.I.S. CI and replace the added
second-account requirement only after complete success. Stop before Rust migration.

## Methods and baseline

Authenticated Vesper rulesets and effective main rules are empty; classic branch
protection returns HTTP 404 Branch not protected. Vesper's established workflow
uses CI plus maintainer discipline and RRC exact-SHA release gates, rather than a
mandatory second account. Vesper was only queried, never modified.

J.A.R.V.I.S. starts at 692985a1ceeb2740024c8a4b26770d5afb4ee889 with active main
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

## First hosted attempt and scoped correction

PR 1 first head 24dd48eaced653417286e0e8d9c4de0dd0bd385a passed audit/deny
and all three web-driver jobs. Foundation run 38513529452 Windows job
115687140384 failed five command_settlement timing/startup assertions in the
parallel workspace suite (8 passed, 5 failed). Exact log retained outside the
repository. The existing dedicated settlement step already uses serial tests.
Windows workspace execution now uses the same serial scheduling, retaining every
case and timing assertion. This is CI execution policy only, not Rust repair.
A fresh exact-head matrix is required; no old-head success will be reused.

## Second hosted attempt and naming-policy correction

Head e802bcd63bf26ac7d9c33343453a55557ae36fa0 cleared the Windows workspace
step after serial scheduling, with timing assertions unchanged. Canonical run
38514525807 failed the inherited upstream-name embargo on nine new governance
prose/link occurrences. New prose now uses the approved dotted product spelling
and neutral document filenames. The source naming guard, frozen baseline, Rust
code and package identities remain unchanged. An early canonical invocation
checks naming before the expensive full build. Stage 1 technical identity work
requires an explicit naming-policy design; this bootstrap does not relax it.
The matrix verifier refused the failed canonical run, demonstrating fail-closed
hosted behavior. A new complete exact-head matrix is required.
