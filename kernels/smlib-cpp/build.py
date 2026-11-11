#!/usr/bin/env python3
"""Build the brep-bench runner for SMLib through its C++ API, SM_API.

    python3 kernels/smlib-cpp/build.py

usd-brep publishes SMLib and SM_API as source and, built, in its Python wheel. The runner is
compiled against the headers of the release named below, checked out sparsely into
kernels/smlib-cpp/upstream/usd-brep/, and linked against the two shared libraries of the same
release's wheel from PyPI, unpacked into kernels/smlib-cpp/upstream/lib/: the wheel is a zip
archive, so no Python 3.12 or pip is needed to read it. Building SMLib from source needs NVIDIA's
own build tooling; the wheel's libraries are the ones the Python runner (kernels/smlib) calls.
Needs git, CMake and a C++17 compiler, on Linux (x86_64 or aarch64). Standard library only.
"""
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import urllib.request
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
UPSTREAM = os.path.join(HERE, "upstream")
SOURCE = os.path.join(UPSTREAM, "usd-brep")
LIB = os.path.join(UPSTREAM, "lib")
BUILD = os.path.join(HERE, "build")

VERSION = "1.0.1"
REPOSITORY = "https://github.com/NVIDIA-Omniverse/usd-brep"
COMMIT = "3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5"   # the tag v1.0.1
# the public headers of SM_API and of the kernel under it, whose types SM_API's take and return
PATHS = ["/LICENSE", "/source/SM_API/inc/", "/source/SMLib/inc/"]
LIBRARIES = ["libSM_API.so", "libSMLib.so"]


def git(*args):
    subprocess.run(["git", "-C", SOURCE, *args], check=True)


def headers():
    have = ""
    if os.path.isdir(os.path.join(SOURCE, ".git")):
        have = subprocess.run(["git", "-C", SOURCE, "rev-parse", "HEAD"], capture_output=True,
                              text=True).stdout.strip()
    if have == COMMIT:
        return
    os.makedirs(SOURCE, exist_ok=True)
    if not have:
        git("init", "-q")
        git("sparse-checkout", "set", "--no-cone", *PATHS)
    git("fetch", "-q", "--depth", "1", "--filter=blob:none", REPOSITORY, COMMIT)
    git("checkout", "-q", "--detach", "FETCH_HEAD")


def libraries():
    stamp = os.path.join(LIB, "VERSION")
    if os.path.exists(stamp) and open(stamp).read().strip() == VERSION:
        return
    if sys.platform != "linux":
        sys.exit("the SMLib C++ runner builds on Linux only")
    tag = f"manylinux_2_34_{platform.machine()}"
    with urllib.request.urlopen(f"https://pypi.org/pypi/usd-brep/{VERSION}/json") as reply:
        files = json.load(reply)["urls"]
    wheels = [f for f in files if f["filename"].endswith(f"{tag}.whl")]
    if not wheels:
        sys.exit(f"usd-brep {VERSION} has no wheel for {tag}")
    wheel = wheels[0]
    os.makedirs(LIB, exist_ok=True)
    archive = os.path.join(UPSTREAM, wheel["filename"])
    print(f"downloading {wheel['filename']}", flush=True)
    with urllib.request.urlopen(wheel["url"]) as reply, open(archive, "wb") as f:
        shutil.copyfileobj(reply, f)
    with open(archive, "rb") as f:
        if hashlib.sha256(f.read()).hexdigest() != wheel["digests"]["sha256"]:
            sys.exit(f"{wheel['filename']} does not match its sha256 on PyPI")
    with zipfile.ZipFile(archive) as z:
        for name in LIBRARIES:
            with z.open(f"usd_brep/{name}") as src, open(os.path.join(LIB, name), "wb") as dst:
                shutil.copyfileobj(src, dst)
    os.remove(archive)
    with open(stamp, "w") as f:
        f.write(VERSION + "\n")


def main():
    headers()
    libraries()
    configure = ["cmake", "-S", HERE, "-B", BUILD, "-DCMAKE_BUILD_TYPE=Release",
                 f"-DUSD_BREP_VERSION={VERSION}"]
    if subprocess.run(configure).returncode != 0:
        sys.exit(1)
    sys.exit(subprocess.run(["cmake", "--build", BUILD, "--parallel"]).returncode)


if __name__ == "__main__":
    main()
