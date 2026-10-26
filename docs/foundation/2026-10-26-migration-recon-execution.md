# Independent migration reconnaissance — execution checkpoint (2026-10-26)

## Objective

Establish a source-pinned baseline and begin the exhaustive migration audit without mutating owner state. This checkpoint **does not close the mission**, prove complete inventory, authorize versioning, or prepare an alpha release.

## Methods and exact receipts

- Read root, documentation and foundation ownership instructions before editing; inspected current branch/worktrees, PR #21 (draft, head `e6aa53fbe79b778435ef9e81c005e4b600160e82`, base `59d174ef032a7cd6cb82edfa2fe093c30d4fa138`), Stage 7B workflow `40217349974` (`completed`, `success`, five candidate archives and exact-commit gate), source status, package scripts, registry and version manifests. Owner canonical checkout had unrelated modifications, left unchanged. Created an isolated audit worktree from PR #21 head.
- `git ls-files -z` counted **2,304** tracked files at PR head. `git grep -I -n -i vesper` yielded **12,511** text hits at PR head; the broader pinned-main `vesper|0.24.14` scan yielded **12,683** matching lines across **1,084** files. The latter is archived as [compressed per-file inventory](2026-10-26-migration-identifier-inventory.jsonl.gz), deterministic gzip (`mtime=0`), with per-file ID, line numbers, ownership candidates, classification, risk, test need, disposition and pinned-source citation. Its plain JSONL SHA-256 before compression was `993df78e04b8a10014a703620efbd0bc4aed49537fc0f6b0536216a04d51be17`. Only the original registry file is tentatively COMPATIBILITY; the other 1,083 files are **UNKNOWN**, not approved. Binary, nonmatching and semantic ownership paths remain unassessed. The full-tree exit gate therefore **FAILS**.
- `Cargo.toml:40` remains `version = "0.24.14"`; an alpha `v0.0.1-alpha` tag would misidentify these artifacts. Original registry is a historical compatibility boundary, not an independent registry entry. The existing PR #21 quality run `40257161806` **failed** with `naming embargo violated: 10 new upstream-brand hit(s)` because the inherited pattern also matches unregistered product prose in its two audit files. The mandatory matrix verifier `40257161875` also **failed** because exact-head jobs were not complete and successful. Corrected only audit prose to comply with the unchanged frozen naming baseline. Local `CARGO_BUILD_JOBS=2 cargo xtask naming-guard`: `naming-guard: clean (36 hits, all frozen in baseline)`. No CI result for the amended head is yet known.
- Found a concrete active identity defect: ACP, TUI and shared-worker interruption text still displayed the legacy product name. A separate scoped draft repair PR #22 changes only user-facing prefixes and tests. Local Linux focused tests pass in that branch, but PR #22 remains unmerged; the baseline candidate does **not** contain this repair. An attempted independent reviewer worker could not start: its configured provider returned `Selected OpenAI model is not in the current account model list`; this is an unavailable review, not approval.

## Files and dispositions

- New baseline manifest, compressed identifier scan, and this report; amended PR #21 audit plan and initial findings to conform to naming policy. The owning index and DOX link these records. No production source is modified on the audit branch.
- `VERSION-01`: **FAIL/blocking** for alpha, workspace version mismatch; owner: release/version lane, test: offline order/version/package/lockfile matrix plus exact-source CI. No transition attempted.
- `DISPLAY-01`: **DEFECT/open pending PR #22**; owner: ACP/TUI/shared hosted-worker; test: three host outcome tests and native candidate check.
- `REGISTRY-01`: **COMPATIBILITY/preserve**; original registry remains original-owner metadata; independent registration requires separate authorization.
- `ISOLATION-01`: **NOT TESTED** as a complete matrix; owner: auth/private storage, test: disposal-only no-adoption byte snapshots, signed-out cold startup and five-target acceptance.
- `DISTRIBUTION-01`: **NOT TESTED on a final corrected source**; owner: nonpublishing native candidate, test: five package and installer receipts with exact immutable input.
- `REVIEW-01`: **NOT TESTED** independently due unavailable reviewer provider; another independent session/reviewer required.

## Deviations, unresolved and readiness

A text-hit census is a lead list, **not** a complete classified semantic inventory. No paid auth, real device, destructive uninstall, five-target corrected-source candidate, final exact-commit checks, independently executed reviewer, protected merge, tag or publication was performed. Baseline Stage 7B archive evidence remains valid only for its own source SHA. Final decision matrix for the present source: **migration completeness FAIL; experimental alpha readiness FAIL; production readiness FAIL**. Accepted final release commit: **none**. Further separately authorized protected integration is required before any final-source release acceptance; do not conflate a draft PR or green old workflow with release readiness.
