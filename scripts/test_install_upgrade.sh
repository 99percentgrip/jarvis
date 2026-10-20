#!/bin/sh
# Full offline Jarvis installer/uninstaller regression. No network and no home directory.
set -eu
repository=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
python3 "$repository/scripts/test_jarvis_package.py"
