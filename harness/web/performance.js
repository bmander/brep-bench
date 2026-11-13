// The performance page: what each kernel's runs cost, in one measure at a time (time, CPU time,
// peak memory, cores kept busy, speedup on every core).  Every case's measure is a dot on one
// logarithmic axis, a row a kernel, with a table of the same; then each kernel against each other
// on the cases both pass, the medians by category, the runs that cost most, and how well each
// kernel uses many cores.  One row of filters above scopes them all, and is kept in the address.
// Kernels keep kernels.json's order: the board ranks nothing.
import { load, esc, casePage, kernelPage, failed, secondsText, megabytesText, solved, quantile,
         option, placeTip, PARALLEL_FROM, FINE_TIME_FLOOR, GROUPS, groupOf } from './board.js';
import { sortable } from './sort.js';

const main = document.getElementById('performance');
load().then(start, (e) => failed(main, e));

// What can be measured of a run: each reads one of a kernel's lists (board.js), a value a case.
// A time is the kernel's own, what starting its runner costs left out (`key`), unless the reader
// asks for it counted (`raw`); `start` names the field of a kernel's start-up to show beside it.
// `floor` is the least value a logarithmic scale and the ratios take (a microsecond for a time, as
// finely as the harness keeps them), and a `timed` measure's ratios each kernel's floor
// for the case instead (board.js's `timeFloor`); `additive` says whether a total means anything,
// `compare` whether a ratio of two kernels' does; `least` and `most` head the summary's ends,
// `top` the runs that cost most; `higher` says more is better (else less is), for the summary's
// shading.
const timeTick = (x) => (x === 0 ? '0' : x < 0.001 ? `${Number((x * 1e6).toPrecision(1))} µs`
  : x < 1 ? `${Number((x * 1000).toPrecision(1))} ms` : `${Math.round(x)} s`);
const mbTick = (x) => (x < 1000 ? `${x} MB` : `${x / 1000} GB`);
const ratioText = (x) => (x == null ? '–' : `${x.toFixed(x < 10 ? 2 : 1)}`);
const speedupText = (x) => (x == null ? '–' : `${ratioText(x)}×`);
const short = `Loading the runner is left out of both, and so are runs whose own time
  is under ${PARALLEL_FROM} s, too short to say.`;
const METRICS = [
  {
    key: 'built', raw: 'seconds', start: 'seconds', label: 'time', text: secondsText,
    tick: timeTick, floor: FINE_TIME_FLOOR, timed: true, zero: true, additive: true,
    compare: true, least: 'fastest', most: 'slowest', top: 'The slowest runs',
    about: (counted) => (counted ? `Each dot is one case's wall-clock time, from starting the
      kernel's runner to its exit, starting it counted.` : `Each dot is one case's wall-clock
      time with loading the runner left out: the kernel's own time, building the solid and
      writing it as STEP where it can, not meshing or measuring it. The runner pauses once it has
      loaded and found its case and again once it has answered, and the harness times between
      the two; a run that did not pause is taken less the runner's start-up, and one no longer
      than that is 0. The scale starts at 0 and is logarithmic from about 10 µs up.`),
  },
  {
    key: 'builtCpu', raw: 'cpu', start: 'cpu', label: 'CPU time', text: secondsText,
    tick: timeTick, floor: FINE_TIME_FLOOR, timed: true, zero: true, additive: true,
    compare: true, least: 'least', most: 'most', top: 'The runs that spent the most CPU time',
    about: (counted) => `Each dot is the CPU time one case spent, user and system, over every
      thread${counted ? '' : `, between the runner's pauses (else less its start-up)`}: more than its
      time where it kept more than one core busy.`,
  },
  {
    key: 'mb', start: 'mb', label: 'peak memory', text: megabytesText, floor: 1,
    tick: mbTick,
    additive: false, compare: true, least: 'least', most: 'most',
    top: 'The runs that held the most memory',
    about: () => `Each dot is the most memory one case's runner held resident at once, with the
      runtime it runs on: a Python or .NET runtime holds far more before it builds anything than
      a Rust binary does, which the table's start-up column says.`,
  },
  {
    key: 'cores', label: 'cores busy', text: ratioText, tick: (x) => `${x}`, floor: 0.1, higher: true,
    additive: false, compare: false, least: 'least', most: 'most',
    top: 'The runs that kept the most cores busy',
    about: () => `Each dot is how many cores one case kept busy on average, its CPU time over its
      time: 1 for a run on one core throughout. ${short}`,
  },
  {
    key: 'speedup', label: 'speedup on every core', text: speedupText, higher: true,
    tick: (x) => `${x}×`, floor: 0.1, additive: false, compare: false, least: 'least',
    most: 'most', top: 'The runs that more cores sped up most',
    about: () => `Each dot is how many times faster one case ran free on every core than pinned to
      one: its time on one core over its time on all. ${short}`,
  },
];

const WHICH = [
  ['', 'every case it ran'],
  ['solved', 'the cases it passes or builds'],
];

function start(b) {
  // only the measures some kernel's runs have
  const metrics = METRICS.filter((m) => b.ran.some((k) => k[m.key].some((x) => x != null)));
  if (!metrics.length) {
    main.innerHTML = '<p class=muted>No kernel has measured runs yet.</p>';
    return;
  }
  const params = new URLSearchParams(location.search);
  const state = {
    metric: metrics.find((m) => m.key === params.get('m')) || metrics[0],
    which: params.get('which') === 'solved' ? 'solved' : '',
    // whether a time counts what starting the runner costs: left out unless asked for
    start: params.get('start') === 'counted' ? 'counted' : '',
    cat: b.categories.some((c) => c.name === params.get('cat')) ? params.get('cat') : '',
  };
  main.innerHTML = `
    <div class=controls>
      <select name=metric aria-label='Measure'${metrics.length > 1 ? '' : ' hidden'}>
        ${metrics.map((m) => option(m.key, m.label, state.metric.key)).join('')}</select>
      <select name=start aria-label='Starting the runner'>
        ${option('', 'start-up left out', state.start)}${option('counted', 'start-up counted',
          state.start)}</select>
      <select name=which aria-label='Which cases'>
        ${WHICH.map(([v, l]) => option(v, l, state.which)).join('')}</select>
      <select name=cat aria-label='Category'>${option('', 'every category', state.cat)}
        ${b.categories.map((c) => option(c.name, `${c.name} (${c.cases.length})`, state.cat))
          .join('')}</select>
    </div>
    ${machines(b)}
    <div id=figures></div>
    <div class=tip hidden></div>`;
  const figures = main.querySelector('#figures');
  const tip = main.querySelector('.tip');

  // the strip is drawn to the page's width, and again when that changes (not its height, which
  // each drawing changes)
  let redraw = null;
  let width = null;
  new ResizeObserver(([entry]) => {
    if (entry.contentRect.width === width) return;
    width = entry.contentRect.width;
    if (redraw) redraw();
  }).observe(figures);

  function address() {
    const p = new URLSearchParams();
    if (state.metric !== metrics[0]) p.set('m', state.metric.key);
    if (state.which) p.set('which', state.which);
    if (state.start) p.set('start', state.start);
    if (state.cat) p.set('cat', state.cat);
    const s = p.toString();
    history.replaceState(null, '', s ? `?${s}` : location.pathname);
  }

  function update() {
    address();
    // the measure as read: a time with its runner's start counted, if asked for
    const counted = !!state.start && !!state.metric.raw;
    const m = { ...state.metric, key: counted ? state.metric.raw : state.metric.key,
                about: state.metric.about(counted), counted };
    main.querySelector('select[name=start]').hidden = !state.metric.raw;
    const indices = state.cat ? b.categories.find((c) => c.name === state.cat).cases
      : b.cases.map((c) => c.i);
    // each kernel's cases in the filter that it has a measure of; the kernels with none at all
    // made their last runs before the harness measured this
    const measured = (k) => k[m.key].some((x) => x != null);
    const rows = b.ran.filter(measured).map((k) => {
      const at = indices.filter((i) => k[m.key][i] != null &&
                                       (!state.which || solved(k.outcomes[i])));
      const sorted = at.map((i) => k[m.key][i]).sort((x, y) => x - y);
      return { k, at, sorted };
    });
    const unmeasured = b.ran.filter((k) => !measured(k));
    const some = rows.some((r) => r.at.length);
    figures.innerHTML = some ? `
      <h2>By kernel</h2>
      <p class=muted>${m.about}${m.zero ? '' : ' The scale is logarithmic.'} Over the dots, a faint box spans the
        middle half of the cases each kernel passes or builds, with their median across it, and
        its whiskers reach the furthest of those within 1.5 times the box's width. Point at a dot
        for its case, or open it.</p>
      <p class=legend>${GROUPS.filter((g) => g.key !== 'missing').map((g) =>
        `<span class=key><span class='${g.key} dot'></span>${esc(g.label)}</span>`).join('')}<span
        class=key><span class=tick></span>median of the passed or built</span></p>
      <div class=perf-strip></div>
      ${summary(rows, m)}
      ${unmeasured.length ? `<p class=muted>Not measured: ${unmeasured.map((k) => esc(k.name))
        .join(', ')}, whose last runs were made before the harness measured their
        ${esc(m.label)}.</p>` : ''}
      ${m.compare && rows.length > 1
        ? `<h2>Kernel against kernel</h2>${versus(rows, indices, m)}` : ''}
      <h2>By category</h2>
      ${byCategory(b, rows, m)}
      <h2>${esc(m.top)}</h2>
      ${slowest(b, rows, m)}` : `<p class=muted>No runs in this selection have a measure of
      ${esc(m.label)}.</p>`;
    figures.insertAdjacentHTML('beforeend', multicore(b, indices, state.which) + sizes(b));
    sortable(figures);
    const holder = figures.querySelector('.perf-strip');
    redraw = holder ? () => strip(b, rows, m, holder, tip) : null;
    if (redraw) redraw();
  }

  main.addEventListener('change', (event) => {
    const el = event.target;
    if (el.name === 'metric') state.metric = metrics.find((m) => m.key === el.value);
    else if (el.name === 'which') state.which = el.value;
    else if (el.name === 'start') state.start = el.value;
    else if (el.name === 'cat') state.cat = el.value;
    else return;
    update();
  });
  update();
}

// A cell's wash for a value `t` of the way from its column's best (0) to its worst (1): green
// through amber to red.
function heat(t) {
  const [from, to, s] = t < 0.5 ? ['--m-built', '--m-refused', t * 2] : ['--m-refused', '--m-failed', t * 2 - 1];
  return `background: color-mix(in srgb, color-mix(in srgb, var(${to}) ${Math.round(100 * s)}%,
    var(${from})) 35%, transparent)`;
}

// A column's shading: `style(x)` washes a value from green at the best of `values` to red at
// the worst, on a logarithmic scale (values under `floor` taken as it) or a linear one.
function shading(values, { higher = false, log = false, floor = 0 } = {}) {
  const at = (x) => (log ? Math.log10(Math.max(x, floor)) : x);
  const known = values.filter((x) => x != null).map(at);
  const [lo, hi] = [Math.min(...known), Math.max(...known)];
  return (x) => {
    if (x == null || !(hi > lo)) return '';
    const t = (at(x) - lo) / (hi - lo);
    return ` style='${heat(higher ? 1 - t : t)}'`;
  };
}

// The table beside the strip, which says in numbers what it draws, and what starting each
// kernel's runner costs of the measure: left out of the strip's times (unless counted), only
// said of its memory.  Only the cases each kernel passes or builds count, whatever the filter.
// Each column but the count of cases is washed from green at its best value to red at its
// worst, on the strip's logarithmic scale.
function summary(measured, m) {
  const rows = measured.map(({ k, at }) => ({ k, sorted: at.filter((i) => solved(k.outcomes[i]))
    .map((i) => k[m.key][i]).sort((x, y) => x - y) }));
  // the columns washed, each a value a row
  const columns = [
    m.start && ((k) => k.start[m.start]),
    m.additive && ((k, sorted) => (sorted.length ? sorted.reduce((s, x) => s + x, 0) : null)),
    (k, sorted) => sorted[0],
    (k, sorted) => quantile(sorted, 0.5),
    (k, sorted) => quantile(sorted, 0.9),
    (k, sorted) => sorted[sorted.length - 1],
  ].filter(Boolean).map((value) => {
    const values = rows.map(({ k, sorted }) => value(k, sorted));
    return { values, style: shading(values, { higher: m.higher, log: true, floor: m.floor }) };
  });
  const cell = (col, r, extra = '') => {
    const x = col.values[r];
    return x == null ? '<td class=num><span class=muted>–</span></td>'
      : `<td class=num data-v=${x}${col.style(x)}${extra}>${m.text(x)}${extra ? '<sup>*</sup>' : ''}</td>`;
  };
  const notMeasured = ` title='not measured: the kernel&#39;s fastest run stands in for it'`;
  const estimated = m.start && rows.some(({ k }) => !k.start.measured);
  return `<div class=scroll><table><tr><th>kernel</th>${m.start ? `<th class=num
    title='starting the runner, building nothing'>start-up</th>` : ''}<th class=num
    title='the cases it passes or builds'>cases</th>
    ${m.additive ? '<th class=num>total</th>' : ''}<th class=num>${m.least}</th>
    <th class=num>median</th><th class=num title='nine cases in ten have no more'>90th
    percentile</th><th class=num>${m.most}</th></tr>${rows.map(({ k, sorted }, r) => {
      const [start, ...rest] = m.start ? columns : [null, ...columns];
      return `<tr><td><a href='${kernelPage(k.name)}'>${esc(k.name)}</a>${k.measured === 'mesh'
        ? ' <span class=muted>(mesh)</span>' : ''}</td>${start
        ? cell(start, r, k.start.measured ? '' : notMeasured) : ''}
        <td class=num>${sorted.length}</td>${rest.map((col) => cell(col, r)).join('')}</tr>`;
    }).join('')}</table></div>
    <p class=muted><em>Start-up</em> is the cost of starting the kernel's runner with nothing to
      build: the harness gives it a case id no case has and takes the median of five starts. It
      is subtracted only from a run that did not pause, loaded, for the harness to time the
      kernel alone.${
      estimated ? ` <sup>*</sup> Not measured yet (the kernel's last run was made before the
      harness did): its fastest run stands in for it.` : ''} The table counts only the cases each
      kernel passes or builds. Each column is shaded from green at its best value to red at its
      worst, on a logarithmic scale.</p>`;
}

// The ticks of a logarithmic axis from 10^lo to 10^hi: each power of ten, and 2 and 5 times it
// where the axis spans few enough of them to have room.
function logTicks(lo, hi) {
  const steps = hi - lo > 2.5 ? [1] : [1, 2, 5];
  const out = [];
  for (let d = Math.floor(lo); d <= Math.ceil(hi); d++) {
    for (const f of steps) {
      const v = Number((f * 10 ** d).toPrecision(1));
      if (Math.log10(v) >= lo - 1e-9 && Math.log10(v) <= hi + 1e-9) out.push(v);
    }
  }
  return out;
}

// How far in from the left a strip's axis starts: past the widest kernel name, as drawn.
function labelWidth(names) {
  const measure = document.createElement('canvas').getContext('2d');
  measure.font = '13px system-ui, sans-serif';
  return Math.ceil(Math.max(...names.map((n) => measure.measureText(n).width))) + 14;
}

// An axis's grid lines at `ticks`, from `top` down to its labels at the foot of `height`.
function grid(ticks, x, top, height, tick) {
  return ticks.flatMap((v) => [
    `<line class=grid x1=${x(v)} x2=${x(v)} y1=${top} y2=${height - 22} />`,
    `<text class=axis x=${x(v)} y=${height - 6} text-anchor=middle>${tick(v)}</text>`]);
}

// A time's place on an axis that starts at 0: linear below SYMLOG_LINEAR, logarithmic well above
// it (a symmetric logarithm), so a time of 0 has a place and a microsecond one is told from it.
const SYMLOG_LINEAR = 0.000001;
const symlog = (v) => Math.log10(1 + Math.max(v, 0) / SYMLOG_LINEAR);

// Every case a dot, a row a kernel, on one axis that does not move with the filters: for a time,
// from 0 (symlog); for any other measure, logarithmic.
function strip(b, rows, m, holder, tip) {
  const all = [];
  for (const k of b.ran) for (const x of k[m.key]) if (x) all.push(Math.max(x, m.floor));
  if (!all.length) all.push(m.floor);
  const scale = m.zero ? symlog : (v) => Math.log10(Math.max(v, m.floor));
  // from 0, or just before the least value above 0, to just past the greatest
  const lo = m.zero ? 0 : scale(Math.min(...all)) - 0.08;
  const hi = Math.max(lo + 0.5, scale(Math.max(...all)) + 0.08);
  // a time's ticks: 0, then each power of ten from 10 µs, where the scale is logarithmic
  const ticks = m.zero ? [0, ...logTicks(-5, Math.log10(Math.max(...all)) + 0.08)]
    : logTicks(lo, hi);
  const width = Math.max(holder.clientWidth, 360);
  const left = labelWidth(rows.map((r) => r.k.name));
  const right = 16;
  const rowHeight = 34;
  const top = 4;
  const height = top + rows.length * rowHeight + 26;
  const x = (v) => left + (Math.max(scale(v), lo) - lo) / (hi - lo) * (width - left - right);
  // a dot's place across its row: the same for a case in every row, so a case lines up
  const jitter = (i) => ((Math.imul(i + 1, 2654435761) >>> 0) / 2 ** 32) * 2 - 1;
  const dots = [];
  const medians = [];
  const out = grid(ticks, x, top, height, m.tick);
  rows.forEach(({ k, at }, r) => {
    const mid = top + r * rowHeight + rowHeight / 2;
    out.push(`<a href='${kernelPage(k.name)}'><text class=name x=${left - 10} y=${mid}
      text-anchor=end dominant-baseline=middle>${esc(k.name)}</text></a>`);
    // the ones not passed first, so the passes are drawn over them
    const order = [...at].sort((i, j) => solved(k.outcomes[i]) - solved(k.outcomes[j]));
    for (const i of order) {
      const v = k[m.key][i];
      const dot = { k, i, v, cx: x(v), cy: mid + jitter(i) * (rowHeight / 2 - 7) };
      dots.push(dot);
      // coloured by outcome, as the overview's grid is
      out.push(`<circle class='${groupOf(k.outcomes[i])}' cx=${dot.cx.toFixed(1)}
        cy=${dot.cy.toFixed(1)} r=4 />`);
    }
    // a ghost of a box and whiskers over the cases it passes or builds, as the table counts them
    const passed = at.filter((i) => solved(k.outcomes[i])).map((i) => k[m.key][i])
      .sort((p, q) => p - q);
    if (!passed.length) return;
    const [q1, median, q3] = [0.25, 0.5, 0.75].map((q) => quantile(passed, q));
    const log = scale;
    const reach = 1.5 * (log(q3) - log(q1));
    const inside = passed.filter((v) => log(v) >= log(q1) - reach && log(v) <= log(q3) + reach);
    const [low, high] = [inside[0], inside[inside.length - 1]];
    const h = rowHeight / 2 - 3;
    medians.push(`${k.name}: median ${m.text(median)}`);
    out.push(`<g class=ghost><line x1=${x(low)} x2=${x(q1)} y1=${mid} y2=${mid} />
      <line x1=${x(q3)} x2=${x(high)} y1=${mid} y2=${mid} />
      <line x1=${x(low)} x2=${x(low)} y1=${mid - h / 2} y2=${mid + h / 2} />
      <line x1=${x(high)} x2=${x(high)} y1=${mid - h / 2} y2=${mid + h / 2} />
      <rect x=${x(q1)} y=${mid - h} width=${Math.max(x(q3) - x(q1), 1)} height=${2 * h} rx=2 />
      </g><line class=median x1=${x(median)} x2=${x(median)} y1=${mid - h} y2=${mid + h} />`);
  });
  const text = medians.join('; ');
  holder.innerHTML = `<svg width=${width} height=${height} role=img aria-label='${esc(
    `Each case's ${m.label}, a row a kernel, with a box and whiskers over the cases each passes `
    + `or builds. ${text}. The table below gives more.`)}'>
    ${out.join('')}<circle class=hl r=7 visibility=hidden /></svg>`;
  const svg = holder.querySelector('svg');
  const ring = svg.querySelector('.hl');
  let hit = null;
  svg.addEventListener('pointermove', (event) => {
    const box = svg.getBoundingClientRect();
    const px = event.clientX - box.left;
    const py = event.clientY - box.top;
    const r = Math.floor((py - top) / rowHeight);
    hit = null;
    if (r >= 0 && r < rows.length && px >= left - 8) {
      // the nearest dot in the row, within reach
      let best = 24 * 24;
      for (const d of dots) {
        if (d.k !== rows[r].k) continue;
        const dd = (d.cx - px) ** 2 + (d.cy - py) ** 2;
        if (dd < best) { best = dd; hit = d; }
      }
    }
    if (!hit) {
      ring.setAttribute('visibility', 'hidden');
      tip.hidden = true;
      svg.style.cursor = '';
      return;
    }
    ring.setAttribute('cx', hit.cx);
    ring.setAttribute('cy', hit.cy);
    ring.setAttribute('visibility', 'visible');
    svg.style.cursor = 'pointer';
    const c = b.cases[hit.i];
    const whole = m.raw && !m.counted ? hit.k[m.raw][hit.i] : null;
    tip.innerHTML = `<b>${esc(m.text(hit.v))}</b>${whole != null ? ` <span class=muted>(${
      esc(m.text(whole))} with the start)</span>` : ''}<span>${esc(hit.k.name)} · ${
      esc(hit.k.outcomes[hit.i])}</span><div>${esc(c.title)}</div><span class=muted>${
      esc(c.id)}</span>`;
    tip.hidden = false;
    placeTip(tip, event);
  });
  svg.addEventListener('pointerleave', () => {
    ring.setAttribute('visibility', 'hidden');
    tip.hidden = true;
  });
  svg.addEventListener('click', () => { if (hit) location.href = casePage(b.cases[hit.i].id); });
}

// A ratio as a cell shows it: 26×, 2.3×, 0.42×, 0.04×.
function times(r) {
  if (r >= 10) return `${Math.round(r)}×`;
  if (r >= 1) return `${r.toFixed(1)}×`;
  return `${r.toPrecision(r >= 0.1 ? 2 : 1)}×`;
}

// A wash for a cell, from how far a ratio is from 1 (a factor of 10 to the `t`): blue where the
// row's kernel takes less, orange where it takes more, none near even.
function diverging(t) {
  const a = Math.abs(t);
  if (a < 0.05) return '';
  const strength = a < 0.3 ? 14 : a < 0.7 ? 26 : a < 1.3 ? 40 : 55;
  return `background: color-mix(in srgb, var(${t < 0 ? '--m-pass' : '--slower'}) ${strength}%,
    transparent)`;
}

// Each kernel against each other, over the cases both pass or build: the geometric mean of the
// row kernel's measure over the column kernel's.  Comparing on the same cases is what makes the
// numbers comparable at all: each kernel passes different ones.
function versus(rows, indices, m) {
  const ks = rows.map((r) => r.k);
  const cells = ks.map((a) => ks.map((c) => {
    if (a === c) return '<td class="muted empty">·</td>';
    let sum = 0;
    let n = 0;
    for (const i of indices) {
      const va = a[m.key][i];
      const vc = c[m.key][i];
      if (!solved(a.outcomes[i]) || !solved(c.outcomes[i]) || va == null || vc == null) continue;
      // a ratio is only as fine as the coarser of the two times
      const f = m.timed ? Math.max(a.timeFloor[i], c.timeFloor[i]) : m.floor;
      sum += Math.log(Math.max(va, f) / Math.max(vc, f));
      n++;
    }
    if (!n) return '<td class="muted empty">·</td>';
    const r = Math.exp(sum / n);
    const words = `${a.name} takes ${times(r)} the ${m.label} ${c.name} takes, over the ${n} `
      + `case${n === 1 ? '' : 's'} both pass or build`;
    return `<td class=num data-v=${r} style='${diverging(Math.log10(r))}' title='${esc(words)}'>${times(r)}
      <small class=muted>${n}</small></td>`;
  }));
  const keys = [[-1.5, '20×'], [-1, '5×'], [-0.5, '2×'], [0, 'even'], [0.5, '2×'], [1, '5×'],
                [1.5, '20×']].map(([t, label]) => `<span class=swatch style='${diverging(t)}'>${
                  label}</span>`).join('');
  return `<p class=muted>Each cell is the row kernel's ${esc(m.label)} as a multiple of the column
    kernel's, the geometric mean over the cases both pass or build (how many, beside it). Only the
    same cases compare: each kernel passes different ones.</p>
    <p class=legend>row takes less ${keys} row takes more</p>
    <div class=scroll><table class=versus><tr><th></th>${ks.map((k) =>
      `<th class=num>${esc(k.name)}</th>`).join('')}</tr>${ks.map((k, r) => `<tr><th>${
      esc(k.name)}</th>${cells[r].join('')}</tr>`).join('')}</table></div>`;
}

// Each category's median for each kernel, washed blue the longer it is, on the strip's scale.
function byCategory(b, rows, m) {
  const all = [];
  for (const { k } of rows) for (const x of k[m.key]) if (x != null) all.push(Math.max(x, m.floor));
  if (!all.length) return '';
  const lo = Math.log10(Math.min(...all));
  const hi = Math.max(lo + 1, Math.log10(Math.max(...all)));
  const wash = (v) => `background: color-mix(in srgb, var(--m-pass) ${Math.round(
    4 + 46 * (Math.log10(Math.max(v, m.floor)) - lo) / (hi - lo))}%, transparent)`;
  const chosen = new Set(rows.flatMap((r) => r.at));
  const cats = b.categories.filter((c) => c.cases.some((i) => chosen.has(i)));
  const body = cats.map((c) => {
    const inCat = new Set(c.cases);
    return `<tr><th><a href='explore.html?cat=${encodeURIComponent(c.name)}'>${esc(c.name)}</a>
      </th>${rows.map(({ k, at }) => {
        const vs = at.filter((i) => inCat.has(i)).map((i) => k[m.key][i]).sort((x, y) => x - y);
        const med = quantile(vs, 0.5);
        return med == null ? '<td class="muted empty">·</td>' : `<td class=num data-v=${med} style='${wash(med)}'
          title='${esc(`${k.name}: median ${m.text(med)} over ${vs.length} cases`)}'>${
          m.text(med)}</td>`;
      }).join('')}</tr>`;
  }).join('');
  return `<p class=muted>The median ${esc(m.label)} of each kernel's cases in each category, washed
    deeper the longer it is. Each kernel's median is over the cases it ran there, so two in one
    row may be over different cases.</p><div class=scroll><table class=versus><tr><th>category</th>
    ${rows.map(({ k }) => `<th class=num>${esc(k.name)}</th>`).join('')}</tr>${body}</table></div>`;
}

// The runs that took longest, of every kernel.
function slowest(b, rows, m) {
  const runs = rows.flatMap(({ k, at }) => at.map((i) => ({ k, i, v: k[m.key][i] })))
    .sort((x, y) => y.v - x.v).slice(0, 20);
  return `<div class=scroll><table><tr><th>case</th><th>kernel</th><th class=num>${
    esc(m.label)}</th><th>outcome</th></tr>${runs.map(({ k, i, v }) => `<tr><td class=case>
      <a href='${casePage(b.cases[i].id)}'>${esc(b.cases[i].title)}</a><small>${
      esc(b.cases[i].id)}</small></td><td>${esc(k.name)}</td><td class=num data-v=${v}>${m.text(v)}</td>
      <td class='cell ${k.outcomes[i]}'>${k.outcomes[i]}</td></tr>`).join('')}</table></div>`;
}

// Which machine each kernel's last run was made on: its measures are that machine's.
function machines(b) {
  const groups = new Map();
  for (const k of b.ran) {
    const h = k.host ? `${k.host.cpu}, ${k.host.cores} cores (${k.host.system})${k.host.least_mb
      ? `, where no run's peak memory reads under ${megabytesText(k.host.least_mb)}, the
        harness's measurer's own footprint` : ''}` : '';
    if (!groups.has(h)) groups.set(h, []);
    groups.get(h).push(k.name);
  }
  const named = [...groups].map(([h, names]) => (h ? esc(h) : 'a machine not recorded') +
    (groups.size > 1 ? ` <span class=muted>(${names.map(esc).join(', ')})</span>` : ''));
  return `<p class=muted>Measured on ${named.join('; ')}.</p>`;
}

// How well each kernel turns a machine's cores into speed, over its runs long enough to say.
function multicore(b, indices, which) {
  const rows = b.ran.filter((k) => k.cores.some((x) => x != null)).map((k) => {
    const at = indices.filter((i) => k.cores[i] != null && (!which || solved(k.outcomes[i])));
    const both = at.filter((i) => k.speedup[i] != null);
    const sum = (list, key) => list.reduce((s, i) => s + k[key][i], 0);
    // start-up left out, as in each run's `cores` and `speedup`
    const busy = at.length ? sum(at, 'builtCpu') / sum(at, 'built') : null;
    const speedup = both.length ? sum(both, 'builtOne') / sum(both, 'built') : null;
    const sorted = at.map((i) => k.cores[i]).sort((x, y) => x - y);
    return { k, at, both, busy, median: quantile(sorted, 0.5), most: sorted[sorted.length - 1],
             speedup, cores: k.host && k.host.cores };
  });
  if (!rows.length) return '';
  const anySpeedup = rows.some((r) => r.speedup != null);
  for (const r of rows) r.efficiency = r.speedup != null && r.cores ? r.speedup / r.cores : null;
  // each column of a ratio shaded, more being better; the counts are not
  const shade = Object.fromEntries(['busy', 'median', 'most', 'speedup', 'efficiency'].map((key) =>
    [key, shading(rows.map((r) => r[key]), { higher: true })]));
  const num = (x, text, key) => `<td class=num data-v='${x ?? ''}'${key ? shade[key](x) : ''}>${x == null
    ? '<span class=muted>–</span>' : text(x)}</td>`;
  return `<h2>Multicore</h2>
    <p class=muted>How many of the machine's cores each kernel keeps busy, and the speedup they buy it,
      over its runs of ${PARALLEL_FROM} s or more. <em>Cores busy</em> is the CPU time it spent over
      its time, all its runs together: 1 for a kernel that works on one core throughout.
      ${anySpeedup ? `<em>Speedup</em> is its time pinned to one core over its time free on every
      core, the same runs together (those it built), and <em>efficiency</em> that speedup shared
      among the cores it had: 100% would turn every core wholly into speed.` : `Its speedup on every core is
      measured only by a run made with <code>run.py --one-core</code>, which runs each case again
      pinned to one core; the last runs were not.`}</p>
    <div class=scroll><table><tr><th>kernel</th><th class=num>cores it had</th>
      <th class=num>runs</th><th class=num>cores busy</th><th class=num>median</th>
      <th class=num>most</th>${anySpeedup ? `<th class=num>runs on one core</th>
      <th class=num>speedup</th><th class=num>efficiency</th>` : ''}</tr>
      ${rows.map((r) => `<tr><td><a href='${kernelPage(r.k.name)}'>${esc(r.k.name)}</a></td>
        ${num(r.cores, String)}<td class=num>${r.at.length}</td>${num(r.busy, ratioText, 'busy')}
        ${num(r.median, ratioText, 'median')}${num(r.most, ratioText, 'most')}${anySpeedup
          ? `<td class=num>${r.both.length}</td>${num(r.speedup, speedupText, 'speedup')}
             ${num(r.efficiency, (x) => `${Math.round(100 * x)}%`, 'efficiency')}` : ''}</tr>`)
        .join('')}</table></div>
    <p class=muted>Each ratio's column is shaded from green at its highest to red at its
      lowest.</p>`;
}

// The size of each kernel's binary, a dot a kernel on one logarithmic axis, and a table of the
// same with what each counts.  Of the kernel, not of a case: the filters do not touch it.
function sizes(b) {
  const rows = b.ran.filter((k) => k.binary);
  if (!rows.length) return '';
  const unmeasured = b.ran.filter((k) => !k.binary);
  const mb = (k) => k.binary.bytes / 1e6;
  const lo = Math.log10(Math.min(...rows.map(mb))) - 0.15;
  const hi = Math.max(lo + 1, Math.log10(Math.max(...rows.map(mb))) + 0.15);
  const left = labelWidth(rows.map((k) => k.name));
  const width = 720;
  const right = 90;
  const rowHeight = 28;
  const height = rows.length * rowHeight + 26;
  const x = (v) => left + (Math.log10(v) - lo) / (hi - lo) * (width - left - right);
  const out = grid(logTicks(lo, hi), x, 0, height, mbTick);
  rows.forEach((k, r) => {
    const mid = r * rowHeight + rowHeight / 2;
    out.push(`<text class=name x=${left - 10} y=${mid} text-anchor=end
      dominant-baseline=middle>${esc(k.name)}</text>`,
      `<circle class=solved cx=${x(mb(k))} cy=${mid} r=5 style='fill-opacity:1' />`,
      `<text class=axis x=${x(mb(k)) + 10} y=${mid} dominant-baseline=middle>${
        megabytesText(mb(k))}</text>`);
  });
  return `<h2>Binary size</h2>
    <p class=muted>The size on disk of each kernel's binary, as built for the last run: the files
      kernels.json names for it, beside the language runtime its runner runs on (Python, .NET)
      and the system's own libraries. A Rust runner's is its executable, the kernel and its cases
      compiled in; Aetheris's, its assemblies; OCCT's, the cadquery-ocp wheel, OCCT's libraries,
      their bundled dependencies and OCP's Python bindings (OCP also loads VTK, installed beside it, which
      is not counted). The scale is logarithmic.</p>
    <div class=perf-strip><svg viewBox='0 0 ${width} ${height}' style='max-width:${width}px;
      width:100%' role=img aria-label='${esc(rows.map((k) => `${k.name}: ${megabytesText(mb(k))}`)
        .join('; '))}'>${out.join('')}</svg></div>
    <div class=scroll><table><tr><th>kernel</th><th class=num>size</th><th class=num>files</th>
      <th>target</th></tr>${rows.map((k) => `<tr><td><a href='${kernelPage(k.name)}'>${
        esc(k.name)}</a></td><td class=num data-v=${k.binary.bytes}>${megabytesText(mb(k))}</td><td class=num>${
        k.binary.files}</td><td class=muted>${k.binary.of.map((o) => (o.distribution
          ? `the Python distribution <code>${esc(o.distribution)}</code>` : `<code>${esc(o)}</code>`))
        .join(', ')}</td></tr>`).join('')}</table></div>
    ${unmeasured.length ? `<p class=muted>Not measured: ${unmeasured.map((k) => esc(k.name))
      .join(', ')}, whose last runs were made before the harness measured binaries.</p>` : ''}`;
}
