# Stage 7B native candidate preparation repair

## Objective and boundary

Correct the two observed pre-acceptance failures in candidate run `39795920384` through a protected PR. This unit does **not** authorize or execute another release-candidate dispatch. Stage 7B remains incomplete until separately authorized five-target acceptance.

## Methods and files

- Initial local branch `fix/stage7b-workflow-syntax-guard` at `077c440dda24a67648a22085782889cafeb4e13e`; remote `main` at `bfec51df515f164230d6f66abb2d81d72322293a`. No staged files. Local uncommitted documentation, `.agent/` (including historical macOS archives), existing recovery ledger and five stashes were preserved. `git fetch origin main` reported zero file changes between old head and remote main; `git switch -c fix/stage7b-native-candidate-preparation origin/main` retained all local modifications and established the corrective branch at the exact main SHA. No worktree, extra target root, reset, stash or local installer was used.
- `.github/workflows/release.yml`: Linux-only hosted `prepare-credential-runner.py --native` step with a five-minute bound before the Rust build; Windows-only PowerShell PE audit selects `--build-targets`, checks the four staged build binaries and propagates the actual auditor exit code without asserting a specific VC-runtime cause. No publishing path, workflow permissions or matrix target changed.
- `.github/test_credential_backend.py` and `scripts/test_windows_package_audit.py`: focused candidate-workflow ordering, platform guard, audit inventory and accurate-error propagation tests; existing lower-level D-Bus and PE refusal fixtures are unchanged.

## Red-first and verification receipts

Before the workflow correction, the Linux-focused unittest failed with `ValueError: substring not found` for the missing preparation step, and the Windows-focused unittest failed asserting the absent `--build-targets` invocation. After the correction, each focused test returned `Ran 1 test ... OK`.

Focused commands and results:

- `python3 -m unittest discover -s scripts -p test_windows_package_audit.py`: `Ran 9 tests ... OK` (including missing-file and forbidden-runtime cases).
- `python3 -m unittest discover -s .github -p test_credential_backend.py`: `Ran 5 tests ... OK` (including fail-closed hosted-only refusal and D-Bus command bounds).
- `python3 -m unittest discover -s scripts -p test_release_gate.py`: `Ran 17 tests ... OK`.
- `python3 -m unittest discover -s scripts -p test_jarvis_candidate_acceptance.py`: `Ran 11 tests ... OK`.
- `bash .github/verify_workflows.sh`: `actionlint 1.7.12: validated 9 workflows` (including disabled candidate workflow).
- `cargo xtask naming-guard` against the dirty working tree failed on *previously uncommitted* historical candidate report and PRD text; the new report initially also had two unregistered prose mentions, which were removed. The historical local evidence must not be rewritten to manufacture a green dirty-tree receipt. Check naming again against the exact staged commit/PR head, and record its result separately.

Pending: exact-commit formatting/naming/whitespace, thirteen final PR-head checks and four exact-merge workflows. Record exact identities below before making a protected-integration claim.

## Deviations, unresolved items and readiness

The historical macOS Intel/Apple Silicon archives and native receipts belong only to `bfec51df515f164230d6f66abb2d81d72322293a`, not the corrected source. Linux and Windows candidate acceptance remains unexecuted for the corrected commit. No release-candidate enablement, dispatch, job rerun, tag, Release, registry mutation, deployment, RRC mutation, production credential or user installation is in scope. This report is an execution ledger, not a Stage 7B completion claim.
