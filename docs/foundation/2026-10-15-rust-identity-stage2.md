# Stage 2 Rust identity migration — execution report

## Objective and scope

Implement Rust package, source-directory, import and executable identity migration
in `99percentgrip/jarvis` only, from exact main commit
`653a5213cd3fbdafa75b7261991b2d74590148e8`, on
`feat/stage2-rust-identity-migration`. No state migration, installation, registry
publication, RRC operation or release is authorized.

Status: implementation and local verification complete; protected PR CI pending.
No protected merge or Stage 2 completion is claimed yet.
The [owning PRD](../rust-identity-stage2-prd.md) and
[architecture and full mapping](../architecture/rust-identity-stage2.md) are the
acceptance and compatibility source of truth.

## Preflight evidence

Stage 1 PR #3 head `51d8133efc65bc84a1ba3c8788824847571dd738` merged as
`653a5213cd3fbdafa75b7261991b2d74590148e8`; all thirteen required PR checks
succeeded. Exact-main canonical run 38591203108, MSRV 38591203124, five-target
foundation 38591203389 and web-driver 38591203385 completed successfully.
Main ruleset 25049950 retains PR-only merging, zero mandatory human approvals,
resolved threads, strict thirteen-check enforcement from GitHub Actions app 15368,
no force-push, no deletion and no bypass. Tag ruleset 25049955 freezes `v*` creation,
updates and deletion. Publication and recovery acceptance workflows stay disabled.

The isolated Stage 2 worktree belongs solely to the independent repository's Git
administrative directory and object database. The original dirty Vesper checkout
remains at `0b5630d271965e7d9df3a0a09c116c7f8c44042e` with 158 existing dirty paths.
Its protected state snapshot contains 14,598 entries; six unrelated dirty governance
and Stage 1 reports in earlier worktrees are separately hash-pinned and preserved.
Private evidence is retained outside Git in a dedicated Stage 2 evidence directory;
secrets are not copied into tracked logs.

## Methods, changed files and implementation

Read applicable DOX chains and Stage 1 registry; inspect authenticated GitHub PR,
commit, workflow and ruleset state; compare actual Cargo metadata and dependency
edges. Use registry-derived Git moves and scoped Rust token/manifests/build-reference
changes. No unrestricted branding replacement is performed.

The [source inventory](2026-10-15-rust-identity-stage2-source.json) records exact
changed paths and package mappings. Thirty package directories move; all 32 packages
are explicitly registered.
Manifests, first-party lock entries, imports, feature forwarding, Cargo binary test
variables, active CI selections, private PE installer fixtures and developer source
references follow the reviewed mapping. `xtask` and the pure identity crate keep
existing names and locations. Exact complete mapping is linked above.

Scoped non-mechanical files include sandbox discovery and its regression test,
TUI updater bundle validation, browser Docker build-output copy, PE inventory
selection, offline explicit workspace ownership, identity registry, naming guards
and current owning DOX/developer documents. Temporary root README, original licenses,
all frozen fixtures, historical release and Stage 0/1 records remain unchanged.

## Local evidence and deviations

- `cargo check --workspace --all-targets --all-features` passed after completing
  scoped import correction; initial lexical misses were corrected, not suppressed.
- Identity contracts: nine tests passed, including actual metadata and negative
  unregistered-package mutation. Maintenance tests and invocation-root regressions
  passed; all nine final maintenance unit cases, including registered directory
  normalization and unknown-name refusal, also passed.
- `cargo xtask architecture` passed for all 32 packages.
- `cargo xtask naming-guard` passed with the unchanged 36 frozen historical hits.
- Explicit renamed-workspace release-lock ownership regression passed; no controller
  operation was run.
- Windows PE audit regression: eight tests passed; release-gate offline regression:
  eleven tests passed. Native Windows packaging still requires hosted evidence.
- `cargo audit` reported zero vulnerabilities; `cargo deny check` passed advisories,
  bans, licenses and sources with existing duplicate-version warnings.
- All 523 third-party lockfile entries match the baseline; `rustls` stays 0.23.46.
  A test-only `tempfile` edge uses the existing workspace version, with no version
  update. Initial full verification caught the missing declaration; it was added
  and full verification restarted.
- `cargo build --locked --workspace` passed for default features. Four isolated
  host launches (`--version` and `--help` for both TUI and ACP) passed; ACP stdout
  remained reserved for JSON-RPC. Isolated HOME stayed empty. The TUI retained
  its inherited pre-meta-flag `.vesper-scope-id` creation in the private project.
- New and changed Markdown links pass; eleven inherited missing evidence links
  are preserved without rewriting historical records. Whitespace checks pass.
- Original source HEAD, dirty status and worktrees match the baseline; all 14,598
  protected entries, 158 existing dirty paths and six earlier dirty reports match.
- `cargo +1.88.0 check --locked --workspace --all-targets --all-features` passed.
- `cargo build --locked --workspace --all-features` passed; final adapter Clippy
  checks also passed with warnings denied. The voice pure-core dependency guard
  checks actual renamed package names. Supervisor integration discovery builds
  the actual renamed target; real namespace security/timeout acceptance passed
  (one executed, zero ignored). Updated sandbox/voice suites and the new updater
  sibling-discovery test passed. Initial full-workspace output exposed stale
  supervisor test build references; these were corrected and separately verified.
- DOX child indices resolve. One inherited index entry for the absent held VRO-19
  subtree was removed; its hold instruction and historical records remain.
- `cargo xtask verify` exited successfully: all-feature workspace suites reported
  3405 passes, zero failed and 42 inherited ignored cases. Doc tests, fixture
  validation/digests, coverage, provider/runtime/ACP/session checks and all 165
  mandatory exact acceptance cases passed; ignored optional cases are not
  represented as executed acceptance or native-platform proof.
- Rust 1.88 checks also passed after the final updater/discovery corrections.
- Exact-head thirteen-check matrix and merge remain pending here.

## Platform, security and readiness limitations

Local verification is Linux x86_64 only and is not native five-platform proof.
Native Linux ARM64, macOS Intel/Apple Silicon, Windows, both contained-browser jobs
and complete PR matrix must succeed before protected merge. No tests or assertions
are removed, ignored, replaced with synthetic success or granted a ruleset bypass.
The excluded research workspace's separate outstanding advisory remains tracked;
it is not a shipping dependency and its lockfile is not modified.

Deferred legacy contracts are explicitly classified in the architecture document.
Inherited disabled publishing workflows still require later-stage repository and
artifact review. No independent public release or installer is claimed. Stage 3
requires explicit approval and fresh integrated verification.

## Review-driven build-selector correction

Automated PR review identified three real integration omissions on initial head
`1f113cebebbd683d2ca6f530ad7454ebbf9d478d`: executed RRC release-build argv still
selected original packages despite a renamed displayed command; the Windows
governor fixture selected the old package/library artifact; both Lens browser
launchers selected the old package and one used a removed directory path.
These are Stage 2 build/discovery defects, not authority to publish or rename protocols.

The actual selectors and fixture path are corrected. A new Rust unit compares all
executed local-gate commands and build package/feature selectors to actual Cargo
metadata without running a release. The focused exact case executed once and
passed; an earlier zero-match invocation is retained but is not proof. Two Python
regressions inspect the executed Windows command/artifact filter and both Lens
launchers against actual metadata; canonical CI now runs them. Existing native
Job Object assertions and all browser assertions are preserved.

Both real Lens browser and interview flows pass using the existing Chrome binary
and the CI-pinned Playwright version in private test-tool storage. The first
attempt failed because the long private temporary directory exceeded Chrome’s
Unix socket-path limit; an unchanged retry with a shorter private temporary root
passed. No timeout, assertion, browser protocol field or frozen fixture was changed.
Warning-free harness Clippy and Rust 1.88 all-target/all-feature checks pass;
the eleven release-prerequisite/trigger regressions still pass. Original source
and protected state remain unchanged after both browser runs. The corrective candidate requires a fresh
complete thirteen-check matrix; earlier-head successes cannot authorize merge.

## Final hosted receipts

Pending candidate PR, exact head, thirteen successful conclusions, normal merge,
post-merge status and final source/state preservation comparison. This pending
section is not evidence of completion.
