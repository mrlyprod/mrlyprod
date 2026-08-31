import { ready, $, ink, say, paint, blit, out } from '../lib/mrly.js';
import { board, line, axis, tag, keep } from '../lib/chart.js';
import { picker, seeds, roll } from '../lib/select.js';
import { query, stamp, share } from '../lib/query.js';
import { stage, faces } from '../lib/stage.js';

const m = await ready();
const MAX_SLOTS = 6;
const PLANE = 243;
const SOLID = 128;
const CUBES = 150000;

const WORDS = {
  doctest: { view: 'plane', compare: 'swap', chart: 'exponent', letters: [['7', 2, 3], ['14', 2, 7], ['9', 2, 5]] },
  periodic: { view: 'plane', compare: 'double', chart: 'exponent', letters: [['7', 2, 3], ['9', 2, 5]] },
  staircase: { view: 'nest', compare: 'double', chart: 'stair', letters: [['7', 2, 3], ['7', 2, 3], ['7', 2, 5], ['7', 2, 3], ['7', 2, 5], ['7', 2, 7]] },
  order: { view: 'plane', compare: 'swap', chart: 'exponent', letters: [['3', 2, 2], ['6', 2, 2]] },
  morse: { view: 'nest', compare: 'swap', chart: 'exponent', letters: [['3', 2, 2], ['7', 2, 2], ['7', 2, 2], ['3', 2, 2], ['7', 2, 2], ['3', 2, 2]] },
  collide: { view: 'plane', compare: 'collide', chart: 'exponent', letters: [['9', 2, 2], ['273', 3, 3]] },
  sponge: { view: 'solid', compare: 'swap', chart: 'stair', letters: [['23', 2, 3], ['9', 2, 3]] },
};

const NAMES = [
  ['doctest', 'the constructor doctest word'],
  ['periodic', 'a pair, and the same pair doubled'],
  ['staircase', 'the staircase, three blocks'],
  ['order', 'the minimal order pair, swapped'],
  ['morse', 'Thue-Morse over the gasket and the domino'],
  ['collide', 'one tile, two words'],
  ['sponge', 'a solid word'],
];

const COLLISION = [
  { codes: ['9', '273'], numbers: [2, 3], bases: [2, 3] },
  { codes: ['273', '9'], numbers: [3, 2], bases: [3, 2] },
];

const SAYS = {
  double: 'Block reduction: a word repeated is not a word, it is a letter. The doubled word is the ordinary self-similar theory of its one-period composite, at side the product of the sides and fill the product of the fills, so the side and the fill square and the dimension does not move. Proved, research/magic.md.',
  swap: 'Order is the object. Side, fill, density and the main-diagonal count are functions of the letter multiset alone, so a swap moves none of them; components, Euler characteristic, holes and boundary are order-sensitive, so a swap moves the piece count. Proved, research/magic.md and research/connectivity.md.',
  collide: 'Factorisation in the tile monoid is not unique. I(2) x I(3) and I(3) x I(2) are the same side-6 tile spelt by two different words, and the identity (nm - 1) - x = (n - 1 - i) m + (m - 1 - j) is symmetric in the two sides, so the collision happens at every pair of sides. Both sides here are prime, so all four letters are irreducible. Proved, cited rather than claimed, research/magic.md. A base-3 code whose digit set is not a parity rule is a tile and not a design, and inherits nothing from the design census.',
};

const CHARTS = {
  exponent:
    'Over the 15 plane codes at side two, where both letters occur with strictly positive frequency, the component exponent exists, depends on the letter frequencies alone, and equals the fill exponent on 104 of the 105 letter pairs; the periodic control at the same frequencies lands on the same limit, so a difference against the prediction refutes the prediction and not stationarity. The constant-word functional Phi(f) = (f_6 + f_9) log 2 is then refuted on 78 of the 105 letter pairs and exact on 27. Proved, research/connectivity.md. The interior-frequency hypothesis rides inside the statement: at a boundary frequency three words over one pair give rates 0, log 2 and no limit at all.',
  outside:
    'These two letters lie outside the alphabet the closed forms cover, the 15 plane codes at side two, so the curves below are exact counts of the drawn word and no rate is claimed for them. Switch to a pair of side-2 letters, or to the Thue-Morse preset, for the proved reading. Research/connectivity.md.',
  stair:
    'The staircase stacks prefixes, so the letter in place j occurs n - j + 1 times in the first n blocks and the dimension is the occurrence-weighted average of the per-letter dimensions. It is not monotone: it dips at the second block because the base-5 carpet is less dense than the base-3 carpet, then climbs, and its limit is the ambient dimension. Proved, research/magic.md; the five printed values are Verified, lab/slice-ladder-controls.',
};

let slots = [];
let pickers = [];
let dimension = 2;
let solid = null;
const shared = seeds();

// WORD

const spell = (codes, numbers, bases) =>
  (dimension === 3 ? 'd3: ' : '') +
  codes.map((code, i) => `c${code}${bases[i] === 2 ? '' : `.q${bases[i]}`}(${numbers[i]})`).join(', ');

function token(codes, numbers, bases) {
  if (dimension !== 2 || bases.some((base) => base !== 2)) return '';
  try {
    return m.magic_name(codes, numbers);
  } catch {
    return '';
  }
}

function snap() {
  slots = slots.map((slot, i) => ({
    code: $(`l${i}code`).value.trim(),
    base: $(`l${i}base`) ? +$(`l${i}base`).value : 2,
    number: Math.min(16, Math.max(2, +$(`l${i}n`).value || 2)),
  }));
}

function mount() {
  $('builder').innerHTML = slots
    .map(
      (slot, i) => `
    <div class="row">
      <span class="badge">letter ${i + 1}</span>
      <span class="set" id="l${i}pick"></span>
      <label>side <input type="number" id="l${i}n" value="${slot.number}" min="2" max="16"></label>
      <button id="l${i}swap">swap next</button>
      <button id="l${i}drop">remove</button>
    </div>`,
    )
    .join('');
  pickers = slots.map((slot, i) =>
    picker({
      host: $(`l${i}pick`),
      m,
      dimension,
      base: dimension === 2 ? [2, 3] : [2],
      code: slot.code,
      build,
      key: `l${i}`,
      extra: [`l${i}n`],
      seeds: shared,
      button: false,
    }),
  );
  slots.forEach((slot, i) => {
    $(`l${i}code`).value = slot.code;
    $(`l${i}n`).value = slot.number;
    if ($(`l${i}base`)) $(`l${i}base`).value = slot.base;
    $(`l${i}n`).oninput = build;
    $(`l${i}drop`).disabled = slots.length < 3;
    $(`l${i}swap`).disabled = slots.length < 2;
    $(`l${i}drop`).onclick = () => {
      snap();
      slots.splice(i, 1);
      mount();
      build();
    };
    $(`l${i}swap`).onclick = () => {
      snap();
      const at = (i + 1) % slots.length;
      [slots[i], slots[at]] = [slots[at], slots[i]];
      mount();
      build();
    };
  });
  $('add').disabled = slots.length >= MAX_SLOTS;
}

function load(name) {
  const preset = WORDS[name];
  slots = preset.letters.map(([code, base, number]) => ({ code, base, number }));
  dimension = preset.view === 'solid' ? 3 : 2;
  $('view').value = preset.view;
  $('compare').value = preset.compare;
  $('chart').value = preset.chart;
  if (name === 'morse') $('schedule').value = 'thue-morse';
  mount();
  build();
}

// STATS

function flags(census) {
  const chips = [];
  if (census.constant) chips.push('<span class="chip proved">constant</span>');
  if (census.periodic) chips.push('<span class="chip proved">periodic</span>');
  if (census.composite) chips.push(`<span class="chip proved">composite at base ${census.residue_base}</span>`);
  if (census.native) chips.push('<span class="chip verified">native</span>');
  return chips.join(' ');
}

function repeats(codes, numbers, bases) {
  const seen = new Map();
  for (let i = 0; i < codes.length; i++) {
    const key = `${codes[i]}.${bases[i]}(${numbers[i]})`;
    seen.set(key, (seen.get(key) ?? 0) + 1);
  }
  const doubled = [...seen].filter(([, count]) => count > 1);
  if (!doubled.length) return '';
  return `There is no level control: the word length is the depth, and a repeated letter is how a level is spelt. This word repeats ${doubled.map(([key, count]) => `${key} ${count} times`).join(', ')}.`;
}

function readout(census, codes, numbers, bases, taken) {
  const name = token(codes, numbers, bases);
  $('name').textContent = spell(codes, numbers, bases);
  const members = m.word_count(codes, numbers, dimension, bases);
  const agrees = members === census.fill;
  let profile = [];
  try {
    profile = m.word_profile(codes, numbers, dimension, bases).map(Number);
  } catch {
    profile = [];
  }
  const peak = profile.length ? Math.max(...profile) : 0;
  const head = Number(members) <= 4096 ? m.word_members(codes, numbers, dimension, bases).slice(0, 6).join(', ') : '';
  $('stats').innerHTML = `
    ${name ? `<span>name <b>${name}</b></span>` : ''}
    <span>side <b>${census.side}</b></span>
    <span>cells <b>${census.cells}</b></span>
    <span>filled <b>${census.fill}</b></span>
    <span>empty <b>${census.voids}</b></span>
    <span>density <b>${census.ratio.toFixed(4)}</b></span>
    <span>dimension <b>${census.dimension.toFixed(4)}</b></span>
    ${census.components ? `<span>pieces <b>${census.components}</b> <span class="dim">${census.counted}</span></span>` : ''}
    <span>press members <b>${members}</b> <span class="chip ${agrees ? 'verified' : 'refuted'}">${agrees ? 'agrees with the fill' : 'differs from the fill'}</span></span>
    ${profile.length ? `<span>diagonal profile <b>${profile.length}</b> heights, peak <b>${peak}</b> at <b>${profile.indexOf(peak)}</b></span>` : ''}
    ${head ? `<span>first members <b>${head}</b></span>` : ''}
    ${flags(census)}`;
  const probe = $('probe').value.trim();
  try {
    $('probe-out').textContent = probe ? String(m.word_member(codes, numbers, dimension, bases, probe)) : '';
  } catch (error) {
    $('probe-out').textContent = String(error.message ?? error);
  }
  $('badges').innerHTML = census.letters
    .map(
      (letter, i) =>
        `<span class="badge">${i + 1} <b>${letter.name}</b> side ${letter.number} fill ${letter.fill} dim ${letter.dimension.toFixed(4)}${letter.native ? ' native' : ''}</span>`,
    )
    .join('');
  const cut = taken < codes.length ? `Drawing <b>${taken} of ${codes.length} letters</b>, the box cover of the whole word at side ${census.letters.slice(0, taken).reduce((a, l) => a * l.number, 1)}. ` : '';
  $('scale').innerHTML = `${cut}The readout is a product over the letters, so it outruns the raster on purpose: every number above is exact at the full length. ${repeats(codes, numbers, bases)} <a href="../moire">Moire</a> stacks one design over its scales instead.`;
  $('tosponge').href = `../sponge${share({ code: codes[0], base: bases[0], number: numbers[0], level: 3 })}`;
}

// DRAW

function theStage() {
  if (!solid) solid = stage($('stage'));
  return solid;
}

function nesting(codes, numbers, bases, taken) {
  const full = m.magic_grid(codes.slice(0, taken), numbers.slice(0, taken), bases.slice(0, taken));
  const size = full.width;
  const field = new Float32Array(size * size);
  for (let depth = 1; depth <= taken; depth++) {
    const grid =
      depth === 1
        ? m.two_grid(codes[0], numbers[0], 1, 0, bases[0])
        : m.magic_grid(codes.slice(0, depth), numbers.slice(0, depth), bases.slice(0, depth));
    const step = size / grid.width;
    for (let r = 0; r < size; r++) {
      const row = Math.floor(r / step) * grid.width;
      for (let c = 0; c < size; c++) {
        if (grid.types[row + Math.floor(c / step)]) field[r * size + c] += 1;
      }
    }
  }
  blit($('sheet'), m.sheet(field, size, 'fire', taken, false));
}

function draw(codes, numbers, bases, taken) {
  const view = $('view').value;
  const cut = (list) => list.slice(0, taken);
  $('sheet').hidden = view === 'solid';
  $('stage').hidden = view !== 'solid';
  if (view === 'solid') {
    const cubes = Number(m.word_count(cut(codes), cut(numbers), 3, cut(bases)));
    if (cubes > CUBES) throw new Error(`${cubes} cubes is more than this page draws; drop a letter or lower a side.`);
    const st = theStage();
    st.show(faces(m.magic_faces(cut(codes), cut(numbers), cut(bases)), ink.blue, 1));
    st.project($('iso').checked ? 'iso' : 'eye');
    st.spin = $('spin').checked ? 0.004 : 0;
    return;
  }
  if (view === 'nest') {
    nesting(codes, numbers, bases, taken);
    return;
  }
  paint($('sheet'), m.magic_grid(cut(codes), cut(numbers), cut(bases)), ink.gold);
}

// PAIR

function tile(id, word, budget) {
  const taken = m.magic_cap(word.numbers, word.dimension, budget);
  if (taken < 2) throw new Error('the first two letters already pass the page budget; lower a side.');
  const grid = m.magic_grid(word.codes.slice(0, taken), word.numbers.slice(0, taken), word.bases.slice(0, taken));
  paint($(id), grid, ink.gold);
  const census = JSON.parse(m.magic_census(word.codes, word.numbers, word.dimension, word.bases));
  $(`${id}-stats`).innerHTML = `
    <span>${spell(word.codes, word.numbers, word.bases)}</span>
    <span>side <b>${census.side}</b></span>
    <span>filled <b>${census.fill}</b></span>
    <span>dim <b>${census.dimension.toFixed(4)}</b></span>
    ${census.components ? `<span>pieces <b>${census.components}</b></span>` : ''}
    ${taken < word.codes.length ? `<span class="dim">drawn ${taken} of ${word.codes.length} letters</span>` : ''}`;
  return { census, grid };
}

function pair(codes, numbers, bases) {
  const kind = $('compare').value;
  if (dimension !== 2 && kind !== 'collide') throw new Error('the pair panel is a plane reading; switch the view off solid.');
  const plain = { codes, numbers, bases, dimension };
  let left = plain;
  let right = plain;
  if (kind === 'double') {
    right = { codes: codes.concat(codes), numbers: numbers.concat(numbers), bases: bases.concat(bases), dimension };
  } else if (kind === 'swap') {
    const order = (list) => [list[1], list[0], ...list.slice(2)];
    right = { codes: order(codes), numbers: order(numbers), bases: order(bases), dimension };
  } else {
    left = { ...COLLISION[0], dimension: 2 };
    right = { ...COLLISION[1], dimension: 2 };
  }
  const a = tile('pair-a', left, PLANE);
  const b = tile('pair-b', right, PLANE);
  const same = a.grid.width === b.grid.width && a.grid.types.every((byte, i) => byte === b.grid.types[i]);
  const square = (text) => (BigInt(text) * BigInt(text)).toString();
  const reads = [];
  if (kind === 'double') {
    reads.push(`side squares <b>${b.census.side === square(a.census.side)}</b>`);
    reads.push(`fill squares <b>${b.census.fill === square(a.census.fill)}</b>`);
    reads.push(`dimension unmoved <b>${Math.abs(a.census.dimension - b.census.dimension) < 1e-12}</b>`);
  } else if (kind === 'swap') {
    reads.push(`side equal <b>${a.census.side === b.census.side}</b>`);
    reads.push(`fill equal <b>${a.census.fill === b.census.fill}</b>`);
    reads.push(`pieces equal <b>${a.census.components === b.census.components}</b>`);
  } else {
    reads.push(`same tile <b>${same}</b>`);
    reads.push(`same word <b>${false}</b>`);
  }
  $('pair-title').textContent = kind === 'collide' ? 'one tile, two words' : `the word beside ${kind === 'double' ? 'itself doubled' : 'its swap'}`;
  $('pair-note').innerHTML = reads.map((text) => `<span>${text}</span>`).join(' ');
  $('pair-say').textContent = SAYS[kind];
}

// CHARTS

const paintChart = keep((rows, options) => {
  const canvas = $('chart-canvas');
  const b = board(canvas, 240, { left: 46, right: 16 });
  const { low, high, marks, lines, labels } = options;
  const span = high - low || 1;
  const at = (value) => (value - low) / span;
  axis(b, labels, { wall: true });
  for (const [value, color, text] of marks) {
    line(b, [[0, at(value)], [1, at(value)]], color, { width: 1, dash: [4, 4] });
    tag(b, text, color, 'right', b.x(1), b.y(at(value)) - 4);
  }
  for (const [points, color, width] of lines) {
    line(b, points.map(([x, y]) => [x, at(y)]), color, { width, dots: points.length < 12 ? 3 : 0 });
  }
  b.ctx.fillStyle = ink.dim;
  b.ctx.fillText(high.toFixed(3), 2, b.roof + 4);
  b.ctx.fillText(low.toFixed(3), 2, b.floor);
  return rows;
});

function exponent(codes, numbers, bases) {
  const length = +$('length').value;
  out('length', length);
  const read = JSON.parse(m.magic_rates(codes, numbers, bases, $('schedule').value, length));
  const total = read.length;
  const at = (i) => (total > 1 ? i / (total - 1) : 0);
  const component = read.rows.map((row, i) => [at(i), row[0]]);
  const fill = read.rows.map((row, i) => [at(i), row[1]]);
  const control = read.control.map((value, i) => [at(i), value]);
  const values = read.rows.flat().concat(read.control, [read.phi, read.limit]);
  const high = Math.max(...values) * 1.08;
  const low = Math.min(0, ...values);
  paintChart(read, {
    low,
    high,
    marks: [
      [read.limit, ink.green, `interior exponent ${read.limit.toFixed(9)}`],
      [read.phi, ink.pink, `Phi(f) ${read.phi.toFixed(4)}`],
    ],
    lines: [
      [fill, ink.blue, 1],
      [control, ink.orange, 1.2],
      [component, ink.gold, 1.8],
    ],
    labels: [
      [0, 'L = 1'],
      [1, `L = ${total}`],
    ],
  });
  const last = read.rows[total - 1];
  $('chart-stats').innerHTML = `
    <span><span class="swatch" style="background:${ink.gold}"></span> ${read.schedule} component rate <b>${last[0].toFixed(9)}</b></span>
    <span><span class="swatch" style="background:${ink.orange}"></span> periodic control <b>${read.control[total - 1].toFixed(9)}</b></span>
    <span><span class="swatch" style="background:${ink.blue}"></span> fill rate <b>${last[1].toFixed(9)}</b></span>
    <span><span class="swatch" style="background:${ink.green}"></span> interior exponent <b>${read.limit.toFixed(15)}</b></span>
    <span><span class="swatch" style="background:${ink.pink}"></span> Phi(f) <b>${read.phi.toFixed(9)}</b></span>
    <span>letters <b>${read.letters.join(' and ')}</b></span>
    <span>lengths <b>1 to ${total}</b></span>
    <span class="chip ${read.alphabet ? 'proved' : 'conjecture'}">${read.alphabet ? 'inside the closed-form alphabet' : 'outside the closed-form alphabet'}</span>`;
  $('chart-say').textContent = read.alphabet ? CHARTS.exponent : CHARTS.outside;
}

function stair() {
  const blocks = +$('blocks').value;
  out('blocks', blocks);
  const read = JSON.parse(m.magic_staircase(blocks));
  const rows = read.rows.map((row) => row.dimension);
  const at = (i) => (rows.length > 1 ? i / (rows.length - 1) : 0);
  const low = Math.min(read.constant, ...rows);
  const high = Math.max(read.constant, ...rows);
  const pad = (high - low) * 0.25 || 0.001;
  paintChart(read, {
    low: low - pad,
    high: high + pad,
    marks: [[read.constant, ink.blue, `the constant word ${read.constant.toFixed(9)}`]],
    lines: [[rows.map((value, i) => [at(i), value]), ink.gold, 1.8]],
    labels: [
      [0, 'one block'],
      [1, `${blocks} blocks`],
    ],
  });
  $('chart-stats').innerHTML = `
    <span><span class="swatch" style="background:${ink.gold}"></span> staircase dimension</span>
    ${read.rows.map((row) => `<span>${row.blocks} blocks, ${row.length} letters <b>${row.dimension.toFixed(9)}</b></span>`).join('')}
    <span>dips at the second block <b>${read.rows[1].dimension < read.rows[0].dimension}</b></span>`;
}

function chart(codes, numbers, bases) {
  const kind = $('chart').value;
  $('schedule').parentElement.hidden = kind !== 'exponent';
  $('length').parentElement.hidden = kind !== 'exponent';
  $('blocks').parentElement.hidden = kind !== 'stair';
  $('chart-say').textContent = CHARTS[kind];
  if (kind === 'stair') {
    stair();
    return;
  }
  if (dimension !== 2) throw new Error('the component exponent is a plane reading; switch the view off solid.');
  exponent(codes, numbers, bases);
}

// PAGE

function dress() {
  const art = $('art').checked;
  for (const id of ['builder', 'slots', 'chart-row', 'chart-canvas', 'chart-stats', 'chart-say', 'foot', 'stats', 'badges', 'scale', 'dressing']) {
    $(id).hidden = art;
  }
  document.querySelector('header').hidden = art;
  $('board').lastElementChild.hidden = art;
  $('spin-wrap').hidden = art || $('view').value !== 'solid';
  $('iso-wrap').hidden = art || $('view').value !== 'solid';
}

function build() {
  say('note');
  say('pair-note');
  dress();
  const wanted = $('view').value === 'solid' ? 3 : 2;
  if (wanted !== dimension) {
    dimension = wanted;
    mount();
  }
  snap();
  for (const one of pickers) one.read();
  const codes = slots.map((slot) => slot.code);
  const numbers = slots.map((slot) => slot.number);
  const bases = slots.map((slot) => slot.base);
  stamp({
    view: $('view').value,
    compare: $('compare').value,
    chart: $('chart').value,
    schedule: $('schedule').value,
    length: $('length').value,
    blocks: $('blocks').value,
    w: token(codes, numbers, bases) || null,
  });
  $('spell').textContent = spell(codes, numbers, bases);
  const budget = dimension === 3 ? SOLID : PLANE;
  try {
    const taken = m.magic_cap(numbers, dimension, budget);
    if (taken < 2) throw new Error(`the first two letters already pass side ${budget}; lower a side.`);
    const census = JSON.parse(m.magic_census(codes, numbers, dimension, bases));
    readout(census, codes, numbers, bases, taken);
    draw(codes, numbers, bases, taken);
  } catch (error) {
    if (solid) solid.clear();
    say('note', error);
  }
  try {
    pair(codes, numbers, bases);
  } catch (error) {
    say('pair-note', error);
  }
  try {
    chart(codes, numbers, bases);
  } catch (error) {
    $('chart-stats').textContent = String(error.message ?? error);
  }
  $('png').textContent = $('view').value === 'solid' ? 'PNG (plane only)' : 'PNG';
}

for (const [key, label] of NAMES) $('preset').append(new Option(label, key));

$('add').onclick = () => {
  snap();
  slots.push({ ...slots[slots.length - 1] });
  mount();
  build();
};
$('preset').onchange = () => load($('preset').value);
$('random').onclick = () => {
  const seed = shared.next();
  const drawn = m.random_codes(dimension, 2, seed, slots.length);
  slots = slots.map((slot, i) => ({ code: drawn[i], base: 2, number: slot.number }));
  mount();
  roll(seed, slots.map((slot, i) => `l${i}n`));
  build();
};
$('png').onclick = () => {
  if ($('view').value === 'solid') {
    say('note', new Error('the solid view saves no picture, and nothing in the crates writes OBJ.'));
    return;
  }
  const link = document.createElement('a');
  link.download = `${token(slots.map((s) => s.code), slots.map((s) => s.number), slots.map((s) => s.base)) || 'mrly_word'}.png`;
  link.href = $('sheet').toDataURL('image/png');
  link.click();
};
for (const id of ['view', 'compare', 'chart', 'schedule', 'length', 'blocks', 'art', 'spin', 'iso', 'probe']) $(id).oninput = build;

const params = query(['view', 'compare', 'chart', 'schedule', 'length', 'blocks']);
if (params.has('w')) {
  try {
    const read = JSON.parse(m.magic_parse(params.get('w')));
    slots = read.codes.map((code, i) => ({ code, base: 2, number: read.numbers[i] }));
    for (let i = 0; i < MAX_SLOTS; i++) stamp({ [`l${i}code`]: null, [`l${i}base`]: null, [`l${i}n`]: null });
  } catch {
    slots = [];
  }
} else {
  for (let i = 0; i < MAX_SLOTS; i++) {
    if (!params.has(`l${i}code`)) break;
    slots.push({ code: params.get(`l${i}code`), base: +(params.get(`l${i}base`) ?? 2), number: +(params.get(`l${i}n`) ?? 3) });
  }
}
dimension = $('view').value === 'solid' ? 3 : 2;
if (slots.length < 2) {
  slots = WORDS[dimension === 3 ? 'sponge' : 'doctest'].letters.map(([code, base, number]) => ({ code, base, number }));
}
mount();
build();
