# Stage 3 runtime identity isolation PRD

## Objective and authority

Implement independent application-private runtime ownership from baseline
`508ab4d962a9aa90eb4e6e37b7e340a6569af90b`, preserving user project access,
external protocols and Vesper-private state. Only Stage 3 is authorized.
The [architecture](architecture/runtime-identity-stage3.md),
[reviewed inventory](foundation/runtime-identity-stage3-inventory.json) and
[execution report](foundation/2026-10-15-runtime-identity-stage3.md) bind this scope.

## Required behavior and source-linked acceptance

| Requirement | Implementation / regression evidence | Candidate status |
| --- | --- | --- |
| 1. Five native platform path strategies | Shared config `JarvisPaths`; identity contract compares real resolver to approved plans | Local injected-platform proof; native CI pending |
| 2. Non-overlapping private roots | Config first-run test retains synthetic legacy roots and user files | Local proof; final matrix pending |
| 3. Safe first-run creation | Security owned-root marker; concurrent/idempotent initialization; exclusive private-workspace and pre-allocation legacy-base tests | Local filesystem proof |
| 4. Credential namespace separation | Auth independent service/application-tag tests and API-key canary | Local proof; native managers pending CI |
| 5. OAuth token non-transfer | Auth rejects unowned tokens; normal provider subscription paths refuse before HTTP | Securely inactive; independent live authorization unavailable |
| 6. Environment precedence/no legacy fallback | Explicit runtime registry; config override validation and ordinary ACP environment regression | Local source/process proof |
| 7. Sessions and MessageJournal | Owned session constructors; normal ACP ownership, writer/crash and queue/PTY suites | Local proof; final matrix pending |
| 8. Memory/checkpoints/operations | Owned memory roots, cognition leaves and checkpoint/cron fencing; persistence suites | Local proof; final matrix pending |
| 9. Concurrent storage/locks | Atomic owned marker/replacement, daemon ownership and store concurrency tests | Local filesystem proof |
| 10. Symlink/traversal refusal | Shared private-path guards; Unix redirect canary and Windows junction tests | Unix local proof; native Windows pending |
| 11. Process and single-instance separation | Generation-bound FLM cleanup; daemon lock records and dual mobile endpoints | Linux local proof; native matrix pending |
| 12. Browser/Lens ownership | Private container names/config; unchanged contained driver adapter and Lens wire protocols | Local mocks/protocol proof; contained CI pending |
| 13. Voice/Bridge private state | Owned capture/settings/logs, unique sidecar temp roots, Bridge settings and IPC checks | Local regression proof; native CI pending |
| 14. User-owned external resources | Real user-project fixture named agent-vesper remains writable under existing authority | Local filesystem proof |
| 15. No automatic import/replay | Ordinary ACP rejects old inputs; cross-subsystem settings fixture refuses valid unowned stores without mutation | Local real-process proof |
| 16. No updater/RRC side effects | Ordinary all-feature release command negative test; dormant updater entry points | Local proof; no real RRC executed |
| 17. Existing protocol compatibility | Frozen serialized/wire fixtures, provider and host-parity acceptance | Existing suites retained; final matrix pending |
| 18. Crash-safe atomic persistence | Session temp/sidecar recovery; atomic private record and capture lease tests | Local filesystem proof |

A local pass does not substitute for native CI or live provider authorization.
No requirement is complete solely from this document or unrelated passing tests.

## Integration acceptance

Default/all-feature builds, Rust 1.88, `cargo xtask verify`, Clippy/formatting,
identity/architecture/naming guards, audit/deny and relevant filesystem/process
regressions are mandatory. All thirteen trusted exact-head checks must succeed,
followed by normal protected merge, integrated-tree verification and all four
exact-merge push workflows. Missing, ignored, skipped, cancelled or failing checks
never count as success. Final receipts remain in the linked report; candidate
hosted evidence is pending until actually observed. Final local verification passed
3,645 test executions and all 165 exact acceptance cases, plus Rust 1.88, both
build modes and shipping audit/deny; 43 ignored executions remain limitations.
The first PR #5 candidate
failed native fixture/voice-path checks and left macOS Intel cancelled; the linked
report records exact causes and corrective evidence. Every required context must
run successfully again on the updated head. A subsequent native Windows
cognition fixture failure was corrected using native absolute overrides, with
a new relative-override no-write regression; production guards remain unchanged. Windows release first-launch inventory
subsequently failed; the exact fixture now has a native ownership/read regression,
and identified the fixture directory ACL boundary. The source-derived explicit
owner-rights entry is now replaced with the approved concrete SIDs; native proof
and a fresh full matrix are still required.

## Explicit limitations and next authority boundary

Independent subscription/Google authorization remains unavailable and securely
blocked; no live entitlement is asserted. Existing API-key paths remain supported.
Legacy TouchDesigner setup is deferred, while explicit existing Bridge protocols
remain. Native OS-manager/browser/installer verification is CI evidence, not a
local installation or real-user-data test. Research workspace advisory remains
excluded from shipping builds and separately tracked.

Stage 4 requires separate authorization. No automatic Vesper import/replay,
credential sharing, installer execution, registry migration, release activation,
tag or public artifact is delivered by this stage.
