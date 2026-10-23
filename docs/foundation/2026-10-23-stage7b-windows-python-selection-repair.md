# Stage 7B Windows candidate Python-selection repair

## Objective and boundary

Repair the interpreter-selection defect observed in nonpublishing candidate run 39900855149 (historical candidate; see linked prior execution report), without changing the pinned `pywinpty` requirement or dispatching another candidate. Stage 7B five-target acceptance remains open.

## Methods and source files

Started from verified `origin/main` `6236f19cdef9fec8ea8fd94380fb3c6f27b1cdaf` on short-lived `fix/stage7b-windows-python-313`. Preserved all existing modified documents, untracked candidate evidence and downloads, five stashes, and original project; no reset, cleaning, or stashing. Changed `.github/workflows/release.yml` and the candidate acceptance regression suite. The workflow now selects `actions/setup-python@v6` CPython 3.13 x64 on Windows only, immediately before Windows dependency preparation; a runtime assertion of Windows, CPython, 3.13, and 64-bit pointers precedes unchanged wheels-only/hash-required pip installation. GitHub setup-python adds the selected interpreter to PATH for subsequent steps; the existing native installer acceptance invokes the candidate acceptance Python entrypoint, which in turn invokes `python` for `first_launch_pty.py`. The Windows selection check fails before pip if interpreter routing is wrong. No unrelated installer path was changed.

## Local evidence

- Regression-first: candidate acceptance suite, `CandidateArchiveTests.test_windows_candidate_selects_pinned_python_before_hash_verified_install` failed on the historical workflow: `AssertionError: 0 != 1` (missing setup step); passes after the correction.
- candidate acceptance regression suite: `Ran 12 tests ... OK`.
- `python3 scripts/test_windows_package_audit.py`: `Ran 9 tests ... OK`.
- `bash .github/verify_workflows.sh`: `actionlint 1.7.12: validated 9 workflows`.
- `git diff --check` and Python syntax compilation of the candidate regression file: passed.
- Official PyPI `https://pypi.org/pypi/pywinpty/3.0.5/json` SHA-256 metadata: `pywinpty-3.0.5-cp312-cp312-win_amd64.whl` = `d62946adf14b15b54c0b8d785f93fe18b04da23f4ad59e2e8c4612646e9abd23`; `pywinpty-3.0.5-cp313-cp313-win_amd64.whl` = `48db1b0ad9d0a1b81dcaaa7163a99a7808deaceb0c1b2344716dc1fc090c3c4c`. The latter matches the unchanged pinned requirements line. This explains the historical failure without weakening pip integrity controls.
- Workflow diff leaves the five `package_target` entries, read-only `actions`/`contents` permissions, mandatory archive acceptance before upload, and nonpublishing triggers unchanged. Linux/macOS never run the new step. These are source/local regressions, **not** a real Windows execution receipt.

## Protected integration and remaining evidence

Pending: corrective PR number and final head; all thirteen exact-head mandatory checks; protected merge SHA and all four exact-merge workflow outcomes. The release-candidate workflow must remain `disabled_manually` throughout. No candidate dispatch, failed-job rerun, release, tag, registry publication, workstation installation, or production RRC mutation is authorized. The successful historical four-platform archives and receipts are evidence only for `6236f19cdef9fec8ea8fd94380fb3c6f27b1cdaf`; they cannot establish acceptance for a new merge SHA. Actual corrected Windows and five-target candidate acceptance requires a separate authorization.
