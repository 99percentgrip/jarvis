#!/bin/sh
# Offline Jarvis installer. Requires an explicit local archive and checksum.
# There is no public release download.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if ! command -v python3 >/dev/null 2>&1; then
    printf 'jarvis installer: python3 is required to validate an offline package\n' >&2
    exit 1
fi
exec python3 "$here/jarvis_package.py" install "$@"
