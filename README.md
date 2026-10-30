# brep-bench

Rosetta Code for B-rep (boundary representation) solid-modelling and CAD kernels. Each case
describes a solid in plain words and gives its volume and area as closed forms. Each kernel builds
every case in its own language and idioms, and every result is graded the same way. The results
are published side by side: **https://bmander.github.io/brep-bench/**

Each case has its own page with:

- the description;
- a drawing of the expected solid (three views and an isometric);
- a 3D viewer of every kernel's result, with any open edges of its mesh marked in red;
- each result's STL to download;
- every kernel's code for it, side by side.

## Why

People keep writing B-rep kernels from scratch, in Rust especially, many of them with an AI agent's
help. Each one invents its own tests, so none can show how it compares with the others. brep-bench
is the shared yardstick: one set of hard cases, one way of grading them, and every kernel in one
table. Pooling effort begins with knowing where each kernel stands.

There is no case language to translate: a case is a description, and each kernel's implementation
is written the way that kernel's users write. Reading one case across the kernels shows how each
one is used, and what each one cannot yet do.

## What it checks

The first 33 cases are textbook solids:

- **Primitives:** box, cylinder, cone, sphere, torus, extrusion, revolution.
- **Booleans:** coincident and partly coincident faces, tangencies, Steinmetz solids, spheres, tori,
  and a run of nine cuts.
- **Sweeps:** a circle along helices and arcs.
- **Fillets:** one edge, every edge of a box (three meeting at each corner), and a cylinder's rim.

Then come harder ones: NURBS surfaces in Booleans, tangent and near-tangent contact, fillets and
chamfers that consume faces, splines, and **solid sweeps**: everything a tool passes through as it
moves, from a ball swung through a block (a closed form) to a wheel screwed along a rod, a rack
rolled round a gear blank and a torus tumbling through space. Their walls are envelopes; only
cad-kernel, with its own solid sweep, builds them yet, so they wait for a second kernel to agree.

Then come real parts, whose scenarios are taken from the example documents of
[geomsolver](https://github.com/bmander/geomsolver) and written out in plain words: a flange, a
pulley, a tray, a star handle, a turned shaft, ducts swept along a bend. Most have closed forms;
where one has none, its answer is accepted only when two kernels, each built independently from the
case's words alone, agree. That agreement also shows the words describe one solid.

A case **passes** when three things hold:

- the kernel's volume and area are within 1e-6 of the closed forms;
- the triangles it writes are watertight and consistently oriented;
- the triangles bound the expected volume.

A kernel that **refuses** a case ranks above one that answers it **wrong**. A case with no
implementation yet is **missing**, which says nothing about the kernel. The full rules are in
[SPEC.md](SPEC.md).

## Layout

```
cases/<category>/<name>.md         a case: its closed forms (TOML header) and its description
kernels.json                       the kernels: how to build and run each, where its cases live
kernels/<kernel>/                  a kernel's runner and its implementation of each case
harness/                           build, run and grade, draw, and render the site
results/<kernel>.json              the last whole run
```

## Running it

```bash
python3 harness/build.py           # every kernel's runner (cargo, pip)
python3 harness/run.py             # every kernel, every case: results/*.json
python3 harness/draw.py            # each case's drawing: site/drawings/
python3 harness/site.py            # the scoreboard and a page per case: site/
```

`harness/run.py --kernel NAME --case boolean/` runs part of the suite. The harness uses only
Python's standard library (3.11 or later).

On GitHub, a push to `main` runs only what it changed (`harness/changed.py` decides: a case, a
kernel's implementation of one, or a kernel's runner), on top of the last run's results; a push
that changes no case skips the bench, building nothing. The site is published after every push.
The whole suite runs nightly, and when the harness's running or grading changes.

## Kernels

| kernel | its implementations |
|---|---|
| [Open CASCADE](https://github.com/Open-Cascade-SAS/OCCT) | `kernels/occt` (Python, through OCP), the reference |
| [arris](https://github.com/arris-labs/arris) | `kernels/arris` (Rust) |
| [cad-kernel](https://github.com/bmander/cad-kernel) | `kernels/cad-kernel` (Rust) |
| [truck](https://github.com/ricosjp/truck) | `kernels/truck` (Rust); measured from its mesh; frozen at the first 46 cases |
| [vcad](https://github.com/ecto/vcad) | `kernels/vcad` (Rust); measured from its mesh |
| [monstertruck](https://github.com/virtualritz/monstertruck), [kentron](https://github.com/rsz269/kentron), [brepkit](https://github.com/andymai/brepkit), [popkern](https://github.com/popkern-dev/popkern), [next.BREP](https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL) | wanted |

## Contributing

- **A kernel:** add `kernels/<kernel>/` with a runner (it is given a case id and an output path,
  and prints one line of JSON; see [SPEC.md](SPEC.md#a-runner)) and an implementation of as many
  cases as you like, then add the kernel to `kernels.json`. Unwritten cases show as missing. If
  your kernel cannot do something, write the case anyway and refuse with the reason: that is an
  honest result.
- **A case:** add `cases/<category>/<name>.md` with its closed forms and a description that two
  careful readers would build the same way. Hard cases are the most valuable, especially ones
  your own kernel gets wrong.
- **A better implementation:** welcome when it is more idiomatic, or when it builds what the
  description asks where the old one refused.

## Licence

The harness, the cases and the implementations written here are MIT OR Apache-2.0, at your option.
Each kernel keeps its own licence. An implementation only calls its kernel; it does not include it.
