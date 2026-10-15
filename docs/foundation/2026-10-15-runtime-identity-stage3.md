# Stage 3 runtime identity isolation — execution report

## Objective and current status

Implement only the owner's revised Stage 3 directive in the isolated branch
`feat/stage3-runtime-identity-isolation`.
Status: LOCAL GATES PASSED; protected PR/CI/merge and post-merge workflows pending.
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
22 resource groups with source symbols/lines, consumers, writers and regression
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
