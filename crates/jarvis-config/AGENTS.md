# Configuration contracts

## Purpose

Own independent platform runtime paths, explicit legacy path descriptors, profile validation, typed application
configuration, provider envelopes, secret references, and persistence ports.

## Local Contracts

- LM Studio private settings use one absolute resolver and guarded reader shared by both hosts. `JARVIS_LMSTUDIO_ROOT` participates in bootstrap ownership validation; default resolution remains project `.jarvis/lmstudio` and never creates state while reading. Unknown ownership, redirected files and legacy roots refuse.

- `JarvisPaths` implements the approved platform roots and explicit first-run ownership checks. `VesperPaths` remains a pure legacy specification for provenance/disjointness tests, not an active runtime resolver. Native roots, explicit private overrides and credential-file overrides are validated before startup writes. Project config readers use `.jarvis`, require ownership for existing nonempty stores and refuse redirected paths; no legacy fallback or import.

- The legacy resolver remains descriptive; Stage 3 independently initializes only empty application-private runtime roots. Migration is not implemented.
- Legacy Native GLM ACP locations are always read-only descriptors.
- Config, data, cache, state, and logs remain distinct.
- Provider values are opaque envelopes; raw secrets are prohibited.
- `sandbox_config.rs` owns the `[sandbox]` scope demand reader and
  `web_config.rs` the `[web]` opt-in scope reader (VRO-14 PR-5:
  `enabled`/`respect_robots`/`output_budget_bytes`/`allowlist`, plus the
  default-off interaction/render gates, fetch gate, user agent, immutable
  `[web.driver] image`, and fail-closed network/private-address settings);
  web keys accept their documented dotted/nested forms. Both readers are
  deliberately dependency-free TOML-subset readers (no `toml` crate —
  see their module docs), missing files yield the inactive default, and
  unknown keys are ignored for forward compatibility.
- An explicitly saved `.jarvis/web-settings.json` is the complete web
  settings snapshot and takes precedence over `[web]` TOML. Missing snapshots
  fall back to TOML; malformed snapshots fail closed. This crate only reads
  them; explicit atomic persistence belongs to `jarvis-harness::web_settings`.

## Verification

- Run `cargo test -p jarvis-config`.
- Run `cargo xtask architecture`.

## Child DOX Index

No children.
