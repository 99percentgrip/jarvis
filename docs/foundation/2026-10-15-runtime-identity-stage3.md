# Stage 3 runtime identity isolation — execution report

## Objective and current status

Implement only the owner's revised Stage 3 directive in the isolated branch
`feat/stage3-runtime-identity-isolation`.
Status: PR #5 corrective candidate passed final local verification; initial hosted matrix failed,
normal protected merge and post-merge workflows remain pending.
The [PRD](../runtime-identity-stage3-prd.md), [architecture](../architecture/runtime-identity-stage3.md)
and [source inventory](runtime-identity-stage3-inventory.json) own the acceptance trace.

## Preflight and preservation

Authenticated main equals `508ab4d962a9aa90eb4e6e37b7e340a6569af90b`.
Exact-baseline runs 38678031109 (canonical), 38678031139 (MSRV), 38678031113
(five-target foundation) and 38678033433 (web-driver) all succeeded. The final
Stage 2 head `68d0e2bdd47f7bd11c6ddffbd3d19b0869f9e40c` retains all thirteen
successful trusted required checks. Rulesets 25049950 and 25049955 remain active
without bypass. The feature worktree was created only after this gate passed.

Original Vesper remains at `0b5630d271965e7d9df3a0a09c116c7f8c44042e`.
The existing preservation comparison checked 14,598 entries, 158 dirty paths and
six older reports with zero changes. All nine dirty reports across the three
previous independent worktrees are additionally hash-pinned for Stage 3 preservation.
Private evidence/logs are retained outside Git; their absolute location is
delivered in the conversation.
Source changes stay in the new worktree; builds use the existing independent test cache,
never Vesper Git storage, build products, installations or user data.

## Methods, files and implementation

The [reviewed source inventory](runtime-identity-stage3-inventory.json) contains
26 resource groups with source symbols/lines, consumers, writers and regression
entry points, 96 reviewed environment mappings, platform roots, compatibility
exceptions and the complete changed/new file list. It includes Rust consumers,
seed helpers, fixtures, verification tooling and nearest owning instructions.
The initial 663-literal scan across 122 Rust files was only reconnaissance;
serialization/canary symbols were false environment candidates and remain unchanged.

Implementation followed six dependency-ordered units: platform/private ownership;
credentials; sessions/persistence; Browser/Voice/Bridge; process/IPC and dormant
updater/recovery; integrated regression verification. Exact changes are enumerated
in the inventory rather than repeated as an unreviewable filename list here.

- Shared config/security: pure native roots, private override preflight, ownership
  markers, owner-only permissions, guarded paths and atomic private replacements.
  Existing nonempty unowned roots refuse; no automatic import or parent chmod.
- Auth/provider boundaries: independent native service/application-tagged vaults,
  unchanged explicit external API-key inputs, no legacy fallback and no token
  transfer. Windows empty native reads do not create a disk lease. Independent
  subscription authorization is unavailable, so ordinary login, refresh and
  retained-token dispatch refuse before network contact.
- Sessions/cognition/memory/checkpoints: independent native/project roots, owned
  runtime readers/writers and guarded database/config leaves; stable IDs and
  serialized formats. Normal ACP does not discover legacy sessions even with all
  features. Generic legacy compatibility remains explicit offline fixture authority.
- Settings/skills/helpers: independent persisted settings and skill-store binding;
  no writes to installed skill libraries. Citation cache owns its files; explicit
  user-selected output stays allowed. Google authorization and obsolete TouchDesigner
  setup are inactive before old-state access, pending independent adapters.
- Voice/Browser/Bridge: owned capture/pack/tool/log roots, unique sidecar temporary
  state and safe process records; existing cancellation/lost-wakeup assertions,
  device semantics and external protocols remain. Container/machine identity is
  private; already healthy user runtimes are not reset or adopted.
- Process/IPC: application-tagged daemon records; generation-bound FLM orphan
  cleanup with pinned Linux pidfd handles; ephemeral independent mobile endpoints.
  Unsupported process generation refuses orphan cleanup rather than signalling a guessed PID; foreground owned-child teardown remains available.
- Updater/recovery: ordinary entrypoints remain dormant before ledger/channel access.
  Only an explicit non-default integration executable enables controlled offline
  controller fixtures; a production feature flag alone grants no such authority.

## Platform and security evidence

The [architecture matrix](../architecture/runtime-identity-stage3.md) records Linux
XDG, macOS Support/Caches/Logs and Windows roaming/local separation. Real filesystem
first-run tests inject each platform, seed old stores and prove repeated initialization
preserves their bytes and ordinary user project files. Native Windows DACL/junction
execution remains separately required in CI; cross-compilation is not execution.

Unix sensitive roots/files use 0700/0600, with ownership and no redirected/traversing
private paths. Windows uses safe pinned native handles and protected current-user,
SYSTEM and administrator DACLs. Only fresh/empty designated roots are secured;
existing parent ACLs/modes remain unchanged. Same-directory synced temporary writes
preserve whole records and existing session crash-recovery semantics.

Synthetic old credential/session/memory canaries remain unchanged. Normal ACP
process tests ignore old environment inputs and refuse explicit private-state
adoption. A real user project named agent-vesper remains usable; application
private containment does not restrict authorized general project access.

## Commands, outcomes and causal corrections

All local runtime tests use a private synthetic HOME/USERPROFILE, XDG and Windows
app-data environment, scrubbed provider secrets/private legacy variables, bounded
Cargo concurrency and the independent shared build cache. Provider HTTP proxies
refuse external destinations while loopback protocol fixtures remain available.
Audit/deny alone receive public registry/advisory access, without provider credentials.

Private logs live under the owned external evidence directory, whose path is
reported at delivery. These are execution receipts, not user-state data.

| Receipt / command | Observed outcome |
| --- | --- |
| `core-final.log`: locked auth/config/security all-target/all-feature tests | PASS; 81 test executions, 2 explicit hosted-native cases ignored locally |
| `windows-msrv-final.log`: Rust 1.88 auth/security Windows MSVC all-target/all-feature check | PASS; cross-check only |
| `audit-final.log`: `cargo audit` | PASS; 557 lockfile dependencies, no vulnerability reported |
| `deny-final.log`: `cargo deny check` | PASS advisories/bans/licenses/sources; allowed duplicate-version warnings retained |
| `guards-final.log`: `cargo xtask naming-guard` | PASS; all 36 hits frozen, no baseline regeneration |
| `verify-pass8.log`: complete workspace tests and architecture before final guard | 3,428 passing test executions across 252 blocks; stopped on nine naming-guard hits, not a full verify pass |
| `verify-serialized-candidate.log`: serialized `cargo xtask verify` | PASS; 3,631 passing test executions across 272 blocks, including repeated targeted suites; all 165 exact acceptance cases passed |
| `transcript-serialized.log`: all-feature ACP transcript suite | PASS; all seven tests, unchanged assertions |
| `voice-pack-ownership-final.log` and `msrv-pack-candidate.log`: all-target/all-feature voice-pack suite | PASS on default compiler and Rust 1.88; new native ownership/redirect regression included |
| Final empty legacy-platform guard | Narrow source delta after the broad workspace graph; separately verified by `core-candidate.log`, final Clippy/build/MSRV checks and exact-head CI |
| `core-candidate.log`, `clippy-candidate.log`, `msrv-frozen-candidate.log`, `windows-msrv-frozen-candidate.log`, `default-frozen-candidate.log`, `all-features-frozen-candidate.log` | PASS, all six serialized commands; core suite 82 passing executions and 2 hosted-only ignored cases; final guard included |
| `workflow-guards-final.log`: offline GitHub verification guard tests | PASS; 18 tests |
| `deny-all-features-final.log`: shipping feature supply-chain policy | PASS advisories/bans/licenses/sources; duplicate warnings remain visible |
| Native five-target, two contained browsers, installer, namespace and exact-head verification | Pending hosted candidate |

Earlier failures identified real missing ownership checks and stale fixture roots.
Private-root and auth preflight were corrected before tests could write; positive
fixtures now explicitly own synthetic roots before seeding. Normal runtime still
refuses unowned records. Existing timing bounds, required tests, frozen digests and
all required check names are preserved. The voice capture byte-count test excludes
only the exact validated ownership marker, retaining assertions for all audio and
unknown residual bytes. The native release-admission test now asserts the intended
explicit inactive error under the original deadline, rather than an old Git error.
Nine naming hits were corrected with scoped technical declarations/fixture construction;
all thirteen upstream patterns and the frozen baseline remain unchanged.

### Final voice-pack boundary correction

The final residual-path scan found a non-shipping audio example with a hard-coded
old user home and a pack-child redirection gap beneath an otherwise owned data
root. The example now resolves native data/cache roots. Native pack discovery
refuses unowned nonempty or redirected roots, setup owns fresh roots/staging and
reestablishes ownership after backup, and record/asset/lease boundaries validate
private paths. New filesystem regression retains an old canary and refuses a
redirected pack. Both unavailable-root sentinels refuse setup before transfer.
Earlier broad logs are interim evidence; the subsequent candidate verification
and focused native/MSRV pack checks cover this final correction. No live model
setup, synthesis, microphone or speakers were executed locally.

### Verification scheduling correction

An overlapping default build replaced the shared ACP executable while all-feature
process tests were running: the advertisement contained 47 commands instead of
the required 49. This run is invalid candidate evidence; no assertion was changed.
Final build/test phases are serialized, with a focused transcript rerun and a
fresh complete candidate verification required. The earlier full MSRV receipt
likewise remains interim; a fresh Rust 1.88 all-target workspace check and focused
pack tests passed, while exact-head CI separately requires the full MSRV suite.

## First hosted candidate and corrective verification

Destination repository `99percentgrip/jarvis`, PR #5, initial head
`9bfbfe204e4150776c14715dc628f234008cd53e`, did not satisfy the merge gate.
Supply-chain, Rust 1.88, both contained browsers and Native namespace Hive
succeeded. Quality, Windows installer, Linux x86_64/ARM64, macOS Apple Silicon,
Windows x86_64 and the exact-head aggregate failed; macOS Intel was cancelled.
None of those failed/cancelled contexts authorizes merging.

The retrieved job logs establish four fixture corrections:

- Windows ACP used `/tmp` as a session cwd, which is not a native absolute Windows
  path. Strict private-path validation correctly refused the skill store and
  emitted a diagnostic before streamed reasoning. The process harness now uses
  its actual private temporary workspace. Its shared environment restoration
  also assigns synthetic HOME/USERPROFILE/roaming/local roots rather than
  inheriting real Windows profile roots. A deterministic no-write regression
  verifies all four explicit roots and excludes private roots from inherited keys.
- The Windows installer built its real release executable successfully, then
  signed-out authentication fixtures failed native privacy validation: chmod
  cannot establish a Windows DACL. Synthetic vault parents now receive the
  existing native inheritable protected ACL helper before records are written.
  LM Studio has no subscription tombstone; an invalid synthetic vault refuses
  native lookup without inventing an API key. Original terminal assertions remain.
- Both Linux lanes and macOS Apple Silicon reached the old release-host test,
  whose ordinary shipping hosts correctly refused the intentionally dormant
  controller. CI now builds explicit non-shipping integration hosts for the
  unchanged synthetic ledger status/retry/cancellation assertions. A separately
  exercised ordinary-host branch additionally requires release-disabled responses,
  unchanged supplied ledger bytes and zero provider connections. No worker-start,
  publication, external GitHub mutation or real release ledger is exercised.
- Canonical voice PTY still looked in `data/agent-vesper/captures`; it now uses
  the common native J.A.R.V.I.S. data-root fixture resolver. The retained capture,
  retry/discard, bounded long transcription and zero-residual assertions remain.

macOS Intel reached the 90-minute job boundary without an assertion failure:
workspace tests had succeeded after approximately 69 minutes; architecture/
acceptance was still running. The verified Stage 2 baseline took approximately
68 minutes for the complete same lane. This is incomplete evidence, not a pass
or proof of a test defect. The macOS Intel outer job budget is now 120 minutes, based on that observed
cold-compilation/complete-acceptance workload; other native lanes remain at 90.
Individual test and step deadlines remain unchanged. The aggregate retains its
170-minute bounded exact-head wait. All original cases must actually pass;
increasing total scheduling capacity cannot turn a failed assertion into success.
Fresh native CI must finish successfully before integration.

Local corrective receipts include `acp-native-workspace-repair-repeat.log`
(full all-target/all-feature ACP suite), `first-launch-fixture-repair.log`,
`settings-auth-fixture-repair.log`, `voice-pty-path-repair.log`,
`offline-host-parity-repair.log` (all four selected providers) and
`clippy-ci-repair.log` (workspace/all targets/all features, warnings denied).
The first ACP corrective run had one `ETXTBSY` spawn error in the unchanged
controlled-Cargo fixture; five bounded exact-case repeats and the full ACP suite
then passed without source/test relaxation. Its precise OS cause remains unproven.
The registry guard also rejected a newly introduced fixture variable during
corrective verification. The host now uses the already reviewed explicit test-gate
input; no new identity or guard exemption was introduced. Final candidate
verification and fresh thirteen-context CI remain mandatory.

### Intermediate corrective local gate

`verify-ci-repair-complete.log` passed the full `cargo xtask verify` command:
3,635 passing test executions across 273 result blocks and all 165 exact acceptance
cases (249,887 ms). Counts include repeated targeted suites, not unique tests.
The 43 ignored executions remain disclosed; hardware, live providers and hosted
native credentials are not inferred from them. This supersedes the corrective
runs stopped by the registry/document naming guards. Both guards remain unchanged;
all 36 naming hits are frozen and all current environment inputs are registered.
The separate workflow guard suite passed 18 cases and the trigger/release contract
suite passed 11 cases. No actual release workflow was invoked.

Rust 1.88 workspace/all-target/all-feature checking, locked default and all-feature
workspace builds and the final explicit offline host build passed. The final
ordinary-host negative fixture initially over-constrained `/ci`; this is a
legitimate read-only user-project integration, not release admission. Its corrected
assertion requires truthful unavailable status without authentication and unchanged
ledger bytes, while status/retry/cancel retain release-disabled assertions. The
four-provider synthetic parity assertions remain unchanged and passed again in
`offline-host-parity-final.log`. These receipts precede the final private-preference and temporary-resource audit
below; that source delta requires fresh complete verification.

### Final private-preference and temporary-resource audit

An additional rendered Settings run exposed a fixture viewport assumption:
only fourteen menu rows are visible, so hidden rows require the existing focus
scroll handler before clicking. The original twenty-second observation deadline
is retained. Restart execution choices are restored after leaving the landing
screen; the fixture now opens Settings before checking the same saved model.
No production menu, command, model value or preference assertion is removed.

The follow-up source review found private leaf gaps beyond default-root separation.
Settings choices/theme/provider preferences, keybindings, grouped-save snapshots/
rollback and native plugin trust records now validate private paths and owned
stores; writes use the shared same-directory atomic helper. Failed native trust
persistence restores the previous in-memory authority and reports failure.
Filesystem tests refuse unowned/public/redirected settings and publisher records,
retain exact foreign bytes and prove read-only discovery of missing state.

Editor drafts now use uniquely owned ephemeral directories; screenshot files
live in the private native cache so existing sixel path rendering remains valid.
The application-owned screenshot output is secured before queuing. User-selected
project exports and arbitrary authorized project files retain ordinary authority.

FLM VAD executes the unchanged embedded Python source directly: source inspection
confirmed it has no file-relative imports. Predictable script materialization and
prefix-based stale-directory deletion are gone. Its audio workspace is fresh and
private, held by the blocking worker through settlement even after caller
cancellation, preventing late output from recreating a deleted directory.
The new filesystem test proves distinct workspaces, private canonical WAV data,
owned cleanup and survival of foreign matching-prefix audio. No live Silero model
or device integration is inferred from that test.

All active generic snapshot/probe/browser/setup staging now uses a shared exclusive
private temporary-workspace allocator. It validates the base before creation and
secures native ACLs before writes; it never adopts an existing directory. A live
RAII allocation is its ownership evidence and leaves copied project trees free of
application metadata. Persistent stores still require ownership markers. Swarm
artifacts initialize only owned project-private state and newly allocated run roots.
A synthetic legacy-base test proves refusal before any entry is created. Dormant
updater staging uses the same primitive without activating its ordinary route.

Project Web/Sandbox/Skills/Swarm/Bridge/acceptance readers now refuse nonempty
unowned project state before parsing it. The cross-subsystem filesystem regression
generates valid records through real writers, copies only synthetic records without
an ownership marker, and proves refusal with byte-for-byte preservation. This does
not constrain ordinary user project files. Acceptance Save initializes owned state.

The earlier logs above remain intermediate evidence. Final candidate receipts follow.
The first complete run after this audit, `verify-private-final-source.log`,
stopped at four sandbox integration cases: their shared positive fixture wrote
settings without establishing private ownership. The fixture now uses the real
owned-directory initializer before writing its unchanged sandbox demand.
`sandbox-owned-fixture-final.log` passed all six end-to-end cases, preserving
destructive-command denial, unattended approval refusal and unsatisfied-isolation
assertions. The fresh full run below passed; the failed run is not passing evidence.

Fresh terminal receipts `first-launch-private-final.log`,
`settings-auth-private-final.log`, `offline-host-private-final.log` and
`voice-private-final.log` passed against separately copied candidate binaries,
avoiding concurrent compiler replacement. They cover all four signed-out providers,
authentication cancellation, ordinary controller dormancy and synthetic parity,
and unchanged capture/retry/discard/cleanup behavior. The Settings restart fixture
also verifies the saved model's explicit `(current)` tag rather than menu focus;
`settings-private-current-model-pty.log` passed. VAD protocol fixtures passed in
`vad-protocol-private-final.log`; they do not establish live model/device behavior.
The complete offline workflow guard suite passed 18 tests and trigger/release
contracts passed 11 in `workflow-all-guards-private-final.log` and
`trigger-contract-private-final.log`.

External editor/screenshot programs and live VAD are not exercised against real
user data or devices. Their private-storage wiring is source/compile evidence;
existing media/voice and native platform regression suites remain mandatory.

## Final corrective candidate local receipts

`verify-private-candidate.log` passed complete `cargo xtask verify`: 3,645 passing
executions across 275 result blocks, including repeated targeted suites, with
43 ignored executions and all 165 exact acceptance cases (244,004 ms). This run
includes the final private-leaf/workspace changes and owned sandbox fixture.
Clippy denies warnings; formatting, 32-package architecture, identity contracts,
unchanged frozen naming baseline and existing functional suites all passed.

The seven-command serialized gate receipt `final-local-suite-results.json` records
exit zero for that full verify; Rust 1.88 workspace/all-target/all-feature check;
Rust 1.88 Windows MSVC auth/security all-target/all-feature cross-check; locked
default and all-feature workspace builds; `cargo audit`; and all-feature
`cargo deny check`. Cross-compilation is not native Windows execution. The audit
reported no shipping vulnerability; deny advisories/bans/licenses/sources passed.
Existing duplicate-version warnings remain visible.

The final source hash receipt pins active changed sources/configuration and the
four copied terminal-fixture binaries. JSON/Python parsing, owned-document links,
lockfile comparison and whitespace checks passed. External additions are exactly
windows-permissions 0.2.4 and bitflags 1.3.2; no existing dependency version changed.
Authenticated readback still shows approved main, thirteen strict trusted checks,
zero bypass, a read-only default token and three disabled publishing workflows.
Vesper preservation checked 14,598 protected entries, 158 dirty paths and six older
reports with zero differences; nine prior independent-project report hashes also match.
These local receipts do not authorize integration before the fresh hosted matrix.

## Dependency integrity

The only new external lockfile entries are pinned windows-permissions 0.2.4 and
its bitflags 1.3.2 dependency, required for native ACL ownership without first-party
unsafe code. Existing external versions are unchanged; bitflags references become
version-qualified where necessary. Existing rustix 1.1.4 supplies safe Unix UID/process
identity; tempfile was already locked. No new production dependency is added to the
voice core. Fixed rustls 0.23.46 and quinn-proto 0.11.18 remain locked.
No advisory exception, license relaxation or required-check change is introduced.

## Verification limitations

Linux generation-pinned orphan cleanup is implemented and locally tested. Where
process generation cannot be established, orphan signalling remains refused; this
is a deliberate security boundary, not cross-platform crash-reaping evidence.
Foreground owned-child teardown and native regression gates remain required.


Default local suites intentionally ignore live provider/LM Studio, microphone,
external Resolve/MPRIS, real container/browser/namespace cases, hosted credential
fixtures, selected scale/soak/performance cases and one documentation example.
An ignored case is not passing evidence. Hosted native credential and contained
runtime/browser cases remain separate required CI acceptance; CI does not prove
real-user credentials, device listening quality or independent vendor authorization.
The exact ignored-case list is retained in private test logs.

OpenAI ChatGPT subscription, xAI Grok subscription and bundled Google OAuth are
securely inactive pending independent supported registration/storage paths. Protocol
fixtures verify internal behavior only. Existing explicit API-key paths remain.
Legacy TouchDesigner setup is deferred; external configured control semantics remain.
No new OAuth IDs, vendor approval or live entitlements are invented. The excluded
research workspace advisory remains tracked and excluded from active shipping builds.

## Protected integration and release freeze

Stage 3 PR URL/head, merge commit, integrated tree and thirteen exact-head required
conclusions are pending. All four exact-merge push workflows must succeed before
completion. The active branch ruleset remains unchanged: PR required, zero independent
human approvals, resolved review threads, strict trusted required checks, no bypass,
force-push or main deletion. Publication workflows remain manually disabled and the
v* creation/update/delete freeze remains active; no release/RRC operation is executed.
Post-CI receipts are retained in this owned local report without an automatic extra
source/docs commit after the fully verified candidate.

### Candidate preservation and control readback

At 12:18:59 UTC, preservation again checked all 14,598 protected entries and
158 original dirty paths with zero differences. All nine older dirty report hashes
remain unchanged. Authenticated candidate main remains the approved baseline;
strict thirteen-check rules, trusted Actions app 15368, zero bypass, read-only
default workflow token and all three disabled publication workflows are confirmed.
No repository control was changed.

## DOX pass and readiness effect

Nearest owning config/security/auth/session/memory/provider/voice/sandbox/helper,
application/test and verification instructions reflect the new contracts. Root and
composition contracts distinguish dormant inherited release behavior from permission
to execute it. Documentation ownership/index entries include this report, architecture,
PRD and machine-readable inventory. Historical records, original licenses, frozen
fixtures, public README notice, release receipts and installer destinations remain
intentionally unchanged: they preserve provenance or belong to later authorized stages.

Unreleased behavior: application-private runtime ownership is independent; existing
private legacy state is neither discovered nor imported. No installation, registry
publication, release tag or independent release is delivered. Stage 4 import requires
separate explicit authorization and safe identity translation/no-replay design.
Completion remains pending until final local, protected PR and post-merge evidence exists.

### Excluded research alert readback

Authenticated Dependabot alert #1, `GHSA-2mjx-qc3c-rqvc`, is open against
`spikes/rust-sse-transport/Cargo.lock`. This excluded research workspace is outside
Stage 3 and shipping builds; it must be remediated before any future inclusion.
The active root lockfile retains rustls 0.23.46 and passed audit/deny. The alert
has not been dismissed or suppressed.
