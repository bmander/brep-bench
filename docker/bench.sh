#!/usr/bin/env bash
# Build every kernel's runner and run the suite, all in one container on this machine, so that
# every kernel is timed on the same hardware with the same toolchains (docker/Dockerfile).
#
#     docker/bench.sh                                   # every kernel
#     docker/bench.sh occt next.BREP                    # these
#     docker/bench.sh occt -- --one-core --timeout 120  # run.py's own options after --
#
# The repository is mounted at /work: each runner builds into it as it does outside Docker, and
# results/ and site/ are written there, as the user running this script. Cargo's and NuGet's
# downloads are kept between runs in $BREP_BENCH_CACHE (by default ~/.cache/brep-bench-docker).
# A kernel that fails to build or to run is reported and the others still run; the site is made
# again at the end (harness/site.py).
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
CACHE=${BREP_BENCH_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/brep-bench-docker}
IMAGE=${BREP_BENCH_IMAGE:-brep-bench}

kernels=()
while [[ $# -gt 0 && $1 != -- ]]; do kernels+=("$1"); shift; done
[[ ${1:-} == -- ]] && shift

mkdir -p "$CACHE"
docker build -t "$IMAGE" "$ROOT/docker"

tty=()
[[ -t 1 ]] && tty=(-it)
docker run --rm --init "${tty[@]}" \
    --user "$(id -u):$(id -g)" \
    -v "$ROOT:/work" -v "$CACHE:/cache" -w /work \
    -e HOME=/cache -e CARGO_HOME=/cache/cargo -e NUGET_PACKAGES=/cache/nuget \
    -e KERNELS="${kernels[*]}" \
    "$IMAGE" bash docker/inside.sh "$@"
