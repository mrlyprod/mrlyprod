import { ready, $, ink, paint, say, out, fit } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, bars, axis, tag } from '../lib/chart.js';

const m = await ready();
const BUDGET = '500000';
const WINDOW = 6;
const NAMED = 3;
const FLAT = [['low corner', '1'], ['tree', '3'], ['carpet', '7'], ['void', '9'], ['corner and centre', '11'], ['solid', '15']];
const s = seeds();
const records = JSON.parse(m.ledger_records());
const flatCap = m.level_cap(3, 2, 500000);
const cubeCap = m.level_cap(3, 3, 60000);
const cutCap = m.level_cap(2, 3, 500000);

const row = (code, d, measure, axis, count) => JSON.parse(m.ledger_row(code, d, 2, measure, axis, count, BUDGET));
const census = (code, n) => JSON.parse(m.two_census(code, n, 1, 0, 2));
const slice = (n) => JSON.parse(m.slice_census('23', n, 1, 2));
const odds = (k) => Array.from({ length: k }, (_, i) => 2 * i + 1);
const upto = (k) => Array.from({ length: k }, (_, i) => i + 1);
const least = (n) => Math.max(n, NAMED);
const item = (r, label) => ({ label, terms: r.terms, closed: r.closed, capped: r.capped, ledger: true });

function sheet(grid, on = ink.blue, off = ink.deep) {
  const canvas = document.createElement('canvas');
  paint(canvas, grid, on, off);
  return canvas;
}

function svg(text) {
  const div = document.createElement('div');
  div.innerHTML = text;
  return div.firstElementChild;
}

function hex(number, level, view) {
  const side = number ** level;
  return svg(m.hex_svg('23', number, level, 2, view, Math.max(1, Math.round(65 / side))));
}

function chart(host, terms) {
  const canvas = document.createElement('canvas');
  canvas.className = 'bars';
  canvas.style.cssText = 'width: 100%; height: 200px';
  host.append(canvas);
  const b = board(canvas, 200);
  bars(b, terms.map((t) => Math.log10(Number(t))), { color: ink.gold, inset: 3 });
  axis(b, terms.map((_, i) => [(i + 0.5) / terms.length, i + 1]));
  tag(b, 'log scale by dimension', ink.dim);
}

function stack(host, order) {
  const canvas = document.createElement('canvas');
  canvas.style.cssText = 'width: 100%; height: 200px';
  host.append(canvas);
  const [ctx, w, h] = fit(canvas, 200);
  ctx.strokeStyle = ink.pink;
  for (const [num, den, bright] of JSON.parse(m.farey(order))) {
    const x = 6 + (w - 12) * num / den;
    ctx.globalAlpha = 0.25 + 0.75 * bright / order;
    ctx.beginPath();
    ctx.moveTo(x, h - 6);
    ctx.lineTo(x, h - 6 - (h - 12) * bright / order);
    ctx.stroke();
  }
}

const cards = [
  {
    key: 'sides', title: 'The odd-side law', wide: true, slider: ['side up to', 4, 8, 4], show: (k) => 2 * k - 1,
    say: 'At odd side 2k - 1 an axis splits into k low and k - 1 high positions, so a flat design fills a polynomial in k, and the six designs of the plane read as the polygonal numbers: the low corner the squares, the tree the hexagonal numbers, the carpet the octagonal numbers, before any of them is a fractal.',
    build: (k) => ({
      lines: FLAT.map(([name, code]) => ({
        label: `${name}, code ${code}`,
        draw: (host) => odds(k).slice(1).forEach((n) => host.append(sheet(m.two_grid(code, n, 1, 0, 2)))),
        terms: odds(k).slice(1).map((n) => String(census(code, n).fills)),
        closed: m.ledger_closed(code, 2, 2, 'fills', 'side'),
        ledger: true,
      })),
    }),
  },
  {
    key: 'carpet', title: 'The carpet', slider: ['level', 1, flatCap, 3],
    say: 'Eight of nine cells survive every level, so the carpet fills 8^L, and its dimension log 8 / log 3 is this count read against the side 3^L.',
    build: (L) => ({ draw: (host) => host.append(sheet(m.two_grid('7', 3, L, 0, 2))), lines: [item(row('7', 2, 'fills', 'level', least(L)), 'filled cells')] }),
  },
  {
    key: 'voids', title: 'What the carpet drops', slider: ['level', 1, flatCap, 3],
    say: 'The cells the carpet leaves empty are the grid less the fill, 9^L - 8^L, the powers of nine racing the powers of eight.',
    build: (L) => ({ draw: (host) => host.append(sheet(m.two_grid('7', 3, L, 0, 2), ink.deep, ink.orange)), lines: [item(row('7', 2, 'voids', 'level', least(L)), 'empty cells')] }),
  },
  {
    key: 'perimeter', title: 'The perimeter of the carpet', slider: ['level', 1, flatCap, 3],
    say: 'Two adjacent blocks bury one edge per spanning position, so the perimeter closes as a sum of the powers 8^L and 3^L, the entry the OEIS lists for the carpet at iteration n.',
    build: (L) => ({ draw: (host) => host.append(sheet(m.two_grid('7', 3, L, 0, 2), ink.gold)), lines: [item(row('7', 2, 'surface', 'level', least(L)), 'exposed edges')] }),
  },
  {
    key: 'sponge', title: 'The sponge', slider: ['level', 1, cubeCap, 2],
    say: 'Twenty of twenty-seven subcubes survive every level, so the sponge fills 20^L.',
    build: (L) => ({ draw: (host) => host.append(hex(3, L, 'iso')), lines: [item(row('23', 3, 'fills', 'level', least(L)), 'filled cells')] }),
  },
  {
    key: 'surface', title: 'The surface of the sponge', slider: ['level', 1, cubeCap, 2],
    say: 'The same burial count in three dimensions: the exposed faces are a sum of the powers 20^L and 8^L, the surface area of the stage-n Menger sponge.',
    build: (L) => ({ draw: (host) => host.append(hex(3, L, 'pro')), lines: [item(row('23', 3, 'surface', 'level', least(L)), 'exposed faces')] }),
  },
  {
    key: 'tile', title: 'The odd sponge tile', slider: ['side', 2, 8, 3], show: (k) => 2 * k - 1,
    say: 'Widen the sponge tile to odd side 2k - 1 and its fills and voids are cubics in k: the divisor count of 240^n read as a solid, and the entry this tree contributed.',
    build: (k) => ({
      draw: (host) => host.append(hex(2 * k - 1, 1, 'iso')),
      lines: [item(row('23', 3, 'fills', 'side', least(k - 1)), 'filled cells'), item(row('23', 3, 'voids', 'side', least(k - 1)), 'empty cells')],
    }),
  },
  {
    key: 'designs', title: 'How many designs', slider: ['base up to', 2, 6, 4],
    say: 'A design is a Boolean function on the corners up to the symmetries of the cube: in base 2 the count by dimension is the irreducible Boolean functions of the record, and by base the rows are the bracelets, the toroidal squares and the toroidal cubes.',
    build: (Q) => {
      const lines = [{ label: 'base 2, by dimension', terms: m.counting_sequence(4), closed: '' }];
      ['a line', 'a square', 'a cube'].forEach((name, i) => {
        const terms = [];
        for (let q = 2; q <= least(Q - 1) + 1; q++) {
          try {
            terms.push(m.baseq_sequence(q, i + 1)[i]);
          } catch {
            break;
          }
        }
        lines.push({ label: `${name}, by base from 2`, terms, closed: '', capped: terms.length < least(Q - 1) });
      });
      const draw = (host) => {
        const strip = document.createElement('div');
        strip.className = 'strip';
        for (const d of JSON.parse(m.universe(2)).designs) strip.append(sheet(m.two_grid(d.code, 3, 2, 0, 2), ink.gold));
        host.append(strip);
      };
      return { draw, lines };
    },
  },
  {
    key: 'classes', title: 'How many fractals', slider: ['dimension', 1, 8, 4],
    say: 'Two base-2 designs draw the same fractal exactly when they fill the same number of corners of each weight, so the distinct fractals of a dimension number the product over the weights of one more than the corners of that weight.',
    build: (D) => {
      const terms = m.classes_sequence(least(D));
      return { draw: (host) => chart(host, terms), lines: [{ label: 'fill classes, by dimension', terms, closed: '' }] };
    },
  },
  {
    key: 'slices', title: 'The middle slice of the sponge', slider: ['level', 1, cubeCap, 2],
    say: 'Cut the sponge on its middle diagonal plane and count the filled triangles: one index up from the star holes of the record, whose recurrence gives the slice its dimension.',
    build: (L) => ({
      draw: (host) => host.append(hex(3, L, 'cut')),
      lines: [{ label: 'filled triangles, from the unit hexagon', terms: [String(slice(1).fills), ...m.ledger_terms('23', 3, 2, 'triangles', 'level', least(L - 1) + 1, BUDGET)], closed: '', ledger: true }],
    }),
  },
  {
    key: 'vertices', title: 'The vertices of the slice', slider: ['side', 2, 8, 4], show: (k) => 2 * k - 1,
    say: 'The middle slice of the odd cube is a hexagon whose vertex count at side n is the centered hexagonal number 3n(n + 1) + 1, the sequence the corner-and-centre design fills at odd sides; a prime among them is a difference of consecutive cubes, a cuban prime, and the gold terms are the primes.',
    build: (k) => {
      const terms = odds(least(k)).map((n) => String(slice(n).vertices));
      return {
        draw: (host) => host.append(hex(2 * k - 1, 1, 'cut')),
        lines: [
          { label: 'vertices, by odd side', terms, marks: terms.map((t) => JSON.parse(m.factor(t)).prime), closed: '' },
          item(row('11', 2, 'fills', 'side', least(k - 1)), 'the corner-and-centre fills, by odd side'),
        ],
      };
    },
  },
  {
    key: 'gasket', title: 'The gasket cut', slider: ['level', 1, cutCap, 4],
    say: 'Every diagonal plane through the octahedral design at side 2 holds exactly 3^L points at every admissible height, by the uniqueness of the binary expansion of the height, so the cut is a Sierpinski gasket at every depth.',
    build: (L) => {
      const profiles = upto(least(L)).map((j) => JSON.parse(m.diagonal_profile('126', 2, j, 2)));
      const last = profiles[L - 1];
      return {
        draw: (host) => host.append(svg(m.diagonal_svg('126', 2, L, 2, last.central, Math.max(2, Math.round(180 / last.side))))),
        sub: `${last.heights} heights, ${last.constant ? 'all alike' : 'not all alike'}`,
        lines: [{ label: 'points on a diagonal plane, by level', terms: profiles.map((p) => p.max), closed: '' }],
      };
    },
  },
  {
    key: 'farey', title: 'The Farey nodes', slider: ['order', 1, 24, 12],
    say: 'Stack the carpet at every odd scale and the lit nodes are the Farey fractions, each new scale n lighting phi(n) new nodes: the count is one more than the totients summed, and the primes are the scales of most novelty.',
    build: (order) => {
      const stacks = upto(least(order)).map((j) => JSON.parse(m.farey_novelty(j)));
      const last = stacks[order - 1];
      return {
        draw: (host) => stack(host, order),
        sub: `${last.lit} nodes lit, ${last.match ? 'as the totients say' : 'against the totients'}, primes ${last.primes.join(' ')}`,
        lines: [{ label: 'nodes lit, by order', terms: stacks.map((f) => String(f.lit)), closed: '' }],
      };
    },
  },
];

function hits(terms) {
  if (terms.length < NAMED) return { html: '<span class="dim">three terms name the record</span>', formula: '' };
  const found = JSON.parse(m.ledger_identify(terms.slice(0, WINDOW).join(', ')));
  const formula = found.length ? records.find((r) => r.id === found[0].id)?.formula : '';
  const html = found.length
    ? found.slice(0, 3).map((r) => `<a class="badge" href="https://oeis.org/${r.id}" target="_blank" rel="noopener">${r.id}</a> ${r.name}, from index ${r.shift}`).join(' · ')
    : 'no curated record holds these terms';
  return { html, formula: formula && formula !== 'none' ? formula : '' };
}

function line(host, it) {
  const el = document.createElement('div');
  el.className = 'line';
  if (it.draw) {
    const strip = document.createElement('div');
    strip.className = 'strip';
    it.draw(strip);
    el.append(strip);
  }
  const text = document.createElement('div');
  text.className = 'text';
  const spelled = it.terms.map((t, i) => (it.marks?.[i] ? `<span class="gold">${t}</span>` : t)).join(', ') + (it.capped ? ', to the budget' : '');
  const found = hits(it.terms);
  const forms = [it.closed, found.formula].filter((f, i, all) => f && all.indexOf(f) === i).map((f) => `<span class="mono">${f}</span>`);
  const open = it.ledger ? ` · <a href="../sequences/?q=${encodeURIComponent(it.terms.join(', '))}">open in the ledger</a>` : '';
  text.innerHTML = `${it.label ? `<span class="dim">${it.label}</span><br>` : ''}<b class="num">${spelled}</b><br>${forms.length ? forms.join(' · ') + ' · ' : ''}${found.html}${open}`;
  el.append(text);
  host.append(el);
}

function card(spec, index) {
  const el = document.createElement('section');
  el.className = `panel tour${spec.wide ? ' wide' : ''}`;
  el.id = `card-${spec.key}`;
  const [label, min, max, value] = spec.slider;
  el.innerHTML = `
    <div class="pic" id="${spec.key}-pic"></div>
    <div class="body">
      <h2>${index + 1} · ${spec.title} <span id="${spec.key}-sub"></span></h2>
      <p class="story">${spec.say}</p>
      <div class="row"><label>${label} <input type="range" id="${spec.key}" min="${min}" max="${max}" value="${value}"><span class="num" id="${spec.key}-out"></span></label></div>
      <div id="${spec.key}-lines"></div>
      <div class="note" id="${spec.key}-note"></div>
    </div>`;
  return el;
}

function build(spec) {
  const value = +$(spec.key).value;
  out(spec.key, spec.show ? spec.show(value) : value);
  stamp({ [spec.key]: value });
  say(`${spec.key}-note`);
  const pic = $(`${spec.key}-pic`), lines = $(`${spec.key}-lines`);
  pic.replaceChildren();
  lines.replaceChildren();
  try {
    const made = spec.build(value);
    if (made.draw) made.draw(pic);
    $(`${spec.key}-sub`).textContent = made.sub ?? '';
    for (const it of made.lines) line(lines, it);
  } catch (error) {
    say(`${spec.key}-note`, error);
  }
}

function shuffle(seed) {
  const draw = $('draw');
  draw.min = 0;
  draw.max = cards.length - 1;
  roll(seed, ['draw']);
  const spec = cards[+draw.value];
  roll(seed, [spec.key]);
  build(spec);
  $(`card-${spec.key}`).scrollIntoView({ behavior: 'smooth', block: 'center' });
}

$('cards').append(...cards.map(card));
query(cards.map((spec) => spec.key));
for (const spec of cards) {
  $(spec.key).oninput = () => {
    s.drop();
    build(spec);
  };
  build(spec);
}
$('random').onclick = () => shuffle(s.next());
if (s.get()) shuffle(s.get());
