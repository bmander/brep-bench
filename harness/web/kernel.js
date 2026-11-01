// A kernel's page (kernel.html?k=NAME): what it is, its outcomes by category, its refusals by
// reason, what it got wrong, what it worked around, and how long it took.
import { load, tally, bar, legend, esc, casePage, kernelPage, OUTCOMES,
         failed } from './board.js';

const main = document.getElementById('kernel');
load().then(draw, (e) => failed(main, e));

function draw(b) {
  const name = new URLSearchParams(location.search).get('k');
  const k = b.kernels.find((x) => x.name === name);
  if (!k) {
    main.innerHTML = `<h1>Kernels</h1>${name ? `<p>No kernel is called ${esc(name)}.</p>` : ''}
      <ul>${b.kernels.map((x) => `<li><a href='${kernelPage(x.name)}'>${esc(x.name)}</a></li>`)
        .join('')}</ul>`;
    return;
  }
  document.title = `${k.name} · brep-bench`;
  const about = [k.backend && `written in ${k.backend}`,
                 k.frontend && `its cases written in ${k.frontend}`,
                 k.license && `licence ${k.license}`].filter(Boolean).join(' · ');
  let out = `<h1>${esc(k.name)}</h1>
    <p class=muted>${esc(about)}${k.home ? ` · <a href='${esc(k.home)}'>home page</a>` : ''}</p>`;
  if (k.note) out += `<p>${esc(k.note)}</p>`;
  if (!k.outcomes) {
    main.innerHTML = `${out}<p>Awaiting an implementation: none of its cases has been written yet.
      Implementations are welcome from anyone, the kernel's authors first.</p>`;
    return;
  }
  const n = b.cases.length;
  const t = tally(k);
  const explore = (extra) => {
    const p = new URLSearchParams(extra);
    return `explore.html?${p}`;
  };
  const counted = (o, count, extra = {}) => (count
    ? `<a href='${explore({ ...extra, [`f.${k.name}`]: o })}'>${count}</a>` : '<span class=muted>·</span>');
  out += `<p class=muted>${esc(k.version)} · run ${esc(k.run.slice(0, 10))}${
    k.measured === 'mesh' ? ' · no exact measures: graded on its triangles, to 1e-3' : ''}</p>
    <p>It has an implementation of ${n - t.missing} of the ${n} cases.</p>
    <div class=scroll><table><tr><th></th>${OUTCOMES.map((o) => `<th class=num>${o}</th>`).join('')}
      </tr><tr><td>${bar(t)}</td>${OUTCOMES.map((o) => `<td class=num>${counted(o, t[o])}</td>`)
      .join('')}</tr></table></div>
    <p class=legend>${legend()}</p>`;

  out += `<h2>By category</h2><div class=scroll><table><tr><th>category</th>
    <th class=num>cases</th><th></th>${OUTCOMES.map((o) => `<th class=num>${o}</th>`).join('')}
    </tr>${b.categories.map((c) => {
      const ct = tally(k, c.cases);
      return `<tr><td><a href='${explore({ cat: c.name })}'>${esc(c.name)}</a></td>
        <td class='num muted'>${c.cases.length}</td><td>${bar(ct)}</td>${OUTCOMES.map((o) =>
          `<td class=num>${counted(o, ct[o], { cat: c.name })}</td>`).join('')}</tr>`;
    }).join('')}</table></div>`;

  const link = (i) => `<a href='${casePage(b.cases[i].id)}'>${esc(b.cases[i].title)}</a>
    <small class=muted>${esc(b.cases[i].id)}</small>`;
  const all = b.cases.map((c) => c.i);

  // refusals, by the reason given, the commonest first
  const reasons = new Map();
  for (const i of all) {
    if (k.outcomes[i] !== 'refused') continue;
    const why = k.why[i] || '(no reason given)';
    if (!reasons.has(why)) reasons.set(why, []);
    reasons.get(why).push(i);
  }
  const grouped = [...reasons].sort((x, y) => y[1].length - x[1].length || x[0].localeCompare(y[0]));
  out += `<h2>Refused</h2>`;
  out += grouped.length ? `<p class=muted>${t.refused} cases, by the reason given.</p>${grouped.map(
    ([why, cases]) => `<details><summary>${cases.length} × <span style='font-weight:normal'>${
      esc(why)}</span></summary><ul>${cases.map((i) => `<li>${link(i)}</li>`).join('')}</ul>
      </details>`).join('')}` : '<p class=muted>Nothing.</p>';

  // wrong, crashed and timed out
  const bad = all.filter((i) => ['wrong', 'crashed', 'timeout'].includes(k.outcomes[i]));
  out += `<h2>Wrong, crashed or timed out</h2>`;
  out += bad.length ? `<div class=scroll><table><tr><th>case</th><th>outcome</th><th>why</th></tr>
    ${bad.map((i) => `<tr><td class=case>${link(i)}</td><td class='cell ${k.outcomes[i]}'>${
      k.outcomes[i]}</td><td class=why>${esc(k.why[i])}</td></tr>`).join('')}</table></div>`
    : '<p class=muted>Nothing.</p>';

  // workarounds, by the feature lacked (several are joined with commas)
  const lacks = new Map();
  for (const i of all) {
    if (!k.workaround[i]) continue;
    for (const lack of k.workaround[i].split(',').map((s) => s.trim()).filter(Boolean)) {
      if (!lacks.has(lack)) lacks.set(lack, []);
      lacks.get(lack).push(i);
    }
  }
  out += `<h2>Workarounds</h2>`;
  out += lacks.size ? `<p class=muted>Results reached by building the solid from other operations,
    where the kernel lacks a feature the natural construction calls for, by what it lacks.</p>${
    [...lacks].sort((x, y) => y[1].length - x[1].length || x[0].localeCompare(y[0])).map(
      ([lack, cases]) => `<details><summary>${cases.length} × <span style='font-weight:normal'>${
        esc(lack)}</span></summary><ul>${cases.map((i) => `<li>${link(i)} <span class=muted>${
        k.outcomes[i]}</span></li>`).join('')}</ul></details>`).join('')}`
    : '<p class=muted>None.</p>';

  // times
  const timed = all.filter((i) => k.outcomes[i] !== 'missing' && k.seconds[i] != null);
  if (timed.length) {
    const secs = timed.map((i) => k.seconds[i]).sort((x, y) => x - y);
    const total = secs.reduce((s, x) => s + x, 0);
    const median = secs[Math.floor(secs.length / 2)];
    const slowest = [...timed].sort((x, y) => k.seconds[y] - k.seconds[x]).slice(0, 10);
    out += `<h2>Times</h2><p class=muted>${total.toFixed(1)} s over ${timed.length} cases;
      the median ${median} s. Times include starting the runner's process, and are not graded.</p>
      <div class=scroll><table><tr><th>slowest</th><th class=num>seconds</th><th>outcome</th></tr>
      ${slowest.map((i) => `<tr><td class=case>${link(i)}</td><td class=num>${k.seconds[i]}</td>
        <td class='cell ${k.outcomes[i]}'>${k.outcomes[i]}</td></tr>`).join('')}</table></div>`;
  }
  main.innerHTML = out;
}
