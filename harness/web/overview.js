// The overview: the kernels, the categories by kernels, and a strip of every case.
import { load, tally, words, bar, legend, esc, casePage, kernelPage, groupOf, markColours,
         failed, placeTip, solved, secondsText, megabytesText, STEP_VERDICTS, stepStanding,
         stepTally } from './board.js';
import { sortable } from './sort.js';

/** Kernel `k`'s geometric mean time over the cases it passes or builds, its runner's start-up
 * left out (each time no less than its `timeFloor`, board.js), and how many cases it is over;
 * null if none. */
function meanTime(k) {
  const at = k.built.flatMap((x, i) => (x != null && solved(k.outcomes[i]) ? [i] : []));
  if (!at.length) return null;
  const logs = at.reduce((s, i) => s + Math.log(Math.max(k.built[i], k.timeFloor[i])), 0);
  return { seconds: Math.exp(logs / at.length), n: at.length };
}

/** Kernel `k`'s geometric mean lines of code over the cases it passes or builds, and how many
 * cases it is over; null if none. */
function meanLoc(k) {
  const xs = k.loc.filter((x, i) => x != null && x > 0 && solved(k.outcomes[i]));
  if (!xs.length) return null;
  const logs = xs.reduce((s, x) => s + Math.log(x), 0);
  return { lines: Math.exp(logs / xs.length), n: xs.length };
}

const main = document.getElementById('board');
load().then(draw, (e) => failed(main, e));

function draw(b) {
  const n = b.cases.length;
  main.innerHTML = `
    <h2>Kernels</h2>${kernels(b)}
    <p class=legend>${legend()}</p>
    <p class=muted>Missing means no implementation yet, and says nothing about the kernel.
      Implementations for the kernels awaiting one are welcome from anyone, the kernel's authors
      first. <em>Workarounds</em> counts results reached by working around a feature the kernel
      lacks. <em>Faceted</em> counts those whose solid is a mesh stored as B-rep faces: at least
      ten times as many faces as the other kernels' solids of the case have (their median), by
      each runner's own count. <em>STEP</em> is how many of the solids it built its own STEP
      writer wrote as a closed B-rep of faces on surfaces (the rest faceted, open, refused or
      failed), or that it <em>cannot</em> write STEP, or that its runner is <em>not set up</em> to
      yet. <em>Mean time</em> is the geometric mean (the mean on a logarithmic scale) of the
      kernel's own time over the cases it passes or builds, what starting its runner costs left
      out; <em>mean LOC</em> is the geometric mean of the lines of code (comments and blank lines
      left out) of its implementations of those same cases; <em>binary</em>
      is the size on disk of what it runs. <a href='performance.html'>Performance</a> has
      both in full.</p>
    <h2>By category</h2>
    <p class=muted>How many of each category's cases each kernel passes (or, unsettled, builds
      soundly). Open a category to read its cases.</p>
    ${categories(b)}
    ${elegance(b)}
    <h2>Every case</h2>
    <p class=muted>All ${n} cases, a column each, grouped by category; a row a kernel. Point at
      one for its outcomes, or open it. <a href='explore.html'>The explorer</a> lists them.</p>
    <p class=legend>${legend()}</p>
    <div class=strip><canvas role=img aria-label='The outcome each kernel had of every case. The explorer lists them as a table.'></canvas></div>
    <div class=tip hidden></div>`;
  sortable(main);
  strip(b, main.querySelector('.strip canvas'), main.querySelector('.tip'));
}

function kernels(b) {
  const total = b.cases.length;
  const rows = b.kernels.map((k) => {
    let name = `<a href='${kernelPage(k.name)}'>${esc(k.name)}</a>`;
    if (k.measured === 'mesh') {
      name += ` <span class=muted title='no exact measures of its own: graded on its triangles, to 1e-3, where it writes no STEP'>(mesh)</span>`;
    }
    if (k.outcomes) name += `<small class=muted>${esc(k.version)} · run ${esc(k.run.slice(0, 10))}</small>`;
    if (k.note) name += `<small class=muted>${esc(k.note)}</small>`;
    const about = `<td class=kernel data-v='${esc(k.name)}'>${name}</td><td>${esc(k.backend || '')}</td>
      <td class=wrap>${esc(k.frontend || '')}</td><td class=wrap>${esc(k.license || '')}</td>`;
    if (!k.outcomes) {
      return `<tr>${about}<td class=muted colspan=9>awaiting an implementation</td></tr>`;
    }
    const t = tally(k);
    const worked = k.workaround.filter((w, i) => w && solved(k.outcomes[i])).length;
    const faceted = k.faceted.filter((f, i) => f != null && solved(k.outcomes[i])).length;
    return `<tr>${about}<td class=num data-v=${total - t.missing}>${total - t.missing} <span class=muted>of ${total}</span></td>
      <td data-v=${t.pass + t.built}>${bar(t)}</td><td class=counts>${esc(words(t))}</td>
      <td class='num muted' data-v=${worked}>${worked || ''}</td>
      <td class='num muted' data-v=${faceted}>${faceted ? `<a href='${kernelPage(k.name)}#faceted'>${faceted}</a>` : ''}</td>${stepCell(k)}${timeCell(k)}${locCell(k)}${sizeCell(k)}</tr>`;
  });
  return `<div class=scroll><table><tr><th>kernel</th>
    <th title='the language the kernel is written in'>back end</th>
    <th title='the language its implementations of the cases are written in'>front end</th>
    <th>license</th><th class=num title='cases it has an implementation of'>implemented</th>
    <th>outcomes</th><th></th>
    <th class=num title='passed or built by working around a feature the kernel lacks'>workarounds</th>
    <th class=num title='passed or built as a faceted solid: a mesh stored as B-rep faces, at least ten times as many as the median of the other kernels&#39; solids of the case'>faceted</th>
    <th title='what its STEP files are: of the solids it built, how many its STEP judged a closed B-rep; or that it cannot write STEP, or has not been set up to'>STEP</th>
    <th class=num title='the geometric mean of its own time (start-up left out) over the cases it passes or builds'>mean time</th>
    <th class=num title='the geometric mean of the lines of code (comments and blank lines left out) of its implementations of the cases it passes or builds'>mean LOC</th>
    <th class=num title='the size on disk of its binary, as built for its last run'>binary</th></tr>
    ${rows.join('')}</table></div>`;
}

const dash = '<td class=num><span class=muted>–</span></td>';

/** Kernel `k`'s STEP, in a cell: how many of the solids it built its STEP files hold as closed
 * B-reps, the rest in its title; or that it cannot write STEP, or has not been set up to. */
function stepCell(k) {
  const { standing, why } = stepStanding(k);
  const link = (text) => `<a href='${kernelPage(k.name)}#step'>${text}</a>`;
  if (standing === 'cannot') {
    return `<td class=muted data-v=-1 title='${esc(`cannot write STEP: ${why}`)}'>${link('cannot')}</td>`;
  }
  if (standing === 'unset') {
    return `<td class=muted data-v=-2 title='its runner has not been set up to write STEP'>${
      link('not set up')}</td>`;
  }
  const t = stepTally(k);
  const n = t.reduce((s, [, c]) => s + c, 0);
  if (!n) return dash;
  const brep = (t.find(([v]) => v === 'brep') || [, 0])[1];
  const title = t.map(([v, c]) => `${c} ${STEP_VERDICTS[v].word}`).join(', ');
  return `<td data-v=${brep / n} title='${esc(title)}'>${link(`${brep} <span class=muted>of ${n} B-rep</span>`)}</td>`;
}

function timeCell(k) {
  const m = meanTime(k);
  if (!m) return dash;
  // timed between its runner's pauses (its warm start) where it paused, else less the start-up
  const start = k.paused ? ', loading the runner left out'
    : k.start.measured ? `, start-up of ${secondsText(k.start.seconds)} left out`
    : ', its fastest run left out as start-up';
  return `<td class=num data-v=${m.seconds} title='geometric mean over ${m.n} case${m.n === 1 ? '' : 's'}${
    esc(start)}'>${secondsText(m.seconds)}</td>`;
}

function locCell(k) {
  const m = meanLoc(k);
  if (!m) return dash;
  return `<td class=num data-v=${m.lines} title='geometric mean over ${m.n} case${m.n === 1 ? '' : 's'}'>${
    Math.round(m.lines)}</td>`;
}

function sizeCell(k) {
  if (!k.binary) return dash;
  const files = `${k.binary.files} file${k.binary.files === 1 ? '' : 's'}`;
  return `<td class=num data-v=${k.binary.bytes} title='${esc(files)}'>${megabytesText(k.binary.bytes / 1e6)}</td>`;
}

// The implementations each kernel's authors call elegant (its elegance.toml), kernel by kernel in
// kernels.json's order: what each case asks, said cleanly, or an idea in the kernel's API.
function elegance(b) {
  const rows = [];
  for (const k of b.ran) {
    for (const c of b.cases.filter((x) => k.elegance[x.i])) rows.push(`<tr><td><a href='${kernelPage(k.name)}#elegance'>${
      esc(k.name)}</a></td><td class=case><a href='${casePage(c.id, k.name)}'>${esc(c.title)}</a>
      <small class=muted>${esc(c.id)}</small></td><td class=why>${esc(k.elegance[c.i])}</td></tr>`);
  }
  if (!rows.length) return '';
  return `<h2 id=elegance>Elegance</h2>
    <p class=muted>Implementations that stand out when a case is read across the kernels: a
      solution notably cleaner than the rest, or an idea in a kernel's API or design that makes
      the case say what it means. Open one to read its code beside the others'.</p>
    <div class=scroll><table class=elegance><tr><th>kernel</th><th>case</th><th>why</th></tr>
    ${rows.join('')}</table></div>`;
}

function categories(b) {
  const head = b.ran.map((k) =>
    `<th class=k><a href='${kernelPage(k.name)}'>${esc(k.name)}</a></th>`).join('');
  const row = (label, href, indices, fixed) => {
    const cells = b.ran.map((k) => {
      const t = tally(k, indices);
      const good = t.pass + t.built;
      return `<td class=c data-v='${indices.length ? good / indices.length : ''}'
        title='${esc(k.name)}: ${esc(words(t))}'><a href='${href}'>${bar(t)}
        <span class=n>${good}/${indices.length}</span></a></td>`;
    }).join('');
    return `<tr${fixed ? ' data-fixed' : ''}><th><a href='${href}'>${esc(label)}</a></th>
      <td class='num muted'>${indices.length}</td>${cells}</tr>`;
  };
  const all = b.cases.map((c) => c.i);
  const rows = [row('every case', 'explore.html', all, true)].concat(b.categories.map((c) =>
    row(c.name, `explore.html?cat=${encodeURIComponent(c.name)}`, c.cases)));
  return `<div class=scroll><table class=grid-cats><tr><th>category</th>
    <th class=num>cases</th>${head}</tr>${rows.join('')}</table></div>`;
}

// The strip: a column a case, grouped by category with a column's gap between them and each
// category's name above it where it fits; a row a kernel.  It wraps into bands, its cells as
// large as keep it under about 600 px tall.
function strip(b, canvas, tip) {
  const ran = b.ran;
  const ctx = canvas.getContext('2d');
  let layout = null;

  function plan(width) {
    ctx.font = '12px system-ui, sans-serif';
    const left = Math.ceil(Math.max(...ran.map((k) => ctx.measureText(k.name).width))) + 12;
    const columns = (cell) => Math.max(8, Math.floor((width - left) / cell));
    let best = null;
    for (const cell of [14, 12, 10, 8, 7, 6, 5, 4, 3, 2]) {
      const per = columns(cell);
      // each case's band and column, and each category's run of columns in each band it is in
      const at = [];
      const runs = [];
      let band = 0;
      let col = 0;
      for (const cat of b.categories) {
        if (col && col + 1 >= per) { band++; col = 0; }
        else if (col) col++;
        let run = null;
        for (const i of cat.cases) {
          if (col >= per) { band++; col = 0; }
          if (!run || run.band !== band) runs.push(run = { name: cat.name, band, col, span: 0 });
          run.span++;
          at[i] = [band, col++];
        }
      }
      // a row is never less than 10 px tall, so its kernel's name fits beside it
      const row = Math.max(cell, 10);
      const bandHeight = 16 + ran.length * row + 10;
      best = { cell, row, per, left, at, runs, bands: band + 1, bandHeight };
      if (best.bands * bandHeight <= 640) break;
    }
    // the case at each band and column
    best.grid = new Map();
    best.at.forEach(([band, col], i) => best.grid.set(band * best.per + col, i));
    return best;
  }

  function paint() {
    const width = canvas.clientWidth;
    layout = plan(width);
    const { cell, row, left, at, runs, bands, bandHeight } = layout;
    const height = bands * bandHeight;
    const ratio = window.devicePixelRatio || 1;
    canvas.style.height = `${height}px`;
    canvas.width = Math.round(width * ratio);
    canvas.height = Math.round(height * ratio);
    ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
    ctx.clearRect(0, 0, width, height);
    const colours = markColours();
    const ink = getComputedStyle(canvas).color;
    const muted = getComputedStyle(document.documentElement).getPropertyValue('--muted').trim();
    ctx.font = `${Math.min(12, row)}px system-ui, sans-serif`;
    ctx.textBaseline = 'middle';
    ctx.fillStyle = ink;
    for (let band = 0; band < bands; band++) {
      const top = band * bandHeight + 16;
      ran.forEach((k, r) => ctx.fillText(k.name, 0, top + r * row + row / 2));
    }
    // each category's name over each of its runs, where it fits; a run continued from the band
    // before is marked so
    ctx.font = '12px system-ui, sans-serif';
    ctx.fillStyle = muted;
    ctx.textBaseline = 'alphabetic';
    runs.forEach((run, j) => {
      const label = j && runs[j - 1].name === run.name ? `${run.name} (continued)` : run.name;
      const fits = [label, run.name].find((t) => ctx.measureText(t).width <= (run.span + 1) * cell);
      if (fits) ctx.fillText(fits, left + run.col * cell, run.band * bandHeight + 12);
    });
    const gapX = cell >= 5 ? 1 : 0;
    for (const [i, [band, col]] of at.entries()) {
      const x = left + col * cell;
      const top = band * bandHeight + 16;
      ran.forEach((k, r) => {
        ctx.fillStyle = colours[groupOf(k.outcomes[i])];
        ctx.fillRect(x, top + r * row, cell - gapX, row - 1);
      });
    }
  }

  function hit(event) {
    if (!layout) return null;
    const box = canvas.getBoundingClientRect();
    const x = event.clientX - box.left - layout.left;
    const y = event.clientY - box.top;
    if (x < 0) return null;
    const band = Math.floor(y / layout.bandHeight);
    const row = Math.floor((y - band * layout.bandHeight - 16) / layout.row);
    if (row < 0 || row >= ran.length) return null;
    const i = layout.grid.get(band * layout.per + Math.floor(x / layout.cell));
    return i === undefined ? null : { i, row };
  }

  canvas.addEventListener('mousemove', (event) => {
    const h = hit(event);
    if (!h) { tip.hidden = true; return; }
    const c = b.cases[h.i];
    const lines = ran.map((k, r) => {
      const o = k.outcomes[h.i];
      const extra = [k.workaround[h.i] && `† ${k.workaround[h.i]}`, k.why[h.i]]
        .filter(Boolean).join(' · ');
      const text = `${esc(k.name)}: ${o}${extra ? ` <span class=muted>${esc(extra)}</span>` : ''}`;
      return r === h.row ? `<div><strong>${text}</strong></div>` : `<div>${text}</div>`;
    }).join('');
    tip.innerHTML = `<b>${esc(c.title)}</b><span class=muted>${esc(c.id)}</span>${lines}`;
    tip.hidden = false;
    placeTip(tip, event);
  });
  canvas.addEventListener('mouseleave', () => { tip.hidden = true; });
  canvas.addEventListener('click', (event) => {
    const h = hit(event);
    if (h) location.href = casePage(b.cases[h.i].id, ran[h.row].name);
  });
  new ResizeObserver(paint).observe(canvas);
  matchMedia('(prefers-color-scheme: dark)').addEventListener('change', paint);
}
