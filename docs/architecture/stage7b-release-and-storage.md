# Stage 7B nonpublishing candidate and storage boundaries

## Current scope

Stage 7B prepares independent release identities and owned local storage; it does not authorize public release, tagging, registry submission or enabling disabled workflows. The current branch is a candidate until protected exact-head and exact-merge evidence exists. The detailed execution history is in [the Stage 7B report](../foundation/2026-10-21-stage7b-final-acceptance-continuation.md).

## Producers and consumers

| Surface | Producer | Consumer / boundary | Retention and cleanup authority |
| --- | --- | --- | --- |
| Candidate workflow | `.github/workflows/release.yml` (manual dispatch, read-only token) | `scripts/release_gate.py` checks main exact-commit workflows; five native jobs call `scripts/jarvis_release_candidate.py` to validate SHA/triple, lockfile, compiler, tested driver/run and shipping binaries before using the Stage 6 `scripts/jarvis_package.py` packer | Short-lived archives/checksums, nonpublishing provenance JSON and native-install acceptance receipts from the exact produced archives; workflow is currently disabled on GitHub |
| Browser image | `.github/workflows/web-driver.yml` builds `jarvis-web-driver:validated` and emits matching `jarvis-web-driver-<arch>` immutable archives, digests and IDs | Candidate workflow downloads the admitted exact-commit run and verifies the archive digest before staging `web-driver/image.*`; native Settings uses the same image tag | Actions artifact retention; no rebuild in the candidate packaging lane |
| Release identity | `crates/jarvis-identity/src/lib.rs` and `src/mappings.rs` | RRC in `crates/jarvis-harness/src/release_recovery.rs` and `src/release_executor.rs` | Legacy identifiers remain provenance, not active controller ownership |
| Storage registry | `crates/jarvis-harness/src/host_storage.rs` | `host_resources.rs` and `release_executor.rs` | Marker-bound controller state; only registered, eligible, inactive and exclusively owned disposable paths may be removed |
| Cargo output | Current repository `target/` | Governed controller Cargo invocations | Preserve active and unknown data; a single canonical target is not an authorization to delete it |
| External registry | `registry/independent-registration.json` | Offline descriptor only | No external submission or release credentials |

## Storage lifecycle

The controller registers durable resources with lifecycle state and process leases. The governor observes actual filesystem space, forecasts pressure, reclaims only safe eligible resources, remeasures free space and retries admission when safe. High-water marks cause maintenance rather than automatic refusal. Active builds, foreign/unknown state, symlink redirects, source edits and required recovery evidence remain protected. Owner locks and governor leases prevent concurrent unsafe compilation. Platform-specific process and filesystem observations must be proved on native runners; Linux local simulation alone is not cross-platform acceptance.

## Acceptance boundary

`cargo xtask verify`, security tools and offline fixtures are local evidence. The candidate workflow checks each exact produced archive against its manifest, checksum, provenance and tested browser image, then runs native disposable install/uninstall before writing its acceptance receipt and uploading unchanged bytes. Passing offline fixture installs do not prove that candidate; [the exact-archive preflight report](../foundation/2026-10-22-stage7b-candidate-acceptance-preflight.md) records the local proof and the missing native evidence. PR #11 has thirteen exact-head successes, a protected merge and four successful exact-merge workflows; these do not substitute for a candidate run. The workflow's current `disabled_manually` state must not be changed merely to pass acceptance.
