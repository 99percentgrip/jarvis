# Stage 1 identity architecture PRD

## Authorized scope

New pure identity module, reviewed mappings, explicit package/executable/import
ownership, five-platform path/distribution specification and future import policy.
No runtime activation, package rename, credential/session migration, controller
rewrite, installation, tag, release or Stage 2 work is authorized.

## Acceptance evidence map

All Rust cases live in the new identity crate's tests/contracts.rs. Architecture
also enforces the registry against real Cargo metadata, so missing enrollment fails
ordinary verification. Full CI remains required for the exact PR head.

| Requirement | Current verification |
|---|---|
| Product identity consistency | acp_repository_provenance_and_release_ownership_agree_with_real_source |
| Every current package accounted for | actual_cargo_packages_and_executables_are_registered; cargo xtask architecture |
| Package/executable/import targets collision-free | package_executable_and_import_targets_are_collision_free |
| All five default roots separate | all_five_planned_platform_roots_are_disjoint_from_actual_legacy_defaults |
| No duplicate environment target | env_targets_are_unique_and_classified_without_automatic_fallback; active_source_and_shipped_skill_environment_inputs_are_all_classified |
| Serialized/credential identities sensitive | sensitive_data_is_not_mechanically_renamed_and_preserved_names_have_no_target |
| ACP/registry identity consistent | acp_repository_provenance_and_release_ownership_agree_with_real_source |
| RRC ownership independent | same ownership case; import policy compile-time assertions |
| Historical/third-party names preserved | sensitive-data case; declared_legacy_contracts_have_current_source_evidence; naming-scope regressions |
| New unregistered package fails | newly_added_packages_binaries_or_runtime_dependencies_fail_verification, using mutated actual metadata |

## Status and evidence

LOCAL_VERIFIED / FRESH CI PENDING. The owner authorized correction of the four
production TLS advisories and the proven voice completion race. Root rustls is
0.23.46 with no unrelated dependency upgrades; security policies and voice fixture
assertions remain unchanged. Deterministic registration-window proof is red on
baseline ordering and green after correction. PR 3 remains unmerged until all
thirteen exact-head checks pass. Existing identity tests and full verification
remain mandatory; no Stage 2 or release is authorized.

[Architecture inventory](architecture/identity-architecture.md) ·
[Execution report](foundation/2026-10-14-identity-stage1.md).
