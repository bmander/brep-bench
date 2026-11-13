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

// a case's page, showing `kernel`'s entry if one is named
export const casePage = (id, kernel) => `cases/${id}.html${
  kernel ? `#k=${encodeURIComponent(kernel)}` : ''}`;
export const thumbOf = (id) => `drawings/${id}/thumb.png`;
export const kernelPage = (name) => `kernel.html?k=${encodeURIComponent(name)}`;
export const categoryOf = (id) => id.slice(0, id.lastIndexOf('/'));
// whether an outcome is a sound solid: passed, or built where no answer is settled
export const solved = (o) => o === 'pass' || o === 'built';

/** The value a fraction `q` of the way up `sorted` (ascending), or null for an empty list. */
export function quantile(sorted, q) {
  return sorted.length ? sorted[Math.max(0, Math.ceil(q * sorted.length) - 1)] : null;
}

/** A select's option, picked when its value is `chosen`. */
export const option = (value, label, chosen) =>
  `<option value='${esc(value)}'${value === chosen ? ' selected' : ''}>${esc(label)}</option>`;

/** Places a shown tooltip beside the pointer, kept inside the window. */
export function placeTip(tip, event) {
  const pad = 14;
  tip.style.left = `${Math.min(event.clientX + pad, innerWidth - tip.offsetWidth - 8)}px`;
  tip.style.top = `${event.clientY + pad + tip.offsetHeight > innerHeight
    ? event.clientY - tip.offsetHeight - pad : event.clientY + pad}px`;
}

// Below this many seconds a run is mostly the starting of its runner, so how many cores it kept
// busy, and what they bought it, are not reckoned from it.
export const PARALLEL_FROM = 0.1;

/** board.json, read: `cases` (in id order, each with its index `i`), `categories` (in order,
 * each with its cases' indices), `kernels` (every one; those with results carry `outcomes`,
 * `seconds`, `cpu`, `mb`, `built`, `builtCpu`, `builtOne`, `cores`, `speedup`, `why`,
 * `workaround`, `elegance` (what makes its implementation elegant, '' where it is not flagged so),
 * `loc` (its implementation's lines of code), `faces` (how many faces its solid has, by its
 * runner's word) and `faceted` (where its solid is faceted, a mesh stored as B-rep faces, the
 * median of the other kernels' face counts; else null), `stepVerdict` and `stepWhy` (its STEP
 * verdict, a name in STEP_VERDICTS, null where it built no solid or writes no STEP, and why its
 * writer refused or the run failed), one entry a case (a measure null where there was no run), and `start`) and
 * `ran` (those with results).
 *
 * `start` is what starting the kernel's runner costs, building nothing: as the harness measured
 * it (`measured`), or for results from before it did, the kernel's fastest run standing in for
 * it.  `built`, `builtCpu` and `builtOne` are the kernel's own time, CPU time and time on one
 * core: as the harness timed them between its runner's pauses (its warm start, SPEC.md "A
 * runner") where it paused, else the run's less the start's.
 * `cores` is how many cores a run kept busy on average while building (its CPU time over its
 * time, the start's left out of both) and `speedup` its time on one core over its time on all of
 * them (likewise), both null for a run shorter than PARALLEL_FROM so reckoned.
 * `timeFloor` is the least each case's own time and CPU time are taken as, in a mean of their
 * logarithms or a ratio: FINE_TIME_FLOOR where the run was timed between its pauses, else
 * TIME_FLOOR where the start-up was measured and the run's times were kept finer than it
 * (board.json's `places`), else COARSE_TIME_FLOOR. */
export async function load() {
  const response = await fetch('board.json');
  if (!response.ok) throw new Error(`board.json: ${response.status}`);
  const b = await response.json();
  const names = Object.fromEntries(Object.entries(b.outcomes).map(([o, c]) => [c, o]));
  const verdicts = b.stepVerdicts || {};
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
    const outcomes = Array.from(r.codes, (c) => names[c]);
    const { seconds, cpu, mb } = r;
    const least = (list) => {
      const xs = list.filter((x) => x != null);
      return xs.length ? Math.min(...xs) : null;
    };
    const start = k.startup ? { ...k.startup, measured: true }
      : { seconds: least(seconds), cpu: least(cpu), mb: least(mb), one: null,
          measured: false };
    const timeFloor = r.places.map((p, i) => (r.own?.[i] != null ? FINE_TIME_FLOOR
      : start.measured && 10 ** -p <= TIME_FLOOR ? TIME_FLOOR : COARSE_TIME_FLOOR));
    // the kernel's own: as timed between its runner's pauses (its warm start) where it paused.
    // Results from before the harness timed pauses take the run's less the start-up; for a
    // kernel that does pause, a run that did not (it failed before it had loaded) has no own
    // time, rather than a difference that can come to nothing
    const less = (list, by, own) => list.map((x, i) => (own ? own[i] ?? null
      : x == null ? null : Math.max(0, x - (by || 0))));
    const built = less(seconds, start.seconds, r.own);
    const builtCpu = less(cpu, start.cpu, r.ownCpu);
    const builtOne = less(r.one, start.one ?? start.seconds, r.ownOne);
    const long = (i) => built[i] != null && built[i] >= PARALLEL_FROM;
    return {
      ...k,
      outcomes,
      seconds,
      cpu,
      mb,
      start,
      built,
      builtCpu,
      builtOne,
      // whether some run paused for the harness to time the kernel alone (its warm start)
      paused: !!r.own,
      timeFloor,
      cores: builtCpu.map((c, i) => (c != null && long(i) ? c / built[i] : null)),
      speedup: builtOne.map((t, i) => (t != null && long(i) ? t / built[i] : null)),
      why: r.why.map((i) => (i < 0 ? '' : b.reasons[i])),
      workaround: r.workaround.map((i) => (i < 0 ? '' : b.workarounds[i])),
      elegance: r.elegance ? r.elegance.map((i) => (i < 0 ? '' : b.elegance[i]))
        : r.codes.split('').map(() => ''),
      loc: r.loc || r.codes.split('').map(() => null),
      faces: r.faces || r.codes.split('').map(() => null),
      faceted: r.faceted || r.codes.split('').map(() => null),
      // its STEP verdict of each case (a name in STEP_VERDICTS), null where it built no solid or
      // its kernel writes no STEP; and why, where its writer refused or the run failed
      stepVerdict: r.step ? Array.from(r.step, (c) => verdicts[c] || null)
        : r.codes.split('').map(() => null),
      stepWhy: r.stepWhy ? r.stepWhy.map((i) => (i < 0 ? '' : b.stepReasons[i]))
        : r.codes.split('').map(() => ''),
    };
  });
  return { repo: b.repo, cases, categories, kernels, ran: kernels.filter((k) => k.outcomes),
           tags: b.tags };
}

// The least a run's own time is taken as, on a logarithmic scale and in a ratio, where the harness
// timed it between its runner's pauses (its warm start): a microsecond, as finely as the harness
// keeps times (run.py).
export const FINE_TIME_FLOOR = 0.000001;
// The same where the run did not pause and its own time is its time less its runner's start-up:
// a tenth of a millisecond, about what starting one runner varies by from start to start, so the
// difference says nothing finer.
export const TIME_FLOOR = 0.0001;
// The least a run's own time is taken as where it can't be told that finely: its times were
// recorded to the millisecond, or its kernel's fastest run stands in for the start-up.  This
// keeps the means and ratios of results from before microseconds where they were.
export const COARSE_TIME_FLOOR = 0.001;

/** A time in seconds, as a table shows it: 0, 4.2 µs, 42 µs, 420 µs, 3.7 ms, 420 ms, 4.21 s,
 * 12.3 s; one above 0 under a microsecond, as finely as the harness keeps times, <1 µs. */
export function secondsText(x) {
  if (x == null) return '–';
  if (x <= 0) return '0';
  if (x < FINE_TIME_FLOOR) return '<1 µs';
  if (x < 0.001) return `${Number((x * 1e6).toPrecision(2))} µs`;
  if (x < 0.01) return `${Number((x * 1000).toPrecision(2))} ms`;
  if (x < 1) return `${Math.round(x * 1000)} ms`;
  return x < 10 ? `${x.toFixed(2)} s` : `${x.toFixed(1)} s`;
}

/** Megabytes, as a table shows them: 12 MB, 245 MB, 1.24 GB. */
export function megabytesText(x) {
  if (x == null) return '–';
  if (x < 10) return `${x.toFixed(1)} MB`;
  return x < 1000 ? `${Math.round(x)} MB` : `${(x / 1000).toFixed(2)} GB`;
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

// A result's STEP verdict (harness/step.py, SPEC.md "STEP"): its word, what it means, and the
// outcome whose colour it takes (as site.py's STEP_VERDICTS). In the order a tally lists them.
export const STEP_VERDICTS = {
  brep: { word: 'B-rep', colour: 'pass',
          means: 'a solid of advanced faces, each on a surface, its shell closed' },
  faceted: { word: 'faceted', colour: 'built',
             means: 'a mesh stored as a B-rep: polygons, or faces that are mostly flat triangles' },
  triangles: { word: 'triangles', colour: 'built', means: 'tessellated triangles, and no B-rep' },
  open: { word: 'open', colour: 'wrong', means: 'faces that do not close into a solid' },
  empty: { word: 'empty', colour: 'wrong', means: 'no solid and no faces' },
  refused: { word: 'refused', colour: 'refused',
             means: 'its kernel\'s STEP writer declined the solid' },
  failed: { word: 'failed', colour: 'crashed', means: 'the runner failed to write STEP' },
  unrun: { word: 'not yet run', colour: 'missing',
           means: 'built before its runner was asked for STEP' },
};

/** Where kernel `k` stands on STEP, from kernels.json's `step`: `writes` (its runner writes it),
 * `cannot` (its kernel has no STEP writer: `why` says) or `unset` (its runner has not been set up
 * to write it). */
export function stepStanding(k) {
  if (k.step === true) return { standing: 'writes' };
  if (k.step) return { standing: 'cannot', why: k.step };
  return { standing: 'unset' };
}

/** A tally of kernel `k`'s STEP verdicts over the solids it built, by verdict, in
 * STEP_VERDICTS' order (only those it has). */
export function stepTally(k) {
  const t = {};
  for (const v of k.stepVerdict || []) if (v) t[v] = (t[v] || 0) + 1;
  return Object.keys(STEP_VERDICTS).filter((v) => t[v]).map((v) => [v, t[v]]);
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
