import { $, mrly, bind, out } from './mrly.js';
import { query, stamp } from './query.js';

const FLAT3 = [['carpet', '495'], ['runner', '127']];

function designs(m, dimension, bases) {
  const list = [];
  if (bases.includes(2)) {
    for (const design of JSON.parse(m.universe(dimension)).designs) list.push([`${design.code} · ${design.anf}`, design.code, 2]);
  }
  if (dimension === 2 && bases.includes(3)) {
    for (const [word, code] of FLAT3) list.push([`${word} · ${code} · base 3`, code, 3]);
  }
  return list;
}

export function seeds() {
  let seed = +(query([]).get('seed') ?? 0);
  return {
    get: () => seed,
    next: () => {
      seed += 1;
      stamp({ seed });
      return seed;
    },
    drop: () => {
      seed = 0;
      stamp({ seed: null });
    },
  };
}

export function roll(seed, ids, skip = {}) {
  const inputs = ids.map($);
  const span = (input) => (input.options ? [0, input.options.length - 1] : [+input.min, +input.max]);
  const draws = mrly.random_between(seed, inputs.map((input) => span(input)[0]), inputs.map((input) => span(input)[1]));
  inputs.forEach((input, k) => {
    if (!input.options) {
      input.value = draws[k];
      return;
    }
    const dull = skip[input.id] ?? [];
    let at = draws[k];
    while (dull.includes(input.options[at].value)) at = (at + 1) % input.options.length;
    input.selectedIndex = at;
  });
  return draws;
}

export function cap(id, number, dimension, budget) {
  const range = $(id), top = mrly.level_cap(number, dimension, budget);
  range.max = top;
  const level = Math.min(+range.value, top);
  range.value = level;
  out(id, level);
  return level;
}

export function picker({ host, m, dimension, base = 2, code, build, extra = [], key = '', more, seeds: shared, button = true }) {
  const bases = [].concat(base);
  const id = (name) => `${key}${name}`;
  host.innerHTML = `
    <label>design <select id="${id('pick')}"></select></label>
    <label>code <input type="text" id="${id('code')}" value="${code}"></label>
    ${bases.length > 1 ? `<label>base <select id="${id('base')}">${bases.map((b) => `<option>${b}</option>`).join('')}</select></label>` : ''}
    ${button ? `<button id="${id('random')}">Randomize</button>` : ''}`;
  query([id('code'), id('base'), ...extra].filter($));
  const s = shared ?? seeds();
  const pick = $(id('pick')), input = $(id('code'));
  const baseOf = () => (bases.length > 1 ? +$(id('base')).value : bases[0]);
  pick.append(new Option('type a code', ''));
  for (const [label, value, b] of designs(m, dimension, bases)) pick.append(new Option(label, `${value}:${b}`));
  const apply = () => {
    input.value = m.random_code(dimension, baseOf(), s.get());
    if (more) more(s.get());
  };
  pick.onchange = () => {
    if (!pick.value) return;
    const [value, b] = pick.value.split(':');
    input.value = value;
    if (bases.length > 1) $(id('base')).value = b;
    s.drop();
    build();
  };
  input.oninput = () => {
    s.drop();
    build();
  };
  if (bases.length > 1) $(id('base')).oninput = build;
  const randomize = () => {
    s.next();
    apply();
    build();
  };
  if (button) $(id('random')).onclick = randomize;
  if (!shared && s.get()) apply();
  const read = () => {
    const value = input.value.trim(), b = baseOf();
    pick.value = pick.querySelector(`option[value="${value}:${b}"]`) ? `${value}:${b}` : '';
    const values = { [id('code')]: value };
    if (bases.length > 1) values[id('base')] = b;
    for (const x of extra) values[x] = $(x).value;
    stamp(values);
    return { code: value, base: b, name: m.name_of(value, dimension, b) };
  };
  return { read, apply, randomize };
}

export function sources(m, host, build, more) {
  host.innerHTML = `
    <label>source <select id="source">
      <option value="flat">flat design</option>
      <option value="moire">moire stack</option>
      <option value="slice">hex slice</option>
    </select></label>
    <span id="flat-row">
      <span id="flat-pick"></span>
      <label>side <select id="number"><option selected>3</option><option>5</option><option>7</option></select></label>
      <label>level <input type="range" id="level" min="1" max="5" value="4"><span class="num" id="level-out">4</span></label>
    </span>
    <span id="moire-row" hidden>
      <label>preset <select id="preset"></select></label>
      <button id="mrandom">Randomize</button>
      <label>scales up to <input type="range" id="limit" min="1" max="41" step="2" value="9"><span class="num" id="limit-out">9</span></label>
    </span>
    <span id="slice-row" hidden>
      <span id="slice-pick"></span>
      <label>tile <select id="tile"><option selected>3</option><option>5</option></select></label>
      <label>level <input type="range" id="slevel" min="1" max="3" value="2"><span class="num" id="slevel-out">2</span></label>
    </span>`;
  for (const name of m.moire_names()) $('preset').append(new Option(name, name));
  const s = seeds();
  const flat = picker({ host: $('flat-pick'), m, dimension: 2, base: [3, 2], code: '495', build, extra: ['number', 'level'], seeds: s, more });
  const cut = picker({ host: $('slice-pick'), m, dimension: 3, code: '23', build, key: 's', extra: ['tile', 'slevel'], seeds: s, more });
  query(['source', 'preset', 'limit']);
  bind(['source', 'number', 'level', 'preset', 'limit', 'tile', 'slevel'], build);
  const kind = () => $('source').value;
  const apply = () => {
    if (kind() === 'flat') flat.apply();
    else if (kind() === 'slice') cut.apply();
    else {
      roll(s.get(), ['preset', 'limit']);
      if (more) more(s.get());
    }
  };
  $('mrandom').onclick = () => {
    s.next();
    apply();
    build();
  };
  if (s.get()) apply();
  const read = () => {
    const which = kind();
    for (const row of ['flat', 'moire', 'slice']) $(`${row}-row`).hidden = row !== which;
    stamp({ source: which });
    if (which === 'flat') {
      const { code, base, name } = flat.read();
      const number = +$('number').value;
      const level = cap('level', number, 1, 243);
      const grid = m.two_grid(code, number, level, 0, base);
      const fills = JSON.parse(m.two_census(code, number, level, 0, base)).fills;
      return { kind: which, grid, field: Float32Array.from(grid.types), size: grid.width, name, fills };
    }
    if (which === 'moire') {
      const limit = +$('limit').value, name = $('preset').value;
      out('limit', limit);
      stamp({ preset: name, limit });
      return { kind: which, grid: null, field: m.moire_field(name, limit, 256), size: 256, name, fills: '' };
    }
    const { code, name } = cut.read();
    const tile = +$('tile').value;
    const level = cap('slevel', tile, 1, 27);
    const grid = m.slice_grid(code, tile, level, 2, 384);
    const fills = JSON.parse(m.slice_census(code, tile, level, 2)).fills;
    return { kind: which, grid, field: Float32Array.from(grid.types), size: 384, name, fills };
  };
  return { read };
}
