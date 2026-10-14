# Stage 1 identity architecture execution

## Objective and methods

Implement the owner-approved Stage 1 specification only. Inspected authenticated
main/PR/rulesets/workflows, Cargo manifests and active source contracts; read owning
DOX. Verified baseline 57d8280a8d4a470af5878088b9d8d0e1645a12da has successful
canonical 38548536997, MSRV 38548537044, five-target 38548536966 and web-driver
38548537295 push runs. All thirteen contexts remain required. Publishing and both
RRC workflows are disabled, release tags frozen, original Vesper independent.

Created feat/stage1-identity-architecture in a clean destination-only worktree. The existing destination checkout's three local
report edits are not copied, discarded or included. Original Vesper is untouched.

## Files and implementation

Added the identity crate's manifest, DOX, pure typed library, explicit package
registry, classified mapping registry, injected platform specification and metadata
integration tests. Workspace membership/lockfile and xtask verification are updated;
all existing package versions/targets and release code remain unchanged. Normal
identity dependencies are empty; config/JSON are test-only. Architecture enforces
actual package/binary/import ownership and prohibits active runtime consumers.

Owning root/crates/xtask/documentation DOX and indexes are updated. GitHub/scripts
DOX now correctly records the already-approved unfiltered required-check policy.
Added architecture inventory, owning PRD and source provenance receipt. Existing
historical records and the temporary README notice remain intact.

## Commands and evidence ownership

Local commands: `cargo test -p` for the identity package; the same command under
Rust 1.88; `cargo test -p xtask`; `cargo clippy` for both changed packages with
all targets and `-D warnings`; `cargo xtask architecture`; `cargo xtask naming-guard`;
`cargo fmt --all`; `git diff --check`. Local compiler work uses two jobs, disabled
incremental/debug artifacts, and this destination worktree's own target directory.
No Vesper target cache or runtime was used. Provenance is captured in the owned
JSON receipt; links and JSON are checked independently. Full `cargo xtask verify`,
acceptance, audit/deny and all five native targets are required in hosted CI.

## Verification and deviations

Nine deterministic identity cases pass locally on Rust 1.95 and 1.88. Eight xtask
unit cases and two invocation-root cases pass. Clippy on both changed packages passes. Architecture purity regressions also
reject clock, I/O, output and grouped standard-library import attempts.
Architecture validates 32 workspace packages; five excluded research manifests are
also inventoried. No new external dependency versions were resolved.

The collision tests initially caught duplicate ACP target declarations and a
protocol/application planned-package collision; corrected the registry without
weakening tests. Proposed protocol package now has a distinct identity. The old
naming guard's first-party stem collision required a narrow ownership rule; all
thirteen upstream patterns and the frozen baseline remain unchanged. Historical
scopes, unknown names, other upstream stems and bare prose are not exempted.
Source tag provenance is verified through authenticated read-only GitHub evidence;
it is unsigned and no cryptographic verification is claimed. CI tests use the
receipt rather than assuming a shallow checkout has historical Git tags.

## Pending acceptance and limitations

STAGE 1 VERIFICATION PENDING. The source candidate and authorized blocker
corrections are locally verified; the fresh mandatory hosted matrix remains required. No merge or completed-stage
claim is made. Registry name availability, checkpoint
wire identity and undecided fixture/adapter metadata remain explicitly uncertain.
Paths are proposed and tested defaults; filesystem aliases/overrides require future
ownership checks. Import is Stage 4 work, not implemented. No RRC operation, runtime
initialization, migration, installation, version tag or release occurred.

## Readiness effect

Local Stage 1 design and blocker corrections are implemented; fresh CI/merge is pending. Stage 2 is not
authorized. The [owning PRD](../identity-stage1-prd.md) and
[full inventory](../architecture/identity-architecture.md) define the next boundary.

## Observed excluded-research security limitation

Authenticated GitHub Dependabot alert 1 reports GHSA-2mjx-qc3c-rqvc
(RUSTSEC-2026-0285) for rustls 0.23.42 in
spikes/rust-sse-transport/Cargo.lock. The initial production Cargo.lock contained 0.23.45, patched only for the older
advisory; the corrective candidate resolves production rustls to 0.23.46. This existing excluded research dependency is not upgraded by Stage 1
and is not a production/release dependency. Production audit/deny gates do not
prove that every excluded research lockfile is clean. Remediation and a separate
research-lock security policy remain follow-up decisions; no release is authorized.

## Review-driven boundary regressions

PR review identified path-method/whitespace purity bypasses, custom build targets
and missing shipped-skill/legacy-voice environment inputs. Source policy now
normalizes whitespace and rejects filesystem path methods and standard-library
aliases; custom-build metadata is rejected and the identity manifest disables
automatic build scripts. The environment census compares actual apps/crates/
scripts/shipped-skill source symbols against all 109 registered input names.
GLM_VENV_PATH and the two supported skill overrides are sensitive; a fixture-only
voice input remains RequiresEvidence. No original source reader is renamed.

## Hosted failure and scope-preserving correction

Candidate f0836f2fb0c99f214dd513ca28728ca395779acc failed the unchanged
repository_version_plan_covers_every_owned_manifest_and_lockfile test because
the new identity package used a standalone version instead of inheriting the
workspace version. Only the new package and its xtask dependency are corrected
to the existing 0.24.14 workspace lifecycle. No existing version, controller or
assertion changes; future independent v0.1.0 preparation remains unexecuted.
Apple Silicon also failed the unchanged MessageJournal Show more PTY assertion
while the displayed host was still RUNNING. Its assertion is preserved; a fresh
complete matrix is required and no failed/old-head evidence is admitted.

Focused immutable planning proof passed locally after the version correction:
repository_version_plan_covers_every_owned_manifest_and_lockfile (1 selected,
1 passed, no release worker or version mutation executed). Nine identity tests
also pass. Existing workspace/controller behavior and PTY assertions are intact.


## Blocked closeout receipt

PR: #3 in the independent destination repository.
Branch: feat/stage1-identity-architecture
Exact candidate: 28de7586b79bdefd1807886f88bb01dc87efa38e
Main remains: 57d8280a8d4a470af5878088b9d8d0e1645a12da
Merged commit: NONE.

Supply-chain run 38570429624, job 115873875358 failed cargo audit on
production rustls 0.23.45. Newly reported RUSTSEC-2026-0346, 0347, 0348 and
0349 each require at least 0.23.46. This supersedes the earlier observation that
0.23.45 was patched for the older excluded-spike advisory; it is now affected by
these new production advisories. The authoritative job log is retained. External
RustSec web pages were unavailable, so independent webpage verification is UNKNOWN.
No exception, security suppression, dependency upgrade or ruleset bypass was made.
The owner subsequently authorized targeted production TLS and voice CI blocker
corrections; the section below records that repair before a fresh matrix.

Current thirteen-check snapshot (pending jobs are not successful evidence):

| Context | Status | Conclusion |
|---|---|---|
| Contained browser linux-aarch64 | completed | success |
| Contained browser linux-x86_64 | completed | success |
| Complete PR matrix gate | in_progress | PENDING |
| Native namespace Hive | completed | success |
| linux-arm64 | in_progress | PENDING |
| linux-x86_64 | in_progress | PENDING |
| macos-apple-silicon | in_progress | PENDING |
| macos-intel | in_progress | PENDING |
| quality | in_progress | PENDING |
| rust-1-88 | in_progress | PENDING |
| supply-chain | completed | failure |
| windows-installer | in_progress | PENDING |
| windows-x86_64 | in_progress | PENDING |

The remaining remote CI may finish after this receipt; it cannot override the red
mandatory security check. Earlier failed and superseded/cancelled attempts remain
retained and are never counted as current success. The existing PTY failure also
remains historical acceptance evidence until the current native lane settles.

Authenticated readback confirms release/RRC disabled, tag freeze active and all
thirteen required contexts unchanged. Original Vesper HEAD/status/worktree inventory
and 14,598 protected entries are unchanged. The independent destination checkout's
three prior local report files retain their exact bytes. No existing runtime source,
package version, executable, data root, credential, session, installer, ACP identity
or browser field was changed. No installation, migration, RRC worker, release tag
or publication occurred. The new module inherits baseline version for compatibility.

Machine receipts/logs are in the independent local stage1 evidence directory;
blocked-closeout.json and supply-chain-failure.log are the final gate references.
These post-CI closeout updates remain local, with no follow-up documentation push
that would restart CI under an already-known unresolved security gate. Source is
reviewable in PR 3; the local report/PRD/index record BLOCKED, not completion.
Stage 2 remains unauthorized. After separate security remediation and complete
Stage 1 CI/merge, Stage 2 still requires owner approval and resolution of external
package/ACP availability, uncertain mappings and future version-lifecycle decisions.


## Authorized Stage 1 security and voice remediation

Objective: repair PR 3 blockers without technical identity activation, Stage 2,
release activity, verification weakening or changes to the original source project.
The prior blocked receipt is historical; none of its old-head checks is reused.

Security method: `cargo tree -i rustls --locked --offline` traces reqwest through
hyper-rustls, tokio-rustls and platform verification, plus xAI's tokio-tungstenite /
tungstenite transport. Consumers include both hosts, GLM/OpenAI/xAI providers,
MCP, web-fetch and harness composition. `cargo update -p rustls --precise 0.23.46`
changes only the rustls version/checksum in the root lockfile; no manifests or
other resolved versions change. The fetched package declares Rust 1.71 MSRV,
below workspace 1.88. Current RustSec database entries 0346/0347/0348/0349 all
require >=0.23.46. Local `cargo audit --json`: zero vulnerabilities/warnings.
`cargo deny check`: advisories, bans, licenses and sources pass; existing duplicate
version warnings remain. No advisory exception or security policy change occurs.

Intel investigation: foundation run 38570429468, job 115873875175 fails the
voice pipeline's inner assertion at line 416, `fresh turn segment must settle`,
after 30.93 seconds. The outer line-79 subprocess assertion preserves that cause.
This lane uses macos-15-intel, pinned Rust 1.95, isolated Python WAV synthesis and
controlled pipe playback, without CoreAudio/devices. Its unchanged boundary test
passes. The failing test and worker/playback/bridge sources are byte-identical to
main 57d8280 before correction; an unchanged-source Linux run passes both cases
in 4.83 seconds, which does not prove a transient or macOS reproduction.

Confirmed implementation defect: WaitSlot previously checked the completion value
before installing its waker, using separate mutexes. Delivery between those steps
stores the result, finds no waker and leaves the caller Pending indefinitely.
This reachable baseline race explains a stranded synthesis awaiter; the native
failure log has no thread-state trace, so the specific lost wake in that one run
is an inference, not a captured stack. Classification is a proven synchronization
implementation defect, not an assumed runner outage or harmless flake.

Deterministic proof: completion_during_waker_registration_cannot_be_lost forces
delivery in the registration window on the original ordering and fails immediately
(Pending without a wake). Registering the waker BEFORE inspecting completion fixes
that exact interleaving. The same test also verifies delivery before the first poll
and after Pending. No deadline, voice fixture assertion, queue size or production
engine/playback identity changes. The empty production hook uses the same path as
the deterministic registration test. The correction is in the voice composition
bridge and applies wherever that shared bridge is used; it adds no new host feature.

Changed corrective files: root Cargo.lock; crates/vesper-voice/src/composition/
blocking.rs; root and voice DOX; this report, evidence index and owning Stage 1 PRD.
The three pre-existing local Stage 1 closeout edits are retained and reconciled
here. Original project and primary destination dirty reports remain untouched.

Local command receipts live in the private sibling evidence directory: baseline
voice log, deterministic red log, voice-core green log, voice-host green log,
network-compatibility green log, RustSec audit JSON and cargo-deny log. The core
adapter suite passes 193 tests including the new ordering regression. Targeted
host pipeline, ten-turn playback and interruption/lifecycle checks, provider/network
compatibility, identity tests and ordinary verification are recorded before push.
Targeted host tests pass 18 cases (5 interruption, 11 playback, 2 pipeline).
Provider/network tests pass 194 cases; two pre-existing live/browser cases remain
ignored and are not counted as proof. Full local verify is running; hosted
exact-candidate security, MSRV and all native targets remain PENDING until fresh CI.
No local Linux result substitutes for native Intel evidence. Live vendor TLS/device
calls are outside deterministic verification; public-provider credentials are unused.

Acceptance: a fresh exact-head thirteen-context matrix and normal protected merge
remain mandatory. Release/RRC workflows remain disabled and release tags frozen.
No Stage 2 or publication is authorized. The excluded research lockfile limitation
above remains separate from production root security evidence.
