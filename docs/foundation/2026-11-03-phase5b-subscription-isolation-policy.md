# Phase 5B provider subscription and test-isolation decision

## Objective

Adopt subscription-neutral acceptance without claiming that absent paid provider access proves a live authentication path. This records a documentation/policy decision; production and test repairs are recorded separately.

## Method and inspected scope

Read the repository and documentation DOX chain, `docs/runtime-identity-stage3-prd.md` requirements 4–5 and the Phase 5B integration paragraph, `docs/foundation/phase4-auth-persistence-coverage.json`, and the PR #29 exact-merge report. Checked the matrix's subscription row, hosted native-store row and malformed-vault row separately. Current `crates/jarvis-provider-openai/src/factory.rs` and `crates/jarvis-provider-xai/src/factory.rs` expose credential-present, store, logout and device-login ports (xAI also browser-login), so the historical Stage 3 deferral alone is not a current NOT SUPPORTED finding; this source observation does not prove a successful live account flow. No external provider was contacted; the supplied independent reference analysis was treated as untrusted guidance, not proof of current code or CI.

## Adopted acceptance rule

Subscription access and paid API billing are distinct; neither grants permission to call a provider during foundation tests. Mandatory CI and release gates must not require any paid provider subscription. Cloud providers need synthetic identities, deterministic offline responses and protocol-contract tests. LM Studio needs disposable local loopback fixtures or an explicitly available isolated local endpoint; an unavailable optional server is not a mandatory-gate failure. Maintain all supported provider adapters.

Negative-path PASS requires a fixture-specific refusal with bounded completion, correct classification, no crash/hang, secret exposure, unauthorized retries, reads of another provider's native credentials, cross-provider fallback or provider interference. Missing credentials, invalid or expired credentials, missing entitlement, rate limits and transport failures are different cases; a generic nonzero exit does not establish each one. Private test roots, synthetic identities, no user OS-manager reads and blocked external provider traffic are prerequisites. An actual unexpected leak, fallback, crash or unauthorized access remains FAIL and warrants red-first correction. Live positive sign-in that was not executed is **NOT TESTED**; **NOT SUPPORTED** describes genuinely absent product behavior, not a missing subscription. Do not infer live compatibility from offline tests or promote historical Stage 3 deferred-authorization language into a current-provider verdict.

The override is limited to Phase 5B's ambiguous live-positive subscription requirement. Platform-native storage/locked/missing-service behavior, two-host consumer boundaries, restart, rotation, sign-out, provider isolation, packaging and protected CI are still required where applicable. Offline negative-path acceptance is not overall Phase 5B completion.

## Files and evidence

- `AGENTS.md` owns the global, durable subscription/isolation rule.
- `docs/runtime-identity-stage3-prd.md` owns the Phase 5B-specific override and exclusions.
- `docs/foundation/phase4-auth-persistence-coverage.json` classifies the live positive path separately from offline negative coverage and retains exact historical receipts; an existing Stage 3 inactivity note is historical, not proof that currently registered provider authentication is unsupported.
- `docs/AGENTS.md`, `docs/foundation/AGENTS.md`, and `docs/foundation/evidence-index.md` own this decision and its links.

Baseline receipt: PR #29's exact-main integration and named native corruption tests remain recorded in [the integration report](2026-11-01-phase5b-credential-reliability.md) and [receipt](phase5b-pr29-integration-receipt.json). Neither supplies live subscription acceptance. Verification of the present prose and JSON is reported below; no previously passing test suite is re-run for a policy-only update.

## Verification receipt

`python3` parsed the seven-row coverage JSON, checked that the live-positive row says `NOT TESTED`, verified the changed documents' relevant local links and checked their whitespace; all passed. `git diff --check` exited 0. `cargo xtask naming-guard` returned `naming-guard: clean (36 hits, all frozen in baseline)`. These checks establish document integrity, not execution of provider acceptance, native CI or release readiness.

## Deviations, unresolved items and readiness

No CI configuration, provider production behavior or tests changed, and no newly isolated negative-path test was executed here. The existing local ACP CLI refusal probes in the integration report assert nonzero status, empty stdout and unchanged vault bytes; they **do not** establish specific classification, cross-provider native-read absence or network blocking and are not promoted to complete negative-path PASS. Z.ai/xAI live positive authentication, OpenAI live subscription/API outcomes, macOS/Windows native unavailable-service negatives and full two-host consumer acceptance remain **NOT TESTED** at this scope. A subscription gap alone is not a software FAIL. This decision improves acceptance truthfulness but does not complete Phase 5B or authorize a release, tag, installer, additional merge or a documentation-only push.
