#!/usr/bin/env python3
"""Render results/*.json as one static page, site/index.html. Standard library only."""
import html
import json
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTCOMES = ["pass", "refused", "wrong", "crashed", "timeout"]

STYLE = """
:root { --bg: #fff; --fg: #1d1d1f; --muted: #6b6b70; --line: #e3e3e6;
        --pass: #d8f0dd; --refused: #f3ecd0; --wrong: #f6d5d2; --crashed: #e9d6f2;
        --timeout: #e9d6f2; --none: transparent; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #151517; --fg: #ececf0; --muted: #9a9aa2; --line: #2c2c31;
          --pass: #1f4429; --refused: #4a4120; --wrong: #5a2420; --crashed: #44264f;
          --timeout: #44264f; }
}
body { background: var(--bg); color: var(--fg); margin: 0 auto; max-width: 1100px;
       padding: 24px 16px; font: 15px/1.5 system-ui, sans-serif; }
h1 { font-size: 1.6em; margin: 0 0 4px; }
p.lede { color: var(--muted); margin: 0 0 24px; }
.scroll { overflow-x: auto; }
table { border-collapse: collapse; margin: 0 0 32px; font-variant-numeric: tabular-nums; }
th, td { border-bottom: 1px solid var(--line); padding: 6px 10px; text-align: left;
         white-space: nowrap; }
th { font-weight: 600; }
td.cell { text-align: center; font-size: 0.85em; }
td.pass { background: var(--pass); } td.refused { background: var(--refused); }
td.wrong { background: var(--wrong); } td.crashed, td.timeout { background: var(--crashed); }
.muted { color: var(--muted); }
a { color: inherit; }
"""


def main():
    with open(os.path.join(ROOT, "adapters.json")) as f:
        registry = {a["name"]: a for a in json.load(f)["adapters"]}
    results = []
    rdir = os.path.join(ROOT, "results")
    for name in sorted(os.listdir(rdir)) if os.path.isdir(rdir) else []:
        if name.endswith(".json"):
            with open(os.path.join(rdir, name)) as f:
                results.append(json.load(f))
    results.sort(key=lambda r: (-r["tally"].get("pass", 0), r["adapter"]))
    case_ids = sorted({c["case"] for r in results for c in r["cases"]})
    e = html.escape

    out = ["<!doctype html><html lang=en><head><meta charset=utf-8>",
           "<meta name=viewport content='width=device-width, initial-scale=1'>",
           "<title>brep-bench</title>",
           "<meta name=description content='A neutral conformance suite and scoreboard for "
           "B-rep and CAD kernels: the same cases against every kernel.'>",
           f"<style>{STYLE}</style></head><body>",
           "<h1>brep-bench</h1>",
           "<p class=lede>The same solid-modelling cases run against every B-rep kernel, each answer "
           "a closed form. A refusal ranks above a wrong answer. "
           "<a href='https://github.com/bmander/brep-bench'>Cases, harness and adapters</a>.</p>",
           "<h2>Kernels</h2><div class=scroll><table><tr><th>kernel</th>"]
    out += [f"<th>{o}</th>" for o in OUTCOMES]
    out.append("<th>version</th><th>licence</th><th>run</th></tr>")
    for r in results:
        a = registry.get(r["adapter"], {})
        name = e(r["adapter"])
        if a.get("kernel"):
            name = f"<a href='{e(a['kernel'])}'>{name}</a>"
        out.append(f"<tr><td>{name}</td>")
        out += [f"<td>{r['tally'].get(o, 0)}</td>" for o in OUTCOMES]
        out.append(f"<td class=muted>{e(r.get('version') or '')}</td>"
                   f"<td class=muted>{e(a.get('license', ''))}</td>"
                   f"<td class=muted>{e(r['run'])}</td></tr>")
    waiting = [a for n, a in registry.items() if not a.get("command")]
    out.append("</table></div>")
    if waiting:
        out.append("<p class=muted>Awaiting an adapter: " + ", ".join(
            f"<a href='{e(a['kernel'])}'>{e(a['name'])}</a>" for a in waiting) +
            ". Adapters are welcome from anyone, the kernel's authors first.</p>")

    out.append("<h2>Cases</h2><div class=scroll><table><tr><th>case</th>")
    out += [f"<th>{e(r['adapter'])}</th>" for r in results]
    out.append("</tr>")
    by = [{c["case"]: c for c in r["cases"]} for r in results]
    for cid in case_ids:
        out.append(f"<tr><td><a href='https://github.com/bmander/brep-bench/blob/main/cases/"
                   f"{e(cid)}.json'>{e(cid)}</a></td>")
        for rows in by:
            c = rows.get(cid)
            if c is None:
                out.append("<td class=cell></td>")
                continue
            tip = c["why"] or f"{c['seconds']} s"
            out.append(f"<td class='cell {c['outcome']}' title='{e(tip)}'>{c['outcome']}</td>")
        out.append("</tr>")
    out.append("</table></div>")
    out.append("<p class=muted>Times include starting the adapter's process. Hover a cell for "
               "why it failed or how long it took.</p></body></html>")
    os.makedirs(os.path.join(ROOT, "site"), exist_ok=True)
    with open(os.path.join(ROOT, "site", "index.html"), "w") as f:
        f.write("\n".join(out))
    print("site/index.html written")


if __name__ == "__main__":
    main()
