# Stage 1 identity architecture execution

## Objective and methods

Implement the owner-approved Stage 1 specification only. Inspected authenticated
main/PR/rulesets/workflows, Cargo manifests and active source contracts; read owning
DOX. Verified baseline 57d8280a8d4a470af5878088b9d8d0e1645a12da has successful
canonical 38548536997, MSRV 38548537044, five-target 38548536966 and web-driver
38548537295 push runs. All thirteen contexts remain required. Publishing and both
RRC workflows are disabled, release tags frozen, original Vesper independent.

Created feat/stage1-identity-architecture in a clean destination-only worktree. The existing destination checkout's three local
report edits are not copied, discarded or included. Original Vesper is untouched.

## Files and implementation

Added the identity crate's manifest, DOX, pure typed library, explicit package
registry, classified mapping registry, injected platform specification and metadata
integration tests. Workspace membership/lockfile and xtask verification are updated;
all existing package versions/targets and release code remain unchanged. Normal
identity dependencies are empty; config/JSON are test-only. Architecture enforces
actual package/binary/import ownership and prohibits active runtime consumers.

Owning root/crates/xtask/documentation DOX and indexes are updated. GitHub/scripts
DOX now correctly records the already-approved unfiltered required-check policy.
Added architecture inventory, owning PRD and source provenance receipt. Existing
historical records and the temporary README notice remain intact.

## Commands and evidence ownership

Local commands: `cargo test -p` for the identity package; the same command under
Rust 1.88; `cargo test -p xtask`; `cargo clippy` for both changed packages with
all targets and `-D warnings`; `cargo xtask architecture`; `cargo xtask naming-guard`;
`cargo fmt --all`; `git diff --check`. Local compiler work uses two jobs, disabled
incremental/debug artifacts, and this destination worktree's own target directory.
No Vesper target cache or runtime was used. Provenance is captured in the owned
JSON receipt; links and JSON are checked independently. Full `cargo xtask verify`,
acceptance, audit/deny and all five native targets are required in hosted CI.

## Verification and deviations

Eight deterministic identity cases pass locally on Rust 1.95 and 1.88. Eight xtask
unit cases and two invocation-root cases pass. Clippy on both changed packages passes. Architecture purity regressions also
reject clock, I/O, output and grouped standard-library import attempts.
Architecture validates 32 workspace packages; five excluded research manifests are
also inventoried. No new external dependency versions were resolved.

The collision tests initially caught duplicate ACP target declarations and a
protocol/application planned-package collision; corrected the registry without
weakening tests. Proposed protocol package now has a distinct identity. The old
naming guard's first-party stem collision required a narrow ownership rule; all
thirteen upstream patterns and the frozen baseline remain unchanged. Historical
scopes, unknown names, other upstream stems and bare prose are not exempted.
Source tag provenance is verified through authenticated read-only GitHub evidence;
it is unsigned and no cryptographic verification is claimed. CI tests use the
receipt rather than assuming a shallow checkout has historical Git tags.

## Pending acceptance and limitations

CI_PENDING. Exact PR SHA, all thirteen conclusions, normal merge and merged source
receipts must be recorded before completion. Registry name availability, checkpoint
wire identity and undecided fixture/adapter metadata remain explicitly uncertain.
Paths are proposed and tested defaults; filesystem aliases/overrides require future
ownership checks. Import is Stage 4 work, not implemented. No RRC operation, runtime
initialization, migration, installation, version tag or release occurred.

## Readiness effect

Local Stage 1 design is implemented; CI/merge evidence is pending. Stage 2 is not
authorized. The [owning PRD](../identity-stage1-prd.md) and
[full inventory](../architecture/identity-architecture.md) define the next boundary.
