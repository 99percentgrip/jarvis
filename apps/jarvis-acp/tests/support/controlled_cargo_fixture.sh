#!/bin/sh
# Immutable executable; only the fixture-local path bindings are generated.
IFS= read -r receipts < "$0.receipts-path" || exit 2
IFS= read -r release < "$0.release-path" || exit 2
tag=gate
case " $* " in
  *" metadata "*) tag=metadata ;;
  *" check "*) tag=check ;;
esac
printf '%s|%s|%s|%s\n' "$tag" "${CARGO_BUILD_JOBS-}" "${RUST_TEST_THREADS-}" "${CARGO_TARGET_DIR-}" >> "$receipts"
if [ "$tag" = metadata ]; then
  printf '%s\n' '{"packages":[]}'
  exit 0
fi
if [ "$tag" = check ]; then
  exit 0
fi
while [ ! -e "$release" ]; do sleep 0.02; done
exit 7
