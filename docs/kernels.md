# The kernels

Each kernel's entry in `kernels.json` says how to build and run it and where its implementations
live; [SPEC.md](../SPEC.md#kernelsjson) describes the fields. The `name` there (`occt`,
`solidcraft`, `aetheris`, …) is the argument to `harness/run.py --kernel`.

| kernel | its implementations |
|---|---|
| [Open CASCADE](https://github.com/Open-Cascade-SAS/OCCT) | `kernels/occt` (Python, through OCP); the reference, whose results the drawings are made from where it passes |
| [arris](https://github.com/arris-labs/arris) | `kernels/arris` (Rust) |
| [cad-kernel](https://crates.io/crates/cad-kernel) | `kernels/cad-kernel` (Rust, on a release pinned in its `Cargo.toml`) |
| [vcad](https://github.com/ecto/vcad) | `kernels/vcad` (Rust); measured from its mesh |
| [popkern](https://github.com/popkern-dev/popkern) | `kernels/popkern` (Rhai, popkern's scripts, run by a Rust runner) |
| [SolidCraft](https://solidcraft.io) | `kernels/solidcraft` (Rust, on SolidCraft's kernel crate, solidcraft-kernel, and, where that has no operation for a case, on solidcore-ops, the modelling layer of solidcore, SolidCraft's port of SMLib beneath it; both from SolidCraft's repository at a release tag pinned in its `Cargo.toml`) |
| [Aetheris](https://github.com/yuechen-li-dev/Aetheris) | `kernels/aetheris` (Firmament, Aetheris's modeling language, compiled by a C# runner, built from Aetheris's source at the `AetherisCommit` in its `.csproj`); measured from its mesh |
| [next.BREP](https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL) | `kernels/next.BREP` (Rust, on its kernel crate, `BREP_kernel` from crates.io, pinned in its `Cargo.toml`) |
| [kentron](https://github.com/rsz269/kentron) | `kernels/kentron` (Rust, on its crate, `kentron` from crates.io, pinned in its `Cargo.toml`) |
| [brepkit](https://github.com/andymai/brepkit) | `kernels/brepkit` (Rust, on the `brepkit-*` crates from crates.io, pinned in its `Cargo.toml`) |
| [geop](https://github.com/geop-cad/geop) | `kernels/geop` (Rust programs of geop's operations, on its crates from its repository at a release tag pinned in its `Cargo.toml`) |
| [cadrs](https://github.com/rvdende/cadrs) | `kernels/cadrs` (Rust, through the `Kernel` trait of cadrs's kernel crate, `cadrs_kernel`, from its repository at a commit pinned in its `Cargo.toml`) |
| [springrep](https://github.com/springcraft-ai/springrep) | `kernels/springrep` (Rust, on its crate, `springrep` from crates.io, pinned in its `Cargo.toml`) |
| [SMLib](https://github.com/NVIDIA-Omniverse/usd-brep) | `kernels/smlib` (Python, through NVIDIA's `usd-brep` package from PyPI, pinned in its `build`); no STEP, which the package cannot write |
| [geomsolver](https://github.com/bmander/geomsolver) | `kernels/geomsolver` (Solvent, geomsolver's declarative constraint language: each case a `.sv` document naming its solid `part`, compiled by geomsolver's own `solventc`, built from its source at the release tag `build.py` pins; its solids are built and written as STEP by the core's B-rep kernel, a newer cad-kernel than the one listed above) |
| [AnvilCAD](https://github.com/swperb/anvilcad) | `kernels/anvilcad` (Rust, on AnvilCAD's exact kernel, `anvil-brep`, and its STEP writer, `anvil-step`, from AnvilCAD's repository at a commit pinned in its `Cargo.toml`); measured from its mesh where it writes no STEP |
| [monstertruck](https://github.com/virtualritz/monstertruck) | listed, with no runner yet: wanted |

A kernel **measured from its mesh** has no exact measures of its own: the harness takes its volume
and area from the triangles it writes and grades them to 1e-3 rather than 1e-6.
