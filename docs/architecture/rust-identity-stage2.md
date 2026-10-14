# Rust workspace identity migration — Stage 2

## Ownership and scope

This is the authorized Rust identity implementation for J.A.R.V.I.S. in
`99percentgrip/jarvis`, based on `653a5213cd3fbdafa75b7261991b2d74590148e8`.
The [Stage 1 architecture](identity-architecture.md) remains historical design
and provenance. The Stage 2 directive overrides its proposed TUI and maintenance
executable names: `jarvis-tui` and `xtask` are authoritative.

The pure registry in `crates/jarvis-identity/src/packages.rs` owns current package,
import, manifest and executable identities, plus explicit original package and
manifest identities. Actual Cargo metadata is checked against this registry;
adding an unregistered package or executable fails verification.

## Complete package and directory map

All 32 workspace packages are accounted for. Thirty directories move; the identity
crate and maintenance package retain their locations. Rust imports use each
registered library target, rather than inferring the ACP protocol name from its
application name. Versions, authors, licenses and third-party dependencies remain
inherited; this migration is not a version bump.

| Source package | Current package | Source → current directory |
|---|---|---|
| `jarvis-identity` | `jarvis-identity` | `crates/jarvis-identity` → `crates/jarvis-identity` |
| `vesper-config` | `jarvis-config` | `crates/vesper-config` → `crates/jarvis-config` |
| `vesper-domain` | `jarvis-domain` | `crates/vesper-domain` → `crates/jarvis-domain` |
| `vesper-security` | `jarvis-security` | `crates/vesper-security` → `crates/jarvis-security` |
| `agent-vesper-acp` | `jarvis-acp` | `apps/agent-vesper-acp` → `apps/jarvis-acp` |
| `vesper-acp` | `jarvis-acp-protocol` | `crates/vesper-acp` → `crates/jarvis-acp-protocol` |
| `vesper-runtime` | `jarvis-runtime` | `crates/vesper-runtime` → `crates/jarvis-runtime` |
| `vesper-provider` | `jarvis-provider` | `crates/vesper-provider` → `crates/jarvis-provider` |
| `vesper-sessions` | `jarvis-sessions` | `crates/vesper-sessions` → `crates/jarvis-sessions` |
| `vesper-testkit` | `jarvis-testkit` | `crates/vesper-testkit` → `crates/jarvis-testkit` |
| `vesper-bridge` | `jarvis-bridge` | `crates/vesper-bridge` → `crates/jarvis-bridge` |
| `vesper-policy` | `jarvis-policy` | `crates/vesper-policy` → `crates/jarvis-policy` |
| `vesper-agent` | `jarvis-agent` | `crates/vesper-agent` → `crates/jarvis-agent` |
| `vesper-auth` | `jarvis-auth` | `crates/vesper-auth` → `crates/jarvis-auth` |
| `vesper-cognition` | `jarvis-cognition` | `crates/vesper-cognition` → `crates/jarvis-cognition` |
| `vesper-harness` | `jarvis-harness` | `crates/vesper-harness` → `crates/jarvis-harness` |
| `vesper-checkpoints` | `jarvis-checkpoints` | `crates/vesper-checkpoints` → `crates/jarvis-checkpoints` |
| `vesper-mcp` | `jarvis-mcp` | `crates/vesper-mcp` → `crates/jarvis-mcp` |
| `vesper-memory` | `jarvis-memory` | `crates/vesper-memory` → `crates/jarvis-memory` |
| `vesper-sandbox` | `jarvis-sandbox` | `crates/vesper-sandbox` → `crates/jarvis-sandbox` |
| `vesper-swarm` | `jarvis-swarm` | `crates/vesper-swarm` → `crates/jarvis-swarm` |
| `vesper-web` | `jarvis-web` | `crates/vesper-web` → `crates/jarvis-web` |
| `vesper-web-fetch` | `jarvis-web-fetch` | `crates/vesper-web-fetch` → `crates/jarvis-web-fetch` |
| `vesper-provider-synthetic` | `jarvis-provider-synthetic` | `crates/vesper-provider-synthetic` → `crates/jarvis-provider-synthetic` |
| `vesper-provider-glm` | `jarvis-provider-glm` | `crates/vesper-provider-glm` → `crates/jarvis-provider-glm` |
| `vesper-provider-openai` | `jarvis-provider-openai` | `crates/vesper-provider-openai` → `crates/jarvis-provider-openai` |
| `vesper-provider-xai` | `jarvis-provider-xai` | `crates/vesper-provider-xai` → `crates/jarvis-provider-xai` |
| `agent-vesper-tui` | `jarvis-tui` | `apps/agent-vesper-tui` → `apps/jarvis-tui` |
| `vesper-observability` | `jarvis-observability` | `crates/vesper-observability` → `crates/jarvis-observability` |
| `vesper-voice` | `jarvis-voice` | `crates/vesper-voice` → `crates/jarvis-voice` |
| `vesper-voice-kokoro` | `jarvis-voice-kokoro` | `crates/vesper-voice-kokoro` → `crates/jarvis-voice-kokoro` |
| `xtask` | `xtask` | `xtask` → `xtask` |

## Dependency direction and executables

Existing foundation → runtime → harness → host dependency direction is preserved.
`jarvis-acp-protocol` is the protocol library; `jarvis-acp` is the composition host.
Provider-neutral ports, concrete adapters and test-only dependencies retain their
existing boundaries. The identity crate remains side-effect-free and is consumed
by repository verification, not wired into production state initialization.

| Source executable | Compiled target | Compatibility behavior |
|---|---|---|
| `agent-vesper-tui` | `jarvis-tui` | Help/version identify the compiled executable; CLI behavior is retained. |
| `agent-vesper-acp` | `jarvis-acp` | ACP JSON-RPC behavior and existing registration remain unchanged. |
| `agent-vesper-acp-test-driver` | `jarvis-acp-test-driver` | Private test discovery follows actual Cargo targets. |
| `vesper-web-fetch` | `jarvis-web-fetch` | Browser image copies the renamed build output to its existing `/usr/local/bin/vesper-web-fetch` runtime path. |
| `sandbox_init` | `jarvis-sandbox-init` | Explicit `VESPER_SANDBOX_INIT` override remains first; sibling discovery prefers the new name and accepts the old name with native executable suffix. |
| `xtask` | `xtask` | Maintenance alias and package identity remain unchanged. |

Windows offline installer fixtures stage new compiled executables under their
legacy payload filenames. PE auditing supports an explicit build-target inventory
while keeping its legacy release inventory as the default. Production installers,
installation roots, update endpoints and ACP registry publication are unchanged.
The updater's existing bundle validation accepts either compiled ACP sibling name;
this does not authorize a new installation path or publication.

## Developer commands

From a clean development checkout, use `cargo build --locked --workspace` or
`cargo build --locked --workspace --all-features`. Run `cargo xtask verify`,
`cargo test -p jarvis-identity`, `cargo xtask architecture`, `cargo audit` and
`cargo deny check`; MSRV verification uses `cargo +1.88.0 test --workspace --all-features`.
For isolated development metadata checks, compiled targets are
`target/debug/jarvis-tui --version` and `target/debug/jarvis-acp --version`.
These are build outputs, not approved installation commands. Existing public
installation documentation stays under the interim migration notice.

## Guards and release-tooling boundaries

Naming enforcement preserves every upstream pattern and the immutable frozen
baseline. Only registry-approved technical tokens and directory moves receive
scoped handling. A moved historical hit maps to its original path and reviewed
technical-token bytes; unknown names, new prose and duplicate counts remain gated.

Offline version-only lockfile comparison uses package names read from actual
workspace member manifests rather than a name prefix. Its negative test rejects an
unowned package sharing the product prefix. Controller state, repository routing,
release ownership, epochs, publishing and disabled workflow state are unchanged.
Inherited publishing workflows deliberately retain deferred build references and
must be reviewed before any separately authorized release infrastructure migration.

## Remaining legacy identifiers

| Class | Examples and source owners | Reason and next boundary |
|---|---|---|
| Environment and credentials | `AGENT_VESPER_*`, `VESPER_*`; `jarvis-config`, `jarvis-auth`, provider adapters | Existing explicit environment controls and confidential credential-service contracts remain active; no sharing or migration. |
| Persistence and project scope | `.agent-vesper`, legacy session/checkpoint markers and scope stamps; `jarvis-sessions`, `jarvis-checkpoints`, `jarvis-memory` | Existing readers, identities and MessageJournal restoration remain byte-compatible; future optional import is separately authorized. |
| Protocol and integrations | ACP registration/client metadata, Lens/browser serialized fields, provider schema labels, bridge bindings | External consumers and existing installations still use these contracts; production fields are not mechanical Rust identifiers. |
| Distribution and runtime aliases | Legacy installer payload names, browser runtime path, supervisor fallback | Explicit adapters preserve discovery without activating new installation/update ownership. |
| Deferred implementation identifiers | `VRO`, `VesperLens`, `vesper_lens`, `VesperPaths`, `vesper_format` | They are not Cargo package/import identities; broader subsystem/type branding and legacy reader names remain outside this stage. |
| Release identity | Source repository URLs, Vesper archive/registry names and RRC ledger namespace in inherited publishing machinery | No release operation, tag or old epoch is adopted; independent publishing remains deferred. |
| Historical and third-party | Original release reports, Stage 0/1 evidence, frozen fixtures, Apache notices, upstream package names | Preserve attribution, immutable evidence and third-party identity. |

The excluded research workspace and its separate lockfile are outside the shipping
workspace. Its outstanding advisory remains open and cannot be admitted to a
shipping build without remediation. No current workspace member depends on it.

## Verification and future approval

Registry tests inspect actual Cargo metadata and reject unknown package additions,
collisions and missing executable ownership. Platform state-root specifications,
credential/serialization classifications and ACP/RRC separation retain Stage 1
coverage. Focused tests cover native-suffix supervisor discovery, strict PE payload
inventories, explicit lockfile ownership and frozen-hit directory normalization.

The [Stage 2 PRD](../rust-identity-stage2-prd.md) and
[execution report](../foundation/2026-10-15-rust-identity-stage2.md) own actual test,
CI and merge evidence. Linux local tests cannot substitute for native ARM64, macOS,
Windows or contained-browser jobs. Stage 3 requires explicit owner approval;
state migration, registration, installation, publishing and release remain excluded.
