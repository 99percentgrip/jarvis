# Stage 3 runtime ownership architecture

## Authority and execution order

Baseline: `508ab4d962a9aa90eb4e6e37b7e340a6569af90b`. All four exact-baseline
post-merge workflows and all thirteen Stage 2 PR checks passed before implementation.
Only runtime isolation is authorized. Import, distribution, registration and release
activation remain deferred. Existing Stage 1 mappings and Stage 2 Rust names apply.

Implementation proceeds in dependency order:

1. Resolve platform config/data/state/cache/log roots; establish private-path
   validation and separate project-private `.jarvis` state from user project files.
2. Route provider credential paths and native-service identities to independent
   namespaces. Review each account authorization contract; block unsupported
   application-bound flows before network contact, without changing provider APIs.
3. Isolate sessions, journals, memory, checkpoints, skills and operational records.
   Retain serialization compatibility; disable legacy discovery and replay.
4. Isolate Browser/Lens, Voice and Bridge private settings, temporary files and
   records while preserving external protocols, permissions and device behavior.
5. Isolate process/IPC ownership and establish shared dormant updater/release
   boundaries. Keep offline controller/build tests without admitting real releases.
6. Run filesystem-backed negative/concurrency/crash tests, full local verification,
   exact-head protected CI, normal merge and four exact-merge push workflows.

Each unit updates its nearest owning instructions and regression evidence. No
partial unit or passing unrelated test establishes Stage 3 completion.

## Resource ownership rules

- Application-private: new platform roots, `.jarvis`, `.jarvis-scope-id`, own credentials,
  process records and application-managed artifacts. Explicit overrides must not
  authorize adoption of Vesper-private records or follow unsafe redirections.
- Vesper-private: previous platform roots, `.agent-vesper`, `.vesper-scope-id`,
  credential services and release/recovery ledgers. No automatic read/import/write.
- User-owned/shared: explicitly authorized workspaces, documents and external tools.
  Private-state containment must not become a blanket project-file restriction.
- External/protocol: provider credentials explicitly supplied by the user, provider
  API identifiers, browser wire fields and serialized compatibility markers.
  Preserve interfaces; never infer independent OAuth authorization from a rename.

## Platform roots and first-run ownership

| Platform | Configuration | Data/state | Cache/logs |
| --- | --- | --- | --- |
| Linux x86_64/ARM64 | XDG config base + `jarvis` | XDG data/state bases + `jarvis` | XDG cache base + `jarvis`; state root + logs |
| macOS Intel/Apple Silicon | Application Support + `Jarvis` + Config | Same application root + Data/State | Library Caches/Logs + `Jarvis` |
| Windows x86_64 | APPDATA + `Jarvis` | LOCALAPPDATA + `Jarvis` + Data/State | Same local application root + Cache/Logs |

Linux relative XDG inputs are ignored in favor of home-based defaults. Explicit
private overrides receive the same ownership checks as defaults. The pure
`JarvisPaths` resolver never creates anything; normal initialization validates all
private overrides before creating native roots. ACP protocol-only startup validates
without creating state; full harness composition initializes application roots.

The security foundation owns `.jarvis-owner`, with exact application identity
`99percentgrip.jarvis` and runtime-v1 marker bytes. First-run ownership is permitted
only for missing/empty roots. Existing nonempty unowned stores refuse; no marker is
retroactively installed over existing records. Marker publication is synced,
atomic and no-clobber. Unix sensitive directories/files require owner-only modes;
new roots receive 0700 and files 0600. Windows uses pinned native object handles,
protected DACLs and current-user/SYSTEM/administrators-only access. Existing parent
permissions are never rewritten. Symlinks, Windows reparse points, traversal and
known private legacy locations refuse at sensitive boundaries. Trusted macOS
system directory aliases are narrowly recognized; arbitrary redirections refuse.

A project named agent-vesper is a legitimate user resource. Application-private
path rules apply to the private descendants and explicit store overrides, rather
than prohibiting ordinary authorized project tools or third-party application IPC.

## Identity consumers and dependencies

The [reviewed inventory](../foundation/runtime-identity-stage3-inventory.json)
records 22 resource groups, source symbols/line references, readers, writers,
verification entry points and 96 reviewed environment mappings. It includes
non-Rust seed helpers as well as the workspace; the initial literal scan is not
presented as a complete consumer inventory.

Resolution flows from shared config paths through security ownership validation
to auth/session/memory/checkpoint stores and both composition hosts. The pure
`jarvis-identity` specification remains a verification-only dependency. Active
state uses `.jarvis` and `.jarvis-scope-id`; native services use
`99percentgrip.jarvis`. Standard provider keys and OS directory inputs remain
external contracts. Reviewed private environment variables use explicit registry
mappings with no default legacy fallback. Test/distribution-only inputs retain
separate roles in the inventory.

Session/message IDs, serialized format markers, browser DOM/wire identifiers,
OAuth client IDs and third-party APIs are preserved. Generic legacy decoders and
controller code remain available to explicit offline fixtures; ordinary runtime
legacy discovery is off, even with all features enabled. Only the integration
executable explicitly activates synthetic legacy/controller seams.

## Authentication and deferred capabilities

API keys explicitly supplied by users are stored independently. Credential vaults
require an application tag; untagged or differently owned records refuse before
native lookup, refresh or deletion. Windows empty-store reads create no disk lease.

Independent subscription authorization has not been established for OpenAI ChatGPT
or xAI Grok. Ordinary login, refresh and retained subscription dispatch are blocked
before network contact; loopback protocol fixtures do not establish vendor approval.
Bundled Google authorization is similarly inactive before token access. The legacy
TouchDesigner setup helper is inactive pending an independent configuration adapter;
existing explicitly configured application-control protocols remain available.

## Process, subsystem and dormant-release boundaries

Voice capture/pack/tool state and stderr logs use private roots. Temporary sidecar
work uses uniquely owned RAII directories. Orphan cleanup requires application
identity and process generation; Linux pins the process with a pidfd before signals.
Unsupported generation identification fails closed. Daemon locks carry application
ownership; mobile endpoints bind ephemeral loopback ports and separate secrets.
Browser container names and managed-machine identity are independent; existing
user-installed runtimes, device access and external Bridge IPC retain their authority.

Ordinary updater and release/recovery routes refuse before adopting a ledger or
contacting the old release channel. No independent publication infrastructure is
implemented here. Offline acceptance preserves controlled controller behavior through
explicit non-shipping integration hosts, unavailable merely by enabling Cargo
features. The TUI `offline_release_host` example requires explicit fixture authority,
a workspace-local synthetic ledger and the fixed fixture repository remote; the
ACP integration driver requires its existing explicit dispatch gate. Native CI
first verifies ordinary shipping-feature hosts, then explicitly builds these test
hosts for unchanged cross-host ledger assertions. The same fixture also proves
ordinary TUI/ACP release status/control cannot touch the supplied ledger.

Project settings refuse nonempty unowned state before parsing activation.
Private UI preferences and trust metadata guard both leaves and ownership before
read, write or grouped rollback. Editor drafts use owned temporary stores;
screenshots retain private cache files for existing sixel-path consumers. Generic
copied-project/probe/browser/setup workspaces are exclusive RAII allocations with
native privacy secured before data writes; no marker is inserted into copied
project trees. VAD executes the same embedded script directly and its audio
workspace remains owned until blocking worker settlement. No foreign temporary
directory is reclaimed by prefix.

## Verification and rollback

Filesystem-backed tests additionally refuse empty legacy cache/log roots before first-run writes.
Filesystem-backed tests cover first-run idempotence/concurrency, synthetic legacy
canaries, nonempty ownership refusal, private credentials, atomic replacements,
redirects, normal ACP discovery and unrestricted legitimate user projects. Native
Windows tests exercise broad-ACL refusal and junction redirection. Existing voice,
provider, session, message queue, Browser/Lens, Bridge and completion gates remain.
The execution report distinguishes local, mock, native-CI and unavailable live proof.

Rollback uses a protected revert PR and retained independent state, never moving
current private records into old roots. Old product installations/state are unchanged.
No automatic reverse import or replay is allowed. Stage 4 requires separate owner
authorization and a designed optional import mechanism; Stage 7 owns independent
release infrastructure and the controlled future tag-freeze transition.
