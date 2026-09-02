import { useEffect, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { board, line, axis, tag } from '../lib/chart.js';
import { faces } from '../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stat, Note } from '../lib/app.jsx';
import { Grid, Pixels, Sketch } from '../lib/draw.jsx';
import { Stage } from '../lib/stage.jsx';
import { Picker, useSeeds, roll } from '../lib/select.jsx';
import { useQuery, stamp, share } from '../lib/query.js';

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

const FIRST = { view: 'plane', compare: 'swap', chart: 'exponent', schedule: 'thue-morse', length: 64, blocks: 5 };

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const side = (value) => Math.min(16, Math.max(2, +value || 2));

const spell = (dimension, codes, numbers, bases) =>
  (dimension === 3 ? 'd3: ' : '') +
  codes.map((code, i) => `c${code}${bases[i] === 2 ? '' : `.q${bases[i]}`}(${numbers[i]})`).join(', ');

function token(dimension, codes, numbers, bases) {
  if (dimension !== 2 || bases.some((base) => base !== 2)) return '';
  try {
    return m.magic_name(codes, numbers);
  } catch {
    return '';
  }
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

function flags(census) {
  return [
    census.constant && <span key="constant" className="chip proved">constant</span>,
    census.periodic && <span key="periodic" className="chip proved">periodic</span>,
    census.composite && <span key="composite" className="chip proved">composite at base {census.residue_base}</span>,
    census.native && <span key="native" className="chip verified">native</span>,
  ].filter(Boolean);
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
  return m.sheet(field, size, 'fire', taken, false);
}

function tile(word, budget) {
  const taken = m.magic_cap(word.numbers, word.dimension, budget);
  if (taken < 2) throw new Error('the first two letters already pass the page budget; lower a side.');
  const grid = m.magic_grid(word.codes.slice(0, taken), word.numbers.slice(0, taken), word.bases.slice(0, taken));
  const census = JSON.parse(m.magic_census(word.codes, word.numbers, word.dimension, word.bases));
  return { word, taken, grid, census };
}

function firstSlots(params, dimension) {
  const list = [];
  if (params.has('w')) {
    try {
      const read = JSON.parse(m.magic_parse(params.get('w')));
      for (const [i, code] of read.codes.entries()) list.push({ code, base: 2, number: read.numbers[i] });
      for (let i = 0; i < MAX_SLOTS; i++) stamp({ [`l${i}code`]: null, [`l${i}base`]: null, [`l${i}n`]: null });
    } catch {
      list.length = 0;
    }
  } else {
    for (let i = 0; i < MAX_SLOTS; i++) {
      if (!params.has(`l${i}code`)) break;
      list.push({ code: params.get(`l${i}code`), base: +(params.get(`l${i}base`) ?? 2), number: +(params.get(`l${i}n`) ?? 3) });
    }
  }
  if (list.length >= 2) return list;
  return WORDS[dimension === 3 ? 'sponge' : 'doctest'].letters.map(([code, base, number]) => ({ code, base, number }));
}

function App() {
  const shared = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [slots, setSlots] = useState(() => firstSlots(new URLSearchParams(location.search), pick.view === 'solid' ? 3 : 2));
  const [preset, setPreset] = useState('doctest');
  const [art, setArt] = useState(false);
  const sheetRef = useRef(null);
  const [spin, setSpin] = useState(true);
  const [iso, setIso] = useState(false);
  const [probe, setProbe] = useState('4');
  const [shout, setShout] = useState(null);

  const dimension = pick.view === 'solid' ? 3 : 2;
  const codes = slots.map((slot) => slot.code.trim());
  const numbers = slots.map((slot) => side(slot.number));
  const bases = slots.map((slot) => (dimension === 2 ? slot.base : 2));
  const name = token(dimension, codes, numbers, bases);
  const sig = `${pick.view}|${JSON.stringify(slots)}`;

  useEffect(() => {
    const values = { w: name || null };
    for (let i = 0; i < MAX_SLOTS; i++) {
      values[`l${i}code`] = i < slots.length ? codes[i] : null;
      values[`l${i}base`] = i < slots.length && dimension === 2 ? bases[i] : null;
      values[`l${i}n`] = i < slots.length ? numbers[i] : null;
    }
    stamp(values);
  }, [sig]);

  useEffect(() => {
    setShout(null);
  }, [sig, pick.compare, pick.chart, pick.schedule, pick.length, pick.blocks, art, spin, iso, probe]);

  const budget = dimension === 3 ? SOLID : PLANE;
  const word = attempt(() => {
    const taken = m.magic_cap(numbers, dimension, budget);
    if (taken < 2) throw new Error(`the first two letters already pass side ${budget}; lower a side.`);
    return { taken, census: JSON.parse(m.magic_census(codes, numbers, dimension, bases)) };
  });
  const cut = (list) => list.slice(0, word.taken);
  const shape =
    pick.view === 'solid' && !word.error
      ? attempt(() => {
        const count = Number(m.word_count(cut(codes), cut(numbers), 3, cut(bases)));
        if (count > CUBES) throw new Error(`${count} cubes is more than this page draws; drop a letter or lower a side.`);
        return { buffer: m.magic_faces(cut(codes), cut(numbers), cut(bases)) };
      })
      : { buffer: null, error: null };

  const pair = attempt(() => {
    if (dimension !== 2 && pick.compare !== 'collide') throw new Error('the pair panel is a plane reading; switch the view off solid.');
    const plain = { codes, numbers, bases, dimension };
    let left = plain;
    let right = plain;
    if (pick.compare === 'double') {
      right = { codes: codes.concat(codes), numbers: numbers.concat(numbers), bases: bases.concat(bases), dimension };
    } else if (pick.compare === 'swap') {
      const order = (list) => [list[1], list[0], ...list.slice(2)];
      right = { codes: order(codes), numbers: order(numbers), bases: order(bases), dimension };
    } else {
      left = { ...COLLISION[0], dimension: 2 };
      right = { ...COLLISION[1], dimension: 2 };
    }
    const a = tile(left, PLANE);
    const b = tile(right, PLANE);
    const same = a.grid.width === b.grid.width && a.grid.types.every((byte, i) => byte === b.grid.types[i]);
    const square = (text) => (BigInt(text) * BigInt(text)).toString();
    const reads = [];
    if (pick.compare === 'double') {
      reads.push(['side squares', b.census.side === square(a.census.side)]);
      reads.push(['fill squares', b.census.fill === square(a.census.fill)]);
      reads.push(['dimension unmoved', Math.abs(a.census.dimension - b.census.dimension) < 1e-12]);
    } else if (pick.compare === 'swap') {
      reads.push(['side equal', a.census.side === b.census.side]);
      reads.push(['fill equal', a.census.fill === b.census.fill]);
      reads.push(['pieces equal', a.census.components === b.census.components]);
    } else {
      reads.push(['same tile', same]);
      reads.push(['same word', false]);
    }
    return { a, b, reads };
  });

  const chart = attempt(() => {
    if (pick.chart === 'stair') {
      const read = JSON.parse(m.magic_staircase(pick.blocks));
      const rows = read.rows.map((row) => row.dimension);
      const at = (i) => (rows.length > 1 ? i / (rows.length - 1) : 0);
      const low = Math.min(read.constant, ...rows);
      const high = Math.max(read.constant, ...rows);
      const pad = (high - low) * 0.25 || 0.001;
      const options = {
        low: low - pad,
        high: high + pad,
        marks: [[read.constant, ink.blue, `the constant word ${read.constant.toFixed(9)}`]],
        lines: [[rows.map((value, i) => [at(i), value]), ink.gold, 1.8]],
        labels: [[0, 'one block'], [1, `${pick.blocks} blocks`]],
      };
      const stats = (
        <>
          <span><span className="swatch" style={{ background: ink.gold }}></span> staircase dimension</span>
          {read.rows.map((row) => <span key={row.blocks}>{row.blocks} blocks, {row.length} letters <b>{row.dimension.toFixed(9)}</b></span>)}
          <span>dips at the second block <b>{String(read.rows[1].dimension < read.rows[0].dimension)}</b></span>
        </>
      );
      return { options, stats, say: CHARTS.stair };
    }
    if (dimension !== 2) throw new Error('the component exponent is a plane reading; switch the view off solid.');
    const read = JSON.parse(m.magic_rates(codes, numbers, bases, pick.schedule, pick.length));
    const total = read.length;
    const at = (i) => (total > 1 ? i / (total - 1) : 0);
    const component = read.rows.map((row, i) => [at(i), row[0]]);
    const fill = read.rows.map((row, i) => [at(i), row[1]]);
    const control = read.control.map((value, i) => [at(i), value]);
    const values = read.rows.flat().concat(read.control, [read.phi, read.limit]);
    const options = {
      low: Math.min(0, ...values),
      high: Math.max(...values) * 1.08,
      marks: [
        [read.limit, ink.green, `interior exponent ${read.limit.toFixed(9)}`],
        [read.phi, ink.pink, `Phi(f) ${read.phi.toFixed(4)}`],
      ],
      lines: [[fill, ink.blue, 1], [control, ink.orange, 1.2], [component, ink.gold, 1.8]],
      labels: [[0, 'L = 1'], [1, `L = ${total}`]],
    };
    const last = read.rows[total - 1];
    const stats = (
      <>
        <span><span className="swatch" style={{ background: ink.gold }}></span> {read.schedule} component rate <b>{last[0].toFixed(9)}</b></span>
        <span><span className="swatch" style={{ background: ink.orange }}></span> periodic control <b>{read.control[total - 1].toFixed(9)}</b></span>
        <span><span className="swatch" style={{ background: ink.blue }}></span> fill rate <b>{last[1].toFixed(9)}</b></span>
        <span><span className="swatch" style={{ background: ink.green }}></span> interior exponent <b>{read.limit.toFixed(15)}</b></span>
        <span><span className="swatch" style={{ background: ink.pink }}></span> Phi(f) <b>{read.phi.toFixed(9)}</b></span>
        <span>letters <b>{read.letters.join(' and ')}</b></span>
        <span>lengths <b>1 to {total}</b></span>
        <span className={`chip ${read.alphabet ? 'proved' : 'conjecture'}`}>{read.alphabet ? 'inside the closed-form alphabet' : 'outside the closed-form alphabet'}</span>
      </>
    );
    return { options, stats, say: read.alphabet ? CHARTS.exponent : CHARTS.outside };
  });

  const patch = (i, values) => setSlots(slots.map((slot, k) => (k === i ? { ...slot, ...values } : slot)));

  const load = (key) => {
    setPreset(key);
    setSlots(WORDS[key].letters.map(([code, base, number]) => ({ code, base, number })));
    set({ view: WORDS[key].view, compare: WORDS[key].compare, chart: WORDS[key].chart, ...(key === 'morse' ? { schedule: 'thue-morse' } : {}) });
  };

  const randomize = () => {
    const seed = shared.next();
    const drawn = m.random_codes(dimension, 2, seed, slots.length);
    const sides = roll(seed, slots.map(() => [2, 16]));
    setSlots(slots.map((slot, i) => ({ code: drawn[i], base: 2, number: sides[i] })));
  };

  const swap = (i) => {
    const next = [...slots];
    const at = (i + 1) % next.length;
    [next[i], next[at]] = [next[at], next[i]];
    setSlots(next);
  };

  const png = () => {
    if (pick.view === 'solid') {
      setShout(new Error('the solid view saves no picture, and nothing in the crates writes OBJ.'));
      return;
    }
    const canvas = sheetRef.current;
    if (!canvas) return;
    const link = document.createElement('a');
    link.download = `${name || 'mrly_word'}.png`;
    link.href = canvas.toDataURL('image/png');
    link.click();
  };

  const member = () => {
    const text = probe.trim();
    if (!text) return '';
    try {
      return String(m.word_member(codes, numbers, dimension, bases, text));
    } catch (error) {
      return String(error.message ?? error);
    }
  };

  const sheet = () => {
    if (word.error || pick.view === 'solid') return null;
    if (pick.view === 'nest') return <Pixels data={nesting(codes, numbers, bases, word.taken)} canvasRef={sheetRef} />;
    return <Grid grid={m.magic_grid(cut(codes), cut(numbers), cut(bases))} on={ink.gold} canvasRef={sheetRef} />;
  };

  const onStage = (live) => {
    if (pick.view !== 'solid' || !shape.buffer) {
      live.clear();
      return;
    }
    live.show(faces(shape.buffer, ink.blue, 1));
    live.project(iso ? 'iso' : 'eye');
    live.spin = spin ? 0.004 : 0;
  };

  const drawChart = (canvas) => {
    if (!chart.options) return;
    const b = board(canvas, 240, { left: 46, right: 16 });
    const { low, high, marks, lines, labels } = chart.options;
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
  };

  const readout = () => {
    const census = word.census;
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
    return (
      <>
        {name && <Stat label="name">{name}</Stat>}
        <Stat label="side">{census.side}</Stat>
        <Stat label="cells">{census.cells}</Stat>
        <Stat label="filled">{census.fill}</Stat>
        <Stat label="empty">{census.voids}</Stat>
        <Stat label="density">{census.ratio.toFixed(4)}</Stat>
        <Stat label="dimension">{census.dimension.toFixed(4)}</Stat>
        {census.components ? <span>pieces <b>{census.components}</b> <span className="dim">{census.counted}</span></span> : null}
        <span>press members <b>{String(members)}</b> <span className={`chip ${agrees ? 'verified' : 'refuted'}`}>{agrees ? 'agrees with the fill' : 'differs from the fill'}</span></span>
        {profile.length ? <span>diagonal profile <b>{profile.length}</b> heights, peak <b>{peak}</b> at <b>{profile.indexOf(peak)}</b></span> : null}
        {head ? <Stat label="first members">{head}</Stat> : null}
        {flags(census)}
      </>
    );
  };

  const scale = () => {
    const census = word.census;
    const drawing = word.taken < codes.length
      ? <>Drawing <b>{word.taken} of {codes.length} letters</b>, the box cover of the whole word at side {census.letters.slice(0, word.taken).reduce((a, l) => a * l.number, 1)}. </>
      : null;
    return <>{drawing}The readout is a product over the letters, so it outruns the raster on purpose: every number above is exact at the full length. {repeats(codes, numbers, bases)} <a href="../moire">Moire</a> stacks one design over its scales instead.</>;
  };

  const pane = (one) => (
    <div>
      <Grid grid={one.grid} on={ink.gold} />
      <div className="stats">
        <span>{spell(dimension, one.word.codes, one.word.numbers, one.word.bases)}</span>
        <Stat label="side">{one.census.side}</Stat>
        <Stat label="filled">{one.census.fill}</Stat>
        <Stat label="dim">{one.census.dimension.toFixed(4)}</Stat>
        {one.census.components ? <Stat label="pieces">{one.census.components}</Stat> : null}
        {one.taken < one.word.codes.length ? <span className="dim">drawn {one.taken} of {one.word.codes.length} letters</span> : null}
      </div>
    </div>
  );

  return (
    <Page bare={art} crumb="words" title="The words"
      sub="One design per level. Same letters in a different order, a different set. Build a word letter by letter, first letter outermost, and every number below comes back out of the crates as a product over the letters."
      foot={<>Side, fill, density and dimension are products over the letters, so the readout is exact at any length even where the raster is not: the plane draws to side 243 and the cube to side 128, and a shorter render is labelled <b>k of L letters</b> because it is the box cover of the whole word at that scale and never a shallower word. Words with two or more letters only; a word of one repeated letter is the ordinary fractal under another name. Links: <a href="../sponge">the sponge</a> grows one cube design level by level, <a href="../moire">moire</a> stacks one design over its scales. The grammar of a word, the families that collapse back into one fractal and the ones that do not are in <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/magic.md">the magic words note</a>.</>}>
      <Row>
        <span className="set" hidden={art}>
          <Pick label="preset" value={preset} options={NAMES} onChange={load} />
          <Btn onClick={randomize}>Randomize</Btn>
          <Pick label="view" value={pick.view} options={[['plane', 'plane'], ['nest', 'nesting'], ['solid', 'solid']]} onChange={(v) => set({ view: v })} />
          {pick.view === 'solid' && <Check label="spin" checked={spin} onChange={setSpin} />}
          {pick.view === 'solid' && <Check label="iso" checked={iso} onChange={setIso} />}
        </span>
        <Check label="art" checked={art} onChange={setArt} />
        <Btn onClick={png}>{pick.view === 'solid' ? 'PNG (plane only)' : 'PNG'}</Btn>
      </Row>
      <div hidden={art}>
        {slots.map((slot, i) => (
          <Row key={i}>
            <span className="badge">letter {i + 1}</span>
            <span className="set">
              <Picker dimension={dimension} bases={dimension === 2 ? [2, 3] : [2]} code={slot.code} base={bases[i]} seeds={shared} button={false} onChange={(values) => patch(i, values)} />
            </span>
            <label>side <input type="number" value={slot.number} min={2} max={16} onChange={(e) => patch(i, { number: e.target.value })} /></label>
            <button disabled={slots.length < 2} onClick={() => swap(i)}>swap next</button>
            <button disabled={slots.length < 3} onClick={() => setSlots(slots.filter((one, k) => k !== i))}>remove</button>
          </Row>
        ))}
      </div>
      <Row hidden={art}>
        <button disabled={slots.length >= MAX_SLOTS} onClick={() => setSlots([...slots, { ...slots[slots.length - 1] }])}>add a letter</button>
        <label>is <input type="text" value={probe} onChange={(e) => setProbe(e.target.value)} /> a member <b className="num">{member()}</b></label>
        <span className="badge dim">{spell(dimension, codes, numbers, bases)}</span>
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>the word <span>{spell(dimension, codes, numbers, bases)}</span></h2>
          {sheet()}
          <Stage onStage={onStage} deps={[pick.view, shape.buffer, iso, spin]} hidden={pick.view !== 'solid'} />
          <div className="stats" hidden={art}>{word.error ? null : readout()}</div>
          <div className="stats" hidden={art}>{word.error ? null : word.census.letters.map((letter, i) => (
            <span key={i} className="badge">{i + 1} <b>{letter.name}</b> side {letter.number} fill {letter.fill} dim {letter.dimension.toFixed(4)}{letter.native ? ' native' : ''}</span>
          ))}</div>
          <Note error={shout ?? word.error ?? shape.error} />
          <p className="sub" hidden={art}>{word.error ? null : scale()}</p>
          <p className="sub">The first letter alone grows level by level in <a href={`../sponge${share({ code: codes[0], base: bases[0], number: numbers[0], level: 3 })}`}>the sponge</a>.</p>
        </div>
        <div className="panel" hidden={art}>
          <h2>the pair <span>{pick.compare === 'collide' ? 'one tile, two words' : `the word beside ${pick.compare === 'double' ? 'itself doubled' : 'its swap'}`}</span></h2>
          <Pick label="compare" value={pick.compare} onChange={(v) => set({ compare: v })}
            options={[['swap', 'the first two letters swapped'], ['double', 'the same word doubled'], ['collide', 'I(2) x I(3) beside I(3) x I(2)']]} />
          <div className="arena">
            {pair.error ? null : pane(pair.a)}
            {pair.error ? null : pane(pair.b)}
          </div>
          <Note error={pair.error}>{pair.error ? null : pair.reads.map(([text, value], i) => <span key={i}>{text} <b>{String(value)}</b>{' '}</span>)}</Note>
          <p className="sub">{SAYS[pick.compare]}</p>
        </div>
      </div>
      <Row hidden={art}>
        <Pick label="chart" value={pick.chart} options={[['exponent', 'the component exponent'], ['stair', 'the staircase dimension']]} onChange={(v) => set({ chart: v })} />
        {pick.chart === 'exponent' && <Pick label="schedule" value={pick.schedule} options={[['thue-morse', 'Thue-Morse'], ['periodic', 'periodic'], ['constant', 'constant']]} onChange={(v) => set({ schedule: v })} />}
        {pick.chart === 'exponent' && <Slider label="length" value={pick.length} min={2} max={98} onChange={(v) => set({ length: v })} />}
        {pick.chart === 'stair' && <Slider label="blocks" value={pick.blocks} min={2} max={8} onChange={(v) => set({ blocks: v })} />}
      </Row>
      <Sketch draw={drawChart} deps={[chart]} className="bars" hidden={art} />
      <div className="stats" hidden={art}>{chart.error ? String(chart.error.message ?? chart.error) : chart.stats}</div>
      <p className="sub" hidden={art}>{chart.say ?? CHARTS[pick.chart]}</p>
    </Page>
  );
}

mount(<App />);
