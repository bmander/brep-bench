// The overview: the kernels, the categories by kernels, and a strip of every case.
import { load, tally, words, bar, legend, esc, casePage, kernelPage, groupOf, markColours,
         failed } from './board.js';

const main = document.getElementById('board');
load().then(draw, (e) => failed(main, e));

function draw(b) {
  const n = b.cases.length;
  main.innerHTML = `
    <h2>Kernels</h2>${kernels(b)}
    <p class=legend>${legend()}</p>
    <p class=muted>Missing means no implementation yet, and says nothing about the kernel.
      Implementations for the kernels awaiting one are welcome from anyone, the kernel's authors
      first. † counts results reached by working around a feature the kernel lacks.</p>
    <h2>By category</h2>
    <p class=muted>How many of each category's cases each kernel passes (or, unsettled, builds
      soundly). Open a category to read its cases.</p>
    ${categories(b)}
    <h2>Every case</h2>
    <p class=muted>All ${n} cases, a column each, grouped by category; a row a kernel. Point at
      one for its outcomes, or open it. <a href='explore.html'>The explorer</a> lists them.</p>
    <p class=legend>${legend()}</p>
    <div class=strip><canvas role=img aria-label='The outcome each kernel had of every case. The explorer lists them as a table.'></canvas></div>
    <div class=tip hidden></div>`;
  strip(b, main.querySelector('.strip canvas'), main.querySelector('.tip'));
}

function kernels(b) {
  const total = b.cases.length;
  const rows = b.kernels.map((k) => {
    let name = `<a href='${kernelPage(k.name)}'>${esc(k.name)}</a>`;
    if (k.measured === 'mesh') {
      name += ` <span class=muted title='no exact measures: graded on its triangles, to 1e-3'>(mesh)</span>`;
    }
    if (k.outcomes) name += `<small class=muted>${esc(k.version)} · run ${esc(k.run.slice(0, 10))}</small>`;
    if (k.note) name += `<small class=muted>${esc(k.note)}</small>`;
    const about = `<td class=kernel>${name}</td><td>${esc(k.backend || '')}</td>
      <td class=wrap>${esc(k.frontend || '')}</td><td class=wrap>${esc(k.license || '')}</td>`;
    if (!k.outcomes) {
      return `<tr>${about}<td class=muted colspan=4>awaiting an implementation</td></tr>`;
    }
    const t = tally(k);
    const worked = k.workaround.filter((w, i) => w && (k.outcomes[i] === 'pass' ||
                                                       k.outcomes[i] === 'built')).length;
    return `<tr>${about}<td class=num>${total - t.missing} <span class=muted>of ${total}</span></td>
      <td>${bar(t)}</td><td class=counts>${esc(words(t))}</td>
      <td class='num muted'>${worked || ''}</td></tr>`;
  });
  return `<div class=scroll><table><tr><th>kernel</th>
    <th title='the language the kernel is written in'>back end</th>
    <th title='the language its implementations of the cases are written in'>front end</th>
    <th>licence</th><th class=num title='cases it has an implementation of'>implemented</th>
    <th>outcomes</th><th></th>
    <th class=num title='passed or built by working around a feature the kernel lacks'>†</th></tr>
    ${rows.join('')}</table></div>`;
}

function categories(b) {
  const head = b.ran.map((k) =>
    `<th class=k><a href='${kernelPage(k.name)}'>${esc(k.name)}</a></th>`).join('');
  const row = (label, href, indices) => {
    const cells = b.ran.map((k) => {
      const t = tally(k, indices);
      const good = t.pass + t.built;
      return `<td class=c title='${esc(k.name)}: ${esc(words(t))}'><a href='${href}'>${bar(t)}
        <span class=n>${good}/${indices.length}</span></a></td>`;
    }).join('');
    return `<tr><th><a href='${href}'>${esc(label)}</a></th>
      <td class='num muted'>${indices.length}</td>${cells}</tr>`;
  };
  const all = b.cases.map((c) => c.i);
  const rows = [row('every case', 'explore.html', all)].concat(b.categories.map((c) =>
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
    const pad = 14;
    const w = tip.offsetWidth;
    const hgt = tip.offsetHeight;
    tip.style.left = `${Math.min(event.clientX + pad, innerWidth - w - 8)}px`;
    tip.style.top = `${event.clientY + pad + hgt > innerHeight ? event.clientY - hgt - pad
                                                             : event.clientY + pad}px`;
  });
  canvas.addEventListener('mouseleave', () => { tip.hidden = true; });
  canvas.addEventListener('click', (event) => {
    const h = hit(event);
    if (h) location.href = casePage(b.cases[h.i].id);
  });
  new ResizeObserver(paint).observe(canvas);
  matchMedia('(prefers-color-scheme: dark)').addEventListener('change', paint);
}
