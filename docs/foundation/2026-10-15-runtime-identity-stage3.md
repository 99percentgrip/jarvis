# Stage 3 runtime identity isolation — execution report

## Objective and current status

Current ownership corrections pass complete local regression/security/MSRV/build verification. Thirteen checks passed on the previous head; the corrected head still requires a fresh exact-head matrix and protected integration. No merge has occurred.

Implement only the owner's revised Stage 3 directive in the isolated branch
`feat/stage3-runtime-identity-isolation`.
Current status, 2026-10-18: **CORRECTIVE VERIFICATION IN PROGRESS**.
The owner-authorized maintained libdbus backend correction passes local
audit/deny, complete Rust 1.88 workspace checking, both workspace build modes,
full local verification and real private GNOME acceptance. A new complete
exact-head CI matrix, protected merge and four post-merge workflows remain required.
The blocked e24 candidate and its evidence remain recorded below.
PR #5 remains unmerged. Earlier full local verification passed 3,653 test
executions and all 165 exact acceptance cases, with 43 ignored executions retained.
The hosted candidate settled with eleven successful checks; Windows hit its
90-minute whole-job budget after the workspace and native credentials passed,
and the dependent aggregate failed. Windows now receives the same bounded
120-minute total as Intel macOS, with every test/step deadline and case preserved.
A fresh thirteen-check exact-head matrix, protected merge and four successful
post-merge workflows remain required.
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

## Native cognition fixture follow-up

The second hosted head `492ca71d9c8b029e4be87c03dafc562beac17a56`
failed Windows job 116705800694 in run 38817514427. The exact assertion was
`cognition_binding_derives_layer_paths_and_honors_overrides`, resolving a fixture
`/custom/...` override. Such a path is not absolute on Windows, so the existing
private-path guard correctly returned ownership refusal. The saved native log
records 428 passing agent cases and this one failure; it is not a passing lane.

Only the test fixture now derives both override paths from native temporary
roots. Both override equality assertions remain. A new regression refuses each
relative override before a scope stamp can be persisted. Production scope
resolution and private-path validation are unchanged. `native-cognition-scope-repair.log`
passed 34 scope tests; `native-scope-agent-all.log` passed the complete all-feature
agent suite; `native-scope-msrv.log` passed the 34 scope tests on Rust 1.88.
Formatting and warnings-denied all-target/all-feature agent Clippy also passed.
The previous full workspace/security/build receipts cover the unchanged production
source; this test-only delta still requires a fresh complete thirteen-check head.
No success or cancellation from the superseded head substitutes for that matrix.

## Windows release inventory investigation — unresolved

Head `deb8350a68781d27b91c42556c5815752dd33801` failed installer job
116709785790 after successfully compiling its Windows release executables.
The unchanged signed-out terminal case reached OpenAI management and reported
`Authentication status is unavailable`; it did not render the required panel.
The failure is retained in `deb8350a-windows-installer.log`. Its precise storage
boundary is not yet established; no timing or transient explanation is asserted.

`apps/jarvis-tui/tests/runtime_credentials.rs` now generates the exact Python
terminal tombstone using a UUID-only synthetic account. It separately requires
production directory ownership, file confidentiality, a present matching fallback
and a successful native-first read without changing bytes. The fallback precondition
prevents any need for a user/native credential lookup. This focused test passed on
Linux with both default Rust and Rust 1.88; Clippy and formatting passed. Its native
Windows result is pending and is diagnostic evidence, not a claimed repair.
The Windows installer workflow executes this real regression in the release profile
before the unchanged first-launch gate, with a separate bounded step. No existing
assertion, test timeout, platform or required context is removed or relaxed.
Offline workflow guards and trigger contracts still passed 18 and 11 cases.

## Windows explicit owner-rights fixture correction

Diagnostic head `8e72a87921973f812b0bd8f6ad7bcd3cfe26210c` settled with
10 successful required checks: quality, supply-chain, Rust 1.88, Linux x86_64/ARM64,
macOS Intel/Apple Silicon, both contained browsers and Native namespace Hive.
Windows installer job 116716804530 and native job 116716803685 both failed the
new exact-fixture test at **directory ownership**, before reading any credential.
The aggregate failed accordingly. The successful macOS Intel lane took 62.4 minutes;
other successful native lanes took 31.7–45.5 minutes. These are prior-head receipts,
not authorization to reuse them for a changed head.

The source-derived cause is Python 3.13's explicit `OWNER RIGHTS` (OW) ACE when
creating a 0700 Windows directory. The fixture removed only inherited ACEs, then
added approved principals; the explicit alias survived. Production validation
correctly accepts the actual user SID, SYSTEM and administrators, not that alias.
[The exact Python 3.13.16 source](https://github.com/python/cpython/blob/v3.13.16/Modules/posixmodule.c#L5288-L5295)
records this initial descriptor. Its retrieved source hash and bounded excerpt
are retained in the private evidence directory.

`settings_pty.py::fixture_owned_directory` now replaces only the access section
with a protected inheritable descriptor naming exactly those three approved SIDs,
instead of adding rules to the existing descriptor. Owner/group sections remain
intact, using the documented
[section-specific .NET API](https://learn.microsoft.com/en-us/dotnet/api/system.security.accesscontrol.objectsecurity.setsecuritydescriptorsddlform?view=netframework-4.8.1).
Only synthetic fixture directories are affected. Production Rust, security policy,
credential semantics, existing assertions and all test deadlines are unchanged.
The exact-fixture regression and four-provider signed-out first-launch script
passed locally (`windows-fixture-dacl-local.log`,
`windows-fixture-first-launch-local.log`). Native Windows proof remains mandatory;
Linux cannot establish Windows ACL behavior. JSON/Python/link/whitespace checks and
trigger-contract verification passed. A fresh complete thirteen-context matrix is
required before protected integration.

Superseded candidates with confirmed mandatory failures had their unfinished
verification runs cancelled to free runners. The current candidate was never
cancelled; every cancelled/missing/failed old result remains non-passing evidence.
The scoped cancellation receipts and earlier native cognition case passes are
retained privately. No repository control, release workflow or tag rule changed.

## Credential operation-lock ownership correction

A source review found that OpenAI and xAI acquired their operation lease before
`SecureCredentialStore` checked the selected vault's application ownership.
An explicit unowned private-state location could therefore receive a lock file
even though subsequent credential reading refused it. This was a verified Stage 3
non-mutation violation, not a hypothetical concern or native runner issue.

The production `try_process_lock` entrypoints now call the existing
`prepare_operation_storage` guard before opening or creating any lease file.
Only the existing explicitly injected unit-test store seam bypasses this guard;
ordinary builds, including integration-feature builds, always perform it.
Both hosts inherit the correction through their existing provider adapters.
No credential parsing, external protocol, keyring policy or timeout was weakened.

`provider-lock-ownership-red.log` records both new unowned-vault tests failing
against the previous source. `provider-lock-ownership-green.log` records six
passing focused tests: both providers refuse before creating a lock or ownership
marker, preserve the exact synthetic vault bytes, and support private reusable
leases for properly owned stores. The tests use exclusive synthetic temporary
fixtures without native SDK, live account or user-state access. The nearest
provider DOX contracts were updated with the guard ordering and test-only seam.
Fresh `cargo xtask verify` passed: 3,650 test executions across 275 result
blocks, 43 ignored executions explicitly retained, and all 165 exact acceptance
cases in 257,938 ms. Formatting and all-target/all-feature Clippy passed within
that run. The serialized final sequence also passed Rust 1.88 workspace/all-target/
all-feature verification, Windows MSVC auth/security cross-check, default and
all-feature workspace builds, `cargo audit` and all-feature `cargo deny check`.
Those six command receipts are recorded in `provider-lock-local-suite-results.json`.
A new complete thirteen-check candidate matrix remains required before integration;
earlier remote success does not cover this change.

## Windows constrained-job launcher correction

Candidate `3575710341ae615492394dffb107a638ec403bb2` failed in
Windows native job 116865113634, workflow run 38867981294, in
`99percentgrip/jarvis`.
The Python launcher set `AGENT_VESPER_WINDOWS_JOB_FIXTURE`, but the actual Rust
`windows_job_fixture` guard already required `JARVIS_WINDOWS_JOB_FIXTURE`.
The selected test returned without running its native probe; the existing required
`WINDOWS_JOB_OBSERVATION` receipt was absent and the launcher correctly failed.
This was a deterministic missed fixture identity writer, not a timeout or outage.

Only the launcher's activation identifier changes. The actual 2 GiB Job Object,
production governor, unsafe-Cargo-admission refusal, memory/pressure assertions,
60-second child deadline and owned-child cleanup remain unchanged. This head
settled with eleven successful required contexts; Windows native and the dependent
exact-head aggregate failed. The Windows
log independently records all four new provider lock-ownership cases passing,
native credential persistence/reload, first launch and real voice signals; later
steps in that failed job are not passing evidence.

The new `windows_job_launcher_activates_the_registered_native_probe` identity test
reads the actual Python launcher, actual Rust consumer and approved runtime mapping
on every platform. `windows-job-environment-red.log` proves its pre-fix failure;
`windows-job-environment-green.log` records eleven passing identity contracts;
`windows-job-msrv-green.log` repeats them on Rust 1.88. Focused all-target/all-feature
Clippy, Python syntax, the exact one-identifier launcher delta, report links,
inventory consistency and whitespace passed. The first documentation wording
failed three frozen naming checks; only the prose was corrected, with no baseline
regeneration. `windows-job-naming-corrected.log` records the successful guard rerun.
This is source-derived activation proof, not native Job Object execution. The next
exact-head Windows job must execute and retain the real constrained observation.
No production Rust behavior, external dependency or verification policy changes.

## Recurrent local controlled-wrapper execution correction

`verify-windows-job-candidate.log` stopped at
`controlled_cargo_metadata_emits_valid_json`: Linux returned OS error 26
(`ExecutableFileBusy` / `Text file busy`) while executing the freshly generated
Cargo wrapper. Its other seven process cases passed. This repeats an earlier
observed failure that five bounded focused reruns had not reproduced. The precise
other process holding a writable descriptor was not identified; the recurrence
is retained rather than dismissed as a passing run or hidden behind retries.

The fixture now links each private `cargo` path to the checked-in immutable
`controlled_cargo_fixture.sh`. Receipt and release paths are separate fixture-local
data bindings, read without interpolating them into executable source. Missing
bindings fail before metadata success. The exact JSON, governor environment,
20 ms gate polling, exit classification, original deadlines and all lifecycle
assertions remain. LF bytes are pinned by a targeted `.gitattributes` rule.
This follows the existing immutable executable contract used by container fixtures;
production runtime and controller code are unchanged.

`immutable-cargo-fixture-red.log` proves the new structural/concurrent regression
rejects the old generated executable. `immutable-cargo-fixture-green.log` records
all nine ACP governor tests passing, including eight concurrent metadata wrappers
with quoted/spaced paths, independent receipts and unchanged shared executable
bytes. Focused all-feature Clippy and Rust 1.88 test-target checking, POSIX shell
syntax, LF/executable attributes, the frozen naming guard and changed report
links/JSON/whitespace all passed. This removes the write/execute hazard instead of
retrying an executable failure. Full local verification and the complete new exact-head matrix are still
required. Earlier successful source receipts remain prior-candidate evidence.

## Windows complete-workload budget correction

Candidate `701ca42c9a08d5fcdc662d20b973999c74c80871` retained eleven successful
mandatory checks. Windows job 116899939976 in run 38878713014 exhausted its
90-minute total budget during fixture/architecture validation. Its complete
workspace tests passed in 5,224 seconds, and native credential persistence/reload
passed in 58 seconds. The frozen routing-quality matrix alone passed after
3,767.61 seconds; compilation took 7 minutes 29 seconds. No assertion failure
was reported in those completed suites. Later cancelled/skipped stages remain
unverified, including the corrected constrained-job probe. This is measured total
workload, not an asserted external outage or a reason to omit the expensive cases.

The foundation workflow gives Windows the existing Intel macOS 120-minute total
budget. Linux and Apple Silicon retain 90 minutes; serial Windows execution,
all test cases, individual timing assertions, all step deadlines, five targets,
thirteen required contexts and the exact-head aggregate remain unchanged.
`windows-whole-job-budget-evidence.json` retains the native step timings and log
digest outside Git. A new complete exact-head matrix must run; successful results
from the cancelled candidate cannot substitute for it.

The latest runtime/test source passed `verify-immutable-wrapper-candidate.log`:
3,653 executions across 276 result blocks, 43 ignored executions, all 165 exact
acceptance cases in 209,425 ms, formatting, Clippy, architecture/naming/identity
and fixture/host verification. The budget-only repair leaves those runtime and
test sources unchanged. Offline workflow/trigger guards, parsed YAML and the frozen naming guard passed;
the structural diff confirms that only the intended whole-job budget changes.

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
Completion still requires verified remediation, protected integration and four
successful exact-merge workflows. Earlier blocked-head evidence is historical.

### Excluded research alert readback

Authenticated Dependabot alert #1, `GHSA-2mjx-qc3c-rqvc`, is open against
`spikes/rust-sse-transport/Cargo.lock`. This excluded research workspace is outside
Stage 3 and shipping builds; it must be remediated before any future inclusion.
The active root lockfile retains rustls 0.23.46 and passed audit/deny. The alert
has not been dismissed or suppressed.

## 2026-10-18 security closeout — blocked

### Exact source and failure

PR #5 head `e24ec27e6d34bfcb3b904e51dd527786c307b12a` has tree
`832a90094669bb0fc30b9eb5f7009a84c67f7571`. Main remains the approved
Stage 2 baseline. In `99percentgrip/jarvis`, supply-chain job 117572925434,
workflow run 39092807142, failed in `cargo deny`. No merge was attempted.
Other hosted checks retain their actual latest conclusions in the timestamped
matrix below; an in-progress check is not passing evidence.

| Advisory | Locked package | Classification |
| --- | --- | --- |
| RUSTSEC-2026-0370 | zvariant 5.13.1 | Informational: unmaintained |
| RUSTSEC-2026-0371 | zbus_names 4.3.4 | Informational: unmaintained |
| RUSTSEC-2026-0372 | zvariant_derive 5.13.1 | Informational: unmaintained |

The primary RustSec records are dated 2026-10-16 and list no patched versions.
These are maintenance advisories, not newly demonstrated runtime exploits.
The existing fail-closed policy nevertheless makes them mandatory blockers.
The [upstream retirement record](https://github.com/z-galaxy/zbus/issues/2043)
describes consolidating the crates into zbus 6. No advisory ignore, severity
downgrade, security-policy edit or dependency update was made.

### Dependency and compatibility evidence

An isolated locked/offline all-feature/all-target `cargo tree --invert
zvariant@5.13.1` passed. Both hosts reach this graph through
`jarvis-auth` → keyring 4.1.6 → zbus-secret-service-keyring-store 1.0.0 →
secret-service 5.1.0 / zbus 5.18.0. zbus and its macros introduce the retired
crates. This shipping credential graph is separate from the excluded research alert.

Read-only crates.io metadata and published archive manifests confirm:

- Latest keyring 4.2.0 still selects the same backend major series.
- Latest backend 1.0.1 and secret-service 5.2.0 require zbus `^5`.
- Published zbus 6.0.0 declares Rust 1.89; the project requires Rust 1.88.
- Same-major updates do not remove the retired crate identities.

No compatible published lockfile-only resolution was found. This does not claim
that a reviewed implementation/backport is impossible. The minimal prerequisite
is a maintained native credential dependency path compatible with Rust 1.88,
including upstream/backport and native-store regression evidence. Replacing the
native credential backend or raising the MSRV is not silently admitted here.
Removing Linux native credentials or weakening fallback security is not a fix.

### Commands, receipts and deviations

`maintenance-blocker-local-audit.log` records exit zero for `cargo audit` against
1,338 current advisories and 557 dependencies, with three maintenance warnings.
`maintenance-blocker-local-deny-current.log` reproduces exit one for
`cargo deny --all-features check`: advisories fail; bans/licenses/sources pass.
An initial deny command placed `--all-features` after `check`, exited two for
CLI syntax, and is retained separately; the corrected invocation establishes
the actual failure. No program-suite repeat is needed for this read-only
investigation and local evidence-only update.

Private `stage3-maintenance-blocker-receipt.json` pins source/tree, job/run,
primary advisory/registry/archive hashes, local exits and governance readback.
Published archives were read in memory without extraction or execution.
Provider secrets and user keyrings were not accessed. Earlier full verification
remains prior evidence and cannot establish a current complete security pass.

Only this report, the evidence index and owning PRD receive local closeout updates.
They are not committed or pushed, preserving the exact candidate without a new
matrix for post-CI receipts. Runtime sources, manifests, lockfile, tests, CI
definitions, rulesets and permissions are unchanged during this inquiry.
Hosted verification is left running; later successes cannot override the failed
mandatory gate. No missing/skipped/cancelled result is classified as passing.

### Preservation, controls and readiness

At 03:23:43 UTC, original Vesper HEAD/status/worktree inventory and 14,598
protected entries, 158 dirty paths and six older reports matched preservation
evidence with zero differences. All nine earlier independent-project report
hashes also match. Authenticated controls retain thirteen strict trusted checks
(Actions app 15368), zero bypass, active release-tag freeze, a read-only default
token and three manually disabled publication workflows.

There is no merge SHA or post-merge result: main was not changed. Stage 3 is
blocked until the native credential dependency blocker is resolved within the
MSRV/security contract and a new exact-head matrix passes. Stage 4, state import,
installation, registry migration, releases and real RRC remain unauthorized.
Root and owning DOX instructions stay unchanged because this closeout changes
evidence/status rather than ownership or operating rules.

### Exact-head check snapshot

Observed 2026-10-18T03:25:21.768742+00:00. All rows are trusted Actions app 15368.

| Required context | Actual status / conclusion |
| --- | --- |
| quality | in_progress / pending |
| windows-installer | in_progress / pending |
| supply-chain | completed / failure |
| rust-1-88 | in_progress / pending |
| linux-x86_64 | in_progress / pending |
| linux-arm64 | in_progress / pending |
| macos-intel | in_progress / pending |
| macos-apple-silicon | in_progress / pending |
| windows-x86_64 | in_progress / pending |
| Contained browser linux-x86_64 | completed / success |
| Contained browser linux-aarch64 | completed / success |
| Native namespace Hive | completed / success |
| `Jarvis` verification | in_progress / pending |

Merge: NOT PERFORMED. Four post-merge workflows: NOT RUN.
These observations are time-bounded; continuing hosted runs may settle later.
No later success can clear the current mandatory supply-chain failure.

### Local closeout document checks

The frozen naming guard and whitespace check passed. All changed relative links
resolve, inventory/evidence JSON parses, and the three local document changes are
the only unstaged differences from the candidate. Full index inspection also
retains eleven pre-existing missing historical targets; they are not claimed as
valid links and no new broken link was added. URL-encoded paths were decoded
before checking. `maintenance-blocker-doc-validation.json` lists the old gaps.
At 03:26:41 UTC, read-only Git inspection reconfirmed independent remotes/common
directories/object databases, no alternates and zero shared object-file inodes.
No source or repository settings changed during this closeout.

## Authorized maintained Secret Service correction

### Decision and preserved evidence

The owner explicitly authorized investigation and corrective implementation after
the blocked closeout. All three existing uncommitted reports were first copied
byte-for-byte and SHA-256 pinned in private `pre-libdbus-local-reports.json`.
The failed head remains immutable; no earlier failure is converted into a pass.
Source/lockfile changes remain on the same Stage 3 feature branch and PR #5.

| Option | Evidence and disposition |
| --- | --- |
| A: keyring wrapper feature substitution | Published keyring 4.1.6 `v1.rs` hard-codes the zbus store. Its CLI feature also enables that store. Keeping either cannot produce the required clean all-feature graph. Rejected. |
| B: explicit keyring-core stores | Selected. The maintained libdbus backend supports Rust 1.88 and preserves Secret Service collections, service/username attributes, UTF-8 values and encrypted DH sessions. Existing native macOS/Windows stores remain pinned. |
| C: another native implementation | Unnecessary broader change while B satisfies the native abstraction. Keyutils has no approved equivalent persistence evidence; plaintext/custom cryptographic stores are not accepted. |

Published upstream manifests/source were read, hash-pinned and retained outside
Git. The [backend](https://github.com/open-source-cooperative/dbus-secret-service-keyring-store)
and [transport](https://github.com/open-source-cooperative/dbus-secret-service)
are the maintained keyring ecosystem alternative. This changes the D-Bus client
transport, not the operating system's Secret Service implementation.

### Dependencies, platforms and distribution

`keyring = 4.1.6`, zbus 5.18.0, secret-service 5.1.0 and the three retired crates
leave the shipping lockfile. The explicit core remains 1.0.0; macOS store 1.0.1
and Windows store 1.1.0 are unchanged. Added packages are dbus store 1.0.1,
dbus-secret-service 4.1.0, dbus 0.9.12, libdbus-sys 0.2.7 and
zeroize_derive 1.5.0. Twenty packages become unreachable; no retained external
version changes. Existing rustls 0.23.46 and quinn-proto 0.11.18 stay fixed.
The zeroize derive feature is required by the replacement's session-secret type.

Production selects a store directly rather than mutating keyring-core's global
default. Linux requires `crypto-rust`; the backend negotiates DH and never selects
plain sessions. The existing macOS/Windows constructors and error variants remain.
No keyutils, sample/mock production store, OpenSSL alternative or advisory
exception enters the graph. Pure identity definitions remain side-effect-free.

Linux dynamically links OS-owned `libdbus-1.so.3`, keeping native library updates
with the supported distro. Build packages are libdbus development files and
pkg-config; native CI additionally installs GNOME/D-Bus fixture tools. Every active
Linux workspace-compiling job provisions them before Cargo, with hosted-only
guards and bounded package commands. No workflow context, assertion, platform,
publication setting or test deadline is removed. Independent future installers
must verify the runtime package; no installation/update route is activated here.

Vendored libdbus was evaluated in an initial local build and then removed from
the candidate: its source snapshot would shift C-library update/distribution
ownership to the application. No vendored C or OpenSSL code is shipped by this
selection. Local missing development packages were downloaded as signed Fedora
RPMs, verified and extracted under private evidence; system packages were not
installed or replaced. The build uses the already installed distro library.

### Credential security and real native evidence

The missing-service red test reproduced an existing Linux behavior: native failure
silently wrote an owner-only plaintext fallback. Production service writes now
return `Unavailable` instead. The test passed after that minimal guard. Existing
owned retained-vault read precedence, explicit path-fixture behavior and the
macOS/Windows fallback/native-record contracts remain; no legacy data is adopted.
Successful native replacement still retires an older retained value. Native
service/account identities and OAuth non-transfer remain unchanged.

Real Linux acceptance creates a fresh private bus/home and foreground GNOME daemon;
its synthetic unlock password exists only in memory/pipe input. It proves native
create/read/replace/delete, fresh-process reload, separate synthetic foreign
service identities, a genuinely locked collection and missing-service refusal
without a plaintext vault. A filtered bus observer records only OpenSession
negotiation, never credential reads/writes, and requires encrypted DH with no
plain negotiation. The owned daemon is stopped before temporary-home cleanup.
Default tests never use this native fixture; explicit native CI requires its
nonzero named passing receipts and all existing persistence cases.

GNOME was executed locally. KDE Wallet interoperability is source-documented for
UTF-8 secrets but was not executed in an independently configured private KDE
service; that is an explicit untested integration, not inferred native proof.
No user's desktop keyring, Vesper credentials, provider OAuth registration or
live provider account is accessed. macOS/Windows native results must come from
the fresh five-target matrix, not cross-compilation.

### Focused verification and outstanding gates

`libdbus-five-target-resolution.json` records actual Cargo metadata resolution on
all five native targets in default and all-feature modes. None reaches the three
retired crates or zbus; each resolves its correct native backend.
The deterministic CI graph contract also reads actual all-feature metadata and
checks retained native versions, encrypted feature selection and all producer
preparation steps. Offline fixture tests refuse a desktop bus before spawning.

Local `cargo audit` passes with 542 dependencies and no advisory warning.
All-feature deny passes advisories/bans/licenses/sources with existing duplicate
warnings retained. Focused auth tests, strict Clippy and Rust 1.88 auth tests /
Windows MSVC all-target cross-check pass. Red/green missing-service, real private
GNOME restart/namespace/locked and encrypted-session receipts are retained.
Formatting passed. Full workspace/MSRV/build/acceptance and the fresh exact-head
matrix remain pending at preparation of this candidate evidence.

### Complete local correction verification

The serialized seven-command receipt `libdbus-full-local-suite.json` records exit
zero for full `cargo xtask verify`; Rust 1.88 workspace/all-target/all-feature
checking; Rust 1.88 Windows MSVC auth/security cross-check; default and all-feature
workspace builds; current audit; and all-feature deny. The frozen runtime-source
receipt matched all 1,512 selected files throughout; existing provider/backend
versions and the lockfile remain pinned. No unsupported MSRV override is used.

Full verification passed 3,653 test executions across 277 result blocks and all
165 exact acceptance cases in 288,727 ms. Forty-seven ignored executions remain
explicit limitations: the earlier 43 plus four new opt-in native fixture cases.
Those new behaviors are separately invoked by exact name on the private real
service, including spawned reload/locked children; ignored default execution is
not described as native proof. The default and Rust 1.88 private GNOME runs both
pass encrypted negotiation, persistence/restart, namespace and locked refusal.
The missing-service negative also passes. Current audit has no advisory warnings;
deny advisories/bans/licenses/sources all pass, preserving duplicate warnings.

Twenty-three offline workflow/fixture tests and eleven release-gate contracts
pass. Naming initially rejected new branded test-only controls; those fixture
keys/marker were made neutral consistently, with no production environment
mapping, policy or frozen-baseline change. The guard then passed all 36 frozen
hits. Current source and documentation changes still need the fresh full remote
thirteen-check matrix and four exact-merge push workflows; earlier native/check
successes do not substitute for this correction.

Both `libdbus-first-launch-final.log` and `libdbus-settings-auth-final.log`
passed against the current all-feature TUI: signed-out landing/all four providers,
authentication navigation, masking and cancellation remain verified locally.
These terminal fixtures do not save credentials or claim live provider approval.

## Hosted correction and ACP diagnostic follow-up

Corrective head `5b2eb925ed5414b777a53b06da5fcc29cd52c14c`, tree
`e5040da722cd26e1c6d8b48ecbb6262c79108e00`, includes the maintained native
backend and all local evidence above. Its hosted supply-chain job 117586362147
in run 39097325497 passed: audit scanned 542 dependencies; deny passed
advisories/bans/licenses/sources. Both contained browsers, namespace Hive and
Windows installer also passed on that exact head. These results do not yet
establish the full thirteen-check matrix or authorization to merge.

Windows foundation job 117586362344 in run 39097325500 failed in
`provider_selection::synthetic_provider_serves_an_acp_prompt_lifecycle_without_network_io`:
no initialize response arrived within the existing five-second bound and its
transcript was empty. The test/shared isolation/bootstrap source matched the
earlier candidate byte-for-byte; that earlier individual case passed in 0.35 s,
although its complete Windows job was later cancelled for total capacity.
The first failed log does not establish why the child had not responded.

A one-job diagnostic rerun request was rejected because the workflow was still
running. After retaining the first failure, the already-failed non-publishing
foundation attempt was cancelled and one complete five-target debug attempt
started on the unchanged SHA. Cancelled jobs remain non-passing evidence.
In attempt two, `provider_selection` passed, but Windows job 117589405830 failed
later in `xai_native::grok_session_run_command_executes_once` after the unchanged
30-second channel deadline. Four other xAI cases passed. This is a different
failure, not a reproduced five-second initialization timeout, and neither is
dismissed as harmless flakiness or an external outage.

The receive helper's bare unwrap discarded the pending RPC and protocol state.
The test-only diagnostic correction retains numeric expected-response id, child
exit status and at most twelve allowlisted event shapes/statuses/method classes.
It never dumps prompt/tool contents, credentials, arbitrary errors or raw JSON.
The xAI loopback fixture additionally records six atomic server phases and marker
presence/length before resuming the original panic. All exactly-once, cache-key,
opaque-reasoning and marker assertions, five/30-second bounds and server socket
timeouts remain. No retry is added. This improves diagnosis; it is not a claimed
causal fix for either Windows event.

`acp-diagnostic-full-acp.log` passes the complete all-feature ACP suite: 138 test
executions across 21 result blocks with one inherited ignored case. The new
diagnostic privacy/bounds regression passes alongside the original xAI cases.
Rust 1.88 all-target/all-feature ACP checking passes. Production host source,
all foundational crate source, manifests, lockfile and skills match the fully
verified correction head byte-for-byte; only test diagnostics and owned evidence
change. `acp-diagnostic-local-summary.json` records that comparison and log hash.
The previous complete local runtime/security receipts remain scope-appropriate
evidence; a fresh exact-head hosted full matrix remains mandatory for the changed
test candidate. Windows causal diagnosis and native completion remain open.

The completed debug attempt records ten successful required checks; Windows and
Apple Silicon foundation plus the aggregate fail. Linux x86_64, Linux ARM64 and
macOS Intel foundation passed. Linux native-credential acceptance ran against the
private encrypted Secret Service fixture successfully. Apple Silicon job
117589406011 failed at the existing 30-second fresh-turn settlement assertion in
`voice_speech_pipeline::synthesis_overlaps_playback_and_stop_discards_lookahead`.
Its log does not identify the outstanding worker stage. The test now retains
stage/busy state, numeric progress, synthetic terminal outcomes and fixture-player
counts on failure; it changes no deadline, synchronization or assertion. This is
diagnostic evidence collection, not a voice defect correction or a flaky-test claim.

The 2026-10-18 11:22 UTC preservation recheck retains unchanged Vesper HEAD,
status, worktree inventory and all 158 dirty paths. Of 14,598 protected entries,
two live SQLite sidecars (`cognition.db-wal` and `cognition.db-shm`) have different
inode/mtime metadata from the original capture; their sizes remain 0 and 32,768
bytes. Attribution is UNKNOWN. This task has not written to or opened that
runtime database. `preservation-sidecar-discrepancy.json` retains the original
and observed metadata; no restoration, deletion or baseline replacement occurs.
The source checkout invariants pass; whole-runtime snapshot invariance cannot be
claimed after this discrepancy.

The final focused ACP suite passes 138 executions in 21 result blocks (one
inherited ignored case), `acp-diagnostic-full-acp-corrected.log`. Corrected helper
Rust 1.88 checking passes, and both voice pipeline cases pass with unchanged
bounds in `voice-diagnostic-pipeline.log`. Naming verification retains all 36
frozen hits. Owning ACP and TUI-test DOX describe the diagnostic contracts;
parent ownership/index boundaries remain unchanged and require no new entries.

Both-host all-target/all-feature Clippy passes with `-D warnings`, as do format,
changed-document links/JSON/whitespace checks and live governance readback:
thirteen required contexts, active no-bypass branch/tag rules and all three
publishing/recovery workflows disabled. The diagnostic candidate introduces no
production behavior, dependency, timeout or retry-policy change.

## Protected-merge review closeout

Head `4b2a6bdbda1267bf2193721068e0b6bd69272cee` passed all thirteen required
contexts, including all five native targets. Windows log retains successful
synthetic initialization, xAI exactly-once command execution and the real 2 GiB
Job Object probe. Both earlier native deadline failures remain retained with
unconfirmed causes; none of their assertions, deadlines or test bodies was
weakened. All four post-merge workflows are still unexecuted because no merge
has occurred.

Final protected-merge readiness then correctly refused four unresolved review
threads from the initial Stage 3 head. Source inspection confirms the credential
operation-lock ordering finding was already corrected and tested on all five
native targets. The other three findings remain real acceptance gaps despite
the green matrix: LM Studio settings could create nonempty unowned state, the
LM Studio root override was omitted at bootstrap, and checkpoint/MCP composition
ignored ownership-preparation failures. No thread is dismissed based only on CI.

`review-bootstrap-red.log`, `review-lmstudio-red.log` and
`review-bundles-red.log` reproduce those failures through actual bootstrap,
settings save/read and production store-bundle entrypoints before correction.
The corrective source shares the absolute LM Studio resolver and private-file
reader in `jarvis-config` across TUI, ACP provider and ACP cognition. Atomic
private writes initialize the selected settings directory and `.jarvis` parent;
foreign/legacy/redirected roots and files refuse without adoption. Explicit root
and home precedence remains deterministic. Checkpoint/MCP composition conditions
all store opens on successful ownership preparation, leaving stores unavailable
and publisher trust empty on refusal. Positive owned-store functionality remains.

Synthetic filesystem regressions cover first save/reload, unknown and redirected
state, absolute defaults and foreign/legacy/relative bootstrap overrides. No real
user home, credentials or runtime database is used. The three focused red/green
receipts are retained privately; broader candidate verification remains required.
There is no dependency or timeout change. The owning config/TUI/ACP DOX contracts
are updated; parent ownership and child indexes are unchanged.

The two Vesper SQLite sidecars identified earlier still match their original
content hashes; only inode/mtime metadata differs, with attribution UNKNOWN.
The unchanged protected content and metadata discrepancy remain separate claims.

The first full review-candidate run passed Clippy but failed the unchanged identity
source-evidence guards after LM Studio environment reading moved into config.
`verify-review-stale-registry-failure.log` retains that failure. Both the active
and legacy mapping source citations now follow the actual config reader, with
no identity, classification, environment count or guard assertion change.

The next full workspace suite passed, but the existing naming embargo rejected
a raw application-prefix literal in the new bootstrap test fixture. The fixture
now clears its child environment and restores only OS execution prerequisites
and explicit synthetic roots. No guard, baseline or production contract changed;
`verify-review-fixture-prefix-failure.log` retains the failed gate evidence.

The corrected full local sequence passes: `cargo xtask verify` records 3,675
passing executions across 277 result blocks, 47 ignored executions retained,
and 165 exact acceptance cases in 258,384 ms. Its log SHA-256 is
`9bd744828d09ebadf87491d004c14cca4eb0975be093a4d50cd951dbbd862343`.
The serialized sequence also passes Rust 1.88 workspace/all-target/all-feature
checking, Windows MSVC auth/security/config checking, default and all-feature
workspace builds, fresh audit and all-feature deny. No manifest/lockfile or
third-party version changes accompany this review correction. The 1,028 frozen
source/tool/fixture hashes still match after verification. Runtime changes are
limited to the three proven isolation defects and host-parity settings reading;
no release, updater, import or protocol behavior is enabled.

Current all-feature binaries also pass the real signed-out four-provider landing/
authentication-Back/clean-Quit matrix (`review-first-launch-final.log`) and native
Settings navigation/discard/keep-editing/grouped-save/restart persistence
(`review-settings-final.log`). These terminal checks use isolated homes/workspaces
and submit no provider prompt. Fresh exact-head hosted verification is still
required; the previously green head is not substituted for the new correction.


## Native runner interruption and journal observer correction

The exact `ddac77d2ae2b950f3d2c2bc3a7ddd633709b664c` first hosted matrix
settled with eleven successes and two failures: Windows native and its dependent
aggregate. GitHub's Windows annotation reports lost runner/server communication
during foundational tests. The job log returns BlobNotFound, and the downloaded
run archive contains only the four successful native targets. The underlying
Windows cause remains UNKNOWN; this is not evidence of an external outage or
a passing test suite. The annotation, job state and available archive remain in
`windows-annotations-resume.json`, `windows-job-resume.json` and
`ddac-foundation-logs.zip` in the private evidence directory.

One bounded complete native diagnostic repeat, run 39133950058 attempt 2 with
debug logging, preserves the same source, all five targets, assertions and
deadlines. The aggregate is separately rerun to validate the complete current
attempt. No publication workflow is invoked. The four first-attempt native logs
confirm each new LM Studio/bootstrap/store-bundle ownership case executed and
passed; both Linux logs also confirm real private Secret Service persistence,
restart/namespace evidence and encrypted DH negotiation. They do not substitute
for the current complete matrix.

The repeat exposes a separate Apple Silicon failure in
`terminal_show_more_restore_copy_and_queue_are_distinct`: the journal fixture
samples Show more after one 600 ms pump following provider dispatch. Its captured
frame contains only the beginning of the collapsed prompt and the stale composer.
The identical fixed-delay observation exists in the unchanged Stage 2 baseline;
source comparison establishes inherited coverage, not a claimed baseline native
reproduction. A provider-request event is not a terminal-frame readiness event.

`test_terminal_writer.py::test_collapsed_prompt_observes_frame_not_provider_dispatch`
reproduces this observer defect deterministically through a real gated PTY child:
the title is visible before an independently released Show more frame. The old
single-pump observer fails (`journal-frame-observer-red.log`); the corrected
observer passes (`journal-frame-observer-green.log`) without sleeping longer.
The fixture now uses the existing bounded `Host.wait` reader for the actual
control on first submission and restoration, preserving all collapsed-body,
indentation/Unicode, stale-composer, expansion, copy, journal, restart, wire and
FIFO assertions. Missing frames still fail under the unchanged 20-second reader
budget, the Rust outer lifetime stays 270 seconds, and the held provider remains
bounded at 45 seconds. Dedicated negative tests reject missing, expanded and
stale frames. No production runtime, dependency, workflow or timeout is changed.

Five complete real-TUI journal repetitions pass against the current all-feature
binary with synthetic private homes, state and loopback provider:
`journal-native-frame-repeat-1.log` through `journal-native-frame-repeat-5.log`,
summarized in `journal-native-frame-repeats.json`. The terminal writer suite
passes nine cases. The full local verification receipt for this correction and
a new exact-head thirteen-check matrix remain required before protected merge.

The DOX pass updates the owning terminal-test synchronization contract and corrects
three stale parent descriptions of the application root, same-product voice
environment reuse and native voice-pack data location. Parent ownership and child
indexes are unchanged; no new file or subsystem boundary is introduced. The
existing two Vesper SQLite-sidecar metadata discrepancies remain explicit, with
unchanged original content hashes and UNKNOWN attribution. No restoration or
baseline replacement occurs.

The corrected complete local `cargo xtask verify` passes 3,675 executions in
277 result blocks, retaining 47 ignored executions. All 165 exact acceptance
cases pass in 202,211 ms. Log SHA-256:
`e4cc9d6b97d0d024a1bdb24dc9116fd3e9e3b8a2a9f826d9d5002b17df4841b4`.
All 956 frozen executable source/tool/fixture inputs still match. The serialized
followup gates also pass Rust 1.88 workspace/all-target/all-feature checking,
Windows MSVC auth/security/config checking, both workspace build modes, fresh
`cargo audit` (1,338 advisory records; 542 dependencies) and
`cargo deny --all-features check` (advisories/bans/licenses/sources all OK).
`journal-full-verify-summary.json` and `journal-followup-gates.json` retain exact
commands and outcomes. No Rust source, Cargo manifest or lockfile changes are
in this observer correction. The source inventory now accounts for 250 changed
paths, including the previously unchanged terminal-writer regression file.

Both Linux targets pass the diagnostic repeat and retain complete native logs.
The superseded failed native matrix cannot authorize integration. A new candidate
requires all thirteen successful checks, resolved reviews, a normal protected
merge and all four exact-merge workflows; no completion is claimed in this
pre-CI report. Any later superseded-job cancellation remains cancellation rather
than passing evidence. No release, installation, real RRC operation or Stage 4
work is authorized or performed.
