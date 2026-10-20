#!/bin/sh
# Offline Jarvis uninstaller. Removes installer-owned payloads only.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if ! command -v python3 >/dev/null 2>&1; then
    printf 'jarvis uninstaller: python3 is required\n' >&2
    exit 1
fi
exec python3 "$here/jarvis_package.py" uninstall "$@"
