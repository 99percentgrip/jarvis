#!/bin/sh
# Only the browser image's signed, immutable Debian snapshot acquisition.
# Each phase has at most two attempts; exhaustion remains a build failure.
set -eu
: "${HEADLESS_VERSION:?the Dockerfile must supply the exact pinned package version}"

snapshot_apt() {
    phase=$1
    shift
    attempt=1
    while :; do
        if timeout -k 5s 360s apt-get \
            -o Acquire::Retries=2 \
            -o Acquire::http::Timeout=30 \
            -o Acquire::https::Timeout=30 \
            -o APT::Update::Error-Mode=any \
            "$@"; then
            return 0
        fi
        if [ "$attempt" -ge 2 ]; then
            echo "snapshot APT $phase unavailable after two bounded attempts" >&2
            return 1
        fi
        echo "snapshot APT $phase failed; retrying the same pinned sources" >&2
        sleep 5
        attempt=$((attempt + 1))
    done
}

snapshot_apt update update
snapshot_apt install install -y --no-install-recommends \
    ca-certificates "chromium-headless-shell=$HEADLESS_VERSION" libcap-ng0 procps tini
