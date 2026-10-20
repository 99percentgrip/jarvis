# Stage 5 integration identity

## Authority

Baseline commit `f7f360ab648913428db79e6d48b3ad3ae61b1673`, tree `d1aa9423a116295d8149eecf489052ebd2af6c2d`. Historical data import stays deferred. Installers, release assets, and external registration stay inactive.

## Owned changes

| Surface | Previous | Current | Class |
| --- | --- | --- | --- |
| SDK agent name and initialize `agentInfo.name` | `agent-vesper-acp` / `agent-vesper` | `jarvis-acp` | required |
| Process meta output | mixed inherited name | `jarvis-acp` on stderr | required |
| Shared `/version` | inherited ACP name | `J.A.R.V.I.S.` plus crate version | required |
| MCP `clientInfo.name` | `agent-vesper` and `agent-vesper-tui` | `jarvis-mcp` | required |
| Lens feedback header | `X-Vesper-Lens-Token` | `X-Jarvis-Lens-Token` | paired |
| Lens messages and storage | `vesper:*`, `vesper-lens:` | `jarvis:remove-highlight`, `jarvis-lens:` | paired |
| Review chrome copy | `VesperLens` | `JarvisLens` | required |
| Default fetch user agent | `agent-vesper` | `jarvis-web` | required |

The Rust type `VesperLens` stays internal. Both Lens endpoints are in `jarvis-agent`. A request that still sends `X-Vesper-Lens-Token` is rejected.

## Preserved

- `registry/agent.json` still points at the historical `agent-vesper` archives. No download URL was invented.
- Private roots, ownership markers, and legacy-path refusal.
- Session markers `agent-vesper-session` and `compat.agent-vesper`.
- OpenAI `originator`, xAI `referrer`, and `x-grok-client-identifier`. Subscription and Google authorization stay blocked.
- Bridge session types and external application keys.
- Release archive filenames and historical installer logs.
- User-selected MCP server names.

## Verification

Initialize JSON asserts `agentInfo.name` is `jarvis-acp`. Lens tests accept the new header and reject the old one. Stdout stays limited to protocol frames.
