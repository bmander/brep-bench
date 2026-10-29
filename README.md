# brep-bench

A neutral conformance suite and scoreboard for B-rep (boundary representation) solid-modelling and
CAD kernels. Every kernel runs the same cases. Each expected answer is a closed form, so it can be
checked without trusting any kernel. The results are published side by side:
**https://bmander.github.io/brep-bench/**

## Why

People keep writing B-rep kernels from scratch, in Rust especially, many of them with an AI agent's
help. Each one invents its own tests, so none can show how it compares with the others. brep-bench
is the shared yardstick: one set of hard cases, one way of grading them, and every kernel in one
table. Pooling effort begins with knowing where each kernel stands.

## What it checks

There are 33 cases in version 0. Each one gives a volume and an area as closed forms:

- **Primitives:** box, cylinder, cone, sphere, torus, extrusion, revolution.
- **Booleans:** coincident and partly coincident faces, tangencies, Steinmetz solids, spheres, tori,
  and a run of nine cuts.
- **Sweeps:** a disc along helices and arcs.
- **Fillets:** one edge, every edge of a box (three meeting at each corner), and a cylinder's rim.

An adapter returns the kernel's exact volume and area, and the triangles of its solid. A case
**passes** when three things hold:

- the measures are within 1e-6 of the closed form;
- the triangles are watertight and consistently oriented;
- the triangles bound the expected volume.

A kernel that **refuses** a case ranks above one that answers it **wrong**. The full rules are in
[SPEC.md](SPEC.md).

## Running it

```bash
python3 cases/generate.py                       # write the case files
python3 harness/build.py                        # every adapter's build (cargo, pip)
python3 harness/run.py                          # every adapter, every case: results/*.json
python3 harness/site.py                         # site/index.html
```

`harness/run.py --adapter NAME --case boolean/` runs part of the suite. The harness uses only
Python's standard library.

## Kernels

| kernel | adapter |
|---|---|
| [cad-kernel](https://github.com/bmander/cad-kernel) | `adapters/cad-kernel` (Rust) |
| [Open CASCADE](https://github.com/Open-Cascade-SAS/OCCT) | `adapters/occt` (Python, through OCP), the reference row |
| [truck](https://github.com/ricosjp/truck), [monstertruck](https://github.com/virtualritz/monstertruck), [arris](https://github.com/arris-labs/arris), [kentron](https://github.com/rsz269/kentron), [vcad](https://github.com/ecto/vcad), [brepkit](https://github.com/andymai/brepkit), [popkern](https://github.com/popkern-dev/popkern), [next.BREP](https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL) | wanted |

**Writing an adapter** is a small job. Write a program that reads a case, builds it with your
kernel, writes an STL, and prints one line of JSON (see [SPEC.md](SPEC.md#an-adapter)). Then add it
to `adapters.json` and open a pull request. If your kernel cannot do something, refuse it with a
reason: that is an honest result.

**Adding a case** means adding it to `cases/generate.py` with its closed form. Hard cases are the
most valuable, especially ones your own kernel gets wrong.

## Licence

The harness, the cases and the adapters written here are MIT OR Apache-2.0, at your option. Each
kernel keeps its own licence. An adapter only calls its kernel; it does not include it.
