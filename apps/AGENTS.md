# Composition applications

## Purpose

Own thin deployable binaries that compose production crates without absorbing
domain, provider, runtime, or protocol business logic.

## Local Contracts

- Applications may wire concrete adapters at composition boundaries.
- Stage 3 hosts resolve private state through shared config/security ownership.
  ACP protocol startup validates without initialization; full harness composition
  initializes private roots. Ordinary release/updater and legacy-discovery routes
  remain inactive; explicit offline test-driver authority never follows from a
  production Cargo feature. External project tools retain existing permissions.
- Stdout contracts of protocol binaries are inviolable.
- Startup configuration must be secret-safe and default to secure endpoints.
- Applications may inject bounded session roots and persistence ports at an
  explicit composition boundary. Protocol binaries must keep persistence
  mutation behind bounded writers and must never expose raw secrets.

## Child DOX Index

- `jarvis-acp/AGENTS.md` — ACP stdio composition and process lifecycle.
- `jarvis-tui/AGENTS.md` — Stage 11b Terminal User Interface.
