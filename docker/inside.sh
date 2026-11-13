#!/usr/bin/env bash
# What docker/bench.sh runs in the container: build the kernels named in $KERNELS (every kernel
# with a runner where it is empty), run each one's cases (run.py, given this script's arguments),
# and make the site again. A kernel that fails to build or to run is reported at the end.
set -uo pipefail

read -ra kernels <<< "${KERNELS:-}"
if [[ ${#kernels[@]} -eq 0 ]]; then
    mapfile -t kernels < <(python3 -c '
import json
for k in json.load(open("kernels.json"))["kernels"]:
    if k.get("command"):
        print(k["name"])')
fi

failed=()
for k in "${kernels[@]}"; do
    # build.py says which failed rather than exiting non-zero
    if python3 harness/build.py "$k" 2>&1 | tee /dev/stderr | grep -q '^failed to build'; then
        failed+=("$k (build)")
        continue
    fi
    python3 harness/run.py --kernel "$k" "$@" || failed+=("$k (run)")
done
python3 harness/site.py
if [[ ${#failed[@]} -gt 0 ]]; then
    echo "failed: ${failed[*]}"
    exit 1
fi
