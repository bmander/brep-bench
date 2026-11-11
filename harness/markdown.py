"""A case's description as HTML: the Markdown its cases are written in, with LaTeX math.

The subset a description needs, after CommonMark:

- blocks: paragraphs, headings (`#` as the page's h2, since the case's title is its h1), lists,
  ordered (`1.`) and not (`-`, `*`, `+`), nested by indentation, tables (GitHub's pipes, with
  `:---:` alignment), fenced code, block quotes, rules (`---`) and display math, a block opened
  and closed by `$$`;
- inline: `code`, $math$ and $$display math$$, **strong**, *emphasis* (and the same with `_`,
  only at a word's edge, so snake_case stays as written), [links](https://…), <https://…>, and a
  backslash escaping any punctuation (`\\$` for a dollar).

`$…$` is math where its first character is not a space and its last neither a space nor followed by
a digit (pandoc's rule), so "$5 to $6" stays prose. Math is left as its TeX, in an element of
class `math` with the TeX in `data-tex` (and `display` where it stands alone), for KaTeX to typeset
in the page; until then, or without script, the reader sees the TeX itself. Raw HTML is escaped,
not passed through, and a link goes only to http(s), mailto, a fragment or a relative path. A
relative link to a `.md`, another case's description where GitHub shows this one, goes to that
case's page here, its `.html` (the pages stand as the descriptions do, cases/<case>.md).
Standard library only."""
import html
import re

# a placeholder for an inline piece already rendered, kept away from the passes after it
HOLD = "{}"
HELD = re.compile("(\\d+)")
PUNCT = r"""!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~"""

LIST_ITEM = re.compile(r"^( {0,3})([-*+]|\d{1,9}[.)])( +|$)(.*)$")
FENCE = re.compile(r"^ {0,3}(`{3,}|~{3,})(.*)$")
HEADING = re.compile(r"^ {0,3}(#{1,6})(?: +(.*?))?(?: +#+)? *$")
RULE = re.compile(r"^ {0,3}([-*_])( *\1){2,} *$")
DELIMITER = re.compile(r"^ *\|? *:?-+:? *(\| *:?-+:? *)*\|? *$")
SAFE_URL = re.compile(r"^(https?:|mailto:|#|/|\.{0,2}/|[\w\-.~/]+(#.*)?$)", re.I)
# a relative path to a Markdown file, up to its fragment or query: no scheme, and not from the root
MD_PATH = re.compile(r"^(?![a-z][a-z0-9+.\-]*:|/)([^#?]+)\.md(?=[#?]|$)", re.I)


def render(text):
    """`text`, a description in Markdown, as HTML."""
    return "\n".join(blocks(text.expandtabs(4).split("\n")))


def math(tex, display):
    """The element KaTeX typesets `tex` into (see TEX in site.py), its TeX shown until then."""
    shown = f"$${tex}$$" if display else f"${tex}$"
    cls = "math display" if display else "math"
    return f"<span class='{cls}' data-tex='{html.escape(tex)}'>{html.escape(shown)}</span>"


def starts_block(line):
    """Whether `line` opens a block of its own, so ends the paragraph before it."""
    m = LIST_ITEM.match(line)
    # a list breaks into a paragraph only with a bullet or a 1, so "5) sweeps" wrapped onto a line
    # of its own is still prose (CommonMark's rule)
    item = m and m.group(4).strip() and m.group(2) in ("-", "*", "+", "1.", "1)")
    return bool(FENCE.match(line) or HEADING.match(line) or RULE.match(line)
                or line.lstrip().startswith((">", "$$")) or item)


def blocks(lines):
    """`lines` as a list of HTML blocks."""
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        if m := FENCE.match(line):
            fence, info = m.group(1), m.group(2).strip()
            body = []
            i += 1
            while i < len(lines) and not lines[i].strip().startswith(fence):
                body.append(lines[i])
                i += 1
            i += 1
            lang = f" class='language-{html.escape(info.split()[0])}'" if info else ""
            out.append(f"<pre><code{lang}>{html.escape(chr(10).join(body))}</code></pre>")
            continue
        if line.lstrip().startswith("$$"):
            # display math: from the line that opens with $$ to the one that closes with it
            body = line.lstrip()[2:]
            i += 1
            while "$$" not in body and i < len(lines):
                body += "\n" + lines[i]
                i += 1
            tex, _, rest = body.partition("$$")
            out.append(f"<p class=display>{math(tex.strip(), True)}</p>")
            if rest.strip():
                out.append(f"<p>{inline(rest.strip())}</p>")
            continue
        if m := HEADING.match(line):
            n = min(len(m.group(1)) + 1, 6)
            out.append(f"<h{n}>{inline(m.group(2) or '')}</h{n}>")
            i += 1
            continue
        if RULE.match(line):
            out.append("<hr>")
            i += 1
            continue
        if line.lstrip().startswith(">"):
            quoted = []
            while i < len(lines) and lines[i].strip():
                q = lines[i].lstrip()
                quoted.append(q[1:].removeprefix(" ") if q.startswith(">") else q)
                i += 1
            out.append("<blockquote>" + "\n".join(blocks(quoted)) + "</blockquote>")
            continue
        if LIST_ITEM.match(line):
            i = listing(lines, i, out)
            continue
        if ("|" in line and i + 1 < len(lines) and DELIMITER.match(lines[i + 1])
                and "-" in lines[i + 1]):
            i = table(lines, i, out)
            continue
        para = []
        while i < len(lines) and lines[i].strip() and not (para and starts_block(lines[i])):
            para.append(lines[i].strip())
            i += 1
        out.append(f"<p>{inline(' '.join(para))}</p>")
    return out


def listing(lines, i, out):
    """The list that starts at `lines[i]` appended to `out`; returns the line after it."""
    first = LIST_ITEM.match(lines[i])
    ordered = first.group(2)[-1] in ".)"
    # a list's items share its marker: a bullet's character, or an ordered one's . or )
    kind = first.group(2)[-1]
    items = []
    loose = False
    while i < len(lines):
        m = LIST_ITEM.match(lines[i])
        if not m or m.group(2)[-1] != kind:
            break
        # the item's content starts at its text's column; lines indented that far continue it
        indent = len(m.group(1)) + len(m.group(2)) + (len(m.group(3)) if m.group(4) else 1)
        content = [m.group(4)]
        i += 1
        while i < len(lines):
            line = lines[i]
            if not line.strip():
                # a blank line inside the item when what follows is still indented under it
                j = i
                while j < len(lines) and not lines[j].strip():
                    j += 1
                if j < len(lines) and len(lines[j]) - len(lines[j].lstrip()) >= indent:
                    loose = True
                    content.extend([""] * (j - i))
                    i = j
                    continue
                break
            lead = len(line) - len(line.lstrip())
            if lead >= indent:
                content.append(line[indent:])
            elif LIST_ITEM.match(line) or starts_block(line):
                break
            else:
                # a lazy continuation of the item's paragraph
                content.append(line.strip())
            i += 1
        items.append(content)
        # a blank line between items makes the list loose: each item's text a paragraph
        if i < len(lines) and not lines[i].strip():
            j = i
            while j < len(lines) and not lines[j].strip():
                j += 1
            m = LIST_ITEM.match(lines[j]) if j < len(lines) else None
            if m and m.group(2)[-1] == kind:
                loose = True
                i = j
    tag = "ol" if ordered else "ul"
    start = int(first.group(2)[:-1]) if ordered else 1
    attr = f" start={start}" if start != 1 else ""
    out.append(f"<{tag}{attr}>")
    for content in items:
        inner = blocks(content)
        if not loose:
            inner = [b[3:-4] if b.startswith("<p>") and b.endswith("</p>") else b for b in inner]
        out.append("<li>" + "\n".join(inner) + "</li>")
    out.append(f"</{tag}>")
    return i


def cells(line):
    """A table row's cells, split at its unescaped pipes."""
    line = line.strip()
    if line.startswith("|"):
        line = line[1:]
    if line.endswith("|") and not line.endswith("\\|"):
        line = line[:-1]
    return [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line)]


def table(lines, i, out):
    """The table that starts at `lines[i]` appended to `out`; returns the line after it."""
    head = cells(lines[i])
    align = []
    for c in cells(lines[i + 1]):
        left, right = c.startswith(":"), c.endswith(":")
        align.append(" style='text-align:center'" if left and right
                     else " style='text-align:right'" if right
                     else " style='text-align:left'" if left else "")
    def row(cs, tag):
        cs = (cs + [""] * len(head))[:len(head)]
        return "<tr>" + "".join(f"<{tag}{align[k] if k < len(align) else ''}>{inline(c)}</{tag}>"
                                for k, c in enumerate(cs)) + "</tr>"
    rows = ["<thead>" + row(head, "th") + "</thead><tbody>"]
    i += 2
    while i < len(lines) and lines[i].strip() and "|" in lines[i]:
        rows.append(row(cells(lines[i]), "td"))
        i += 1
    out.append("<div class=scroll><table data-nosort>" + "".join(rows) + "</tbody></table></div>")
    return i


def inline(text):
    """A paragraph's text as HTML: code, math, links, emphasis; everything else escaped."""
    held = []

    def hold(piece):
        held.append(piece)
        return HOLD.format(len(held) - 1)

    # code spans, then math: what is inside either is not Markdown, and keeps its backslashes;
    # an escaped dollar is a dollar, not math
    text = re.sub(r"(`+)(.+?)(?<!`)\1(?!`)",
                  lambda m: hold(f"<code>{html.escape(m.group(2).strip())}</code>"), text)
    text = re.sub(r"\\\$", lambda m: hold("$"), text)
    text = re.sub(r"\$\$(.+?)\$\$", lambda m: hold(math(m.group(1).strip(), True)), text)
    text = re.sub(r"\$(?![\s$])((?:[^$\\]|\\.)*?)(?<![\s\\])\$(?!\d)",
                  lambda m: hold(math(m.group(1), False)), text)
    # then backslash escapes, so an escaped character means only itself
    text = re.sub(r"\\([" + PUNCT + r"])", lambda m: hold(html.escape(m.group(1))), text)
    # autolinks, then links, their text rendered as a paragraph's
    text = re.sub(r"<((?:https?|mailto):[^\s<>]+)>",
                  lambda m: hold(link(m.group(1), html.escape(m.group(1)))), text)
    text = re.sub(r"\[((?:[^\[\]]|\[[^\]]*\])+)\]\(\s*<?([^\s()<>]*(?:\([^\s()]*\)[^\s()<>]*)*)>?"
                  r"(?:\s+\"([^\"]*)\")?\s*\)",
                  lambda m: hold(link(m.group(2), inline(m.group(1)), m.group(3))), text)
    text = emphasis(html.escape(text, quote=False))
    while HELD.search(text):
        text = HELD.sub(lambda m: held[int(m.group(1))], text)
    return text


def link(url, label, title=None):
    if not SAFE_URL.match(url):
        return label
    # written for GitHub, where a case links to another's .md; here that case's page is its .html
    url = MD_PATH.sub(r"\1.html", url)
    t = f" title='{html.escape(title)}'" if title else ""
    return f"<a href='{html.escape(url)}'{t}>{label}</a>"


def emphasis(text):
    """`**strong**` and `*emphasis*`, and their `_` forms where they stand at a word's edge."""
    edge_l, edge_r = r"(?<![\w\\])", r"(?!\w)"
    text = re.sub(r"\*\*(?=\S)(.+?)(?<=\S)\*\*", r"<strong>\1</strong>", text)
    text = re.sub(edge_l + r"__(?=\S)(.+?)(?<=\S)__" + edge_r, r"<strong>\1</strong>", text)
    text = re.sub(r"(?<!\*)\*(?=[^\s*])(.+?)(?<=[^\s*])\*(?!\*)", r"<em>\1</em>", text)
    text = re.sub(edge_l + r"_(?=[^\s_])(.+?)(?<=[^\s_])_" + edge_r, r"<em>\1</em>", text)
    return text


if __name__ == "__main__":
    import sys
    print(render(sys.stdin.read()))
