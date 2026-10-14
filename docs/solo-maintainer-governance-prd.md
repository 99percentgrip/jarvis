# J.A.R.V.I.S. solo-maintainer governance

## Objective and scope

Owner approved Option B: PR-only main updates, zero required independent human
approvals, resolved threads, no main deletion/force-push and mandatory exact-head
CI/security verification. Scope is governance and CI bootstrap only; Rust identity
migration, releases, installations and Vesper changes are excluded.

## Required evidence

- Four inherited non-publishing workflows retain every MSRV, audit/deny, native
  five-target, installer and contained-driver check and documentation exclusion.
- PR jobs compile the actual PR head with persisted checkout credentials disabled.
- Twelve named jobs and the success-only complete-matrix verifier must pass on
  the same PR SHA. Current-attempt identities are read from GitHub, not fabricated.
- Main rules require these checks from the observed GitHub Actions app, PRs and
  resolved threads, with no bypass actors. Remove approval only after proof.
- Publishing/controlled RRC/spike workflows remain disabled and v* tags frozen.
- Vesper governance and local source state remain unchanged.

## Evidence and limitations

See [execution report](foundation/2026-10-14-solo-maintainer-governance.md).
CI_PENDING. Strict required job names mean prose-only PRs whose native workflows
are filtered cannot merge under this bootstrap; skips are never fabricated as
success. A future separately reviewed scope-aware merge gate may address that
without running program suites for prose. GitHub Actions app binding authenticates
the issuer, not an immutable workflow definition; the solo maintainer must review
changes to verification policy. No admin bypass is configured.
