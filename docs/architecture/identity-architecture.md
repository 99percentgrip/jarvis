# Stage 1 first-party identity architecture

## Status and authority

Stage 1 implementation specification; runtime migration is NOT implemented.
Baseline is 57d8280a8d4a470af5878088b9d8d0e1645a12da, independently derived
from Agent Vesper v0.24.14. The owner's full technical identity migration PRD
and Stage 1 execution mission supply the approved architecture. No separate
previous design file was found in the inherited repository; this document makes
those owner decisions concrete without claiming a missing prior artifact.

## Module and dependency direction

`jarvis-identity` is a non-published, zero-production-dependency library. Its
version 0.1.0 is an internal planned contract version, not a release tag or
publication. The workspace and existing packages remain 0.24.14. Typed Product,
Repository, PackageIdentity, Executable, Mapping, Classification, Platform,
PlannedPaths, Distribution and ImportPolicy definitions have no side effects.
Only xtask architecture verification links this library. Tests alone depend on
the real config resolver and JSON reader. No application or production foundation
consumes it. Existing runtime bytes and manifest identities remain unchanged.

The registry explicitly distinguishes applications, foundations, test support,
maintenance and the specification package, as well as shipping executables.
Future release preparation must use reviewed ownership
entries, not a prefix match. Stage 1 does NOT change the RRC implementation.
Architecture compares actual Cargo metadata package names, manifest ownership,
lib imports and every binary target to that registry. New/unregistered packages,
binaries, imports, production dependencies or runtime consumers fail verification.
Five excluded research packages remain historical/non-shipping and are separately
accounted for. ACP protocol and executable packages need different planned names;
the collision regression caught and resolved that ambiguity.

## Central decisions

Product display is J.A.R.V.I.S.; slug and proposed ACP registry ID are `jarvis`.
Destination is `99percentgrip/jarvis`; credential service is `99percentgrip.jarvis`.
Proposed session format is `jarvis-session`, schema 1; proposed checkpoint namespace
is `jarvis-checkpoint`. This does not invent an existing checkpoint wire namespace:
its current contract is RequiresEvidence. Release ownership is independent, with
`jarvis.release-recovery.v1` and `jarvis-release-controller` planned identities.
Repository provenance, historical tags, license text and original authors persist.
Package/registry name availability outside this repository is UNKNOWN; no names
are published or reserved through external package/ACP services by this stage.

Naming-guard changes preserve every frozen hit and all thirteen upstream patterns.
Only reviewed quoted identifiers/imports within the new identity crate and named
owner documents can use the first-party namespace. Bare upstream prose, unknown
names, historical files and other upstream brands are not exempted.

## Platform contracts

| Platform | Config/data/state | Cache/logs | Installation and updater ownership |
|---|---|---|---|
| Linux x86_64 / ARM64 | Injected XDG bases plus product slug | Corresponding XDG bases plus slug/logs | Private data-root Install tree; planned own launchers on PATH |
| macOS Intel / Apple Silicon | `Library/Application Support/Jarvis` with Config/Data/State | `Library/Caches/Jarvis`; `Library/Logs/Jarvis` | Private support Install tree; POSIX launchers owned by this project |
| Windows x86_64 | Roaming base plus `Jarvis`; local base plus Data/State | Local base plus Cache/Logs | Local Programs base plus `Jarvis`; own exe/cmd launchers and shortcuts |

Project state directory is `.jarvis`; scope stamp is `.jarvis-scope-id`.
The pure resolver accepts injected bases and creates nothing. Tests run every
platform specification against the real Vesper defaults, including Windows
case-insensitive equality. Future overrides/symlinks need an ownership/collision
check before any filesystem write; this stage does not claim they are safe.
Archives follow the five supported targets in the distribution registry and use
per-asset .sha256 companions. Both hosts, fetch helper and supervisor remain part
of the planned distribution. Installer entry filenames remain install.sh and
install.ps1 but must download solely from the destination release identity.
Future PATH changes, shell-profile markers, Windows shortcuts, uninstall and
update receipts must be independently owned; old installations remain available.
No .app bundle, MSI product code or application binding is invented.

## Import and external compatibility policy

Stage 4 owns import. It must require explicit consent, a preview/dry run and
separate credential authorization. Credentials go through native stores or
protected storage; no plaintext export/logs or automatic service fallback.
Vesper is read-only. Copy into a destination-owned staging transaction with
idempotent source/message/session identities, duplicate detection, validation
and resumable interruption receipts. Commit only verified artifacts. Rollback
removes/reverts only destination-owned changes; never delete or edit source data.

Session and message IDs remain readable. Pending/active/interrupted work is
imported as non-executing history/paused state; no dispatch, tool replay or
automatic queue resumption. Checkpoint/extension schemas need verified readers.
Vesper RRC epochs, ledgers and release operations are explicitly excluded.
Fresh installations use independent empty roots. Optional import never changes
Vesper's installers, repository, clients, launchers, credentials or runtime.

ACP has a distinct future registration and command. Existing Vesper client
configurations stay unchanged; adding the new client entry requires consent.
Registry publication status and external ID availability require independent
readback before submission. Browser wire fields/storage keys need versioned
negotiation or temporary legacy readers; binary aliases alone cannot solve those
contracts. Bridge application identities and provider/OAuth metadata remain
unchanged until owning adapters establish which fields are project-owned.
Vendor names, provider IDs, official SDKs and external app protocols stay accurate.

## Package and executable ownership

| Current package | Planned package | Planned import | Executable mapping | Current manifest |
|---|---|---|---|---|
| `agent-vesper-acp` | `jarvis-acp` | `jarvis_acp` | `agent-vesper-acp` → `jarvis-acp`; `agent-vesper-acp-test-driver` → `jarvis-acp-test-driver` | `apps/agent-vesper-acp/Cargo.toml` |
| `agent-vesper-tui` | `jarvis-tui` | `jarvis_tui` | `agent-vesper-tui` → `jarvis` | `apps/agent-vesper-tui/Cargo.toml` |
| `vesper-config` | `jarvis-config` | `jarvis_config` | None | `crates/vesper-config/Cargo.toml` |
| `vesper-acp` | `jarvis-acp-protocol` | `jarvis_acp_protocol` | None | `crates/vesper-acp/Cargo.toml` |
| `vesper-agent` | `jarvis-agent` | `jarvis_agent` | None | `crates/vesper-agent/Cargo.toml` |
| `vesper-auth` | `jarvis-auth` | `jarvis_auth` | None | `crates/vesper-auth/Cargo.toml` |
| `vesper-cognition` | `jarvis-cognition` | `jarvis_cognition` | None | `crates/vesper-cognition/Cargo.toml` |
| `vesper-harness` | `jarvis-harness` | `jarvis_harness` | None | `crates/vesper-harness/Cargo.toml` |
| `vesper-bridge` | `jarvis-bridge` | `jarvis_bridge` | None | `crates/vesper-bridge/Cargo.toml` |
| `vesper-checkpoints` | `jarvis-checkpoints` | `jarvis_checkpoints` | None | `crates/vesper-checkpoints/Cargo.toml` |
| `vesper-domain` | `jarvis-domain` | `jarvis_domain` | None | `crates/vesper-domain/Cargo.toml` |
| `vesper-mcp` | `jarvis-mcp` | `jarvis_mcp` | None | `crates/vesper-mcp/Cargo.toml` |
| `vesper-memory` | `jarvis-memory` | `jarvis_memory` | None | `crates/vesper-memory/Cargo.toml` |
| `vesper-observability` | `jarvis-observability` | `jarvis_observability` | None | `crates/vesper-observability/Cargo.toml` |
| `vesper-policy` | `jarvis-policy` | `jarvis_policy` | None | `crates/vesper-policy/Cargo.toml` |
| `vesper-provider` | `jarvis-provider` | `jarvis_provider` | None | `crates/vesper-provider/Cargo.toml` |
| `vesper-provider-glm` | `jarvis-provider-glm` | `jarvis_provider_glm` | None | `crates/vesper-provider-glm/Cargo.toml` |
| `vesper-provider-openai` | `jarvis-provider-openai` | `jarvis_provider_openai` | None | `crates/vesper-provider-openai/Cargo.toml` |
| `vesper-provider-xai` | `jarvis-provider-xai` | `jarvis_provider_xai` | None | `crates/vesper-provider-xai/Cargo.toml` |
| `vesper-provider-synthetic` | `jarvis-provider-synthetic` | `jarvis_provider_synthetic` | None | `crates/vesper-provider-synthetic/Cargo.toml` |
| `vesper-runtime` | `jarvis-runtime` | `jarvis_runtime` | None | `crates/vesper-runtime/Cargo.toml` |
| `vesper-sandbox` | `jarvis-sandbox` | `jarvis_sandbox` | `sandbox_init` → `jarvis-sandbox-init` | `crates/vesper-sandbox/Cargo.toml` |
| `vesper-sessions` | `jarvis-sessions` | `jarvis_sessions` | None | `crates/vesper-sessions/Cargo.toml` |
| `vesper-security` | `jarvis-security` | `jarvis_security` | None | `crates/vesper-security/Cargo.toml` |
| `vesper-swarm` | `jarvis-swarm` | `jarvis_swarm` | None | `crates/vesper-swarm/Cargo.toml` |
| `vesper-testkit` | `jarvis-testkit` | `jarvis_testkit` | None | `crates/vesper-testkit/Cargo.toml` |
| `vesper-voice` | `jarvis-voice` | `jarvis_voice` | None | `crates/vesper-voice/Cargo.toml` |
| `vesper-voice-kokoro` | `jarvis-voice-kokoro` | `jarvis_voice_kokoro` | None | `crates/vesper-voice-kokoro/Cargo.toml` |
| `vesper-web` | `jarvis-web` | `jarvis_web` | None | `crates/vesper-web/Cargo.toml` |
| `vesper-web-fetch` | `jarvis-web-fetch` | `jarvis_web_fetch` | `vesper-web-fetch` → `jarvis-web-fetch` | `crates/vesper-web-fetch/Cargo.toml` |
| `xtask` | `jarvis-xtask` | No library | `xtask` → `jarvis-xtask` | `xtask/Cargo.toml` |
| `jarvis-identity` | `jarvis-identity` | `jarvis_identity` | None | `crates/jarvis-identity/Cargo.toml` |

Excluded research manifests: spikes/acp-rust-protocol-v1, process-sandbox,
rust-sse-transport, sqlite-fts5 and web-oracle-portability. Their names are not
mechanically renamed and they do not participate in production release ownership.

## Identity migration registry

Mechanical means a proposed reviewed rename, not an executed change. Sensitive
entries require compatibility design, even when a target is supplied. LegacyReader,
Provenance and ThirdParty never authorize mechanical replacement. RequiresEvidence
entries have no target and remain open until their source/integration contract is
established. Test/fixture-only environment symbols are explicitly uncertain.

| Surface | Current identifier | Proposed target | Classification | Source evidence |
|---|---|---|---|---|
| Branding | `Agent Vesper` | `J.A.R.V.I.S.` | Mechanical | `README.md` |
| Configuration | `agent-vesper` | `jarvis` | CompatibilitySensitive | `crates/vesper-config/src/paths.rs` |
| Configuration | `Agent Vesper` | `Jarvis` | CompatibilitySensitive | `crates/vesper-config/src/paths.rs` |
| Credential | `agent-vesper` | `99percentgrip.jarvis` | CompatibilitySensitive | `crates/vesper-provider-openai/src/credentials.rs` |
| Credential | `credentials.json` | `credentials.json` | CompatibilitySensitive | `crates/vesper-provider-glm/src/auth.rs` |
| Credential | `openai-credentials.json` | `openai-credentials.json` | CompatibilitySensitive | `crates/vesper-provider-openai/src/credentials.rs` |
| Credential | `xai-credentials.json` | `xai-credentials.json` | CompatibilitySensitive | `crates/vesper-provider-xai/src/credentials.rs` |
| Serialization | `agent-vesper-session` | `jarvis-session` | CompatibilitySensitive | `crates/vesper-sessions/src/vesper_format.rs` |
| Serialization | `compat.agent-vesper` | Unchanged/undecided | LegacyReader | `crates/vesper-sessions/src/vesper_format.rs` |
| Serialization | `agent-vesper-legacy-identity-v1` | Unchanged/undecided | LegacyReader | `crates/vesper-sessions/src/conversion.rs` |
| Serialization | `MessageJournal` | Unchanged/undecided | CompatibilitySensitive | `apps/agent-vesper-tui/src/messages.rs` |
| Serialization | `next_id` | Unchanged/undecided | LegacyReader | `apps/agent-vesper-tui/src/messages.rs` |
| Serialization | `Checkpoint` | Unchanged/undecided | RequiresEvidence | `crates/vesper-checkpoints/src/lib.rs` |
| ProjectScope | `.vesper-scope-id` | `.jarvis-scope-id` | CompatibilitySensitive | `crates/vesper-agent/src/vro/scope.rs` |
| ProjectScope | `.agent-vesper` | `.jarvis` | CompatibilitySensitive | `crates/vesper-agent/src/vro/scope.rs` |
| Acp | `agent-vesper` | `jarvis` | CompatibilitySensitive | `registry/agent.json` |
| Acp | `agent-vesper-acp` | `jarvis-acp` | CompatibilitySensitive | `crates/vesper-acp/src/adapter.rs` |
| Browser | `VesperLens` | `JarvisLens` | Mechanical | `crates/vesper-agent/src/planning/vesper_lens/mod.rs` |
| Browser | `X-Vesper-Lens-Token` | `X-Jarvis-Lens-Token` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/mod.rs` |
| Browser | `vesper:remove-highlight` | `jarvis:remove-highlight` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Bridge | `BridgeSessionId` | Unchanged/undecided | CompatibilitySensitive | `crates/vesper-bridge/src/identity.rs` |
| Bridge | `application_key` | Unchanged/undecided | RequiresEvidence | `crates/vesper-bridge/src/identity.rs` |
| Voice | `voice-kokoro` | Unchanged/undecided | ThirdParty | `apps/agent-vesper-tui/Cargo.toml` |
| Reasoning | `VRO` | `JRO` | Mechanical | `crates/vesper-agent/src/vro/mod.rs` |
| Distribution | `Programs\AgentVesper` | `Programs\Jarvis` | CompatibilitySensitive | `scripts/install.ps1` |
| Release | `vesper-release-controller` | `jarvis-release-controller` | Mechanical | `crates/vesper-harness/src/release_executor.rs` |
| Release | `release-recovery` | Unchanged/undecided | CompatibilitySensitive | `crates/vesper-harness/src/release_recovery.rs` |
| Release | `agent-vesper-acp-{platform}.{extension}` | `jarvis-{platform}.{extension}` | CompatibilitySensitive | `crates/vesper-harness/src/release_closeout.rs` |
| History | `99percentgrip/agent-vesper` | Unchanged/undecided | Provenance | `README.md` |
| History | `Apache-2.0` | Unchanged/undecided | Provenance | `Cargo.toml` |
| Provider | `agent-client-protocol` | Unchanged/undecided | ThirdParty | `Cargo.toml` |
| Provider | `zai` | Unchanged/undecided | ThirdParty | `crates/vesper-provider-glm/src/lib.rs` |
| Provider | `referrer` | Unchanged/undecided | RequiresEvidence | `crates/vesper-provider-xai/src/auth.rs` |
| Environment | `AGENT_VESPER_ALLOW_INSECURE_LOOPBACK` | `JARVIS_ALLOW_INSECURE_LOOPBACK` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_BUNDLE_DIR` | `JARVIS_BUNDLE_DIR` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/process_transcript.rs` |
| Environment | `AGENT_VESPER_CHECKPOINT_ROOT` | `JARVIS_CHECKPOINT_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_COGNITION_EMBEDDING_API` | `JARVIS_COGNITION_EMBEDDING_API` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_COGNITION_EMBEDDING_MODEL` | `JARVIS_COGNITION_EMBEDDING_MODEL` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_COGNITION_MODEL` | `JARVIS_COGNITION_MODEL` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_COGNITION_ROOT` | `JARVIS_COGNITION_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_COGNITION_USER_ID` | `JARVIS_COGNITION_USER_ID` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_CONFIG_DIR` | `JARVIS_CONFIG_DIR` | CompatibilitySensitive | `apps/agent-vesper-tui/src/main.rs` |
| Environment | `AGENT_VESPER_CREDENTIALS_PATH` | `JARVIS_CREDENTIALS_PATH` | CompatibilitySensitive | `apps/agent-vesper-tui/tests/openai_startup_responsiveness.rs` |
| Environment | `AGENT_VESPER_ENABLE_CHECKPOINTS` | `JARVIS_ENABLE_CHECKPOINTS` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_ENABLE_LEGACY_SESSION_READS` | `JARVIS_ENABLE_LEGACY_SESSION_READS` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_ENABLE_SCOPE_STAMP` | `JARVIS_ENABLE_SCOPE_STAMP` | CompatibilitySensitive | `apps/agent-vesper-acp/src/main.rs` |
| Environment | `AGENT_VESPER_ENABLE_SESSION_READS` | `JARVIS_ENABLE_SESSION_READS` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_ENABLE_SESSION_WRITES` | `JARVIS_ENABLE_SESSION_WRITES` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_ENABLE_VESPER_SESSION_READS` | `JARVIS_ENABLE_VESPER_SESSION_READS` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_EXTRA_SCOPES` | `JARVIS_EXTRA_SCOPES` | CompatibilitySensitive | `crates/vesper-agent/src/vro/scope.rs` |
| Environment | `AGENT_VESPER_FIREWALL` | `JARVIS_FIREWALL` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_FULL_HARNESS` | `JARVIS_FULL_HARNESS` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_GLM_BASE_URL` | `JARVIS_GLM_BASE_URL` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_GLM_MODEL` | `JARVIS_GLM_MODEL` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_GLOBAL_COGNITION_ROOT` | `JARVIS_GLOBAL_COGNITION_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_GLOBAL_MEMORY_ROOT` | `JARVIS_GLOBAL_MEMORY_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/process_transcript.rs` |
| Environment | `AGENT_VESPER_HOME` | `JARVIS_HOME` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_INSTALL_DIR` | `JARVIS_INSTALL_DIR` | CompatibilitySensitive | `apps/agent-vesper-tui/src/update_host.rs` |
| Environment | `AGENT_VESPER_LEGACY_PROFILE` | `JARVIS_LEGACY_PROFILE` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_LEGACY_SESSION_ROOT` | `JARVIS_LEGACY_SESSION_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_LMSTUDIO_CREDENTIALS_PATH` | `JARVIS_LMSTUDIO_CREDENTIALS_PATH` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lmstudio_provider.rs` |
| Environment | `AGENT_VESPER_LMSTUDIO_MODEL` | `JARVIS_LMSTUDIO_MODEL` | CompatibilitySensitive | `crates/vesper-agent/tests/live_react_integration.rs` |
| Environment | `AGENT_VESPER_LMSTUDIO_ROOT` | `JARVIS_LMSTUDIO_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/cognition.rs` |
| Environment | `AGENT_VESPER_LMSTUDIO_URL` | `JARVIS_LMSTUDIO_URL` | CompatibilitySensitive | `crates/vesper-agent/tests/live_react_integration.rs` |
| Environment | `AGENT_VESPER_LOG` | `JARVIS_LOG` | CompatibilitySensitive | `apps/agent-vesper-acp/src/main.rs` |
| Environment | `AGENT_VESPER_MCP_ROOT` | `JARVIS_MCP_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_MEMORY_ROOT` | `JARVIS_MEMORY_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/process_transcript.rs` |
| Environment | `AGENT_VESPER_MOBILE_ALLOW_PUBLIC` | `JARVIS_MOBILE_ALLOW_PUBLIC` | CompatibilitySensitive | `apps/agent-vesper-tui/src/mobile.rs` |
| Environment | `AGENT_VESPER_MOBILE_BIND` | `JARVIS_MOBILE_BIND` | CompatibilitySensitive | `apps/agent-vesper-tui/src/mobile.rs` |
| Environment | `AGENT_VESPER_MOBILE_URL` | `JARVIS_MOBILE_URL` | CompatibilitySensitive | `apps/agent-vesper-tui/src/mobile.rs` |
| Environment | `AGENT_VESPER_NATIVE_PROCESS_FIXTURE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-harness/src/host_resources_native.rs` |
| Environment | `AGENT_VESPER_NAV_TRACE` | `JARVIS_NAV_TRACE` | CompatibilitySensitive | `apps/agent-vesper-tui/src/auth_settings.rs` |
| Environment | `AGENT_VESPER_OPENAI_CREDENTIALS_PATH` | `JARVIS_OPENAI_CREDENTIALS_PATH` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/process_transcript.rs` |
| Environment | `AGENT_VESPER_OPENAI_TEST_MODE` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_OPENAI_TEST_URL` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_PROVIDER` | `JARVIS_PROVIDER` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_REGISTRY_PR` | `JARVIS_REGISTRY_PR` | CompatibilitySensitive | `crates/vesper-harness/src/release_executor.rs` |
| Environment | `AGENT_VESPER_RELEASE_BASE_URL` | `JARVIS_RELEASE_BASE_URL` | CompatibilitySensitive | `apps/agent-vesper-tui/src/update_host.rs` |
| Environment | `AGENT_VESPER_RELEASE_ROOT` | `JARVIS_RELEASE_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/release_resource_governor.rs` |
| Environment | `AGENT_VESPER_SANDBOX` | `JARVIS_SANDBOX` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SESSION_MAX_BYTES` | `JARVIS_SESSION_MAX_BYTES` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SESSION_MAX_ENTRIES` | `JARVIS_SESSION_MAX_ENTRIES` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SESSION_ROOT` | `JARVIS_SESSION_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SESSION_WRITE_MAX_BYTES` | `JARVIS_SESSION_WRITE_MAX_BYTES` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SESSION_WRITE_ROOT` | `JARVIS_SESSION_WRITE_ROOT` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_SHELL_PROFILE` | `JARVIS_SHELL_PROFILE` | CompatibilitySensitive | `apps/agent-vesper-tui/src/update_host.rs` |
| Environment | `AGENT_VESPER_TELEMETRY` | `JARVIS_TELEMETRY` | CompatibilitySensitive | `apps/agent-vesper-tui/src/main.rs` |
| Environment | `AGENT_VESPER_TEST_DISPATCH_GATE` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/bin/agent-vesper-acp-test-driver.rs` |
| Environment | `AGENT_VESPER_TRAJECTORY_PATH` | `JARVIS_TRAJECTORY_PATH` | CompatibilitySensitive | `apps/agent-vesper-tui/src/main.rs` |
| Environment | `AGENT_VESPER_VERSION` | `JARVIS_VERSION` | CompatibilitySensitive | `apps/agent-vesper-tui/src/update_host.rs` |
| Environment | `AGENT_VESPER_VOICE_VENV` | `JARVIS_VOICE_VENV` | CompatibilitySensitive | `apps/agent-vesper-tui/src/lib.rs` |
| Environment | `AGENT_VESPER_VRO_ENABLED` | `JARVIS_VRO_ENABLED` | CompatibilitySensitive | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_WEB` | `JARVIS_WEB` | CompatibilitySensitive | `crates/vesper-harness/src/web_service.rs` |
| Environment | `AGENT_VESPER_WHISPER_MODEL` | `JARVIS_WHISPER_MODEL` | CompatibilitySensitive | `apps/agent-vesper-tui/src/voice_shared_stt.rs` |
| Environment | `AGENT_VESPER_WINDOWS_JOB_FIXTURE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-harness/src/host_resources_native.rs` |
| Environment | `AGENT_VESPER_XAI_CREDENTIALS_PATH` | `JARVIS_XAI_CREDENTIALS_PATH` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/provider_selection.rs` |
| Environment | `AGENT_VESPER_XAI_TEST_MODE` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_XAI_TEST_STALE_HOSTED` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `AGENT_VESPER_XAI_TEST_URL` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-acp/src/lib.rs` |
| Environment | `VESPER_BRIDGE_RESOLVE_IPC` | `JARVIS_BRIDGE_RESOLVE_IPC` | CompatibilitySensitive | `crates/vesper-harness/src/bridge_adapter_tests.rs` |
| Environment | `VESPER_CREDENTIAL_FIXTURE_PHASE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-auth/tests/native_persistence.rs` |
| Environment | `VESPER_CREDENTIAL_FIXTURE_SERVICE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-auth/tests/native_persistence.rs` |
| Environment | `VESPER_CREDENTIAL_FIXTURE_VAULT` | Unchanged/undecided | RequiresEvidence | `crates/vesper-auth/tests/native_persistence.rs` |
| Environment | `VESPER_DEPENDENCY_TEST_ENGINE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-harness/src/dependency_setup_tests.rs` |
| Environment | `VESPER_DEPENDENCY_TEST_IMAGE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-harness/src/dependency_setup_tests.rs` |
| Environment | `VESPER_DOCKER_BIN` | `JARVIS_DOCKER_BIN` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/dependency_setup_controls.rs` |
| Environment | `VESPER_DOCKER_DESKTOP` | `JARVIS_DOCKER_DESKTOP` | CompatibilitySensitive | `crates/vesper-harness/src/dependency_setup.rs` |
| Environment | `VESPER_DOCKER_IMAGE` | `JARVIS_DOCKER_IMAGE` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/swarm_native_process.rs` |
| Environment | `VESPER_DUMP_UI` | `JARVIS_DUMP_UI` | CompatibilitySensitive | `apps/agent-vesper-tui/src/ui.rs` |
| Environment | `VESPER_LENS_FEEDBACK` | `JARVIS_LENS_FEEDBACK` | CompatibilitySensitive | `crates/vesper-agent/examples/vesper_lens_fixture.rs` |
| Environment | `VESPER_LENS_URL` | `JARVIS_LENS_URL` | CompatibilitySensitive | `crates/vesper-agent/examples/vesper_lens_fixture.rs` |
| Environment | `VESPER_NATIVE_CREDENTIAL_ACCEPTANCE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-auth/tests/native_persistence.rs` |
| Environment | `VESPER_OUTPUT_CAPTURE_DIR` | `JARVIS_OUTPUT_CAPTURE_DIR` | CompatibilitySensitive | `apps/agent-vesper-tui/src/ui.rs` |
| Environment | `VESPER_PIPELINE_FIXTURE` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-tui/tests/voice_speech_pipeline.rs` |
| Environment | `VESPER_PYTHON_PATH` | `JARVIS_PYTHON_PATH` | CompatibilitySensitive | `apps/agent-vesper-tui/src/main.rs` |
| Environment | `VESPER_R3_PACK_CASE` | `JARVIS_R3_PACK_CASE` | CompatibilitySensitive | `apps/agent-vesper-tui/tests/r3_voice_pack.rs` |
| Environment | `VESPER_R3_PACK_ROOT` | `JARVIS_R3_PACK_ROOT` | CompatibilitySensitive | `apps/agent-vesper-tui/tests/r3_voice_pack.rs` |
| Environment | `VESPER_READ_ENABLE` | `JARVIS_READ_ENABLE` | CompatibilitySensitive | `apps/agent-vesper-acp/tests/persistence_writes.rs` |
| Environment | `VESPER_ROUTING_EVAL_OUTPUT` | `JARVIS_ROUTING_EVAL_OUTPUT` | CompatibilitySensitive | `crates/vesper-memory/tests/routing_quality_eval.rs` |
| Environment | `VESPER_ROUTING_INDEPENDENT_OUTPUT` | `JARVIS_ROUTING_INDEPENDENT_OUTPUT` | CompatibilitySensitive | `crates/vesper-memory/tests/routing_quality_independent.rs` |
| Environment | `VESPER_ROUTING_METADATA_OUTPUT` | `JARVIS_ROUTING_METADATA_OUTPUT` | CompatibilitySensitive | `crates/vesper-memory/tests/routing_quality_development.rs` |
| Environment | `VESPER_ROUTING_POLICY_OUTPUT` | `JARVIS_ROUTING_POLICY_OUTPUT` | CompatibilitySensitive | `crates/vesper-memory/tests/routing_quality_development.rs` |
| Environment | `VESPER_ROUTING_TASK_OUTPUT` | `JARVIS_ROUTING_TASK_OUTPUT` | CompatibilitySensitive | `crates/vesper-memory/tests/routing_quality_development.rs` |
| Environment | `VESPER_RRC_LIFECYCLE_MODE` | `JARVIS_RRC_LIFECYCLE_MODE` | CompatibilitySensitive | `crates/vesper-harness/src/release_executor.rs` |
| Environment | `VESPER_RRC_LIFECYCLE_ROOT` | `JARVIS_RRC_LIFECYCLE_ROOT` | CompatibilitySensitive | `crates/vesper-harness/src/release_executor.rs` |
| Environment | `VESPER_SANDBOX_INIT` | `JARVIS_SANDBOX_INIT` | CompatibilitySensitive | `crates/vesper-sandbox/src/lib.rs` |
| Environment | `VESPER_SECRET_CANARY` | `JARVIS_SECRET_CANARY` | CompatibilitySensitive | `crates/vesper-security/src/secret.rs` |
| Environment | `VESPER_SESSION_FORMAT` | `JARVIS_SESSION_FORMAT` | CompatibilitySensitive | `crates/vesper-sessions/src/vesper_format.rs` |
| Environment | `VESPER_SESSION_VERSION` | `JARVIS_SESSION_VERSION` | CompatibilitySensitive | `crates/vesper-sessions/src/vesper_format.rs` |
| Environment | `VESPER_SETUP_PACKAGE` | `JARVIS_SETUP_PACKAGE` | CompatibilitySensitive | `crates/vesper-harness/src/dependency_setup.rs` |
| Environment | `VESPER_SETUP_REPAIR` | `JARVIS_SETUP_REPAIR` | CompatibilitySensitive | `crates/vesper-harness/src/voice_dependency_setup.rs` |
| Environment | `VESPER_SYNTHETIC_CANARY` | `JARVIS_SYNTHETIC_CANARY` | CompatibilitySensitive | `crates/vesper-provider-glm/src/integration_tests.rs` |
| Environment | `VESPER_TEST_NODE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-harness/tests/swarm_adapter_tests.rs` |
| Environment | `VESPER_TEST_PLAYWRIGHT_CLI` | Unchanged/undecided | RequiresEvidence | `crates/vesper-mcp/src/playwright_live_tests.rs` |
| Environment | `VESPER_TEST_PYTHON` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-tui/tests/message_queue_pty.rs` |
| Environment | `VESPER_TUI_SWARM_CHILD` | `JARVIS_TUI_SWARM_CHILD` | CompatibilitySensitive | `apps/agent-vesper-tui/src/swarm_host_tests.rs` |
| Environment | `VESPER_UPDATE_FIXTURES` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-tui/src/update_host.rs` |
| Environment | `VESPER_WEB_TEST_IMAGE` | Unchanged/undecided | RequiresEvidence | `crates/vesper-web-fetch/src/browser.rs` |
| Provider | `ZAI_API_KEY` | Unchanged/undecided | ThirdParty | `crates/vesper-config/src/app.rs` |
| Provider | `OPENAI_API_KEY` | Unchanged/undecided | ThirdParty | `crates/vesper-provider-openai/src/credentials.rs` |
| Provider | `XAI_API_KEY` | Unchanged/undecided | ThirdParty | `crates/vesper-provider-xai/src/credentials.rs` |
| Provider | `XDG_CONFIG_HOME` | Unchanged/undecided | ThirdParty | `crates/vesper-provider-glm/src/auth.rs` |
| Provider | `XDG_DATA_HOME` | Unchanged/undecided | ThirdParty | `crates/vesper-agent/src/vro/scope.rs` |
| Provider | `XDG_CACHE_HOME` | Unchanged/undecided | ThirdParty | `apps/agent-vesper-acp/tests/process_transcript.rs` |
| Provider | `XDG_STATE_HOME` | Unchanged/undecided | ThirdParty | `crates/vesper-harness/src/release_recovery.rs` |
| Provider | `APPDATA` | Unchanged/undecided | ThirdParty | `crates/vesper-provider-glm/src/auth.rs` |
| Provider | `LOCALAPPDATA` | Unchanged/undecided | ThirdParty | `crates/vesper-voice-kokoro/src/pack.rs` |
| Provider | `HOME` | Unchanged/undecided | ThirdParty | `crates/vesper-agent/src/vro/scope.rs` |
| Provider | `USERPROFILE` | Unchanged/undecided | ThirdParty | `crates/vesper-agent/src/vro/scope.rs` |
| Browser | `__vesperLensSdk` | `__jarvisLensSdk` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper-lens-sdk-style` | `jarvis-lens-sdk-style` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper-lens:` | `jarvis-lens:` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:annotation` | `jarvis:annotation` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:artifact-failure` | `jarvis:artifact-failure` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:layout-diagnostics` | `jarvis:layout-diagnostics` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:ready` | `jarvis:ready` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:request-state` | `jarvis:request-state` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:restore` | `jarvis:restore` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:review-state` | `jarvis:review-state` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `vesper:set-mode` | `jarvis:set-mode` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Browser | `x-vesper-lens-token` | `x-jarvis-lens-token` | CompatibilitySensitive | `crates/vesper-agent/src/planning/vesper_lens/injector.rs` |
| Bridge | `org.mpris.MediaPlayer2.Player` | Unchanged/undecided | ThirdParty | `crates/vesper-harness/src/bridge_adapters.rs` |

| Environment | `GLM_VENV_PATH` | `JARVIS_VOICE_VENV_PATH` | CompatibilitySensitive | `apps/agent-vesper-tui/src/voice_shared_stt.rs` |
| Environment | `AGENT_VESPER_VOICE_PYTHON` | Unchanged/undecided | RequiresEvidence | `apps/agent-vesper-tui/tests/voice_vad_worker.py` |
| Environment | `AGENT_VESPER_GWS_BIN` | `JARVIS_GWS_BIN` | CompatibilitySensitive | `skills/skills/google-workspace/scripts/google_api.py` |
| Environment | `AGENT_VESPER_CITATION_LEDGER` | `JARVIS_CITATION_LEDGER` | CompatibilitySensitive | `skills/skills/grounded-citations/scripts/sources.py` |

## Sequencing and remaining decisions

Stage 2 needs explicit owner approval before any existing package/import rename,
plus migration-safe version editing, Cargo aliases and source-boundary updates.
The protocol/application distinction above must be preserved. External package
name availability and ACP ID collision handling remain open. Later stages own
runtime path activation, migration engine, browser/bridge compatibility, installer
ownership, independent RRC and public documentation. Future security fixes should
be reviewed/cherry-picked by exact source SHA in separate PRs with full matrices
until divergence is intentional. No shared release epoch or automatic source push.

## Evidence

See [Stage 1 PRD](../identity-stage1-prd.md),
[execution report](../foundation/2026-10-14-identity-stage1.md) and
[source provenance](../foundation/stage1-source-provenance.json).

## Observed excluded-research security limitation

Authenticated GitHub Dependabot alert 1 reports GHSA-2mjx-qc3c-rqvc
(RUSTSEC-2026-0285) for rustls 0.23.42 in
spikes/rust-sse-transport/Cargo.lock. The production Cargo.lock contains patched
0.23.45. This existing excluded research dependency is not upgraded by Stage 1
and is not a production/release dependency. Production audit/deny gates do not
prove that every excluded research lockfile is clean. Remediation and a separate
research-lock security policy remain follow-up decisions; no release is authorized.
