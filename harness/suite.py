"""What the harness reads: the cases (cases/**/*.md), the kernels (kernels.json), and where a kernel's
implementation of a case lives. Standard library only (tomllib: Python 3.11 or later)."""
import json
import os
import tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_case(path):
    """A case file: its TOML header (between `+++` lines) and the description after it."""
    with open(path, encoding="utf-8") as f:
        text = f.read()
    if not text.startswith("+++"):
        raise ValueError(f"{path}: no +++ header")
    _, header, body = text.split("+++", 2)
    meta = tomllib.loads(header)
    cid = os.path.relpath(path, os.path.join(ROOT, "cases"))[:-len(".md")].replace(os.sep, "/")
    return {
        "id": cid,
        "title": meta.get("title", cid),
        "tags": meta.get("tags", []),
        # an unsettled case (`settled = false`) has no answer yet: what each kernel builds is shown
        # `tex` is the closed form in LaTeX, for the case's page; absent where there is none. A
        # case may also give the centroid and the inertia tensor (about it, of unit density),
        # graded beside the volume and the area
        "expect": ({k: {"value": (float(meta[k]["value"]) if k in ("volume", "area")
                                  else meta[k]["value"]),
                        "exact": meta[k]["exact"], "tex": meta[k].get("tex")}
                    for k in ("volume", "area", "centroid", "inertia") if k in meta}
                   if meta.get("settled", True) else None),
        # how near a result's measures must be to pass, where the case says (else run.py's EXACT)
        "tolerance": meta.get("tolerance"),
        "description": body.strip(),
        "path": path,
    }


def cases(prefix=""):
    """Every case whose id starts with `prefix`, in id order."""
    out = []
    for dirpath, _, files in os.walk(os.path.join(ROOT, "cases")):
        for f in files:
            if f.endswith(".md"):
                c = read_case(os.path.join(dirpath, f))
                if c["id"].startswith(prefix):
                    out.append(c)
    return sorted(out, key=lambda c: c["id"])


def kernels():
    with open(os.path.join(ROOT, "kernels.json")) as f:
        return json.load(f)["kernels"]


def source(kernel, cid):
    """Where `kernel`'s implementation of case `cid` is, from its `source` pattern, or None."""
    pattern = kernel.get("source")
    if not pattern:
        return None
    category, _, name = cid.rpartition("/")
    return os.path.join(ROOT, pattern.format(case=cid, category=category, name=name,
                                             name_snake=name.replace("-", "_")))


def implements(kernel, cid):
    """Whether `kernel` has an implementation of case `cid` (or keeps none apart from its runner)."""
    src = source(kernel, cid)
    return not src or os.path.exists(src)
