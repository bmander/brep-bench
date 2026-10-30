#!/usr/bin/env python3
"""Render the results as a static site: the scoreboard (site/index.html) and a page for each case
(site/cases/<case>.html) with its description, its drawing, a 3D viewer of every kernel's result,
the results' STL files to download, and every kernel's implementation of it. Run harness/draw.py
first for the drawings. Standard library only."""
import html
import json
import os
import re
import shutil
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from suite import ROOT, cases as read_cases, kernels as read_kernels, source  # noqa: E402
SITE = os.path.join(ROOT, "site")
REPO = "https://github.com/bmander/brep-bench"
OUTCOMES = ["pass", "built", "refused", "wrong", "crashed", "timeout", "missing"]
THREE = "https://cdn.jsdelivr.net/npm/three@0.160.0"

STYLE = """
:root { --bg: #fff; --fg: #1d1d1f; --muted: #6b6b70; --line: #e3e3e6; --card: #f6f6f4;
        --built: #dbe7f6;
        --pass: #d8f0dd; --refused: #f3ecd0; --wrong: #f6d5d2; --crashed: #e9d6f2;
        --timeout: #e9d6f2; --accent: #2f5fb3; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #151517; --fg: #ececf0; --muted: #9a9aa2; --line: #2c2c31; --card: #1e1e21;
          --built: #1f3550;
          --pass: #1f4429; --refused: #4a4120; --wrong: #5a2420; --crashed: #44264f;
          --timeout: #44264f; --accent: #8fb0ff; }
}
body { background: var(--bg); color: var(--fg); margin: 0 auto; max-width: 1100px;
       padding: 24px 16px; font: 15px/1.5 system-ui, sans-serif; }
h1 { font-size: 1.6em; margin: 0 0 4px; }
h2 { margin-top: 32px; }
p.lede { color: var(--muted); margin: 0 0 24px; }
.scroll { overflow-x: auto; }
table { border-collapse: collapse; margin: 0 0 24px; font-variant-numeric: tabular-nums; }
th, td { border-bottom: 1px solid var(--line); padding: 6px 10px; text-align: left;
         white-space: nowrap; vertical-align: middle; }
th { font-weight: 600; }
td.cell { text-align: center; font-size: 0.85em; }
td.cell a { text-decoration: none; display: block; }
td.pass { background: var(--pass); } td.refused { background: var(--refused); }
td.wrong { background: var(--wrong); } td.crashed, td.timeout { background: var(--crashed); }
td.missing { color: var(--muted); }
td.built { background: var(--built); }
td.case small { display: block; color: var(--muted); }
details { margin: 0 0 12px; } summary { cursor: pointer; font-weight: 600; }
.description { max-width: 70ch; font-size: 1.05em; }
td.why { white-space: normal; min-width: 240px; font-size: 0.9em; }
.muted { color: var(--muted); }
a { color: inherit; }
img.thumb { width: 64px; height: 64px; object-fit: contain; border-radius: 4px;
            border: 1px solid var(--line); display: block; }
.paper { border: 1px solid var(--line); border-radius: 4px; display: block; }
.sheet { display: grid; grid-template-columns: auto auto; gap: 6px 10px; width: max-content; }
.sheet figure { margin: 0; }
.sheet figcaption { font-size: 0.8em; color: var(--muted); }
.sheet .up { align-self: end; }
#viewer { width: 100%; height: min(70vh, 560px); border: 1px solid var(--line);
          border-radius: 6px; background: var(--card); touch-action: none; }
.pick { display: flex; flex-wrap: wrap; gap: 6px; margin: 8px 0; }
.pick button { font: inherit; padding: 4px 12px; border-radius: 999px; cursor: pointer;
               border: 1px solid var(--line); background: var(--bg); color: var(--fg); }
.pick button[aria-pressed=true] { border-color: var(--accent); color: var(--accent); }
pre { background: var(--card); padding: 12px; border-radius: 6px; overflow-x: auto;
      font-size: 0.85em; margin: 6px 0 0; }
"""


def head(title, description, depth):
    up = "../" * depth
    return [
        "<!doctype html><html lang=en><head><meta charset=utf-8>",
        "<meta name=viewport content='width=device-width, initial-scale=1'>",
        f"<title>{html.escape(title)}</title>",
        f"<meta name=description content='{html.escape(description)}'>",
        f"<link rel=stylesheet href='{up}style.css'></head><body>",
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
            results.append(r)
    results.sort(key=lambda r: (-r["tally"].get("pass", 0), r["kernel"]))
    cases = {c["id"]: c for c in read_cases()}
    drawings = {}
    index = os.path.join(SITE, "drawings", "index.json")
    if os.path.exists(index):
        with open(index) as f:
            drawings = json.load(f)
    return registry, results, cases, drawings


def paragraphs(text):
    """A description's paragraphs as HTML: blank lines part them, `code` is code."""
    out = []
    for para in re.split(r"\n\s*\n", text.strip()):
        h = html.escape(" ".join(para.split()))
        out.append("<p>" + re.sub(r"`([^`]+)`", r"<code>\1</code>", h) + "</p>")
    return "\n".join(out)


def scoreboard(registry, results, cases, drawings):
    e = html.escape
    out = head("brep-bench", "A neutral conformance suite and scoreboard for B-rep and CAD "
               "kernels: the same cases against every kernel.", 0)
    out += ["<h1>brep-bench</h1>",
            "<p class=lede>Rosetta Code for B-rep kernels. Each case describes a solid in words "
            "and gives its volume and area as closed forms. Each kernel builds it in its own "
            "idioms, and every result is graded the same way; a refusal ranks above a wrong "
            "answer. Open a case for its drawing, every kernel's result in 3D, and every kernel's "
            f"code. <a href='{REPO}'>Cases, harness and implementations</a>.</p>",
            "<h2>Kernels</h2><div class=scroll><table><tr><th>kernel</th>"]
    out += [f"<th>{o}</th>" for o in OUTCOMES]
    out.append("<th>version</th><th>licence</th><th>run</th></tr>")
    for r in results:
        a = registry.get(r["kernel"], {})
        name = e(r["kernel"])
        if a.get("kernel"):
            name = f"<a href='{e(a['kernel'])}'>{name}</a>"
        if r.get("measured") == "mesh":
            name += (" <span class=muted title='no exact measures: graded on its triangles, "
                     "to 1e-3'>(mesh)</span>")
        out.append(f"<tr><td>{name}</td>")
        out += [f"<td>{r['tally'].get(o, 0)}</td>" for o in OUTCOMES]
        out.append(f"<td class=muted>{e(r.get('version') or '')}</td>"
                   f"<td class=muted>{e(a.get('license', ''))}</td>"
                   f"<td class=muted>{e(r['run'])}</td></tr>")
    out.append("</table></div>")
    waiting = [a for a in registry.values() if not a.get("command")]
    if waiting:
        out.append("<p class=muted>Awaiting implementations: " + ", ".join(
            f"<a href='{e(a['kernel'])}'>{e(a['name'])}</a>" for a in waiting) +
            ". They are welcome from anyone, the kernel's authors first.</p>")

    out.append("<h2>Cases</h2><div class=scroll><table><tr><th></th><th>case</th>")
    out += [f"<th>{e(r['kernel'])}</th>" for r in results]
    out.append("</tr>")
    by = [{c["case"]: c for c in r["cases"]} for r in results]
    for cid in sorted(cases):
        page = f"cases/{e(cid)}.html"
        thumb = (f"<a href='{page}'><img class=thumb src='drawings/{e(cid)}/iso.png' alt='' "
                 "loading=lazy></a>" if cid in drawings else "")
        out.append(f"<tr><td>{thumb}</td><td class=case><a href='{page}'>"
                   f"{e(cases[cid]['title'])}</a><small>{e(cid)}</small></td>")
        for rows in by:
            c = rows.get(cid)
            if c is None:
                out.append("<td class=cell></td>")
                continue
            tip = c["why"] or f"{c['seconds']} s"
            out.append(f"<td class='cell {c['outcome']}' title='{e(tip)}'>"
                       f"<a href='{page}#{e(rows_anchor(c))}'>{c['outcome']}</a></td>")
        out.append("</tr>")
    out.append("</table></div>")
    out.append("<p class=muted>Times include starting the runner's process. Hover a cell for "
               "why it failed or how long it took, or open the case. A kernel marked (mesh) has no "
               "exact volume or area of its own: it is graded on its triangles, to 1e-3.</p>"
               "</body></html>")
    with open(os.path.join(SITE, "index.html"), "w") as f:
        f.write("\n".join(out))


def rows_anchor(c):
    return "results"


def relative(got, want):
    if got is None:
        return ""
    return f"{abs(got - want) / max(abs(want), 1e-12):.1e}"


def case_page(c, registry, results, drawing):
    e = html.escape
    cid = c["id"]
    depth = cid.count("/") + 1
    up = "../" * depth
    out = head(f"{c['title']} · brep-bench", f"The brep-bench case {cid}: its description, its "
               "drawing, every kernel's result in 3D and every kernel's code.", depth)
    out.append(f"<p><a href='{up}index.html'>← brep-bench</a></p>")
    out.append(f"<h1>{e(c['title'])}</h1><p class=muted>{e(cid)} · "
               f"{e(', '.join(c['tags']))}</p>")
    out.append(f"<div class=description>{paragraphs(c['description'])}</div>")
    exp = c["expect"]
    if exp is None:
        out.append("<p><strong>Unsettled.</strong> This solid has no closed form, and no two kernels "
                   "yet agree on it to 1e-8, so it has no answer to grade against. Below is what each "
                   "kernel builds: <em>built</em> means a watertight, consistently oriented solid, "
                   "whatever its measures; the table says which kernels agree with which.</p>")
    else:
        out.append("<div class=scroll><table><tr><th></th><th>expected</th>"
                   "<th>where it comes from</th></tr>")
        for key in ("volume", "area"):
            out.append(f"<tr><td>{key}</td><td>{exp[key]['value']:.10g}</td>"
                       f"<td class=muted>{e(exp[key]['exact'])}</td></tr>")
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

    meshes = []
    rows = []
    for r in results:
        row = next((x for x in r["cases"] if x["case"] == cid), None)
        if row is None:
            continue
        stl = os.path.join(ROOT, "results", "meshes", r["kernel"], cid + ".stl")
        link = ""
        if os.path.exists(stl):
            dest = os.path.join(SITE, "meshes", r["kernel"], cid + ".stl")
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            shutil.copyfile(stl, dest)
            href = f"{up}meshes/{e(r['kernel'])}/{e(cid)}.stl"
            meshes.append((r["kernel"], href, row["outcome"]))
            mb = os.path.getsize(stl) / 1e6
            link = f"<a href='{href}' download>STL</a> <span class=muted>{mb:.1f} MB</span>"
        rows.append((r, row, link))

    if meshes:
        out.append("<h2>In 3D</h2>")
        out.append("<div class=pick role=group aria-label='Whose result'>" + "".join(
            f"<button type=button data-src='{href}' aria-pressed=false>{e(name)} "
            f"<span class=muted>{outcome}</span></button>" for name, href, outcome in meshes) +
            "</div>")
        out.append("<canvas id=viewer></canvas>")
        out.append("<p class=muted>Drag to turn, scroll or pinch to zoom, right-drag to move. "
                   "Black lines are where the surface turns sharply; red ones, seen through the solid, are open edges, where the mesh is not closed.</p>")

    # each result's measures, and which other results they agree with: to 1e-8 between exact
    # kernels, to 1e-3 where either was measured from its mesh
    measured = {}
    for r, row, _ in rows:
        from_mesh = row.get("measured") == "mesh"
        m = row.get("mesh") or {}
        vol = m.get("volume") if from_mesh else row.get("volume")
        area = m.get("area") if from_mesh else row.get("area")
        if row["outcome"] in ("pass", "built", "wrong") and vol is not None and area is not None:
            measured[r["kernel"]] = (vol, area, from_mesh)

    def agrees(a, b):
        (va, aa, ma), (vb, ab, mb) = measured[a], measured[b]
        tol = 1e-3 if ma or mb else 1e-8
        return all(abs(x - y) <= tol * max(abs(x), abs(y), 1e-12) for x, y in ((va, vb), (aa, ab)))

    columns = ("<th>volume (rel. error)</th><th>area (rel. error)</th>" if exp else
            "<th>volume</th><th>area</th>")
    out.append("<h2 id=results>Results</h2><div class=scroll><table><tr><th>kernel</th>"
               f"<th>outcome</th>{columns}<th>agrees with</th><th>faces</th><th>time</th>"
               "<th>mesh</th><th>why</th></tr>")
    for r, row, link in rows:
        name = r["kernel"]
        measured_mesh = row.get("measured") == "mesh"
        vol, area, _ = measured.get(name, (None, None, False))
        if exp:
            vcell, acell = relative(vol, exp["volume"]["value"]), relative(area, exp["area"]["value"])
        else:
            vcell = "" if vol is None else f"{vol:.12g}"
            acell = "" if area is None else f"{area:.12g}"
        friends = ", ".join(o for o in measured if o != name and name in measured and agrees(name, o))
        faces = ", ".join(f"{n} {k}" for k, n in (row.get("faces") or {}).items())
        out.append(
            f"<tr><td>{e(name)}{' <span class=muted>(mesh)</span>' if measured_mesh else ''}"
            f"</td><td class='cell {row['outcome']}'>{row['outcome']}</td>"
            f"<td>{vcell}</td><td>{acell}</td><td class=muted>{e(friends)}</td>"
            f"<td class=muted>{e(faces)}</td><td>{row['seconds']} s</td><td>{link}</td>"
            f"<td class=why>{e(row['why'])}</td></tr>")
    out.append("</table></div>")

    out.append("<h2 id=code>How each kernel builds it</h2>")
    for r in results:
        k = registry.get(r["kernel"], {})
        row = next((x for x in r["cases"] if x["case"] == cid), None)
        src = source(k, cid)
        if not src or not os.path.exists(src):
            out.append(f"<p class=muted>{e(r['kernel'])}: no implementation yet.</p>")
            continue
        with open(src, encoding="utf-8") as f:
            code = f.read()
        rel = os.path.relpath(src, ROOT)
        outcome = row["outcome"] if row else ""
        out.append(f"<details open><summary>{e(r['kernel'])} <span class=muted>"
                   f"{e(outcome)} · <a href='{REPO}/blob/main/{e(rel)}'>{e(rel)}</a></span>"
                   f"</summary><pre>{e(code)}</pre></details>")
    out.append(f"<p class=muted><a href='{REPO}/blob/main/cases/{e(cid)}.md'>{e(cid)}.md</a>, "
               "the case as written.</p>")
    if meshes:
        out.append(VIEWER.replace("{cdn}", THREE))
    out.append("</body></html>")
    path = os.path.join(SITE, "cases", cid + ".html")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write("\n".join(out))


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

function show(src, fit) {
  const got = cache.get(src) || new Promise((ok, no) => loader.load(src, ok, undefined, no));
  cache.set(src, got);
  got.then((geometry) => {
    if (shown) scene.remove(shown);
    geometry.computeVertexNormals();
    const solid = new THREE.Mesh(geometry, new THREE.MeshStandardMaterial({
      color: 0xdfe3ea, roughness: 0.75, metalness: 0.0, flatShading: false,
      polygonOffset: true, polygonOffsetFactor: 1, polygonOffsetUnits: 1 }));
    const lines = new THREE.LineSegments(new THREE.EdgesGeometry(geometry, 28),
      new THREE.LineBasicMaterial({ color: dark.matches ? 0x0d0d10 : 0x22252a }));
    shown = new THREE.Group();
    shown.add(solid, lines);
    const open = openEdges(geometry);
    if (open) shown.add(open);
    scene.add(shown);
    if (fit) {
      geometry.computeBoundingSphere();
      const { center, radius } = geometry.boundingSphere;
      const away = new THREE.Vector3(1, -1, 0.9).normalize()
        .multiplyScalar(radius / Math.sin(THREE.MathUtils.degToRad(15)) * 1.05);
      camera.position.copy(center).add(away);
      camera.near = radius / 100; camera.far = radius * 100;
      camera.updateProjectionMatrix();
      controls.target.copy(center);
    }
  });
}

const buttons = [...document.querySelectorAll('.pick button')];
buttons.forEach((b, i) => b.addEventListener('click', () => {
  buttons.forEach((o) => o.setAttribute('aria-pressed', String(o === b)));
  show(b.dataset.src, false);
}));
new ResizeObserver(resize).observe(canvas);
resize();
const first = buttons.find((b) => b.textContent.includes('pass')) || buttons[0];
first.setAttribute('aria-pressed', 'true');
show(first.dataset.src, true);
renderer.setAnimationLoop(() => {
  controls.update();
  sun.position.copy(camera.position);
  renderer.render(scene, camera);
});
</script>"""


def main():
    registry, results, cases, drawings = load()
    os.makedirs(SITE, exist_ok=True)
    with open(os.path.join(SITE, "style.css"), "w") as f:
        f.write(STYLE)
    scoreboard(registry, results, cases, drawings)
    for cid, c in sorted(cases.items()):
        case_page(c, registry, results, drawings.get(cid))
    print(f"site/index.html and {len(cases)} case pages written")


if __name__ == "__main__":
    main()
