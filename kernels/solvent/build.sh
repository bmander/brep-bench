#!/bin/sh
# Build solventc into kernels/solvent/bin from geomsolver's source at GEOMSOLVER_REF.  The exact
# area of a reported solid (geomsolver#703) is not on main until that pull request merges, so the
# default is its branch; set GEOMSOLVER_REF=main once it has.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
ref=${GEOMSOLVER_REF:-claude/optimistic-hypatia-brk44b}
src=$here/geomsolver
if [ -d "$src/.git" ]; then
  git -C "$src" fetch --depth 1 origin "$ref" && git -C "$src" checkout -q FETCH_HEAD
else
  git clone --depth 1 --branch "$ref" https://github.com/bmander/geomsolver "$src"
fi
make -C "$src" solventc
mkdir -p "$here/bin"
cp "$src/build/solventc" "$here/bin/solventc"
git -C "$src" rev-parse HEAD > "$here/bin/COMMIT"
