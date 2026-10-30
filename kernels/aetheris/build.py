#!/usr/bin/env python3
"""Build the brep-bench runner for Aetheris.

    python3 kernels/aetheris/build.py

Aetheris's libraries are built from its source, at the commit `AetherisCommit` in
brep-bench-aetheris.csproj names: only the projects the Firmament compiler needs are checked out,
into kernels/aetheris/upstream/. Then the runner is built against them. It needs the .NET 10 SDK.
Standard library only.
"""
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
UPSTREAM = os.path.join(HERE, "upstream")
REPOSITORY = "https://github.com/yuechen-li-dev/Aetheris"
# the Firmament compiler and the projects it references, with the files they read from the root
PATHS = ["/global.json", "/Directory.Build.props", "/LICENSE", "/docs/public/fields.md",
         "/Aetheris.Kernel.Firmament/", "/Aetheris.Kernel.Core/", "/Aetheris.Kernel.StandardLibrary/",
         "/Aetheris.Cloth3D/", "/Aetheris.Collaboration/", "/Aetheris.Continuum/",
         "/Aetheris.Fields/", "/Aetheris.Firmament.SchemaGenerator/", "/Aetheris.Forge/",
         "/Aetheris.Geometry/", "/Aetheris.Humanoid/", "/Aetheris.Modules/",
         "/Aetheris.Semantics/", "/Aetheris.Surfacing/"]


def git(*args):
    subprocess.run(["git", "-C", UPSTREAM, *args], check=True)


def main():
    with open(os.path.join(HERE, "brep-bench-aetheris.csproj")) as f:
        commit = re.search(r"<AetherisCommit>(\w+)</AetherisCommit>", f.read()).group(1)
    have = subprocess.run(["git", "-C", UPSTREAM, "rev-parse", "HEAD"], capture_output=True,
                          text=True).stdout.strip() if os.path.isdir(os.path.join(UPSTREAM, ".git")) else ""
    if have != commit:
        os.makedirs(UPSTREAM, exist_ok=True)
        if not have:
            git("init", "-q")
            git("sparse-checkout", "set", "--no-cone", *PATHS)
        git("fetch", "-q", "--depth", "1", "--filter=blob:none", REPOSITORY, commit)
        git("checkout", "-q", "--detach", "FETCH_HEAD")
    build = ["dotnet", "build", "-c", "Release", "--nologo", "-v", "quiet", "-clp:ErrorsOnly",
             os.path.join(HERE, "brep-bench-aetheris.csproj")]
    sys.exit(subprocess.run(build).returncode)


if __name__ == "__main__":
    main()
