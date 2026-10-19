# Production crates

## Purpose

Own provider-neutral foundations, the GLM leaf adapter, Stage 4 runtime/ACP
boundaries, and Stage 5 read-only session discovery, conversion, replay, and
test-only conformance support.

## Local Contracts

- `jarvis-identity` owns side-effect-free current Stage 2 Rust identities, explicit legacy
  package ownership and deferred planned runtime identities. It has no production dependencies or runtime
  consumers; only xtask verification may link it. Its dev-only config dependency
  compares planned paths against the real current and legacy platform resolvers.


- `jarvis-domain` depends on no workspace crate.
- `jarvis-provider` depends only on `jarvis-domain`.
- `jarvis-security` depends on no workspace crate.
- `jarvis-auth` depends only on `jarvis-security`; it owns native OS
  credential-manager access and permission-verified retained/explicit file stores;
  production Linux writes require maintained encrypted Secret Service without
  automatic plaintext fallback
  (ADRs 0014 and 0032 — authentication and bounded native records).
- `jarvis-memory` depends only on `jarvis-domain` and `jarvis-security`; it
  owns the durable memory graph, learned skills, user profile, and bounded
  awareness ledger (ADR 0011 — Stage 12).
- `jarvis-cognition` depends only on `jarvis-domain` and `jarvis-security`
  (plus `rusqlite` (bundled), `rust-stemmers`, and standard utility crates);
  it owns the memory-oracle-equivalent V3 cognitive memory engine — single-pass
  ADD-only extraction, hybrid semantic + FTS5 BM25 + entity-boost retrieval,
  and the embedded SQLite backing (ADR 0015 — Stage 16). It is the **only**
  production crate permitted to declare `rusqlite`; provider embeddings +
  extraction LLM + entity NLP are trait ports fulfilled at the composition
  boundary, never inside this crate.
- `jarvis-checkpoints` depends only on `jarvis-domain` and
  `jarvis-security`; it owns the workspace snapshot/rollback, session
  lineage, and bounded cron/export/clipboard/CI surface (ADR 0012 —
  Stage 14). Strict RAII (`Drop`) file-handle discipline — no SQLite, no
  git refs, no auto-snapshotting.
- `jarvis-mcp` depends only on `jarvis-domain` and `jarvis-security`
  (plus `ed25519-dalek`); it owns the MCP stdio client and the
  Ed25519-signed plugin loader (ADR 0013 — Stage 15). The unsigned-plugin
  loading code path is structurally erased from `--release` builds via
  `#[cfg(debug_assertions)]`.
- `jarvis-voice` depends only on `jarvis-domain` and `jarvis-security`;
  it owns the VRO-17 voice pure core — validated PCM framing, STT/TTS
  ports/descriptors, the error taxonomy (NoSpeech ≠ Unavailable), the
  speech-egress policy and `[voice]` scope, host↔core event vocabulary,
  per-turn report types, and deterministic fakes. No adapters, no I/O,
  no runtime/agent/harness/provider dependency; hosts translate
  runtime/provider events into voice-owned inputs. PRD:
  `docs/voice-oracle-extraction-prd.md`.
- `jarvis-voice-kokoro` depends on `jarvis-domain`, `jarvis-security`,
  `jarvis-voice`, and the shared `jarvis-config` path resolver; it owns the R3 Natural Voice pack composition
  adapter — pack descriptor/integrity/lifecycle over one managed
  per-user cache, the espeak-ng IPA pronunciation bridge (reimplemented
  algorithm, no port), the bounded ONNX engine (ORT loaded dynamically
  from the digest-verified pack; no build-time link), the `VoiceTts`
  adapter, and the setup pipeline (confirm → byte-true progress →
  verify → silent-synthesis probe → publish). `#![forbid(unsafe_code)]`;
  `ort` sits behind the default-off `ort` feature (`default-features =
  false`); the crate compiles bare. Evidence:
  `docs/foundation/voice-oracle-kokoro-implementation.md`.
- `jarvis-config` depends only on `jarvis-domain` and `jarvis-security`.
- `jarvis-policy` depends only on `jarvis-domain` and `jarvis-security`.
- `jarvis-sandbox` depends only on `jarvis-security` plus platform `libc`
  confined to the Linux-only `sandbox_init` supervisor binary target; it owns
  the opt-in namespaces backend with probed, honest capabilities (ADR 0022 —
  Sandbox Supervisor as the Sole Raw-Syscall Boundary).
- `jarvis-bridge` depends only on `jarvis-domain` and `jarvis-security`
  (plus `serde`/`serde_json`/`thiserror`); it owns the pure
  provider-neutral application-control core for VB-PRD-001 —
  generation-bound identities, the versioned capability manifest, typed
  operation envelopes and result classification, session/operation state
  machines, fenced exclusive mutation leases, the intent-journal port,
  bounded observations and the error taxonomy. No I/O, clock, transport,
  application or provider names; adapters and supervision compose in
  `jarvis-harness` behind its ports. Unknown implementation status is
  never dispatchable; driver acknowledgments settle as `Applied`, never
  `Verified`.
- `jarvis-swarm` depends only on `jarvis-domain` and `jarvis-security` among
  workspace crates; it owns topology, pooling, priority messaging, assignment,
  the ephemeral ledger, sandbox lease coordination and hive orchestration.
  It performs no network/filesystem I/O or process spawning and names no provider.
  Bus TTL supports caller clock injection; pool/turn deadlines use Tokio
  virtualizable time. Owned blocking retirement retains uncertain capacity until
  verified destruction. Execution is a trait port.
  `jarvis-harness` composes the adapter behind optional default-off `swarm`;
  host activation remains acceptance-gated. `cargo xtask architecture` validates
  optional dependencies and transitive default-feature exclusion; naming guard
  enforces the upstream-brand embargo without baseline weakening.
- `jarvis-testkit` may depend on all foundational crates and owns synthetic
  read-store/no-write helpers; no production crate may depend on it.
- `jarvis-web` depends on `quick-xml`, `rust-stemmers`, `serde`,
  `serde_json`, `thiserror`, and `url`; it owns the pure
  parse/strip/prune/convert DOM pipeline with strictly zero I/O (no network,
  no filesystem, no clock). Transports and the headless renderer are
  composition-boundary ports, never implemented here.
- `jarvis-harness` may depend on pure `jarvis-policy` to enforce the invoking
  host's command firewall during native release progression; this exception
  grants no permission and introduces no concrete provider dependency.
- Explicit native dependency setup in `jarvis-harness` may invoke fixed OS package
  managers and checksum-pinned installer downloads after separate user confirmation.
  This setup-only exception is never a model tool or a host HTTP fallback for web
  execution; provider transport and runtime network permissions remain unchanged.
- `jarvis-harness` may compose `jarvis-web-fetch` behind the shared opt-in
  web service. Helper execution stays inside a network-granted sandbox and
  off the render thread; no host HTTP fallback is permitted.
- `jarvis-web-fetch` (VRO-14 PR-3) may depend on `jarvis-web`,
  `jarvis-sandbox`, and `jarvis-security` (plus `reqwest`/`serde`/`url`); it
  owns the sandbox-routed `FetchTransport` implementation and the
  `jarvis-web-fetch` helper binary — the only production unit besides the
  supervisor that performs network I/O, and only ever **inside** a sandbox
  provisioned with `IsolationRequirement::Network` + an explicit network
  grant. The pure egress gate in `jarvis-web::egress` must clear every
  request before any sandbox is provisioned; the harness process never
  fetches. The reqwest reference is architecture-scanner-exempted for this
  crate's sources only.
- `jarvis-provider-glm` may depend on auth/domain/provider/config/security and
  use `jarvis-testkit` only as a dev dependency.
- `jarvis-provider-openai` owns native OpenAI authentication, catalog, Responses
  transport, passive subscription usage, and tool-free memory extraction; it may depend on
  auth/domain/provider/config/security, HTTP, and `fs2` credential-operation
  locks. Both hosts compose it without a Codex runtime dependency.
- `jarvis-provider-xai` owns native xAI authentication, catalog/discovery,
  isolated API-key Responses and Grok-session proxy transports,
  passive Grok-session subscription usage, and
  stream/error translation;
  it may depend on auth/domain/provider/config/security and HTTP. API-key and
  Grok-account billing paths remain explicitly isolated.
- `jarvis-runtime` may depend on domain/provider and the read-only repository,
  converted-state, and transactional write ports from `jarvis-sessions`;
  filesystem I/O remains implemented only by `jarvis-sessions`, and runtime
  remains independent of ACP and concrete providers.
- `jarvis-acp-protocol` may depend on domain/runtime; it maps read-only persistent
  lifecycle outcomes without directly accessing storage, and all official ACP
  SDK types stay in this crate.
- `jarvis-sessions` may depend on domain/config and owns read-only, bounded
  discovery, legacy decoding, safe metadata, pure runtime-state seeds,
  deterministic identities, ACP-neutral replay plans, bounded persisted
  search, and the Stage 6 transactional Agent Vesper session writer. It must
  not depend on runtime,
  ACP, GLM, SQLite, or testkit in production.
- Provider HTTP belongs to concrete provider adapters; GLM behavior stays in
  `jarvis-provider-glm` and OpenAI behavior in `jarvis-provider-openai`. No crate
  may depend on ACP, SQLite, TUI, MCP, or a disposable spike.
- Unsafe code is denied by the current crates. Future platform exceptions
  require a dedicated module, safety comments, review, and ADR update. ADR 0022
  grants exactly one standing exception: `jarvis-sandbox`'s `sandbox_init`
  supervisor binary (raw syscalls for namespaces/mounts/chroot/capabilities/`fork`/`execve`,
  `#![deny(unsafe_op_in_unsafe_fn)]` + documented-block discipline, enforced by
  `cargo xtask architecture`); the `jarvis-sandbox` library itself stays
  100% safe code.

## Verification

- Run `cargo xtask architecture`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.

## Child DOX Index

- `jarvis-identity/AGENTS.md` — current Rust identity, compatibility registry and pure
  ownership verification; runtime migration is not authorized by this module.


- `jarvis-domain/AGENTS.md` — stable provider-neutral values and events.
- `jarvis-config/AGENTS.md` — platform paths, profiles, and typed configuration.
- `jarvis-provider/AGENTS.md` — provider ports, capabilities, and stream rules.
- `jarvis-security/AGENTS.md` — secret-safe and authority-boundary primitives.
- `jarvis-auth/AGENTS.md` — native-first provider-neutral credential storage
  and owner-only Unix vault fallback.
- `jarvis-policy/AGENTS.md` — pure permission and policy decisions.
- `jarvis-bridge/AGENTS.md` — pure provider-neutral application-control
  core (VB-PRD-001 Phase 1): identities, capability manifest, operation
  and session state machines, leases, journal port, observations, errors.
- `jarvis-testkit/AGENTS.md` — fixture and fake-conformance helpers.
- `jarvis-swarm/AGENTS.md` — pure provider-neutral swarm-coordination
  foundations (VRO-15; topology, pooling, bus, ledger, leases and hive).
- `jarvis-provider-glm/AGENTS.md` — Z.ai GLM provider adapter.
- `jarvis-provider-openai/AGENTS.md` — native OpenAI adapter,
  with no Codex installation or runtime dependency.
- `jarvis-provider-xai/AGENTS.md` — native xAI adapter with isolated API-key
  and Grok-account authentication paths.
- `jarvis-provider-synthetic/AGENTS.md` — deterministic in-process reference
  provider proving multi-provider contract neutrality.
- `jarvis-runtime/AGENTS.md` — provider-neutral session actors and converted
  state acceptance. (Single-turn engine; composed — not modified — by
  `jarvis-agent` under ADR 0010.)
- `jarvis-acp-protocol/AGENTS.md` — official-SDK ACP protocol-v1 adapter.
- `jarvis-sessions/AGENTS.md` — read-only session ports, bounded compatibility
  decoding, conversion, identity, replay plans, layouts, metadata, and the
  Stage 6 transactional writer.
- `jarvis-memory/AGENTS.md` — ADR 0011 (Stage 12) persistent memory graph,
  learned skills, user profile, and bounded epistemic ledger.
- `jarvis-cognition/AGENTS.md` — ADR 0015 (Stage 16) memory-oracle-equivalent V3
- `jarvis-checkpoints/AGENTS.md` — ADR 0012 (Stage 14) workspace snapshots,
  rollback, session lineage, and bounded cron/export/clipboard/CI surface.
- `jarvis-mcp/AGENTS.md` — ADR 0013 (Stage 15) MCP stdio client and
  Ed25519-signed plugin loader with `#[cfg(debug_assertions)]` dev mode.
- `jarvis-agent/AGENTS.md` — Tier C (ADR 0010). The multi-turn
  tool-executing agent loop, tool registry + executors, and permission gating
  that compose `jarvis-runtime`. Owns no provider-wire, ACP mapping, or
  persistence internals.
- `jarvis-harness/AGENTS.md` — shared hosted Python-oracle tool services for
  ACP and TUI compositions; the crate also owns the shared
  `COMPLETION_REPORTING_INSTRUCTION` mandate constant injected by both
  hosts (cross-host parity enforced by host prompt-composition tests).
- `jarvis-observability/AGENTS.md` — opt-in secret-safe trajectory recording
  and bounded reliability aggregation for composed hosts.
- `jarvis-web/AGENTS.md` — VRO-14 PR-1 perception engine: pure parse/strip/
  prune/convert pipeline (zero I/O, offline golden corpus).
- `jarvis-web-fetch/AGENTS.md` — VRO-14 PR-3 sandboxed fetch route: egress
  gate, helper binary, and `FetchTransport` over the ADR-0022 sandbox.
- `jarvis-voice-kokoro/AGENTS.md` — VRO-17 R3 Natural Voice pack adapter:
  pack integrity/lifecycle, pronunciation bridge, bounded engine, port
  adapter, setup pipeline.
