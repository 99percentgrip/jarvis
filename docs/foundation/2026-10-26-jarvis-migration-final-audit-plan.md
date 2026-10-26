# Independent-product migration completion and independent release audit

Status: OPEN — audit plan, not an acceptance verdict.
Baseline: `59d174ef032a7cd6cb82edfa2fe093c30d4fa138`
Candidate execution: workflow 40217349974

## Verified baseline
- Stage 7B nonpublishing workflow: exact-commit gate plus five native package jobs passed.
- Five candidate archive SHA-256 comparisons passed independently on the owner's Fedora host.
- Five provenance JSON SHA-256 comparisons matched acceptance receipts.
- Windows Cargo.lock digest difference was reproduced by LF-to-CRLF conversion, matching the Windows receipt. This is explained byte-level variation, not proof of logical dependency divergence.
- Workflow artifact evidence is short-lived; preserve a durable evidence manifest before expiry.
- Current workspace version remains 0.24.14; do not publish v0.0.1-alpha until version metadata, package names, and install paths agree.

## Mandatory migration closure lanes (each needs owner, evidence, pass/fail)
1. Source and product identity: executable names, crate IDs, UI/help/version output, documentation, updater, and compatibility inventory; classify legacy Vesper strings as historical, deliberate wire compatibility, or defect.
2. Runtime and auth isolation: startup and provider flows work as independent product independently; verify no inherited Vesper tokens, credentials, stores, sessions, hidden home directories, service identities, or subscription claims are read or modified.
3. Persistence and upgrade boundaries: new install, upgrade, rollback, uninstall, failed install, restart, and recovery preserve unrelated and Vesper-owned data across all five native platforms.
4. Integration identity: ACP/MCP/Lens/bridge/web clients, browser driver and helper executable names, provider originator and OAuth registration contracts; do not relabel unregistered clients as independently authorized.
5. Distribution and reproducibility: manifest, package inventory and shipped binary hashes, executable identity, checksums, provenance, Cargo.lock line-ending behavior; test all five platform archives against the same source commit.
6. Security and CI: verify exact-head 13-check PR matrix, post-merge workflows, least privilege, sandbox boundaries, secret leakage prevention, nonpublishing gates, dependency audits, and release authorization.
7. Documentation / scope: reconcile docs/migration-status.md, active PRDs, architecture records, developer instructions, evidence index and claims; distinguish inherited Agent Vesper historical reports from current independent product evidence.
8. User-facing acceptance: smoke-test fresh independent product TUI/ACP setup, basic agent turn, permissions, tools, sessions, optional supported providers and voice where feasible; explicitly identify untested live-device/account capabilities.
9. Release/version: define v0.0.1-alpha intent, update workspace/package metadata coherently, prepare changelog/known limitations, validate installers, sign or attest as policy requires; separately authorize tag/public release.

## Closure requirements
- Produce an inventory with file paths, actual test IDs, PASS/FAIL/NOT TESTED, residual risks, and an accountable resolution for each issue.
- Repairs follow red-first evidence on focused PRs; never alter protected main directly or silently weaken acceptance checks.
- Repeat exact-head CI and post-merge CI after migration fixes. A new exact-commit native candidate is mandatory if source changes affect released artifacts.
- No tag, release, registry publication, deletion of Vesper user state or stage closure from this planning document.
- Final independent audit explicitly distinguishes code migration completeness, tested feature parity, production readiness, and alpha release readiness.

## Executed checkpoint

[Reconnaissance report](2026-10-26-migration-recon-execution.md) and its [pinned baseline](2026-10-26-migration-baseline-manifest.md) retain the current UNKNOWN inventory and failed exact-head matrix. This is not closure; no candidate acceptance or publication follows from these documents.

[Classification/CI continuation](2026-10-26-migration-classification-ci-review-checkpoint.md) retains the current risk-prioritized v2 inventory, the exact-PR-head failure and the unavailable independent reviewer. Rule-based documentary triage is not production acceptance.
