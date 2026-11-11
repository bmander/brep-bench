// A kernel's page (kernel.html?k=NAME): what it is, its outcomes by category, its refusals by
// reason, what it got wrong, what it worked around, which implementations are elegant, which
// solids are faceted, what its STEP files are, and how long it took.
import { load, tally, bar, legend, esc, casePage, kernelPage, OUTCOMES,
         failed, secondsText, megabytesText, quantile, STEP_VERDICTS, stepStanding,
         stepTally } from './board.js';
import { sortable } from './sort.js';

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
                 k.license && `license ${k.license}`].filter(Boolean).join(' · ');
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
    k.measured === 'mesh' ? ' · no exact measures of its own: graded on its triangles, to 1e-3, where it writes no STEP' : ''}${
    { writes: ' · writes STEP', cannot: ' · cannot write STEP', unset: ' · STEP not set up' }[
      stepStanding(k).standing]}</p>
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
        <td class='num muted'>${c.cases.length}</td><td data-v=${(ct.pass + ct.built) / c.cases.length}>${bar(ct)}</td>${OUTCOMES.map((o) =>
          `<td class=num data-v=${ct[o]}>${counted(o, ct[o], { cat: c.name })}</td>`).join('')}</tr>`;
    }).join('')}</table></div>`;

  const link = (i) => `<a href='${casePage(b.cases[i].id, k.name)}'>${esc(b.cases[i].title)}</a>
    <small class=muted>${esc(b.cases[i].id)}</small>`;
  const all = b.cases.map((c) => c.i);

  // the implementations flagged elegant, and why
  const neat = all.filter((i) => k.elegance[i]);
  out += `<h2 id=elegance>Elegance</h2>`;
  out += neat.length ? `<p class=muted>Implementations that stand out beside the other kernels'
    of the same case: notably clean, or showing an idea in the kernel's API or design.</p>
    <div class=scroll><table><tr><th>case</th><th>why</th></tr>${neat.map((i) =>
      `<tr><td class=case>${link(i)}</td><td class=why>${esc(k.elegance[i])}</td></tr>`).join('')}
    </table></div>` : '<p class=muted>None flagged.</p>';

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

  // faceted solids: a mesh stored as B-rep faces, many times as many as the other kernels'
  const faceted = all.filter((i) => k.faceted[i] != null);
  out += `<h2 id=faceted>Faceted</h2>`;
  out += faceted.length ? `<p class=muted>Solids stored as meshes in B-rep form: at least ten times
    as many faces as the median of the other kernels' solids of the case, by each runner's own
    count. The mark changes no grade.</p>
    <div class=scroll><table><tr><th>case</th><th class=num>faces</th>
    <th class=num title='the median of the other kernels&#39; solids of the case'>others'</th>
    <th>outcome</th></tr>${faceted.map((i) => `<tr><td class=case>${link(i)}</td>
      <td class=num data-v=${k.faces[i]}>${k.faces[i]}</td><td class=num data-v=${k.faceted[i]}>${
      k.faceted[i]}</td><td class='cell ${k.outcomes[i]}'>${k.outcomes[i]}</td></tr>`).join('')}
    </table></div>` : '<p class=muted>None.</p>';

  out += stepSection(k, all, link, b.repo);

  // times, CPU time and memory
  const timed = all.filter((i) => k.seconds[i] != null);
  if (timed.length) {
    const median = (list) => quantile(list.sort((x, y) => x - y), 0.5);
    const total = timed.reduce((s, i) => s + k.seconds[i], 0);
    const cpu = timed.filter((i) => k.cpu[i] != null);
    const mb = timed.filter((i) => k.mb[i] != null).map((i) => k.mb[i]);
    const slowest = [...timed].sort((x, y) => k.seconds[y] - k.seconds[x]).slice(0, 10);
    out += `<h2>Times and memory</h2><p class=muted>${total.toFixed(1)} s over ${timed.length}
      cases; the median ${secondsText(median(timed.map((i) => k.seconds[i])))}. Starting its runner,
      with nothing to build, takes ${secondsText(k.start.seconds)}${k.start.measured ? ''
        : ' (its fastest run: not measured yet)'} of each.${cpu.length
        ? ` ${cpu.reduce((s, i) => s + k.cpu[i], 0).toFixed(1)} s of CPU time.` : ''}${mb.length
        ? ` The median peak memory ${megabytesText(median([...mb]))}, the most ${
          megabytesText(Math.max(...mb))}.` : ''} Times include starting the runner's process, and
      none of it is graded.${k.binary ? ` Its binary is ${megabytesText(k.binary.bytes / 1e6)} in
      ${k.binary.files} file${k.binary.files === 1 ? '' : 's'}.` : ''}
      <a href='performance.html'>Performance</a> sets them beside the other kernels', the start
      left out.</p>
      <div class=scroll><table><tr><th>slowest</th><th class=num>time</th>
      <th class=num>CPU time</th><th class=num>memory</th><th>outcome</th></tr>
      ${slowest.map((i) => `<tr><td class=case>${link(i)}</td><td class=num data-v='${k.seconds[i] ?? ''}'>${
        secondsText(k.seconds[i])}</td><td class=num data-v='${k.cpu[i] ?? ''}'>${secondsText(k.cpu[i])}</td>
        <td class=num data-v='${k.mb[i] ?? ''}'>${megabytesText(k.mb[i])}</td><td class='cell ${k.outcomes[i]}'>${
        k.outcomes[i]}</td></tr>`).join('')}</table></div>`;
  }
  main.innerHTML = out;
  sortable(main);
  // drawn after the page loaded: a section the address names (#elegance) is scrolled to now
  if (location.hash) document.getElementById(location.hash.slice(1))?.scrollIntoView();
}

/** What kernel `k`'s STEP files are: its verdicts tallied, and the cases where it wrote none or
 * none that is a closed B-rep, with why; or that its kernel cannot write STEP, or that its
 * runner has not been set up to. */
function stepSection(k, all, link, repo) {
  const { standing, why } = stepStanding(k);
  let out = `<h2 id=step>STEP</h2>`;
  if (standing === 'cannot') {
    return `${out}<p><span class='badge missing'>cannot</span> It cannot write its solids as STEP:
      ${esc(why)}.</p>`;
  }
  if (standing === 'unset') {
    return `${out}<p><span class='badge missing'>not set up</span> Its runner has not been set up
      to write its solids as STEP yet, so there is nothing to judge; that says nothing of whether
      the kernel can. See <a href='${esc(repo)}/blob/main/SPEC.md'>SPEC.md</a>
      (STEP) to add it.</p>`;
  }
  const t = stepTally(k);
  const n = t.reduce((s, [, c]) => s + c, 0);
  const badge = (v) => `<span class='badge ${STEP_VERDICTS[v].colour}' title='${
    esc(STEP_VERDICTS[v].means)}'>${STEP_VERDICTS[v].word}</span>`;
  out += `<p class=muted>Each solid it builds it also writes as STEP, with its kernel's own
    writer, in a run of its own; the harness reads the file and judges what it holds. The verdict
    is shown, not graded.</p>`;
  if (!n) return `${out}<p class=muted>No solid built yet to write.</p>`;
  out += `<div class=scroll><table><tr><th>verdict</th><th class=num>solids</th><th>meaning</th></tr>${
    t.map(([v, c]) => `<tr><td>${badge(v)}</td><td class=num data-v=${c}>${c}</td>
      <td class=why>${esc(STEP_VERDICTS[v].means)}</td></tr>`).join('')}</table></div>`;
  // the solids whose STEP is not a closed B-rep, refused and failed first
  const order = ['refused', 'failed', 'open', 'empty', 'triangles', 'faceted', 'unrun'];
  const odd = all.filter((i) => order.includes(k.stepVerdict[i]))
    .sort((x, y) => order.indexOf(k.stepVerdict[x]) - order.indexOf(k.stepVerdict[y]));
  if (odd.length) {
    out += `<div class=scroll><table><tr><th>case</th><th>STEP</th><th>why</th><th>outcome</th></tr>
      ${odd.map((i) => `<tr><td class=case>${link(i)}</td><td>${badge(k.stepVerdict[i])}</td>
        <td class=why>${esc(k.stepWhy[i])}</td><td class='cell ${k.outcomes[i]}'>${
        k.outcomes[i]}</td></tr>`).join('')}</table></div>`;
  }
  return out;
}
