# Stage 2 — Rust workspace and executable identity migration

## Objective and authorization

Implement the reviewed Rust identity registry in `99percentgrip/jarvis` from
`653a5213cd3fbdafa75b7261991b2d74590148e8`, preserving behavior and security.
Stage 2 only is authorized; Stage 3, installations, state import and release are not.
The full mapping and compatibility boundaries are owned by the
[architecture specification](architecture/rust-identity-stage2.md).

## Acceptance trace

| Requirement | Evidence owner | Current disposition |
|---|---|---|
| All packages, directories, imports and executable targets match reviewed registry | Actual Cargo metadata, identity contract tests and complete architecture mapping | Local metadata and nine registry contracts pass; complete exact-head CI pending. |
| Default and all-feature builds, host launch and existing functional tests | Execution report with exact commands and results | Default/all-feature builds, full verification and 165 exact acceptance cases pass locally; native CI pending. |
| Credentials, persisted sessions, journals and external protocols retain existing identity | Scoped changes, legacy classifications and original-state hash comparison | Explicit adapters preserve former executable discovery; all protected source/state entries match; final closeout comparison remains required. |
| Third-party dependency integrity and Rust 1.88 | Lockfile comparison, audit/deny and MSRV/native jobs | 523 third-party entries unchanged; `rustls` remains 0.23.46; local audit/deny and Rust 1.88 checks pass; exact-head security/platform CI pending. |
| Mandatory exact-head security and platform checks | Thirteen protected PR contexts, trusted GitHub Actions source and normal merge | Pending; missing/skipped/cancelled jobs are not success. |
| Original Vesper and historical evidence remain untouched | Protected source/state hashes, six existing dirty report hashes and historical tree diff | Baseline captured; final comparison required. |
| DOX ownership and durable closeout | Owning instructions, evidence index and execution report | Candidate documentation included before CI; final hosted receipts remain local until next authorized change. |

## Closeout

The [execution report](foundation/2026-10-15-rust-identity-stage2.md) records actual
verification, failed attempts, deviations and unresolved platform evidence.
Completion requires the entire current requirement trace, protected merge and
integrated source verification. No Stage 3 work follows automatically.
