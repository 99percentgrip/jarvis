# Stage 3 runtime identity isolation PRD

**Current status, 2026-10-19: exact-merge native correction in progress.** PR #5
merged normally after all thirteen exact-head checks succeeded. Its exact merge
commit exposed a recurring native speech settlement failure on Intel macOS.
Three red-first regressions prove missing pending cancellation in the blocking
bridge, an unenforced system-TTS deadline and an unregistered worker synthesis
token. All four workflows on that merge now pass after bounded native reruns;
the independently reproduced defects still require correction. The scoped
correction passes complete local verification, security gates, both build modes,
the full Rust 1.88 workflow and twenty exact pipeline repetitions without
changing existing deadlines. A new protected
corrective PR, thirteen-check matrix and four exact-merge workflows remain
required. The [execution report](foundation/2026-10-15-runtime-identity-stage3.md)
retains all earlier security, ownership, native and preservation evidence.

Corrective PR #6 also fixes owned helper-group termination after red-first proof.
Its current fixture repair removes Python cold-start from the unchanged
one-second executor-reuse assertion and answers primary terminal device queries.
Focused Rust 1.88, real Linux first-launch and terminal-peer checks pass; native
Intel/Windows acceptance and a new full matrix remain required. No merge of PR #6
or Stage 3 completion is claimed.

The maintained correction's hosted supply-chain, both contained browsers,
namespace Hive and Windows installer passed on head `5b2eb925ed5414b777a53b06da5fcc29cd52c14c`.
Its Windows native attempts failed at different ACP deadlines: initialize at five
seconds, then a command round-trip at thirty seconds. Neither is assumed flaky.
Test-only bounded content-free diagnostics preserve every original assertion and
deadline; the complete local ACP suite passes 138 executions with one ignored
case and Rust 1.88 checking passes. Production runtime/dependencies are unchanged.
Apple Silicon also failed the existing fresh-turn voice settlement bound; test-only worker/fixture diagnostics retain its original assertions and timing. A new exact-head full matrix and resolved native results remain required.

Preservation recheck confirms unchanged source checkout/dirty paths/worktrees, but two live Vesper cognition SQLite sidecars changed inode/mtime since capture, with attribution UNKNOWN. The report retains the discrepancy; complete runtime snapshot invariance is not claimed.

Protected-merge audit identified three remaining ownership defects despite thirteen green checks on the previous head. Their red-first corrections and new candidate verification are tracked in the [execution report](foundation/2026-10-15-runtime-identity-stage3.md). No merge or completion is claimed.

The current hosted head's first Windows attempt lost runner/server communication,
with no recoverable job log. Its bounded diagnostic repeat exposed an inherited
Apple Silicon journal observer synchronization defect. A real gated-PTY red/green
regression and five complete real-TUI repetitions support the test-only correction;
all assertions and existing bounded observers remain required. Full local verification,
Rust 1.88, both builds and fresh audit/all-feature deny pass for the correction. The
[execution report](foundation/2026-10-15-runtime-identity-stage3.md) retains both
failures and the pending new exact-head acceptance boundary.

Further source inspection reproduced nested explicit-root first-save and
deep `.jarvis` ancestor ownership gaps. Red-first config/security regressions
support the narrow parent-ownership correction; each precedence case now uses
a fresh workspace. The [execution report](foundation/2026-10-15-runtime-identity-stage3.md)
records exact-candidate proof, passing current-source local/native gates and the
pending new exact-head CI matrix.
Previously green checks do not authorize integration of unverified corrections.

## Objective and authority

Implement independent application-private runtime ownership from baseline
`508ab4d962a9aa90eb4e6e37b7e340a6569af90b`, preserving user project access,
external protocols and Vesper-private state. Only Stage 3 is authorized.
The [architecture](architecture/runtime-identity-stage3.md),
[reviewed inventory](foundation/runtime-identity-stage3-inventory.json) and
[execution report](foundation/2026-10-15-runtime-identity-stage3.md) bind this scope.

## Required behavior and source-linked acceptance

The subsequent [Phase 4 authentication/persistence execution](foundation/2026-10-27-phase4-auth-persistence-execution.md), [scoped machine-readable matrix](foundation/phase4-auth-persistence-coverage.json), [release-host fixture follow-up](foundation/2026-10-27-phase4-host-fixture-followup.md) [historical PR #25 exact-head native CI closeout](foundation/2026-10-28-phase4-pr25-native-ci-closeout.md) and [current integration report](foundation/2026-10-29-pr24-pr25-integration.md) record the red-first ordinary-host credential preflight correction, Windows fixture repair and five-platform synthetic credential receipts. Broader consumer and independent-review gates remain open; [sequential integration evidence](foundation/2026-10-29-pr25-pr27-integration.md) tracks the reconciled #25 head, merge and post-merge CI without claiming full Phase 4 acceptance. These do not close Stage 3 integration gates or overall migration acceptance.

| Requirement | Implementation / regression evidence | Candidate status |
| --- | --- | --- |
| 1. Five native platform path strategies | Shared config `JarvisPaths`; identity contract compares real resolver to approved plans | Local injected-platform proof; native CI pending |
| 2. Non-overlapping private roots | Config first-run test retains synthetic legacy roots and user files | Local proof; final matrix pending |
| 3. Safe first-run creation | Security owned-root marker; concurrent/idempotent initialization; exclusive private-workspace and pre-allocation legacy-base tests | Local filesystem proof |
| 4. Credential namespace separation | Auth independent service/application-tag tests, API-key canary and both providers' unowned-vault/no-lock regressions | Local proof; native managers pending CI |
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
3,653 test executions and all 165 exact acceptance cases, plus Rust 1.88, both
build modes and shipping audit/deny; 43 ignored executions remain limitations.
The first PR #5 candidate
failed native fixture/voice-path checks and left macOS Intel cancelled; the linked
report records exact causes and corrective evidence. Every required context must
run successfully again on the updated head. A subsequent native Windows
cognition fixture failure was corrected using native absolute overrides, with
a new relative-override no-write regression; production guards remain unchanged. Windows release first-launch inventory
subsequently failed; the exact fixture now has a native ownership/read regression,
and identified the fixture directory ACL boundary. The source-derived explicit
owner-rights entry is now replaced with the approved concrete SIDs; the native
Windows installer passed. A further reproduced provider credential-lock ordering
defect now has red/green ownership regressions and a production preflight correction;
fresh full verification passed 3,650 executions and 165 exact acceptance cases
with 43 ignored limitations retained. Fresh Rust 1.88 workspace/Windows checks,
both build modes and audit/deny also passed. A fresh exact-head matrix is still
required for that correction. The Windows lane subsequently passed those provider
regressions and native credential/first-launch/voice checks, then failed its
constrained-job launcher: a legacy activation key did not match the registered
Rust guard. The one-line fixture correction has red/green source-contract proof;
native constrained execution and fresh complete verification remain pending.
The subsequent local full run reproduced the generated Cargo wrapper ETXTBSY
fixture failure. Immutable checked-in executable code with private data bindings
now passes all nine ACP governor process tests, including a concurrent ownership
regression; fresh full local and exact-head integration evidence remains required.

## Explicit limitations and next authority boundary

Independent subscription/Google authorization remains unavailable and securely
blocked; no live entitlement is asserted. Existing API-key paths remain supported.
Legacy TouchDesigner setup is deferred, while explicit existing Bridge protocols
remain. Native OS-manager/browser/installer verification is CI evidence, not a
local installation or real-user-data test. Research workspace advisory remains
excluded from shipping builds and separately tracked.

The [Issue #23 Windows private-state verifier-sharing repair](foundation/2026-10-30-phase5a-windows-private-state-reliability.md) preserves the original error-5/rerun receipts, native red-first verifier-handle conflict and narrowly scoped production correction; controlled sharing refusal alone does not prove the historical failing syscall. Windows reliability and release acceptance stay open pending final exact-head native evidence and authorized post-merge verification.

Phase 4B's bounded follow-up [checkpoint crash-recovery report](foundation/2026-10-28-phase4b-checkpoint-torn-append.md) and [trace matrix](foundation/phase4b-runtime-recovery-matrix.json) cover one source-reproduced lost-acknowledgment defect through the TUI checkpoint bundle; the [integration record](foundation/2026-10-29-pr25-pr27-integration.md) tracks its reconciled #26 head and exact-main CI. Remaining credential/session/coexistence cases remain separate gates, not completed by this repair.

The [Phase 4C failed-write report](foundation/2026-10-28-phase4c-session-cron-failed-writes.md) covers scoped session/cron write-error cache consistency, with five native runner receipts on its original stacked head; the [integration record](foundation/2026-10-29-pr25-pr27-integration.md) owns reconciled-head proof. The [coverage matrix](foundation/phase4b-runtime-recovery-matrix.json) retains open hosted, credential and comprehensive recovery acceptance. This does not close Phase 4B or Phase 4C.

Stage 4 requires separate authorization. No automatic Vesper import/replay,
credential sharing, installer execution, registry migration, release activation,
tag or public artifact is delivered by this stage.

The immutable-wrapper candidate subsequently passed full local verification:
3,653 executions, 165 exact acceptance cases and 43 ignored limitations. Its
hosted matrix retained eleven successes; Windows exhausted the 90-minute total
after its complete workspace/native credential suites passed. Windows now shares
Intel macOS's 120-minute total budget, preserving all cases, per-test/per-step
bounds and check contexts. The new exact-head matrix and four post-merge workflows
remain mandatory; cancelled/skipped results remain non-passing.

The initial resumed investigation independently reproduced `cargo deny --all-features check`
failing the three maintenance advisories; `cargo audit` exits zero with warnings.
A maintained Rust-1.88-compatible native credential path, with reviewed
compatibility/native-store evidence, is a prerequisite to further integration.
Advisory suppression, a hidden MSRV increase or removing native credential
functionality is not an accepted resolution. No source/dependency or policy
change was made during this investigation. The report/index/PRD closeout stays
local without another push. Stage 4 remains unauthorized.

The authorized supply-chain correction replaces the retired dependency path,
retains macOS/Windows backend versions, and requires native Secret Service for
production Linux writes without automatic plaintext fallback. Existing owned
retained-vault read precedence and OAuth non-transfer remain. GNOME evidence is
real private-service proof; KDE and live vendor authorization remain explicitly
unverified. Native CI and full local verification must cover the new candidate.

The protected-review corrections pass 3,675 local executions/165 acceptance cases,
Rust 1.88 (including Windows config/auth/security checking), both workspace builds
and fresh audit/deny. The corrected head requires its own thirteen successful
checks and normal protected merge, then four exact-merge workflow successes.

The later [Issue #23 Windows private replacement investigation](foundation/2026-10-26-windows-private-replacement-investigation.md) confirms an induced sharing-denial test on native Windows but does not establish the cause of the separate historical concurrency failure. Its [PR #24 integration record](foundation/2026-10-28-pr24-integration.md) tracks reconciliation and exact-head verification. Neither is Stage 3 acceptance or a production repair.
