# Stage 5 integration identity — execution report

## Objective

Move owned ACP, MCP, Lens, and web client identities off the inherited product name without importing historical private data or publishing a registry entry.

## Methods

Static inventory of baseline `f7f360ab648913428db79e6d48b3ad3ae61b1673`. Paired Lens producer and consumer edits. Focused `cargo test --offline --locked` runs and `cargo xtask naming-guard`. Clippy `-D warnings` on the protocol, agent, MCP, config, web-fetch, and xtask packages.

## Files

Inventory: `docs/foundation/stage5-integration-inventory.json`. Contract: `docs/architecture/stage5-integration-identity.md`.

## Evidence

- `jarvis-acp-protocol` lib tests: 25 passed.
- `jarvis-agent` lib tests: 433 passed, including `legacy_vesper_lens_header_cannot_authorize_feedback`.
- `jarvis-mcp` lib tests: 25 passed, 1 ignored (`real_playwright_navigation_snapshot_click_isolation_close`, local Node/Playwright not supplied).
- `xtask` naming tests: `stage5_lens_wire_tokens_are_technical_but_unregistered_prose_is_not` and `declarations_are_scoped_and_other_upstream_mentions_stay_frozen` passed.
- `cargo xtask naming-guard`: clean, 36 frozen hits.
- `cargo fmt --all --check`: clean after formatting.
- Clippy `-D warnings` passed for `jarvis-acp-protocol`, `jarvis-agent`, `jarvis-mcp`, `jarvis-config`, `jarvis-web-fetch`, and `xtask`.

## Deviations

`registry/agent.json` is unchanged. Provider originator and xAI client headers are unchanged so this stage does not invent an OAuth client registration.

Local `cargo xtask verify` and harness/TUI clippy did not run to completion: `libdbus-sys` failed because Fedora package `dbus-devel` is not installed (`dbus-1.pc` absent). That is a workstation limitation, not a source change. Hosted CI remains the native proof.

## Unresolved

Editor registry publication, KDE interop, live microphone quality, the ignored Playwright MCP case, and the excluded research advisory stay outside this stage. Required pull-request checks and the four post-merge workflows are recorded after GitHub finishes them.

## Readiness

Not complete until the protected pull request merges and the four exact-merge workflows succeed.
