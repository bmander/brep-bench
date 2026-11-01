// What the board's pages share: board.json read into cases and kernels, and the pieces they draw.
// board.json's form is site.py's `board`.  Kernels keep kernels.json's order everywhere: the
// board ranks nothing.

export const OUTCOMES = ['pass', 'built', 'refused', 'wrong', 'crashed', 'timeout', 'missing'];

// The colour groups of the bars and the strip: wrong, crashed and timeout share one red there,
// and are counted apart in words.
export const GROUPS = [
  { key: 'pass', label: 'pass', of: ['pass'] },
  { key: 'built', label: 'built (unsettled, sound)', of: ['built'] },
  { key: 'refused', label: 'refused', of: ['refused'] },
  { key: 'failed', label: 'wrong, crashed or timed out', of: ['wrong', 'crashed', 'timeout'] },
  { key: 'missing', label: 'missing (no implementation yet)', of: ['missing'] },
];
const GROUP = {};
for (const g of GROUPS) for (const o of g.of) GROUP[o] = g.key;
export const groupOf = (outcome) => GROUP[outcome];

export const esc = (s) => String(s).replace(/[&<>"']/g, (c) => (
  { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));

export const casePage = (id) => `cases/${id}.html`;
export const thumbOf = (id) => `drawings/${id}/iso.png`;
export const kernelPage = (name) => `kernel.html?k=${encodeURIComponent(name)}`;
export const categoryOf = (id) => id.slice(0, id.lastIndexOf('/'));

/** board.json, read: `cases` (in id order, each with its index `i`), `categories` (in order,
 * each with its cases' indices), `kernels` (every one; those with results carry `outcomes`,
 * `seconds`, `why` and `workaround`, one entry a case) and `ran` (those with results). */
export async function load() {
  const response = await fetch('board.json');
  if (!response.ok) throw new Error(`board.json: ${response.status}`);
  const b = await response.json();
  const names = Object.fromEntries(Object.entries(b.outcomes).map(([o, c]) => [c, o]));
  const cases = b.cases.map(([id, title, tags, settled, drawn], i) => ({
    i, id, title, tags: tags.map((t) => b.tags[t]), settled: !!settled, drawn: !!drawn,
    category: categoryOf(id),
  }));
  const categories = [];
  for (const c of cases) {
    let last = categories[categories.length - 1];
    if (!last || last.name !== c.category) categories.push(last = { name: c.category, cases: [] });
    last.cases.push(c.i);
  }
  const kernels = b.kernels.map((k) => {
    const r = b.results[k.name];
    if (!r) return k;
    return {
      ...k,
      outcomes: Array.from(r.codes, (c) => names[c]),
      seconds: r.seconds,
      why: r.why.map((i) => (i < 0 ? '' : b.reasons[i])),
      workaround: r.workaround.map((i) => (i < 0 ? '' : b.workarounds[i])),
    };
  });
  return { repo: b.repo, cases, categories, kernels, ran: kernels.filter((k) => k.outcomes),
           tags: b.tags };
}

/** How many of the cases at `indices` (every case if omitted) kernel `k` has of each outcome. */
export function tally(k, indices) {
  const t = Object.fromEntries(OUTCOMES.map((o) => [o, 0]));
  if (indices) for (const i of indices) t[k.outcomes[i]]++;
  else for (const o of k.outcomes) t[o]++;
  return t;
}

/** A tally in words, outcome by outcome, leaving out the ones that are none. */
export const words = (t) => OUTCOMES.filter((o) => t[o]).map((o) => `${t[o]} ${o}`).join(', ');

/** A tally as a bar: a segment a colour group, as long as its count. */
export function bar(t) {
  const parts = GROUPS.map((g) => [g, g.of.reduce((n, o) => n + t[o], 0)]).filter(([, n]) => n);
  const spans = parts.map(([g, n]) =>
    `<span class=${g.key} style='flex-grow:${n}' title='${n} ${esc(g.label)}'></span>`).join('');
  return `<div class=bar role=img aria-label='${esc(words(t))}'>${spans}</div>`;
}

/** The key to the bars' colours. */
export const legend = () => GROUPS.map((g) =>
  `<span class=key><span class=${g.key}></span>${esc(g.label)}</span>`).join('');

/** A mark's colour for an outcome, read from the stylesheet (it changes with the theme). */
export function markColours() {
  const style = getComputedStyle(document.documentElement);
  return Object.fromEntries(GROUPS.map((g) => [g.key, style.getPropertyValue(`--m-${g.key}`).trim()]));
}

/** Says on the page that the board could not be read. */
export function failed(where, error) {
  where.innerHTML = `<p>The board could not be read: ${esc(error.message)}. Its pages read
    <code>board.json</code> beside them, so they must be served over HTTP (for one made locally,
    <code>python3 -m http.server -d site</code>).</p>`;
}
