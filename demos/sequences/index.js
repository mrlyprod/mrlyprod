import { ready, $, ink, paint, say, bind } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, bars, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const TERMS = 8;
const STEP = 4;
const BUDGET = '500000';
const TASTE = '20000';
const CHUNK = 50;
const IDS = ['q', 'measure', 'dimension', 'base', 'rows'];
const s = seeds();
for (const slug of m.ledger_measures()) $('measure').append(new Option(slug, slug));
const params = query(IDS);
const records = JSON.parse(m.ledger_records());
let page = +(params.get('page') ?? 0);
let picked = null;

function filters() {
  return { q: $('q').value.trim(), measure: $('measure').value, d: +$('dimension').value, b: +$('base').value, rows: +$('rows').value };
}

function chip(text) {
  return text ? `<span class="chip ${text.toLowerCase()}">${text}</span>` : '';
}

function badge(row) {
  if (!row.oeis) return '';
  const shift = row.shift ? ` <span class="shift">${row.shift > 0 ? '+' : ''}${row.shift}</span>` : '';
  return `<a class="badge" href="https://oeis.org/${row.oeis}" target="_blank" rel="noopener">${row.oeis}</a>${shift}`;
}

function spell(terms, capped) {
  return terms.join(', ') + (capped ? ', to the budget' : '');
}

function line(row) {
  const tr = document.createElement('tr');
  tr.dataset.name = row.name;
  tr.innerHTML = `<td class="mono">${row.name}${row.extra ? ' ' + chip(row.extra) : ''}</td><td class="num">${spell(row.terms, row.capped)}</td><td class="mono">${row.closed}</td><td>${badge(row)}</td><td>${chip(row.tag)}</td>`;
  tr.onclick = () => pick(row);
  tr.classList.toggle('on', picked?.name === row.name);
  return tr;
}

function read(code, d, b, measures, cells) {
  const out = [];
  for (const measure of measures) {
    for (const way of ['level', 'side']) {
      try {
        out.push({ ...JSON.parse(m.ledger_row(code, d, b, measure, way, TERMS, cells)), extra: 'typed' });
      } catch {
        continue;
      }
    }
  }
  return out;
}

function reading(name, cells) {
  const named = name.match(/^(mrly_bang_\w+)\.(\w+)\.(\w+)$/);
  if (!named) return null;
  try {
    const bang = JSON.parse(m.name_parse(named[1]));
    return JSON.parse(m.ledger_row(bang.code, bang.dimension, bang.base, named[2], named[3], TERMS, cells));
  } catch {
    return null;
  }
}

function fits(row, f) {
  return (!f.measure || row.measure === f.measure) && (!f.d || row.d === f.d) && (!f.b || row.q === f.b);
}

function keyed(f) {
  const out = [];
  for (const found of JSON.parse(m.ledger_identify(f.q))) {
    const record = records.find((r) => r.id === found.id);
    if (!record?.key || JSON.parse(m.ledger_search(record.key, '', 0, 0, 0, 1)).total) continue;
    const row = reading(record.key, BUDGET);
    if (row && fits(row, f)) out.push({ ...row, extra: 'record' });
  }
  return out;
}

function typed(f) {
  const measures = f.measure ? [f.measure] : m.ledger_measures();
  if (/\./.test(f.q)) {
    const row = reading(f.q, TASTE);
    return row ? [{ ...row, extra: 'typed' }] : [];
  }
  if (!/^\d+$/.test(f.q)) return [];
  const b = f.b || +$('base').options[1].value;
  const dims = f.d ? [f.d] : Array.from($('dimension').options, (o) => +o.value).filter(Boolean);
  for (const d of dims) {
    try {
      m.name_of(f.q, d, b);
    } catch {
      continue;
    }
    return read(f.q, d, b, measures, TASTE);
  }
  return [];
}

function identify(host, terms) {
  const found = JSON.parse(m.ledger_identify(terms.slice(0, TERMS).join(', ')));
  host.innerHTML = found.length
    ? found.slice(0, 3).map((r) => `<a href="https://oeis.org/${r.id}" target="_blank" rel="noopener">${r.id}</a> at index ${r.shift}: ${r.name}`).join('<br>')
    : 'no curated record holds these terms';
}

function search() {
  const f = filters();
  stamp({ q: f.q, measure: f.measure, dimension: f.d || null, base: f.b || null, rows: f.rows, page: page || null });
  say('note');
  try {
    const hits = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, page, f.rows));
    if (page && !hits.rows.length) {
      page = 0;
      return search();
    }
    const terms = /^[\d,\s-]+$/.test(f.q) && !/^\d+$/.test(f.q);
    const extra = page ? [] : terms ? keyed(f) : typed(f);
    $('body').replaceChildren(...extra.map(line), ...hits.rows.map(line));
    $('total').textContent = hits.total + (extra.length ? ` + ${extra.length} ${extra[0].extra}` : '');
    $('page-out').textContent = page + 1;
    $('prev').disabled = page === 0;
    $('next').disabled = (page + 1) * f.rows >= hits.total;
    if (terms) identify($('table-note'), f.q.split(/[\s,]+/).filter(Boolean));
    else $('table-note').textContent = f.q ? `matching "${f.q}"` : 'every row';
  } catch (error) {
    say('note', error);
  }
}

const show = keep((row) => {
  const b = board($('plot'), 240);
  const terms = row.terms;
  const heights = terms.map((t) => Math.log10(Math.abs(Number(t)) + 1));
  bars(b, heights, { color: (i) => (terms[i].startsWith('-') ? ink.orange : ink.blue) });
  const every = Math.max(1, Math.ceil(terms.length / 12));
  axis(b, terms.map((_, i) => [(i + 0.5) / terms.length, row.start + i]).filter(([, k]) => (k - row.start) % every === 0));
  tag(b, `log scale, ${row.axis === 'level' ? 'level L' : 'side k'} from ${row.start}`, ink.dim);
  tag(b, `last ${terms.at(-1)}`, ink.fg, 'right');
});

function preview(row) {
  const host = $('preview');
  host.replaceChildren();
  say('preview-note');
  try {
    if (row.d === 2) {
      const canvas = document.createElement('canvas');
      canvas.className = 'sheet';
      const level = m.level_cap(row.number, 2, 60000);
      paint(canvas, m.two_grid(row.code, row.number, level, 0, row.q), ink.blue);
      host.append(canvas);
      $('preview-title').textContent = `side ${row.number}, level ${level}`;
    } else if (row.d === 3) {
      const level = m.level_cap(row.number, 3, 8000);
      host.innerHTML = m.hex_svg(row.code, row.number, level, row.q, 'iso', 4);
      $('preview-title').textContent = `side ${row.number}, level ${level}, isometric`;
    } else {
      const canvas = document.createElement('canvas');
      host.append(canvas);
      if (row.d === 1) {
        const level = m.level_cap(row.number, 1, 729);
        const cells = m.ledger_profile(row.code, 1, row.q, row.number, level);
        canvas.className = 'sheet';
        canvas.style.height = '48px';
        paint(canvas, { width: cells.length, height: 1, types: Uint8Array.from(cells) }, ink.blue);
        $('preview-title').textContent = `side ${row.number}, level ${level}, the strip`;
      } else {
        const level = m.level_cap(row.number, row.d, 600000);
        const counts = m.ledger_profile(row.code, row.d, row.q, row.number, level).map(Number);
        canvas.className = 'bars';
        const b = board(canvas, 220);
        bars(b, counts, { color: ink.pink, inset: 0 });
        axis(b, [[0, 'first plane'], [1, 'last plane']]);
        tag(b, `cells on every diagonal plane, level ${level}`, ink.dim);
        $('preview-title').textContent = `side ${row.number}, level ${level}, the diagonal profile`;
      }
    }
  } catch (error) {
    say('preview-note', error);
  }
}

function pick(row) {
  picked = row;
  for (const tr of $('body').children) tr.classList.toggle('on', tr.dataset.name === row.name);
  stamp({ open: row.name });
  $('detail').hidden = false;
  $('detail-name').textContent = row.name;
  $('code').textContent = row.code;
  $('space').textContent = `dimension ${row.d}, base ${row.q}`;
  $('measure-out').textContent = row.measure;
  $('axis').textContent = row.axis === 'level' ? `level L at side ${row.number}` : 'odd side 2k - 1 at level 1';
  $('closed').textContent = row.closed || 'none known';
  $('record').innerHTML = badge(row) || 'none';
  $('tag').innerHTML = chip(row.tag) || 'unmatched';
  $('depth').textContent = spell([row.terms.length], row.capped);
  $('deeper').disabled = false;
  $('deeper').textContent = 'deeper';
  say('terms-note');
  preview(row);
  show(row);
  identify($('identify'), row.terms);
}

function deeper() {
  const row = picked;
  say('terms-note');
  try {
    const asked = row.terms.length + STEP;
    const more = m.ledger_terms(row.code, row.d, row.q, row.measure, row.axis, asked, BUDGET);
    if (more.length <= row.terms.length) {
      row.capped = true;
      $('deeper').disabled = true;
      $('deeper').textContent = 'at the budget';
    } else {
      row.terms = more;
      row.capped = more.length < asked;
    }
    $('depth').textContent = spell([row.terms.length], row.capped);
    show(row);
    const tr = Array.from($('body').children).find((tr) => tr.dataset.name === row.name);
    if (tr) tr.children[1].textContent = spell(row.terms, row.capped);
  } catch (error) {
    say('terms-note', error);
  }
}

function reopen(name) {
  const row = reading(name, BUDGET);
  if (row) pick(row);
  else stamp({ open: null });
}

function shuffle(seed) {
  const f = filters();
  const total = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, 0, 1)).total;
  if (!total) return;
  const draw = $('draw');
  draw.min = 0;
  draw.max = total - 1;
  roll(seed, ['draw']);
  const at = +draw.value;
  page = Math.floor(at / f.rows);
  const row = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, at, 1)).rows[0];
  search();
  pick(row);
}

function idle() {
  return new Promise((resolve) => (window.requestIdleCallback ? requestIdleCallback(resolve, { timeout: 100 }) : setTimeout(resolve, 0)));
}

async function load() {
  $('count').textContent = m.ledger_build('closed', TERMS);
  $('tier').textContent = 'closed';
  search();
  if (params.get('open') && !s.get()) reopen(params.get('open'));
  for (const tier of ['convolved', 'side']) {
    let state;
    do {
      await idle();
      state = JSON.parse(m.ledger_grow(tier, TERMS, CHUNK));
      $('count').textContent = state.rows;
      $('tier').textContent = `${tier} ${state.done} of ${state.total}`;
    } while (state.done < state.total);
    search();
  }
  $('tier').textContent = 'complete';
  if (s.get() && !picked) shuffle(s.get());
}

bind(['measure', 'dimension', 'base', 'rows'], () => {
  page = 0;
  search();
});
$('q').oninput = () => {
  s.drop();
  page = 0;
  search();
};
$('prev').onclick = () => {
  page -= 1;
  search();
};
$('next').onclick = () => {
  page += 1;
  search();
};
$('random').onclick = () => shuffle(s.next());
$('deeper').onclick = deeper;
load();
