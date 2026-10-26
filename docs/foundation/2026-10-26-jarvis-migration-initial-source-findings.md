# Jarvis migration audit — initial source findings (2026-10-26)

Scope: sampled source-level review of GitHub main `59d174ef032a7cd6cb82edfa2fe093c30d4fa138`. This is NOT a complete repository scan or release clearance.

| ID | Evidence | Classification | Required action |
| --- | --- | --- | --- |
| M-01 | `Cargo.toml` workspace version = `0.24.14`; requested independent alpha is `v0.0.1-alpha` | BLOCKER for alpha tag | Design explicit independent version transition; update workspace/lockfile and runtime version acceptance coherently; re-run exact-head and native acceptance |
| M-02 | `registry/agent.json` still identifies Agent Vesper and links to the original Vesper 0.24.14 assets | Intentional historical registry contract, NOT automatically a Jarvis defect | Preserve original registration; publish independent Jarvis registry metadata only with a validated registration and explicit separate authorization |
| M-03 | `docs/migration-status.md` has an older Stage 3 status plus numerous inherited Vesper release claims | Documentation truthfulness gap | Reconcile current Jarvis stage evidence up front while keeping inherited history clearly labelled |
| M-04 | `docs/foundation/stage5-integration-inventory.json` documents ACP/MCP/Lens/web identity replacements | Partial positive evidence | Expand with live host/client smoke evidence; check both wire and executable identities |
| M-05 | `scripts/install.ps1` defaults to Programs\\Jarvis, `.jarvis\\memory`, and rejects Vesper-owned install paths | Positive static isolation evidence | Verify paths, ownership and uninstall against native acceptance receipts, including Vesper-state preservation |
| M-06 | `.github/workflows/release.yml` is a nonpublishing workflow_dispatch using exact commit and read-only permissions | Release boundary | Create a separately audited publication procedure, never reinterpret Stage 7B artifact uploads as a public release |
| M-07 | Accepted five-platform run `40217349974`; owner locally validated five archive digests and five acceptance-to-provenance digests; Windows lock hash equals CRLF-transformed canonical tracked lockfile | Accepted Stage 7B evidence at bounded scope | Preserve durable summary and explicitly scope CI vs real local device/account acceptance |

## Prioritized implementation
1. Complete full-tree machine-readable identifier inventory: runtime source, Cargo manifests, scripts, config, defaults, skills, docs, registry and assets. Classify every Vesper occurrence before changing it.
2. Exercise independent auth/storage invariants, old install-path refusal, integration handshake identity, and fresh user setup. Add red-first regressions for discovered defects.
3. Reconcile docs/migration-status.md against actual merged PRs and exact-commit workflows (no inherited release claims as current Jarvis release proof).
4. Design alpha semver/version metadata/lockfile policy as a focused PR, after identifying runtime and package expectations; avoid tagging `0.24.14` binaries `v0.0.1-alpha`.
5. Repeat all mandatory CI and native packaging/receipt audits on the final release commit. Approve publication separately.

## Explicit unknowns
- Complete full-tree inventory has NOT been performed by this document.
- Independent live OAuth/subscription registration, provider first-login, and native user-machine acceptance are not established by source inspection.
- No production release, tag, registry update or ownership-state migration is authorized here.
