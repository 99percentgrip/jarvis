# Stage 6 distribution identity

Stage 6 defines offline package assembly and local installation. It does not publish a release, enable the release workflow, sign packages, or register a public ACP agent.

## Archives

| Target | Archive |
|---|---|
| Linux x86_64 | `jarvis-linux-x86_64.tar.gz` |
| Linux AArch64 | `jarvis-linux-aarch64.tar.gz` |
| macOS Intel | `jarvis-darwin-x86_64.tar.gz` |
| macOS Apple Silicon | `jarvis-darwin-aarch64.tar.gz` |
| Windows x86_64 | `jarvis-windows-x86_64.zip` |

These names are the `DISTRIBUTIONS` table in `crates/jarvis-identity/src/platform.rs`. The archive has one top-level directory. `manifest.json` uses schema 1 and layout 1. It records the application, target, canonical archive name, Cargo package version, optional source commit, and a SHA-256 for every payload. Sorted entry order and zero timestamps make the archive assembly repeatable. That is not a claim that Rust compiler output is byte-identical across machines.

## Payloads

User-facing and installed through launchers: `jarvis-acp`, `jarvis-tui` (`.exe` and `.cmd` on Windows).

Installed helpers, not given their own PATH launcher: `jarvis-web-fetch`, `jarvis-sandbox-init`.

Optional resources: `skills/` and `web-driver/image.tar.gz`, `image-id`, and `image.sha256`. Development-only binaries, including `jarvis-acp-test-driver` and `xtask`, are not packed.

The inherited container tag `vesper-web-driver:validated` is still the harness and web-driver workflow identifier. Stage 6 does not retag it. The disabled release workflow still expects the old package names and asset names; Stage 7 has to replace that workflow before any publication. It publishes to this repository, not to the Agent Vesper repository, and current source cannot build those retired package names.

## Install roots

Installer-owned files are not runtime-private state. The Stage 3 `.jarvis-owner` marker does not authorize replacement or deletion.

| Platform | Package root | Launchers | Memory |
|---|---|---|---|
| Linux | `$XDG_DATA_HOME/jarvis/install` | `$XDG_BIN_HOME` or `~/.local/bin` | `~/.jarvis/memory` |
| macOS | `~/Library/Application Support/Jarvis/Install` | `~/.local/bin` | `~/.jarvis/memory` |
| Windows | `%LOCALAPPDATA%/Programs/Jarvis` | `.cmd` files in that directory | `~/.jarvis/memory` |

Each successful install copies the validated archive into `generations/<id>` and points `current` at it only after `--version` succeeds. `.jarvis-installer-owned.json` lists the generations and launchers the installer may replace. A root that already contains unrelated files, a Vesper path, or a symlink is refused.

`JARVIS_PACKAGE` and `JARVIS_PACKAGE_SHA256` are mandatory. There is no network fetch and no `latest` fallback. A checksum shows the bytes match the supplied digest. It does not make an unsigned file an official release.

Rollback points `current` at the previous generation. It does not download an older release or touch memory. Uninstall deletes only recorded generations and launchers whose marker it recognizes. It does not remove shared directories, unowned PATH entries, or user data. There is no purge switch.

## What Stage 7 still owns

Signing, notarization, tag creation, GitHub Release assets, the release workflow, RRC, and public registry publication remain deferred. The macOS voice stall investigation is recorded in the Stage 6 execution report; it is not a packaging exemption.
