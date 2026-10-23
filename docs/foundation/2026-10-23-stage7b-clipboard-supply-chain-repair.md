# Stage 7B supply-chain dependency selection repair (2026-10-23)

## Objective and authorization

Remove the yanked Windows clipboard transitive dependency selected at exact-main `21ee3ec8900c3aba17d27d9e3822434b2daa498e` without changing direct dependencies, policy, source, release-candidate workflow state or release assets. Protected integration and exact-merge CI are separate gates. Stage 7B five-target candidate acceptance remains incomplete and requires new authorization.

## Methods and files

- Read root, docs and foundation DOX; inspected worktree, five stashes, `Cargo.toml`, `Cargo.lock`, `deny.toml`, CI log of run `39923622652`, registry API and workflow registration. Preserved unrelated local documents and untracked evidence.
- `git fetch origin main`; created `fix/stage7b-clipboard-yank` from verified remote main; ran `cargo update -p clipboard-win` (no unscoped update or fabricated checksum).
- Changed `Cargo.lock` only for production dependency resolution. This report, its evidence-index pointer and owning PRD pointer are directly necessary documentation. No other dependency, source or security configuration changes.

## Root cause and exact graph

CI run `39923622652` reported `error[yanked]: clipboard-win 5.4.1` at `Cargo.lock:55`, with `advisories FAILED, bans ok, licenses ok, sources ok` from `cargo deny --all-features check`; `cargo audit` separately warned about the yanked version. Reverse graph: `terminal host 0.24.14 → arboard 3.6.1 → clipboard-win 5.4.1`. The direct manifest keeps `arboard = "=3.6.1"`. The crates.io 3.6.1 dependency metadata reports Windows-only `clipboard-win ^5.3.1`.

Crates.io version metadata reports `5.4.1` yanked and published `5.4.3` **not yanked**; 5.4.3 satisfies `^5.3.1`. The targeted Cargo update selected `5.4.3`, with `source = "registry+https://github.com/rust-lang/crates.io-index"` and Cargo-generated checksum `052af7878466feccda12ba7a6a67d019d712f296809559b47c12c8cc196d3a9a`; crates.io metadata independently reports that checksum. The former 5.4.1 checksum was `bde03770d3df201d4fb868f2c9c59e66a3e4e2bd06692a0fe701e7103c7e84d4`. `git diff -- Cargo.lock` changes exactly those two lines (version and checksum), leaving the dependency list unchanged. `arboard 3.6.1` and all other 541 package entries remain unchanged; no transitive additions or removals. Cargo reports `Locking 1 package to latest Rust 1.88 compatible version`.

## Local verification receipts

- `cargo tree --locked -i clipboard-win --target x86_64-pc-windows-msvc`: `clipboard-win v5.4.3 → arboard v3.6.1 → terminal host v0.24.14`.
- `cargo metadata --locked --no-deps --format-version 1 >/dev/null`: exit 0.
- `cargo audit` (local 0.22.2): exit 0, `Scanning Cargo.lock for vulnerabilities (542 crate dependencies)`; no yanked warning after repair.
- `cargo deny --all-features check` (local 0.20.2): exit 0, `advisories ok, bans ok, licenses ok, sources ok`. Pre-existing duplicate-version warnings are not suppressed.
- `CARGO_BUILD_JOBS=2 cargo check --locked -p` with the terminal package identity from the workspace registry: exit 0, `Finished dev profile ... in 0.14s` on Linux; this does not compile the Windows-specific clipboard backend.
- `git diff --check`: exit 0. No Windows build or five-target candidate run was performed locally.

## Protected integration and limitations

PR number, final head, mandatory checks, retries, merge SHA and four exact-merge workflow results: pending observation. A successful PR and exact-merge prerequisite matrix would not authorize candidate dispatch or publication. Independently read-only checked workflow `.github/workflows/release.yml` (`383935829`): `disabled_manually` before integration. Existing historical candidate artifacts belong to their original SHA. No release candidate, tag, publication, installer or production RRC operation is authorized here.
