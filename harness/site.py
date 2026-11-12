#!/usr/bin/env python3
"""Render the results as a static site (site/):

- board.json: every case and every kernel's outcome of it, in one file (its form is `board`'s);
- the board's pages, harness/web/ copied, the addresses of their scripts and of board.json
  versioned: the overview (index.html), the explorer of every case (explore.html) and a kernel's
  page (kernel.html?k=NAME), which read board.json;
- a page for each case (cases/<case>.html), written here: its description (Markdown with LaTeX
  math, harness/markdown.py), its closed forms, its drawing, a table of every kernel's entry at a
  glance (outcome, errors, time and lines of code), and the entry picked (#k=NAME in the address picks one): its result in 3D, its time, its notes,
  its measures and its STL file, and its implementation, the code highlighted
  (harness/highlight.py).

Kernels are listed in kernels.json's order, everywhere: the board ranks nothing. Run
harness/draw first for the drawings. Standard library only."""
import hashlib
import html
import json
import os
import re
import shutil
import statistics
import sys
import tomllib
import urllib.parse
from decimal import ROUND_HALF_UP, Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import step  # noqa: E402
from highlight import highlight, loc  # noqa: E402
import likeness  # noqa: E402
import markdown  # noqa: E402
from run import FROM_MESH, keep_file  # noqa: E402
from suite import ROOT, cases as read_cases, kernels as read_kernels, source  # noqa: E402
SITE = os.path.join(ROOT, "site")
REPO = "https://github.com/bmander/brep-bench"
WEB = os.path.join(ROOT, "harness", "web")
OUTCOMES = ["pass", "built", "refused", "wrong", "crashed", "timeout", "missing"]
# an outcome's letter in board.json
CODES = {"pass": "p", "built": "b", "refused": "r", "wrong": "w", "crashed": "c", "timeout": "t",
         "missing": "."}
THREE = "https://cdn.jsdelivr.net/npm/three@0.160.0"
# Open CASCADE for the browser, which meshes a STEP file's faces for the viewer
OCCT_JS = "https://cdn.jsdelivr.net/npm/occt-import-js@0.0.23/dist"
KATEX = "https://cdn.jsdelivr.net/npm/katex@0.16.11/dist"

def head(title, description, depth):
    up = "../" * depth
    return [
        "<!doctype html><html lang=en><head><meta charset=utf-8>",
        "<meta name=viewport content='width=device-width, initial-scale=1'>",
        f"<title>{html.escape(title)}</title>",
        f"<meta name=description content='{html.escape(description)}'>",
        f"<link rel=stylesheet href='{up}style.css'>",
        f"<link rel=icon href='{up}favicon.ico' sizes=32x32>",
        f"<link rel=icon href='{up}favicon.svg' type='image/svg+xml'>",
        f"<link rel=apple-touch-icon href='{up}apple-touch-icon.png'></head><body>",
        f"<nav class=top><a href='{up}index.html'>brep-bench</a>"
        f"<a href='{up}explore.html'>Cases</a><a href='{up}performance.html'>Performance</a>"
        f"<a href='{REPO}'>Source</a></nav>",
    ]


def load():
    registry = {k["name"]: k for k in read_kernels()}
    results = []
    rdir = os.path.join(ROOT, "results")
    for name in sorted(os.listdir(rdir)) if os.path.isdir(rdir) else []:
        if name.endswith(".json"):
            with open(os.path.join(rdir, name)) as f:
                r = json.load(f)
            r.setdefault("kernel", r.get("adapter"))
            # a kernel taken out of kernels.json is off the board, whatever results it left
            # (the Actions cache keeps them)
            if r["kernel"] not in registry:
                continue
            # each case's row by id, and what the kernel's implementations work around and which
            # are flagged elegant, read once
            r["rows"] = {c["case"]: c for c in r["cases"]}
            r["workarounds"] = flags(registry[r["kernel"]], WORKAROUNDS)
            r["elegance"] = flags(registry[r["kernel"]], ELEGANCE)
            results.append(r)
    # kernels.json's order: the board ranks nothing
    order = {name: i for i, name in enumerate(registry)}
    results.sort(key=lambda r: order[r["kernel"]])
    faceting(results)
    cases = {c["id"]: c for c in read_cases()}
    drawings = {}
    index = os.path.join(SITE, "drawings", "index.json")
    if os.path.exists(index):
        with open(index) as f:
            drawings = json.load(f)
    return registry, results, cases, drawings


# a kernel's flags of its implementations, beside its runner (SPEC.md): what each works around, and
# what makes each it calls elegant
WORKAROUNDS = "workarounds.toml"
ELEGANCE = "elegance.toml"


# A solid is marked faceted where it has at least FACETED_TIMES as many faces as the median of
# the other kernels' solids of the same case, and at least FACETED_LEAST faces: a mesh stored as
# B-rep faces, a flat face for every triangle or two, where the others bound the same solid by a
# few curved faces. Kernels split faces differently (a cylinder cut in two at its seam, a face
# split where a Boolean crossed it), but by a few times, not tenfold. The counts are the runners'
# own `faces` (SPEC.md), so the mark is the board's reading of them, not a grade.
FACETED_TIMES = 10
FACETED_LEAST = 50


def face_count(row):
    """How many faces a result's solid has: by its STEP file where it wrote one (harness/step.py),
    else by its runner's word; None where it built none or neither says."""
    if not row or row["outcome"] not in ("pass", "built", "wrong"):
        return None
    made = row.get("step") or {}
    if made.get("faces"):
        return made["faces"]
    faces = row.get("faces")
    # (by kind from a runner's reply; a count alone where the harness measured the STEP)
    return (sum(faces.values()) if isinstance(faces, dict) else faces) or None


def faceting(results):
    """Marks each result faceted or not: sets each kernel's `faceted`, by case id, to the median
    of the other kernels' face counts where its own solid is faceted (see FACETED_TIMES)."""
    by_case = {}
    for r in results:
        for cid, row in r["rows"].items():
            n = face_count(row)
            if n is not None:
                by_case.setdefault(cid, []).append((r["kernel"], n))
    for r in results:
        r["faceted"] = {}
        for cid, row in r["rows"].items():
            n = face_count(row)
            others = [m for k, m in by_case.get(cid, []) if k != r["kernel"]]
            if n is None or not others or n < FACETED_LEAST:
                continue
            median = statistics.median(others)
            if n >= FACETED_TIMES * median:
                r["faceted"][cid] = median


def flags(kernel, name):
    """`kernel`'s flags in its file `name` (WORKAROUNDS or ELEGANCE), by case id, or nothing."""
    path = os.path.join(ROOT, "kernels", kernel.get("name", ""), name)
    if not os.path.exists(path):
        return {}
    with open(path, "rb") as f:
        return tomllib.load(f)


def board(registry, results, cases, drawings):
    """Everything the board's pages show, as one JSON object:

    - `kernels`: every kernel in kernels.json, in its order, with what kernels.json says of it
      and, where it has results, its `version` and `run`;
    - `cases`: every case in id order, each `[id, title, [tag], settled, drawn]`, a tag by its
      index in `tags`, `settled` and `drawn` 1 or 0;
    - `results`: by kernel, its outcome of each case in `cases`' order, as a string of one letter
      a case (`codes`), with lists in the same order of what each run cost (null where it has no
      row, or the run was not measured so): its wall-clock `seconds`, its `cpu` seconds, its
      peak memory in megabytes (`mb`) and its wall-clock seconds on one core alone (`one`); and
      of its `why`, its `workaround` and its `elegance` (indices in `reasons`, `workarounds` and
      `elegance`, -1 for none),
      and the lines of code of its implementation of each case (`loc`, null where it has none),
      and the decimal places its times were kept to (`places`: 3, milliseconds, for a row from
      before run.py kept microseconds; null where it has no row), and how many faces its solid
      has by its runner's word (`faces`, null where it built none or did not say) and, where
      that solid is faceted, the median of the other kernels' (`faceted`, else null; see
      FACETED_TIMES); and, for a kernel whose runner writes STEP, its STEP verdict of each case
      as a string of one letter a case (`step`, the letters `stepVerdicts` keys, `-` where it
      built no solid) and why where its writer refused or the run failed (`stepWhy`, indices in
      `stepReasons`, -1 for none). A kernel's `step` is kernels.json's: true where its runner
      writes STEP, why not where its kernel cannot, absent where it has not been set up to.
      A kernel's `host` is the machine its last run was made on, and its `startup` what starting
      its runner cost there, building nothing (`seconds`, `cpu`, `mb`, and `one` on one core);
      its `binary` is the size of its binary (`bytes`, `files`), and `of` what kernels.json counts
      in it.
    """
    ids = sorted(cases)
    tags = sorted({t for c in cases.values() for t in c["tags"]})
    tag_ix = {t: i for i, t in enumerate(tags)}
    strings = {"reasons": {}, "workarounds": {}, "elegance": {}, "stepReasons": {}}

    def intern(table, text):
        if not text:
            return -1
        return strings[table].setdefault(text, len(strings[table]))

    kernels = []
    out = {}
    ran = {r["kernel"]: r for r in results}
    for name in registry:
        a = registry[name]
        k = {"name": name}
        for field, key in (("kernel", "home"), ("license", "license"), ("backend", "backend"),
                           ("frontend", "frontend"), ("note", "note"),
                           ("reference", "reference"), ("step", "step")):
            if a.get(field):
                k[key] = a[field]
        r = ran.get(name)
        if r:
            k["version"] = r.get("version") or ""
            k["run"] = r.get("run") or ""
            k["measured"] = r.get("measured") or a.get("measured") or "exact"
            if r.get("host"):
                k["host"] = r["host"]
            if r.get("binary"):
                k["binary"] = r["binary"]
            start = r.get("startup")
            if start:
                k["startup"] = {key: start.get(field) for key, field in COSTS.items()}
            codes, why, worked, neat, lines, places = [], [], [], [], [], []
            faces, faceted, steps, step_why = [], [], [], []
            spent = {key: [] for key in COSTS}
            for cid in ids:
                row = r["rows"].get(cid)
                if row is None:
                    codes.append(CODES["missing"])
                    for key in COSTS:
                        spent[key].append(None)
                    why.append(-1)
                    worked.append(-1)
                    neat.append(-1)
                    lines.append(None)
                    places.append(None)
                    faces.append(None)
                    faceted.append(None)
                    steps.append("-")
                    step_why.append(-1)
                    continue
                codes.append(CODES[row["outcome"]])
                # a missing case has no run, whatever an older row says of its time
                for key, field in COSTS.items():
                    spent[key].append(None if row["outcome"] == "missing" else row.get(field))
                why.append(intern("reasons", row.get("why")))
                worked.append(intern("workarounds", r["workarounds"].get(cid)))
                neat.append(intern("elegance", r["elegance"].get(cid)))
                lines.append(lines_of(a, cid))
                places.append(row.get("time_places", 3))
                faces.append(face_count(row))
                faceted.append(r["faceted"].get(cid))
                verdict = step_verdict(row, a.get("step"))
                steps.append(STEP_VERDICTS[verdict][0] if verdict else "-")
                made = row.get("step") or {}
                step_why.append(intern("stepReasons", made.get("refused") or made.get("error"))
                                if verdict in ("refused", "failed") else -1)
            out[name] = {"codes": "".join(codes), **spent, "why": why, "workaround": worked,
                         "elegance": neat, "loc": lines, "places": places, "faces": faces,
                         "faceted": faceted}
            if a.get("step") is True:
                out[name]["step"] = "".join(steps)
                out[name]["stepWhy"] = step_why
        elif not a.get("command"):
            k["awaiting"] = True
        kernels.append(k)
    return {
        "repo": REPO,
        "outcomes": {o: CODES[o] for o in OUTCOMES},
        "kernels": kernels,
        "tags": tags,
        "cases": [[cid, cases[cid]["title"], [tag_ix[t] for t in cases[cid]["tags"]],
                   int(cases[cid]["expect"] is not None), int(cid in drawings)] for cid in ids],
        "results": out,
        "reasons": list(strings["reasons"]),
        "workarounds": list(strings["workarounds"]),
        "elegance": list(strings["elegance"]),
        "stepReasons": list(strings["stepReasons"]),
        "stepVerdicts": {v[0]: k for k, v in STEP_VERDICTS.items()},
    }


def lines_of(kernel, cid):
    """The lines of code of `kernel`'s implementation of case `cid`, or None if it has none."""
    src = source(kernel, cid)
    if not src or not os.path.exists(src):
        return None
    with open(src, encoding="utf-8") as f:
        return loc(f.read(), src)


# what a run cost: each measure's name on the board, and the field of a result's row it is read from
COSTS = {"seconds": "seconds", "cpu": "cpu_seconds", "mb": "peak_mb", "one": "one_core_seconds"}


def half_up(x, places):
    """`x` to `places` decimal places, a tie rounded up, as JavaScript's toFixed, toPrecision and
    Math.round round (Python's round and format round a tie to the even digit)."""
    return Decimal(x).quantize(Decimal(1).scaleb(-places), rounding=ROUND_HALF_UP)


def seconds_text(x):
    """A time in seconds as the board's tables show it (board.js's secondsText): 0.42 ms, 3.7 ms,
    420 ms, 4.21 s, 12.3 s; under a tenth of a millisecond, <0.1 ms."""
    if x < 1e-4:
        return "<0.1 ms"
    if x < 0.01:
        ms = x * 1000  # to two figures, as toPrecision(2) and Number() write it
        return f"{half_up(ms, 2 if ms < 1 else 1).normalize():f} ms"
    if x < 1:
        return f"{half_up(x * 1000, 0)} ms"
    return f"{half_up(x, 2)} s" if x < 10 else f"{half_up(x, 1)} s"


def megabytes_text(x):
    return f"{x:g} MB"


def spent(en, field, text):
    """An entry's measure of its run for the case page, as `text` writes it: nothing where it has
    not run, a dash where it was not measured."""
    if en["outcome"] == "missing":
        return ""
    x = en["row"].get(field)
    return "<span class=muted>–</span>" if x is None else text(x)


def relative(got, want):
    if got is None:
        return ""
    return f"{abs(got - want) / max(abs(want), 1e-12):.1e}"


def stl_tris(path):
    """The triangles of a kept STL file (harness/stl.py)."""
    from stl import read
    return read(path)


def case_page(c, registry, results, drawing, prev=None, after=None, v=""):
    """`prev` and `after`: the cases before and after this one in its category, or None; `v`
    the version the board's scripts are addressed with (`web`'s)."""
    e = html.escape
    q = urllib.parse.quote
    cid = c["id"]
    category = cid.rpartition("/")[0]
    depth = cid.count("/") + 1
    up = "../" * depth
    out = head(f"{c['title']} · brep-bench", f"The brep-bench case {cid}: its description, its "
               "drawing, and each kernel's entry: its result in 3D, its measures and its code.",
               depth)
    steps = [f"<a href='{up}explore.html?cat={q(category)}'>{e(category)}</a>"]
    if prev:
        steps.append(f"<a href='{up}cases/{e(prev)}.html' rel=prev>← "
                     f"{e(prev.rpartition('/')[2])}</a>")
    if after:
        steps.append(f"<a href='{up}cases/{e(after)}.html' rel=next>"
                     f"{e(after.rpartition('/')[2])} →</a>")
    out.append(f"<p class=muted>{' · '.join(steps)}</p>")
    tags = ", ".join(f"<a href='{up}explore.html?tag={q(t)}'>{e(t)}</a>" for t in c["tags"])
    out.append(f"<h1>{e(c['title'])}</h1><p class=muted>{e(cid)} · {tags}</p>")
    # the description is Markdown, its math typeset by KaTeX like the closed forms (TEX)
    description = markdown.render(c["description"])
    out.append(f"<div class=description>{description}</div>")
    exp = c["expect"]
    # a reference shape (SPEC.md): each result's likeness to it, shown beside its lines
    ref_path, mesh_path = likeness.reference(c), likeness.reference_mesh(c)
    ref = likeness.triangles(mesh_path) if mesh_path else None
    if ref is not None:
        out.append("<p><strong>A reference shape.</strong> This case's solid is the one in "
                   f"<a href='{REPO}/blob/main/cases/{e(category)}/"
                   f"{e(os.path.basename(ref_path))}'>{e(os.path.basename(ref_path))}</a>, "
                   "and no kernel's solid is it: <em>built</em> means a watertight, consistently "
                   "oriented solid. Beside each is its <em>likeness</em>, the volume it holds in "
                   "common with the reference over the volume either holds (1 for the same "
                   "solid), and its lines of code: as like as it can be, in as few as it can.</p>")
    elif exp is None:
        out.append("<p><strong>Unsettled.</strong> This solid has no closed form, and no two kernels "
                   "yet agree on it to 1e-8, so it has no answer to grade against. Below is what each "
                   "kernel builds: <em>built</em> means a watertight, consistently oriented solid, "
                   "whatever its measures; the table says which kernels agree with which.</p>")
    else:
        out.append("<div class=scroll><table data-nosort><tr><th></th><th>expected</th>"
                   "<th>closed form</th></tr>")
        for key in ("volume", "area"):
            x = exp[key]
            # the closed form typeset by KaTeX, with the case's own words until it loads (and
            # where there is no closed form)
            form = (f"<td class=closed title='{e(x['exact'])}'><span class=tex "
                    f"data-tex='{e(x['tex'])}'>{e(x['exact'])}</span></td>" if x["tex"]
                    else f"<td class='muted why'>{e(x['exact'])}</td>")
            out.append(f"<tr><td>{key}</td><td>{x['value']:.10g}</td>{form}</tr>")
        out.append("</table></div>")


    if drawing:
        d = f"{up}drawings/{e(cid)}"
        sz = drawing["sizes"]
        def fig(view, label, cls=""):
            w, h = sz[view]
            if view == "iso":
                # shown as wide as the right view, so the sheet's columns line up
                w, h = sz["right"][0], round(h * sz["right"][0] / w)
            return (f"<figure class='{cls}'><img class=paper src='{d}/{view}.png' width={w} "
                    f"height={h} alt='{label} view'><figcaption>{label}</figcaption></figure>")
        out.append("<h2>Drawing</h2>")
        out.append("<div class=scroll><div class=sheet>" + fig("top", "Top", "up") +
                   fig("iso", "Isometric", "up") + fig("front", "Front") + fig("right", "Right") +
                   "</div></div>")
        out.append(f"<p class=muted>Third-angle projection, the three views at one scale. Drawn "
                   f"from {e(drawing['from'])}'s result, which passed.</p>")

    # an entry for each kernel with results, in kernels.json's order: its result, its triangles and
    # its code
    entries = []
    for r in results:
        name = r["kernel"]
        row = r["rows"].get(cid)
        stl = os.path.join(ROOT, "results", "meshes", name, cid + ".stl")
        href = mb = None
        if row and os.path.exists(stl):
            keep_file(stl, os.path.join(SITE, "meshes", name, cid + ".stl"))
            href = f"{up}meshes/{e(name)}/{e(cid)}.stl"
            mb = os.path.getsize(stl) / 1e6
        # its STEP file, what the viewer shows where the run kept it (run.py's STEP_KEPT); its
        # triangles are shown only where there is none, or it cannot be read
        step_file, step_href = stl[:-len(".stl")] + ".step", None
        if row and (row.get("step") or {}).get("faces") and os.path.exists(step_file):
            keep_file(step_file, os.path.join(SITE, "meshes", name, cid + ".step"))
            step_href = f"{up}meshes/{e(name)}/{e(cid)}.step"
        src = source(registry.get(name, {}), cid)
        code = None
        if src and os.path.exists(src):
            with open(src, encoding="utf-8") as f:
                code = f.read()
        o = row["outcome"] if row else "missing"
        wa = r["workarounds"].get(cid)
        neat = r["elegance"].get(cid)
        facets = r["faceted"].get(cid)
        entries.append({"name": name, "row": row, "outcome": o, "stl": href, "mb": mb,
                        "step": step_href, "writes": registry.get(name, {}).get("step"),
                        "wa": wa,
                        "neat": neat, "facets": facets, "src": src, "code": code,
                        "loc": None if code is None else loc(code, src),
                        "label": "–" if o == "missing" else
                                 o + (" †" if wa else "") + (" ✦" if neat else "")
                                 + (" ▦" if facets is not None else ""),
                        "measures": measures(row),
                        "likeness": likeness.likeness(stl_tris(stl), ref)["iou"]
                                    if ref is not None and href else None})
    # which other results each agrees with
    for en in entries:
        en["friends"] = [o["name"] for o in entries if o is not en
                         and agree(en["measures"], o["measures"])]
    # whether any entry has a solid to show: its STEP file, or else its triangles
    def solid(en):
        return bool(en["step"] or en["stl"])
    has_view = any(solid(en) for en in entries)

    out.append("<h2 id=results>Results</h2>")
    if not entries:
        out.append("<p class=muted>No kernel has run it yet.</p>")
    else:
        # the entry shown first: a pass with a solid to show, else any result with one, else
        # any result at all (the first such, in kernels.json's order); #k=NAME picks another
        shown = max(entries, key=lambda en: (en["outcome"] == "pass" and solid(en), solid(en),
                                             bool(en["row"])))

        # every entry at a glance, a column each; a click on any of its cells picks it
        def value(en, i):
            m = en["measures"]
            if m is None:
                return ""
            return relative(m[i], exp[KEYS[i]]["value"]) if exp else f"{m[i]:.6g}"
        # each row's head, its cells' tag, class and content, and the field of a result's row its
        # cells sort by (sort.js), where their text is in units
        rows = [
            ("<th></th>", "th", lambda en: "k", lambda en: e(en["name"]), None),
            ("<th scope=row>outcome</th>", "td", lambda en: f"cell {en['outcome']}",
             lambda en: e(en["label"]), None),
            ("<th scope=row title='relative error'>volume err.</th>" if exp
             else "<th scope=row>volume</th>", "td", None, lambda en: value(en, 0), None),
            ("<th scope=row title='relative error'>area err.</th>" if exp
             else "<th scope=row>area</th>", "td", None, lambda en: value(en, 1), None),
            ("<th scope=row>time</th>", "td", None, lambda en: spent(en, "seconds", seconds_text),
             "seconds"),
            ("<th scope=row title='user and system time, over every thread'>CPU time</th>", "td",
             None, lambda en: spent(en, "cpu_seconds", seconds_text), "cpu_seconds"),
            ("<th scope=row title='the most memory the runner held resident at once'>memory</th>",
             "td", None, lambda en: spent(en, "peak_mb", megabytes_text), "peak_mb"),
            ("<th scope=row title='lines of code, comments and blank lines left out'>LOC</th>",
             "td", None, lambda en: "" if en["loc"] is None else str(en["loc"]), None),
        ]
        if ref is not None:
            rows.insert(2, ("<th scope=row title='the volume in common with the reference over "
                            "the volume either holds'>likeness</th>", "td", None,
                            lambda en: "" if en["likeness"] is None else f"{en['likeness']:.4f}",
                            None))
        out.append(f"<div class=scroll><table class=entries data-sort=columns "
                   f"data-default='{e(shown['name'])}'>")
        def key(en, field):
            x = en["row"].get(field) if field and en["row"] else None
            return "" if x is None else f" data-v='{x!r}'"
        for head_cell, tag, cls, body, field in rows:
            out.append("<tr>" + head_cell + "".join(
                f"<{tag} class='{cls(en) if cls else ''}' data-k='{e(en['name'])}'"
                f"{key(en, field)}>"
                f"<a href='#k={e(q(en['name']))}'>{body(en)}</a></{tag}>" for en in entries) +
                "</tr>")
        out.append("</table></div>")
        neat = " ✦ an elegant solution." if any(en["neat"] for en in entries) else ""
        facets = (" ▦ a faceted solid: a mesh stored as B-rep faces." if
                  any(en["facets"] is not None for en in entries) else "")
        out.append(f"<p class=muted>† works around a missing feature.{neat}{facets} Pick an entry "
                   "to see its solid, its measures and its code.</p>")

        # the entry picked (the page's script shows it): its solid in 3D, its time, its notes,
        # its data and its code
        out.append("<div class=pick role=group aria-label='Whose entry'>" + "".join(
            f"<button type=button data-k='{e(en['name'])}'>{e(en['name'])} "
            f"<span class=muted>{e(en['label'])}</span></button>" for en in entries) + "</div>")
        if has_view:
            # what the viewer draws: the entry's B-rep, its STEP file's faces, each colored by its
            # surface, its edges drawn between them; only where it has no STEP file to show, or
            # the file cannot be read, its runner's triangles instead
            out.append("<div id=view hidden><canvas id=viewer></canvas>"
                       "<p class=legend id=kinds hidden></p>"
                       # a B-spline surface's control net, drawn over its faces at a click
                       "<p class=legend id=nets hidden><button type=button class=chip "
                       "title='Draw the control points of each B-spline "
                       "surface, joined in rows along u and along v, faint where the solid "
                       "hides them'>Control nets</button> <span></span></p>"
                       "<p class=muted id=faces-note hidden>Drag to turn, scroll or pinch to zoom, "
                       "right-drag to move. The solid as its STEP file has it, meshed in the "
                       "browser by Open CASCADE (occt-import-js): each face colored by the kind "
                       "of its surface, and the edges that bound each face drawn between "
                       "them.</p><p class=muted id=mesh-note hidden><span id=mesh-why></span> "
                       "Drag to turn, scroll or pinch to zoom, right-drag to move. Black lines "
                       "are where the surface turns sharply; red ones, seen through the solid, "
                       "are open edges, where the mesh is not closed.</p></div>")
        out += [entry(en, exp) for en in entries]
        out.append(PICK)
        # the table of entries sorted by a click on a row's head (the board's sort.js)
        out.append(f"<script type=module>import {{ sortable }} from '{up}sort.js{v}'; "
                   "sortable(document);</script>")

    out.append(f"<p class=muted><a href='{REPO}/blob/main/cases/{e(cid)}.md'>{e(cid)}.md</a>, "
               "the case as written.</p>")
    if has_view:
        out.append(VIEWER.replace("{cdn}", THREE).replace("{occt}", OCCT_JS))
    if (exp and any(exp[k]["tex"] for k in exp)) or "class='math" in description:
        out.append(TEX.replace("{cdn}", KATEX))
    out.append("</body></html>")
    path = os.path.join(SITE, "cases", cid + ".html")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write("\n".join(out))


KEYS = ("volume", "area")

# how a STEP file represents its solid (harness/step.py's `form`), in words
FORMS = {"advanced": "advanced faces, each on a surface",
         "faceted": "faceted B-rep: polygons, no surfaces",
         "tessellated": "triangles (AP242 tessellated), no B-rep",
         "surfaces": "faces on surfaces, but no solid",
         "none": "no solid and no faces"}


# a result's STEP verdict (harness/step.py's `verdict`, SPEC.md "STEP"): its letter in
# board.json, its word on the board, what it means, and the outcome whose colour its badge takes
# (as board.js's STEP_VERDICTS).
# `unrun` is a solid built before its kernel's runner was asked for STEP.
STEP_VERDICTS = {
    "brep": ("b", "B-rep", "a solid of advanced faces, each on a surface, its shell closed", "pass"),
    "faceted": ("f", "faceted", "a mesh stored as a B-rep: polygons, or faces that are mostly "
                "flat triangles", "built"),
    "triangles": ("t", "triangles", "tessellated triangles, and no B-rep", "built"),
    "open": ("o", "open", "faces that do not close into a solid", "wrong"),
    "empty": ("e", "empty", "no solid and no faces", "wrong"),
    "refused": ("r", "refused", "its kernel's STEP writer declined the solid", "refused"),
    "failed": ("x", "failed", "the runner failed to write STEP", "crashed"),
    "unrun": ("?", "not yet run", "built before its runner was asked for STEP", "missing"),
}
# outcomes of a run that built a solid, which a kernel that writes STEP is asked to write
BUILT = ("pass", "built", "wrong")


def step_verdict(row, writes):
    """A result's STEP verdict (a key of STEP_VERDICTS), given its kernel's `step` in
    kernels.json; None where it built no solid, or its kernel writes no STEP."""
    if writes is not True or not row or row["outcome"] not in BUILT:
        return None
    made = row.get("step")
    return step.verdict(made) if made else "unrun"


def step_kernel(writes):
    """What a kernel's `step` in kernels.json says of it, in words, for a solid it built: None
    for a kernel that writes STEP."""
    if writes is True:
        return None
    if writes:
        return f"cannot write STEP: {writes}"
    return "not set up: its runner has not been set up to write STEP"


def size_text(mb):
    return f"{mb:.1f} MB" if mb >= 0.1 else f"{mb * 1e3:.0f} kB"


def counted(counts):
    return ", ".join(f"{n} {k}" for k, n in counts.items())


def step_badge(verdict):
    _, word, means, colour = STEP_VERDICTS[verdict]
    return f"<span class='badge {colour}' title='{html.escape(means)}'>{word}</span>"


def step_facts(row, writes, faces):
    """The data rows for a result's STEP (harness/step.py): its verdict and what its file says
    the solid is made of, or why it has none (its kernel's writer refused, the runner failed, its
    kernel cannot write STEP or has not been set up to); `faces` its runner's own count, set
    beside the file's. None where it built no solid."""
    e = html.escape
    if not row or row["outcome"] not in BUILT:
        return []
    if writes is not True:
        return [("STEP", f"<span class='badge missing'>{'cannot' if writes else 'not set up'}"
                         f"</span> <span class=muted>{e(step_kernel(writes))}</span>")]
    verdict = step_verdict(row, writes)
    made = row.get("step") or {}
    if verdict in ("refused", "failed", "unrun"):
        why = made.get("refused") or made.get("error") or STEP_VERDICTS[verdict][2]
        return [("STEP", f"{step_badge(verdict)} <span class=muted>{e(why)}</span>")]
    form = f"{step_badge(verdict)} {e(FORMS.get(made['form'], made['form']))}"
    if made.get("schema"):
        form += f" <span class=muted>{e(made['schema'])}</span>"
    rows = [("STEP", form)]
    said = (sum(faces.values()) if isinstance(faces, dict) else faces) or None
    surfaces = f"{made['faces']}: {e(counted(made['surfaces']))}" if made["faces"] else "none"
    if made.get("triangles"):
        surfaces += f"; {made['triangles']} flat triangles"
    if said is not None and said != made["faces"]:
        surfaces += f" <span class=muted>(its runner counts {said})</span>"
    rows.append(("STEP faces", surfaces))
    if made["edges"]:
        edges = f"{made['edges']}: {e(counted(made['curves']))}"
        if made.get("unpaired"):
            edges += (f"; {made['unpaired']} not bounding exactly two sides of faces, so the "
                      "shell is open")
        rows.append(("STEP edges", edges))
    rows.append(("STEP topology", f"{made['solids']} solid{'' if made['solids'] == 1 else 's'}, "
                 f"{made['shells']} shell{'' if made['shells'] == 1 else 's'}, "
                 f"{made['faces']} faces, {made['loops']} loops, {made['edges']} edges, "
                 f"{made['vertices']} vertices"))
    if made.get("tessellated"):
        rows.append(("STEP triangles", f"{made['tessellated']} tessellated faces"))
    if made.get("unmeasured"):
        rows.append(("STEP measured", "<span class=muted>not by the harness, which graded the "
                     f"runner's own measures instead: {html.escape(made['unmeasured'])}</span>"))
    return rows


def measures(row):
    """A result's (volume, area, from its mesh), or None where it built no solid to measure."""
    if not row or row["outcome"] not in ("pass", "built", "wrong"):
        return None
    from_mesh = row.get("measured") == "mesh"
    m = row.get("mesh") or {}
    vol = m.get("volume") if from_mesh else row.get("volume")
    area = m.get("area") if from_mesh else row.get("area")
    return None if vol is None or area is None else (vol, area, from_mesh)


def agree(a, b):
    """Whether two results' measures agree: to 1e-8 between exact kernels, to FROM_MESH where
    either was measured from its mesh."""
    if a is None or b is None:
        return False
    tol = FROM_MESH if a[2] or b[2] else 1e-8
    return all(abs(x - y) <= tol * max(abs(x), abs(y), 1e-12) for x, y in zip(a[:2], b[:2]))


# what each outcome means, said where a result gives no reason of its own
MEANS = {
    "pass": "Its volume and area are the expected ones, and its mesh is closed.",
    "built": "A watertight, consistently oriented solid; the case has no settled answer yet to "
             "grade it against.",
    "refused": "The kernel declined to build it.",
    "wrong": "It built a solid, but not the expected one.",
    "crashed": "The runner crashed.",
    "timeout": "The runner ran out of time.",
    "missing": "No result: the kernel has no implementation of this case yet.",
}


def entry(en, exp):
    """One kernel's entry for a case, as HTML, hidden: the page's script shows the one picked.
    Its solid (its STEP file, else its triangles) is drawn by the page's one viewer, above it."""
    e = html.escape
    name, row, o = en["name"], en["row"], en["outcome"]
    out = [f"<section class=entry data-k='{e(name)}' data-src='{en['stl'] or ''}' "
           f"data-step='{en['step'] or ''}' hidden "
           f"aria-label='{e(name)}'>"]
    if not en["stl"] and not en["step"]:
        out.append(f"<div class='muted nomesh'>{e(name)} left no solid to show.</div>")

    out.append("<h3>Time</h3>")
    out.append(f"<p>{spent(en, 'seconds', seconds_text)}; CPU time "
               f"{spent(en, 'cpu_seconds', seconds_text)}; "
               f"peak memory {spent(en, 'peak_mb', megabytes_text)}</p>" if o != "missing"
               else "<p class=muted>It has not run.</p>")

    out.append("<h3>Notes</h3>")
    why = row.get("why") if row else ""
    out.append(f"<p><span class='badge {o}'>{o}</span> {e(why or MEANS[o])}</p>")
    if en["wa"]:
        out.append(f"<p><strong>Workaround †</strong> {e(en['wa'])}</p>")
    if en["neat"]:
        out.append(f"<p><strong>Elegant ✦</strong> {e(en['neat'])}</p>")
    if en["facets"] is not None:
        out.append(f"<p><strong>Faceted ▦</strong> {face_count(row)} faces, where the other "
                   f"kernels' solids have {en['facets']:g} (the median): a mesh stored as B-rep "
                   "faces.</p>")
    if row and row.get("measured") == "mesh":
        out.append("<p class=muted>It wrote no STEP, and has no exact measures: its volume and "
                   f"area are its triangles', graded to {FROM_MESH:g}.</p>")
    elif row and row.get("measured") == "step":
        out.append("<p class=muted>Its volume and area are its STEP file's, as the harness "
                   "measures them from the file's exact geometry (harness/mass.py); its triangles "
                   "are Open CASCADE's mesh of the file, for the pictures.</p>")

    out.append("<h3>Data</h3>")
    facts = []
    if en["measures"]:
        *got, from_mesh = en["measures"]
        whose = ("its triangles'" if from_mesh else
                 "its STEP's" if row.get("measured") == "step" else "measured")
        table = [f"<tr><th></th><th>{whose}</th>"
                 + ("<th>expected</th><th>rel. error</th>" if exp else "") + "</tr>"]
        for key, x in zip(KEYS, got):
            cells = f"<td>{x:.10g}</td>" if exp else f"<td>{x:.12g}</td>"
            if exp:
                want = exp[key]["value"]
                cells += f"<td>{want:.10g}</td><td>{relative(x, want)}</td>"
            table.append(f"<tr><td>{key}</td>{cells}</tr>")
        out.append("<div class=scroll><table data-nosort>" + "".join(table) + "</table></div>")
        facts.append(("agrees with", e(", ".join(en["friends"])) or
                      "<span class=muted>no other kernel</span>"))
    elif row:
        out.append("<p class=muted>No measures: it built no solid to measure.</p>")
    faces = (row or {}).get("faces")
    if isinstance(faces, dict) and faces:
        # (a row the harness measured from its STEP has the count alone: the STEP's own facts
        # say its faces by surface)
        facts.append(("faces", e(", ".join(f"{n} {k}" for k, n in faces.items()))))
    m = (row or {}).get("mesh")
    if m:
        bits = [f"{m['triangles']} triangles",
                "closed" if m.get("closed") else
                f"open: {m.get('unmatched_edges', 0)} edges unmatched"]
        if m.get("degenerate"):
            bits.append(f"{m['degenerate']} degenerate")
        facts.append(("mesh", ", ".join(bits)))
    g, inertia = (row or {}).get("centroid"), (row or {}).get("inertia")
    if g and inertia:
        of = "its STEP's solid" if row.get("measured") == "step" else "its triangles"
        facts.append(("centre of gravity", f"({', '.join(f'{x:.6g}' for x in g)}) "
                      f"<span class=muted>{of}</span>"))
        facts.append(("inertia", "<span class=muted>about it, of unit density:</span> "
                      + "; ".join(f"I<sub>{a}</sub> {inertia[i][j]:.6g}" for a, i, j in
                                  (("xx", 0, 0), ("yy", 1, 1), ("zz", 2, 2), ("xy", 0, 1),
                                   ("yz", 1, 2), ("zx", 2, 0)))))
    if en["stl"]:
        facts.append(("STL", f"<a href='{en['stl']}' download>download</a> "
                             f"<span class=muted>{size_text(en['mb'])}</span>"))
    facts += step_facts(row, en["writes"], faces)
    if facts:
        out.append("<table class=facts>" + "".join(
            f"<tr><th scope=row>{k}</th><td>{v}</td></tr>" for k, v in facts) + "</table>")
    elif not row:
        out.append("<p class=muted>None.</p>")

    out.append("<h3>Implementation</h3>")
    src, code, n = en["src"], en["code"], en["loc"]
    if code is None:
        out.append("<p class=muted>No implementation yet.</p>")
    else:
        rel = os.path.relpath(src, ROOT)
        out.append(f"<p class=muted><a href='{REPO}/blob/main/{e(rel)}'>{e(rel)}</a> · {n} "
                   f"line{'' if n == 1 else 's'} of code, comments left out</p>"
                   f"<pre class=code>{highlight(code, src)}</pre>")
    out.append("</section>")
    return "\n".join(out)


# shows the entry picked, by a click on the table of entries or the buttons, or the address's
# #k=NAME, else the table's default; marks it picked (aria-current) and tells the viewer its
# triangles and its STEP file (an `entry` event, its detail {src, step}, each a path or '')
PICK = """<script>
(() => {
  const entries = [...document.querySelectorAll('section.entry')];
  const marks = [...document.querySelectorAll('[data-k]:not(section)')];
  const view = document.getElementById('view');
  function pick(k) {
    const entry = entries.find((x) => x.dataset.k === k);
    if (!entry) return false;
    for (const x of entries) x.hidden = x !== entry;
    for (const m of marks) m.setAttribute('aria-current', String(m.dataset.k === k));
    if (view) view.hidden = !entry.dataset.src && !entry.dataset.step;
    document.dispatchEvent(new CustomEvent('entry', {
      detail: { src: entry.dataset.src, step: entry.dataset.step } }));
    return true;
  }
  const asked = () => new URLSearchParams(location.hash.slice(1)).get('k');
  for (const m of marks) {
    m.addEventListener('click', (event) => {
      event.preventDefault();
      history.replaceState(null, '', '#k=' + encodeURIComponent(m.dataset.k));
      pick(m.dataset.k);
    });
  }
  addEventListener('hashchange', () => pick(asked()));
  // arriving for one kernel's result (from the explorer, say): show it, and scroll to it
  if (pick(asked())) document.getElementById('results').scrollIntoView();
  else pick(document.querySelector('table.entries').dataset.default);
})();
</script>"""


TEX = """<link rel=stylesheet href='{cdn}/katex.min.css'>
<script src='{cdn}/katex.min.js'></script>
<script>
if (window.katex) {
  // the closed forms (.tex) stand alone; a description's math (.math) runs in its text, but
  // where it is marked display
  for (const el of document.querySelectorAll('.tex, .math')) {
    const display = !el.classList.contains('math') || el.classList.contains('display');
    katex.render(el.dataset.tex, el, { displayMode: display, throwOnError: false });
  }
}
</script>
"""

VIEWER = """<script type=importmap>
{"imports": {"three": "{cdn}/build/three.module.js", "three/addons/": "{cdn}/examples/jsm/"}}
</script>
<script type=module>
import * as THREE from 'three';
import { STLLoader } from 'three/addons/loaders/STLLoader.js';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

const canvas = document.getElementById('viewer');
const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(30, 1, 0.01, 1e6);
camera.up.set(0, 0, 1);
const controls = new OrbitControls(camera, canvas);
controls.enableDamping = true;
scene.add(new THREE.HemisphereLight(0xffffff, 0x8890a0, 2.2));
const sun = new THREE.DirectionalLight(0xffffff, 1.6);
scene.add(sun);
const dark = matchMedia('(prefers-color-scheme: dark)');
let shown = null;
const loader = new STLLoader();
const cache = new Map();

function resize() {
  const w = canvas.clientWidth, h = canvas.clientHeight;
  if (!w || !h) return;  // hidden: the entry picked left no triangles
  renderer.setSize(w, h, false);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
}

// The edges a single triangle uses (vertices joined where their coordinates are equal, as the
// harness joins them): where the mesh is not closed. Drawn red, thick, over everything.
function openEdges(geometry) {
  const p = geometry.attributes.position.array;
  const ids = new Map();
  const id = (k) => {
    const key = p[k] + ',' + p[k + 1] + ',' + p[k + 2];
    let v = ids.get(key);
    if (v === undefined) { v = ids.size; ids.set(key, v); }
    return v;
  };
  const uses = new Map();
  const ends = new Map();
  for (let t = 0; t < p.length; t += 9) {
    const v = [id(t), id(t + 3), id(t + 6)];
    for (let j = 0; j < 3; j++) {
      const a = v[j], b = v[(j + 1) % 3];
      if (a === b) continue;
      const key = a + '>' + b;
      uses.set(key, (uses.get(key) || 0) + 1);
      ends.set(key, [t + 3 * j, t + 3 * ((j + 1) % 3)]);
    }
  }
  const out = [];
  for (const [key, n] of uses) {
    const [a, b] = key.split('>');
    if (n === 1 && uses.get(b + '>' + a) === 1) continue;
    const [i, j] = ends.get(key);
    out.push(p[i], p[i + 1], p[i + 2], p[j], p[j + 1], p[j + 2]);
  }
  if (!out.length) return null;
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(out, 3));
  const m = new THREE.LineBasicMaterial({ color: 0xe0362c, depthTest: false });
  const lines = new THREE.LineSegments(g, m);
  lines.renderOrder = 2;
  // and a dot at each, a fixed size on the screen: an open edge is often a sliver too thin to see
  const dots = new THREE.Points(g, new THREE.PointsMaterial({
    color: 0xe0362c, size: 7, sizeAttenuation: false, depthTest: false }));
  dots.renderOrder = 3;
  const both = new THREE.Group();
  both.add(lines, dots);
  return both;
}

// each solid, built once: its surface, its creases and its open edges
const built = new Map();
function build(src) {
  if (!built.has(src)) {
    built.set(src, new Promise((ok, no) => loader.load(src, ok, undefined, no)).then((geometry) => {
      geometry.computeVertexNormals();
      geometry.computeBoundingSphere();
      const solid = new THREE.Mesh(geometry, new THREE.MeshStandardMaterial({
        color: 0xdfe3ea, roughness: 0.75, metalness: 0.0, flatShading: false,
        polygonOffset: true, polygonOffsetFactor: 1, polygonOffsetUnits: 1 }));
      const lines = new THREE.LineSegments(new THREE.EdgesGeometry(geometry, 28),
        new THREE.LineBasicMaterial({ color: dark.matches ? 0x0d0d10 : 0x22252a }));
      const group = new THREE.Group();
      group.add(solid, lines);
      const open = openEdges(geometry);
      if (open) group.add(open);
      group.userData.sphere = geometry.boundingSphere;
      return group;
    }));
  }
  return built.get(src);
}

// The kinds of surface a face can lie on, as harness/step.py names them, each with its color
// and its entities in a STEP file. A face on any other surface is `other`.
const KINDS = [
  ['plane', 0xb8bec8, ['PLANE']],
  ['cylinder', 0x4c8ed9, ['CYLINDRICAL_SURFACE']],
  ['cone', 0x2fb5a8, ['CONICAL_SURFACE']],
  ['sphere', 0xe58a3a, ['SPHERICAL_SURFACE']],
  ['torus', 0x9b6bd6, ['TOROIDAL_SURFACE', 'DEGENERATE_TOROIDAL_SURFACE']],
  ['bspline', 0x5fb349, ['B_SPLINE_SURFACE_WITH_KNOTS', 'B_SPLINE_SURFACE', 'BEZIER_SURFACE',
                         'UNIFORM_SURFACE', 'QUASI_UNIFORM_SURFACE']],
  ['revolution', 0xd9c23b, ['SURFACE_OF_REVOLUTION']],
  ['extrusion', 0xd95f87, ['SURFACE_OF_LINEAR_EXTRUSION']],
  ['offset', 0x8a6f4d, ['OFFSET_SURFACE']],
  ['other', 0x6b6b70, []],
];
const COLOR = Object.fromEntries(KINDS.map(([k, c]) => [k, new THREE.Color(c)]));
const KIND_OF = Object.fromEntries(KINDS.flatMap(([k, , names]) => names.map((n) => [n, k])));

// A STEP entity's parts, a simple entity's one and a complex entity's each in turn, its strings
// emptied: `NAME(a, (b, c), #3)` as {name: 'NAME', args: ['a', ['b', 'c'], '#3']}, a typed value
// (`LENGTH_MEASURE(1.)`) a part in its place. Enough for a control net and the unit of length.
function parts(body) {
  const tokens = body.match(/'(?:[^']|'')*'|[A-Z_][A-Z0-9_]*\\s*\\(|[(),]|[^\\s(),']+/g) || [];
  let i = tokens[0] === '(' ? 1 : 0;  // a complex entity's parts are in one more '('
  const list = () => {  // the items after a '(', to its ')'
    const out = [];
    while (i < tokens.length) {
      const t = tokens[i++];
      if (t === ')') break;
      if (t === '(') out.push(list());
      else if (t.endsWith('(')) out.push({ name: t.slice(0, -1).trim(), args: list() });
      else if (t !== ',') out.push(t);
    }
    return out;
  };
  return list().filter((x) => x.name);
}

// The millimetres in the file's unit of length (Open CASCADE gives the faces in millimetres),
// as harness/mass.py's Model._unit reads it; null where it does not know it
const PREFIX = { '$': 1, '.MILLI.': 1e-3, '.CENTI.': 1e-2, '.DECI.': 1e-1, '.KILO.': 1e3,
                 '.MICRO.': 1e-6, '.NANO.': 1e-9 };
function millimetres(entities, at) {
  const scale = (e, depth = 0) => {
    if (!e || depth > 4) return null;
    const ps = parts(e.body);
    const si = ps.find((p) => p.name === 'SI_UNIT');
    if (si) {
      const prefix = PREFIX[si.args.length >= 2 ? si.args[si.args.length - 2] : '$'];
      return prefix === undefined ? null : prefix * 1000;
    }
    const by = ps.find((p) => p.name === 'CONVERSION_BASED_UNIT');
    const measure = by && at(by.args[1]);
    const m = measure && parts(measure.body).find((p) => p.name.endsWith('MEASURE_WITH_UNIT'));
    if (!m) return null;
    const v = typeof m.args[0] === 'object' ? m.args[0].args[0] : m.args[0];
    const of = scale(at(m.args[1]), depth + 1);
    return of === null ? null : parseFloat(v) * of;
  };
  for (const e of entities.values()) {
    if (e.names.includes('LENGTH_UNIT')) return scale(e);
  }
  return 1;
}

// The knot spans of a B-spline's parameter, its patches along it: the distinct knots in its
// domain, less one. A Bézier's is one (as harness/mass.py's implicit_knots has it); a uniform
// one's, its control points less its degree.
function spans(bezier, poles, degree, mults, knots) {
  if (bezier) return 1;
  if (!mults) return Math.max(1, poles - degree);
  const all = [];
  mults.forEach((m, k) => { for (let j = 0; j < +m; j++) all.push(parseFloat(knots[k])); });
  const lo = all[degree], hi = all[all.length - degree - 1];
  return Math.max(1, new Set(all.filter((u) => u >= lo && u <= hi)).size - 1);
}

// A B-spline surface's control net, from its entity: its control points (in millimetres) as rows
// along u, its degrees, its knot spans and whether it is rational (as harness/step.py's kind
// has it: the entity says so). Null where it is not one.
function controlNet(e, at, mm) {
  const ps = parts(e.body);
  const bezier = ps.some((p) => p.name === 'BEZIER_SURFACE');
  let pu, pv, grid, knots = [];
  if (ps.length === 1) {
    const [{ name, args }] = ps;
    [pu, pv, grid] = [+args[1], +args[2], args[3]];
    if (name === 'B_SPLINE_SURFACE_WITH_KNOTS') knots = args.slice(8, 12);
  } else {
    const base = ps.find((p) => p.name === 'B_SPLINE_SURFACE');
    if (!base) return null;
    [pu, pv, grid] = [+base.args[0], +base.args[1], base.args[2]];
    const k = ps.find((p) => p.name === 'B_SPLINE_SURFACE_WITH_KNOTS');
    if (k) knots = k.args.slice(0, 4);
  }
  if (!Array.isArray(grid) || !Array.isArray(grid[0])) return null;
  const point = (ref) => {
    const p = at(ref);
    const c = p && parts(p.body).find((x) => x.name === 'CARTESIAN_POINT');
    const xyz = c && c.args.find(Array.isArray);
    return xyz ? [0, 1, 2].map((j) => parseFloat(xyz[j] || 0) * mm) : null;
  };
  const points = grid.map((row) => row.map(point));
  if (points.some((row) => row.some((q) => !q))) return null;
  return {
    points, degree: [pu, pv],
    spans: [spans(bezier, grid.length, pu, knots[0], knots[2]),
            spans(bezier, grid[0].length, pv, knots[1], knots[3])],
    rational: e.names.includes('RATIONAL_B_SPLINE_SURFACE'),
  };
}

// The kind of each face in a STEP file, in the order Open CASCADE reads them: each solid's
// shells in turn (its outer shell, then its voids), each shell's faces as it lists them; and
// the control net of each B-spline surface a face lies on, once however many faces lie on it.
function readStep(text) {
  const entities = new Map();
  const record = /#(\\d+)\\s*=\\s*((?:'(?:[^']|'')*'|[^;'])*);/g;
  let m;
  while ((m = record.exec(text))) {
    // its strings emptied to name it; its own text, which `parts` reads, kept as it is (a part
    // of the file's, not a copy)
    const bare = m[2].replace(/'(?:[^']|'')*'/g, "''");
    const names = bare.trim().startsWith('(')
      ? [...bare.matchAll(/([A-Z_][A-Z0-9_]*)\\s*\\(/g)].map((x) => x[1])
      : [(bare.match(/^\\s*([A-Z_][A-Z0-9_]*)/) || [])[1] || '?'];
    entities.set(+m[1], { names, body: m[2],
                          refs: [...bare.matchAll(/#(\\d+)/g)].map((x) => +x[1]) });
  }
  const surfaceOf = (face) => face.refs[face.refs.length - 1];
  const kindOf = (face) => {
    const surface = entities.get(surfaceOf(face));
    for (const n of surface ? surface.names : []) if (KIND_OF[n]) return KIND_OF[n];
    return 'other';
  };
  const kinds = [];
  const splines = new Set();
  for (const e of entities.values()) {
    if (!e.names.includes('MANIFOLD_SOLID_BREP') && !e.names.includes('BREP_WITH_VOIDS')) continue;
    for (const r of e.refs) {
      let shell = entities.get(r);
      if (shell && shell.names[0] === 'ORIENTED_CLOSED_SHELL') shell = entities.get(shell.refs[0]);
      if (!shell || !shell.names[0].endsWith('_SHELL')) continue;
      for (const f of shell.refs) {
        const face = entities.get(f);
        if (!face) continue;
        const kind = kindOf(face);
        kinds.push(kind);
        if (kind === 'bspline') splines.add(surfaceOf(face));
      }
    }
  }
  const at = (ref) => entities.get(+String(ref).slice(1));  // `#12`'s entity
  const mm = splines.size ? millimetres(entities, at) : null;
  const nets = mm === null ? []
    : [...splines].map((s) => controlNet(entities.get(s), at, mm)).filter(Boolean);
  return { kinds, nets };
}

// The control nets as lines and dots: each row of control points joined along u and along v,
// and a dot at each point (smaller where there are thousands); what the solid hides drawn
// again, faint, through it.
const NETS = 1;  // the layer the nets are drawn on, which the camera sees while they are asked for
function drawNets(nets) {
  const lines = [], dots = [];
  for (const { points } of nets) {
    for (let i = 0; i < points.length; i++) {
      for (let j = 0; j < points[i].length; j++) {
        dots.push(...points[i][j]);
        if (j + 1 < points[i].length) lines.push(...points[i][j], ...points[i][j + 1]);
        if (i + 1 < points.length) lines.push(...points[i][j], ...points[i + 1][j]);
      }
    }
  }
  const color = dark.matches ? 0xff6fae : 0xc2185b;
  const lineGeometry = new THREE.BufferGeometry();
  lineGeometry.setAttribute('position', new THREE.Float32BufferAttribute(lines, 3));
  const dotGeometry = new THREE.BufferGeometry();
  dotGeometry.setAttribute('position', new THREE.Float32BufferAttribute(dots, 3));
  const group = new THREE.Group();
  for (const seen of [true, false]) {
    const look = seen ? {} : { transparent: true, opacity: 0.25, depthTest: false };
    const l = new THREE.LineSegments(lineGeometry, new THREE.LineBasicMaterial({ color, ...look }));
    const d = new THREE.Points(dotGeometry, new THREE.PointsMaterial({
      color, size: dots.length > 9000 ? 3 : 5, sizeAttenuation: false, ...look }));
    l.renderOrder = d.renderOrder = seen ? 1 : 2;
    l.layers.set(NETS);
    d.layers.set(NETS);
    group.add(l, d);
  }
  return group;
}

// What the control nets say of the surfaces: how many, their control points and patches, the
// largest, and whether any is rational
function sizeOf(nets) {
  const count = (n) => n.points.length * n.points[0].length;
  const poles = nets.reduce((a, n) => a + count(n), 0);
  const patches = nets.reduce((a, n) => a + n.spans[0] * n.spans[1], 0);
  const big = nets.reduce((a, n) => (count(n) > count(a) ? n : a));
  const s = (k, one, many = one + 's') => `${k.toLocaleString()} ${k === 1 ? one : many}`;
  return `${s(nets.length, 'B-spline surface')}: ${s(poles, 'control point')}, `
    + s(patches, 'patch', 'patches')
    + (nets.length > 1 ? `; the largest ${big.points.length}×${big.points[0].length} points`
                       : ` (${big.points.length}×${big.points[0].length} points)`)
    + `, degree ${big.degree[0]}×${big.degree[1]}, ${big.spans[0]}×${big.spans[1]} patches`
    + (nets.some((n) => n.rational) ? '; rational' : '');
}

// Open CASCADE compiled for the browser (occt-import-js), loaded the first time faces are shown
const OCCT = '{occt}/';
let occt = null;
function openCascade() {
  if (!occt) {
    occt = new Promise((ok, no) => {
      const script = document.createElement('script');
      script.src = OCCT + 'occt-import-js.js';
      script.onload = ok;
      script.onerror = () => no(new Error('occt-import-js did not load'));
      document.head.append(script);
    }).then(() => window.occtimportjs({ locateFile: (name) => OCCT + name }));
  }
  return occt;
}

// A STEP file's solid, meshed by Open CASCADE: each B-rep face colored by its surface's kind,
// and the edges bounding each face (those of its triangles that only one of them uses) drawn;
// and, hidden till asked for, the control nets of its B-spline surfaces. Where the file's faces
// and Open CASCADE's are not as many, the kinds are not known, and every face is drawn in one
// color.
function buildFaces(src) {
  const key = 'step:' + src;
  if (!built.has(key)) {
    built.set(key, Promise.all([fetch(src).then((r) => {
      if (!r.ok) throw new Error(`${src}: ${r.status}`);
      return r.arrayBuffer();
    }), openCascade()]).then(([buffer, oc]) => {
      const bytes = new Uint8Array(buffer);
      const { kinds, nets } = readStep(new TextDecoder('latin1').decode(bytes));
      const read = oc.ReadStepFile(bytes, { linearUnit: 'millimeter',
        linearDeflectionType: 'bounding_box_ratio', linearDeflection: 0.0005,
        angularDeflection: 0.2 });
      if (!read.success || !read.meshes.length) throw new Error('Open CASCADE could not read it');
      const count = read.meshes.reduce((n, mesh) => n + mesh.brep_faces.length, 0);
      const known = count === kinds.length;
      const positions = [], normals = [], colors = [], seams = [];
      const tally = {};
      let g = 0;
      for (const mesh of read.meshes) {
        const p = mesh.attributes.position.array;
        const n = mesh.attributes.normal ? mesh.attributes.normal.array : null;
        const index = mesh.index.array;
        for (const face of mesh.brep_faces) {
          const kind = known ? kinds[g] : 'other';
          g++;
          tally[kind] = (tally[kind] || 0) + 1;
          const color = COLOR[kind];
          const uses = new Map();
          for (let t = face.first; t <= face.last; t++) {
            const v = [index[3 * t], index[3 * t + 1], index[3 * t + 2]];
            for (const i of v) {
              positions.push(p[3 * i], p[3 * i + 1], p[3 * i + 2]);
              if (n) normals.push(n[3 * i], n[3 * i + 1], n[3 * i + 2]);
              colors.push(color.r, color.g, color.b);
            }
            for (let j = 0; j < 3; j++) {
              const a = v[j], b = v[(j + 1) % 3];
              const edge = a < b ? a + ',' + b : b + ',' + a;
              uses.set(edge, (uses.get(edge) || 0) + 1);
            }
          }
          for (const [edge, used] of uses) {
            if (used !== 1) continue;
            const [a, b] = edge.split(',').map(Number);
            seams.push(p[3 * a], p[3 * a + 1], p[3 * a + 2], p[3 * b], p[3 * b + 1], p[3 * b + 2]);
          }
        }
      }
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
      geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
      if (normals.length === positions.length) {
        geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
      } else {
        geometry.computeVertexNormals();
      }
      geometry.computeBoundingSphere();
      const solid = new THREE.Mesh(geometry, new THREE.MeshStandardMaterial({
        vertexColors: true, roughness: 0.75, metalness: 0.0, side: THREE.DoubleSide,
        polygonOffset: true, polygonOffsetFactor: 1, polygonOffsetUnits: 1 }));
      const lines = new THREE.BufferGeometry();
      lines.setAttribute('position', new THREE.Float32BufferAttribute(seams, 3));
      const group = new THREE.Group();
      group.add(solid, new THREE.LineSegments(lines,
        new THREE.LineBasicMaterial({ color: dark.matches ? 0x0d0d10 : 0x22252a })));
      group.userData.sphere = geometry.boundingSphere;
      group.userData.tally = tally;
      group.userData.known = known;
      if (nets.length) {
        group.add(drawNets(nets));
        group.userData.size = sizeOf(nets);
      }
      return group;
    }));
  }
  return built.get(key);
}

// what the viewer shows: the entry's STEP file, its faces by surface (`faces`), or only where it
// has none, or Open CASCADE cannot read it, its runner's triangles (`mesh`)
let entry = { src: '', step: '' };
const legend = document.getElementById('kinds');
const meshNote = document.getElementById('mesh-note');
const meshWhy = document.getElementById('mesh-why');
const facesNote = document.getElementById('faces-note');
const facesNoteText = facesNote.textContent;
// the control nets' switch, and what they say of the surfaces; whether they are drawn is kept
// from one entry to the next, and from one page to the next where the browser keeps it
const netsBar = document.getElementById('nets');
const netsButton = netsBar.querySelector('button');
const netsSize = netsBar.querySelector('span');
function showNets(on) {
  camera.layers[on ? 'enable' : 'disable'](NETS);
  netsButton.setAttribute('aria-pressed', String(on));
  try { localStorage.setItem('brep-bench.nets', on ? '1' : '0'); } catch (_) {}
}
try { showNets(localStorage.getItem('brep-bench.nets') === '1'); } catch (_) { showNets(false); }
netsButton.addEventListener('click', () => showNets(!camera.layers.isEnabled(NETS)));

function marks(showing) {
  meshNote.hidden = showing !== 'mesh';
  facesNote.hidden = showing !== 'faces';
  legend.hidden = true;
  netsBar.hidden = true;
}

function describe(group) {
  const t = group.userData.tally;
  legend.innerHTML = group.userData.known
    ? KINDS.filter(([k]) => t[k]).map(([k, c]) => `<span class=key><span style='background:#${
      c.toString(16).padStart(6, '0')}'></span>${k} ${t[k]}</span>`).join('')
    : `${Object.values(t).reduce((a, b) => a + b, 0)} faces; their surfaces could not be matched to them`;
  legend.hidden = false;
  netsBar.hidden = !group.userData.size;
  netsSize.textContent = group.userData.size || '';
}

let wanted = null;
function place(group, fit) {
  if (shown) scene.remove(shown);
  shown = group;
  scene.add(shown);
  if (fit) {
    const { center, radius } = group.userData.sphere;
    const away = new THREE.Vector3(1, -1, 0.9).normalize()
      .multiplyScalar(radius / Math.sin(THREE.MathUtils.degToRad(15)) * 1.05);
    camera.position.copy(center).add(away);
    camera.near = radius / 100; camera.far = radius * 100;
    camera.updateProjectionMatrix();
    controls.target.copy(center);
  }
}

// the entry's triangles, saying why they are shown and not its STEP file
function showMesh(current, why, fit) {
  marks('mesh');
  meshWhy.textContent = why;
  build(current.src).then((group) => {
    if (current !== wanted) return;  // another entry was picked while this one loaded
    place(group, fit);
  });
}

function show(fit) {
  const current = entry;
  wanted = current;
  if (!current.step) {
    showMesh(current, "It has no STEP file to show: its runner's triangles instead.", fit);
    return;
  }
  marks('faces');
  facesNote.textContent = 'Reading the STEP file…';
  buildFaces(current.step).then((group) => {
    if (current !== wanted) return;
    place(group, fit);
    facesNote.textContent = facesNoteText;
    describe(group);
  }, (error) => {
    if (current !== wanted) return;
    if (current.src) {
      showMesh(current, `The STEP file could not be shown (${error.message}): its runner's `
        + 'triangles instead.', fit);
    } else {
      facesNote.textContent = `The STEP file could not be shown: ${error.message}.`;
    }
  });
}

function frame() {
  controls.update();
  sun.position.copy(camera.position);
  renderer.render(scene, camera);
}

// the entry picked (the page's other script's `entry` event, its detail the paths of its
// triangles and its STEP file): the camera is fitted to the first solid shown, and stays put as
// others are picked, so they can be compared; nothing is drawn while the viewer is hidden
let fitted = false;
function picked(detail) {
  entry = detail || { src: '', step: '' };
  const any = entry.src || entry.step;
  renderer.setAnimationLoop(any ? frame : null);
  if (!any) return;
  show(!fitted);
  fitted = true;
}
document.addEventListener('entry', (event) => picked(event.detail));
new ResizeObserver(resize).observe(canvas);
resize();
// this module runs after the other script has picked the first entry
const first = document.querySelector('section.entry:not([hidden])');
picked(first && { src: first.dataset.src, step: first.dataset.step });
</script>"""


def web(data):
    """Copy harness/web/ into site/, each address of a script or of board.json given a version
    (`?v=` and a hash of the scripts and of `data`, board.json's bytes). GitHub Pages lets a
    browser keep a file for ten minutes, so without it a browser can pair a deploy's scripts
    with the last deploy's board.json, or the reverse, and draw nothing.  Returns the version,
    for the case pages' addresses of the scripts."""
    names = sorted(os.listdir(WEB))
    digest = hashlib.sha256(data)
    for name in names:
        with open(os.path.join(WEB, name), "rb") as f:
            digest.update(name.encode() + b"\0" + f.read())
    v = "?v=" + digest.hexdigest()[:12]
    for name in names:
        src, dest = os.path.join(WEB, name), os.path.join(SITE, name)
        if not name.endswith((".html", ".js")):
            shutil.copyfile(src, dest)
            continue
        with open(src, encoding="utf-8") as f:
            text = f.read()
        text = text.replace("'board.json'", f"'board.json{v}'")
        if name.endswith(".html"):
            text = re.sub(r"src='(\w+\.js)'", rf"src='\1{v}'", text)
        else:
            text = re.sub(r"from '\./(\w+\.js)'", rf"from './\1{v}'", text)
        with open(dest, "w", encoding="utf-8") as f:
            f.write(text)
    return v


def main():
    registry, results, cases, drawings = load()
    for k in registry.values():
        for name in (WORKAROUNDS, ELEGANCE):
            for cid in sorted(set(flags(k, name)) - set(cases)):
                print(f"kernels/{k['name']}/{name}: no case {cid}", file=sys.stderr)
    # an elegant entry is one that stands: flagged where it neither passes nor builds, it is
    # shown all the same, and said here
    for r in results:
        for cid, note in r["elegance"].items():
            o = r["rows"].get(cid, {}).get("outcome", "missing")
            if cid in cases and o not in ("pass", "built"):
                print(f"kernels/{r['kernel']}/{ELEGANCE}: {cid} is {o}", file=sys.stderr)
    os.makedirs(SITE, exist_ok=True)
    data = json.dumps(board(registry, results, cases, drawings), separators=(",", ":"),
                      ensure_ascii=False).encode()
    with open(os.path.join(SITE, "board.json"), "wb") as f:
        f.write(data)
    v = web(data)
    ids = sorted(cases)
    for i, cid in enumerate(ids):
        category = cid.rpartition("/")[0]
        prev = ids[i - 1] if i and ids[i - 1].rpartition("/")[0] == category else None
        after = (ids[i + 1] if i + 1 < len(ids) and ids[i + 1].rpartition("/")[0] == category
                 else None)
        case_page(cases[cid], registry, results, drawings.get(cid), prev, after, v)
    print(f"site/: the board, board.json and {len(cases)} case pages written")


if __name__ == "__main__":
    main()
