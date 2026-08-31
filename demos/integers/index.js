import { ready, $, ink, rgb, say, fit } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, bars, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const COLS = 40;
const LINES = 25;
const PAGE = 25;
const BUDGET = 14;
const s = seeds();
const params = query(['value']);
const win = JSON.parse(m.census_window());
const ceiling = Number(win.ceiling);
let counts = new Uint32Array(ceiling);
let state = { done: 0, total: win.registry, rows: 0, depth: win.depths[0], next: win.depths[0], pending: 0, complete: false };
let report = null;
let picked = Math.min(ceiling, Math.max(1, +(params.get('value') ?? $('value').value) || 16));
let page = 0;
let walking = true;

function mix(from, to, at) {
  const [a, b] = [rgb(from), rgb(to)];
  return `rgb(${a.map((c, k) => Math.round(c + (b[k] - c) * at)).join(',')})`;
}

function shade(count, peak) {
  if (count === 0) return ink.orange;
  if (count === 1) return ink.dim;
  return mix(ink.blue, ink.gold, Math.log(count) / peak);
}

function cell(text, klass = '') {
  return `<td${klass ? ` class="${klass}"` : ''}>${text}</td>`;
}

const field = keep(() => {
  const canvas = $('field');
  const side = canvas.clientWidth / COLS;
  const [ctx, w, h] = fit(canvas, Math.round(side * LINES));
  ctx.clearRect(0, 0, w, h);
  const peak = Math.log(Math.max(2, ...counts));
  for (let i = 0; i < counts.length; i++) {
    ctx.fillStyle = shade(counts[i], peak);
    ctx.fillRect((i % COLS) * side, Math.floor(i / COLS) * side, Math.max(1, side - 1), Math.max(1, side - 1));
  }
  const at = picked - 1;
  ctx.strokeStyle = ink.fg;
  ctx.lineWidth = 2;
  ctx.strokeRect((at % COLS) * side - 1.5, Math.floor(at / COLS) * side - 1.5, side + 2, side + 2);
});

const split = keep(() => {
  if (!report) return;
  const b = board($('split'), 62, { top: 20, bottom: 24 });
  const parts = [
    ['missed', report.never, ink.orange],
    ['written once', report.once, ink.dim],
    ['written by many', report.multiple, ink.blue],
  ];
  let at = 0;
  let label = b.x(0);
  for (const [name, count, color] of parts) {
    b.ctx.fillStyle = color;
    b.ctx.fillRect(b.x(at / ceiling), b.roof, Math.max(1, (b.wide * count) / ceiling - 1), b.floor - b.roof);
    label = tag(b, `${name} ${count}`, color, 'left', label, b.h - 8) + 14;
    at += count;
  }
  tag(b, `1 to ${ceiling} at ${state.depth} rendered terms`, ink.dim);
});

const champions = keep(() => {
  const rows = JSON.parse(m.census_champions(20));
  const b = board($('top'), 210);
  bars(b, rows.map((row) => row.rows), { color: (i) => (rows[i].value === picked ? ink.gold : ink.blue) });
  axis(b, rows.map((row, i) => [(i + 0.5) / rows.length, row.value]));
  tag(b, 'rows writing the integer, the twenty heaviest', ink.dim);
  tag(b, `leader ${rows[0].value} at ${rows[0].rows} rows`, ink.fg, 'right');
  $('top').onpointerdown = (event) => {
    const box = $('top').getBoundingClientRect();
    const at = Math.floor(((event.clientX - box.left - 14) / (box.width - 28)) * rows.length);
    if (rows[at]) pick(rows[at].value);
  };
});

function tables() {
  $('ladder').innerHTML =
    report.depths
      .map((row) => `<tr>${cell(row.depth, 'num')}${cell(row.written, 'num')}${cell(row.never, 'num')}${cell(row.once, 'num')}${cell(row.first_miss, 'num')}${cell(row.deepenable, 'num')}</tr>`)
      .join('') || `<tr>${cell(`the ${win.depths[0]}-term pass is still walking`, 'dim')}<td colspan="5"></td></tr>`;
  $('ladder-note').textContent = state.complete ? 'the last row is the pinned window' : `${state.pending} rows are still cut by the cap`;
  $('bands').innerHTML = report.bands
    .map((band) => `<tr>${cell(`${band.first} to ${band.last}`, 'num')}${cell(band.width, 'num')}${cell(band.missed, 'num')}${cell(band.density.toFixed(6), 'num')}</tr>`)
    .join('');
  $('tiers').innerHTML = report.tiers
    .map((tier) => `<tr>${cell(tier.tier)}${cell(tier.rows, 'num')}${cell(tier.written, 'num')}${cell(tier.alone, 'num')}</tr>`)
    .join('');
  $('misses').textContent = JSON.parse(m.census_misses(30)).join(', ') || 'no integer of the window is missed';
}

function writers() {
  const found = JSON.parse(m.census_writers(picked, page, PAGE));
  $('rows').textContent = found.rows;
  for (const tier of found.tiers) $(`tier-${tier.tier}`).textContent = tier.rows;
  $('verdict').textContent = found.rows
    ? `${picked} is written by ${found.rows === 1 ? 'exactly one row' : `${found.rows} rows`}`
    : `${picked} is missed: no row of the ${win.registry} writes it inside the window`;
  $('verdict').style.color = found.rows === 0 ? ink.orange : found.rows === 1 ? ink.gold : ink.fg;
  $('verdict-note').textContent = `${state.rows} rows read at ${state.depth} rendered terms`;
  $('page-out').textContent = found.rows ? `${page + 1} of ${Math.ceil(found.rows / PAGE)}` : '0';
  $('prev').disabled = page === 0;
  $('next').disabled = (page + 1) * PAGE >= found.rows;
  $('body').innerHTML =
    found.shown
      .map((row) => {
        const place = row.axis === 'level' ? `level ${row.term}` : `side ${row.side}`;
        return `<tr>${cell(`<a href="../sequences?q=${row.name}">${row.name}</a>`, 'mono')}${cell(`${row.measure} · ${row.axis}`, 'mono')}${cell(row.closed || 'none known', 'mono')}${cell(`term ${row.index + 1}, ${place}`, 'num')}${cell(row.head.join(', '), 'num')}</tr>`;
      })
      .join('') || `<tr>${cell('no row of the registry writes it', 'dim')}<td colspan="4"></td></tr>`;
}

function progress() {
  $('read').textContent = `${state.rows} of ${win.registry} rows`;
  $('depth').textContent = state.complete ? `${state.depth}, the pinned cap` : state.depth;
  $('bar').style.width = `${(100 * state.done) / Math.max(1, state.total)}%`;
  $('bar').style.background = walking ? ink.blue : ink.green;
  $('window-note').textContent = walking
    ? `walking ${state.done} of ${state.total} rows at ${state.depth} terms`
    : state.complete
      ? `complete at the pinned ${win.cap}-term window`
      : `${state.pending} rows are cut by the ${state.depth}-term cap`;
  $('deepen').hidden = walking || state.complete;
  $('deepen').textContent = `deepen to ${state.next} terms · ${state.pending} rows`;
}

function refresh() {
  counts = m.census_counts();
  report = JSON.parse(m.census_report());
  $('never').textContent = report.never;
  $('once').textContent = report.once;
  $('multiple').textContent = report.multiple;
  $('share').textContent = report.share.toFixed(4);
  field();
  split();
  champions();
  tables();
  writers();
}

function pick(value, typed = false) {
  picked = Math.min(ceiling, Math.max(1, value || 1));
  page = 0;
  if (!typed || picked !== +$('value').value) $('value').value = picked;
  stamp({ value: picked });
  field();
  champions();
  writers();
}

function frame() {
  return new Promise((resolve) => requestAnimationFrame(resolve));
}

async function pass() {
  walking = true;
  let span = 4;
  let ticks = 0;
  let broken = false;
  say('note');
  do {
    await frame();
    const opened = performance.now();
    while (performance.now() - opened < BUDGET) {
      const clock = performance.now();
      try {
        state = JSON.parse(m.census_walk(span));
      } catch (error) {
        say('note', error);
        broken = true;
        break;
      }
      const spent = Math.max(performance.now() - clock, 0.5);
      span = Math.max(1, Math.min(span * 2, 4000, Math.round((span * BUDGET) / spent)));
      if (state.done >= state.total || spent >= BUDGET) break;
    }
    progress();
    if (++ticks % 8 === 0) refresh();
  } while (!broken && state.done < state.total);
  walking = false;
  refresh();
  progress();
}

$('value').oninput = () => {
  if ($('value').value !== '') pick(+$('value').value, true);
};
$('field').onpointerdown = (event) => {
  const box = $('field').getBoundingClientRect();
  const side = box.width / COLS;
  const column = Math.floor((event.clientX - box.left) / side);
  const line = Math.floor((event.clientY - box.top) / side);
  if (column >= 0 && column < COLS && line >= 0 && line < LINES) pick(line * COLS + column + 1);
};
$('random').onclick = () => {
  $('draw').min = 1;
  $('draw').max = ceiling;
  roll(s.next(), ['draw']);
  pick(+$('draw').value);
};
$('miss').onclick = () => {
  const misses = JSON.parse(m.census_misses(ceiling));
  if (!misses.length) return;
  $('draw').min = 0;
  $('draw').max = misses.length - 1;
  roll(s.next(), ['draw']);
  pick(misses[+$('draw').value]);
};
$('champion').onclick = () => {
  const rows = JSON.parse(m.census_champions(20));
  $('draw').min = 0;
  $('draw').max = rows.length - 1;
  roll(s.next(), ['draw']);
  pick(rows[+$('draw').value].value);
};
$('prev').onclick = () => {
  page -= 1;
  writers();
};
$('next').onclick = () => {
  page += 1;
  writers();
};
$('deepen').onclick = () => {
  if (!walking && !state.complete) pass();
};

$('registry').textContent = win.registry;
$('ceiling').textContent = ceiling;
$('cells').textContent = win.cells;
$('field-note').textContent = `1 to ${ceiling}, one cell an integer, click to read one`;
$('value').max = ceiling;
for (const [id, color] of [['key-miss', ink.orange], ['key-once', ink.dim], ['key-many', ink.blue]]) $(id).style.background = color;
$('value').value = picked;
state = JSON.parse(m.census_walk(win.tiers[0].keys));
refresh();
progress();
pass();
