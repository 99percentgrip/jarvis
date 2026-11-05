#!/bin/sh
# Runtime-stage APT only: authenticated, timestamp-fixed Debian snapshot.
# Alternate Debian-owned frontend serves the SAME snapshot, not a newer repo.
set -eu
: "${HEADLESS_VERSION:?the Dockerfile must supply the exact pinned package version}"
SNAPSHOT_SOURCES=${SNAPSHOT_SOURCES:-/etc/apt/sources.list.d/vesper-snapshot.list}
transient_only=1

snapshot_apt() {
    phase=$1
    shift
    attempt=1
    while :; do
        log=$(mktemp)
        status=0
        timeout -k 5s 360s apt-get \
            -o Acquire::Retries=2 \
            -o Acquire::http::Timeout=30 \
            -o Acquire::https::Timeout=30 \
            -o APT::Update::Error-Mode=any \
            "$@" > "$log" 2>&1 || status=$?
        if [ "$status" -eq 0 ]; then
            cat "$log"
            rm -f "$log"
            return 0
        fi
        cat "$log" >&2
        # Only 503/504 or a bounded transport timeout may rotate frontends.
        # Download-only keeps interrupted maintainer scripts out of retry scope.
        # Missing, unsigned or corrupt archives fail immediately.
        if { [ "$status" -ne 124 ] && [ "$status" -ne 137 ] &&
             ! grep -Eq '^E: Failed to fetch .* (503|504) ' "$log"; } ||
            { grep -E '^E:' "$log" | grep -Evq '^E: (Failed to fetch .* (503|504) |Unable to fetch some archives|Some index files failed to download)'; } ||
            grep -Eiq 'Hash Sum mismatch|NO_PUBKEY|BADSIG|EXPKEYSIG|not signed|size mismatch' "$log"; then
            transient_only=0
            rm -f "$log"
            echo "snapshot APT $phase failed without exclusively transient fetch errors" >&2
            return 1
        fi
        rm -f "$log"
        if [ "$attempt" -ge 2 ]; then
            echo "snapshot APT $phase unavailable after two bounded attempts" >&2
            return 1
        fi
        echo "snapshot APT $phase failed; retrying the same pinned sources" >&2
        sleep 5
        attempt=$((attempt + 1))
    done
}

# Download, hash-verify and install separately: an APT package-script failure
# is never retried or interpreted as a reason to rotate repositories.
install_snapshot() {
    snapshot_apt update update && snapshot_apt download install -y --download-only --no-install-recommends \
        ca-certificates "chromium-headless-shell=$HEADLESS_VERSION" libcap-ng0 procps tini
}

if ! install_snapshot; then
    if [ "$transient_only" -ne 1 ] ||
        ! grep -q 'snapshot.debian.org/archive/debian/' "$SNAPSHOT_SOURCES" ||
        ! grep -q 'snapshot.debian.org/archive/debian-security/' "$SNAPSHOT_SOURCES"; then
        exit 1
    fi
    echo "snapshot primary unavailable; trying Debian's alternate snapshot frontend" >&2
    sed 's@://snapshot.debian.org/archive/@://snapshot-cloudflare.debian.org/archive/@g' \
        "$SNAPSHOT_SOURCES" > "${SNAPSHOT_SOURCES}.new"
    mv "${SNAPSHOT_SOURCES}.new" "$SNAPSHOT_SOURCES"
    sleep 10
    install_snapshot
fi
# APT accepts only the archives acquired against the signed snapshot indexes.
# No network fetch or retry may conceal an integrity or installation failure.
timeout -k 5s 360s apt-get --no-download install -y --no-install-recommends \
    ca-certificates "chromium-headless-shell=$HEADLESS_VERSION" libcap-ng0 procps tini
