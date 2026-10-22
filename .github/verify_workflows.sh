#!/usr/bin/env bash
# Fail closed on malformed GitHub Actions YAML, including disabled workflows.
set -euo pipefail

version=1.7.12
archive="actionlint_${version}_linux_amd64.tar.gz"
digest=8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8
tmp="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/workflow-lint.XXXXXXXX")"
trap 'rm -rf "$tmp"' EXIT
curl --fail --location --show-error --silent --retry 2 \
  "https://github.com/rhysd/actionlint/releases/download/v${version}/${archive}" \
  --output "$tmp/$archive"
printf '%s  %s\n' "$digest" "$tmp/$archive" | sha256sum --check --status
tar -xzf "$tmp/$archive" -C "$tmp" actionlint
mapfile -d '' -t workflows < <(find .github/workflows -maxdepth 1 -type f \
  \( -name '*.yml' -o -name '*.yaml' \) -print0 | sort -z)
((${#workflows[@]} > 0))
"$tmp/actionlint" "${workflows[@]}"
printf 'actionlint %s: validated %s workflows\n' "$version" "${#workflows[@]}"
