import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { board, bars, line, axis, tag } from '../../lib/chart.js';
import { faces } from '../../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Grid, Markup, Sketch } from '../../lib/draw.jsx';
import { Stage } from '../../lib/stage.jsx';
import { Picker, useSeeds, roll } from '../../lib/select.jsx';
import { useQuery, stamp } from '../../lib/query.js';

const m = await ready();
const MAX_SLOTS = 8;
const DIMS = [[2, 'the plane'], [3, 'the cube'], [6, 'the hexagon']];
const PROJECTIONS = [['cut', 'the middle slice'], ['pro', 'three facing sides'], ['iso', 'the isometric skin']];
const SIDE = { 2: 243, 3: 64, 6: 32 };
const DROP = { 2: 1, 3: 2, 6: 1 };
const CUBES = 30000;
const TRIANGLES = 12000;
const WIDE = 220;

const TOWERS = {
  gasket: { dim: 2, letters: [['7', 2, 2], ['7', 2, 2], ['7', 2, 2], ['7', 2, 2], ['7', 2, 2], ['7', 2, 2]] },
  carpet: { dim: 2, letters: [['495', 3, 3], ['495', 3, 3], ['495', 3, 3], ['495', 3, 3], ['495', 3, 3]] },
  order: { dim: 2, letters: [['7', 2, 2], ['9', 2, 2], ['7', 2, 2], ['9', 2, 2], ['7', 2, 2], ['9', 2, 2]] },
  doctest: { dim: 2, letters: [['7', 2, 3], ['14', 2, 7], ['9', 2, 5]] },
  sponge: { dim: 3, letters: [['23', 2, 3], ['23', 2, 3], ['23', 2, 3]] },
  pair: { dim: 3, letters: [['23', 2, 3], ['9', 2, 3], ['23', 2, 3]] },
  shadow: { dim: 6, letters: [['23', 2, 3], ['23', 2, 3], ['23', 2, 3]] },
};

const NAMES = [
  ['gasket', 'the gasket, one letter six times'],
  ['carpet', 'the carpet at base three'],
  ['order', 'two letters alternating'],
  ['doctest', 'the doctest word, three sides'],
  ['sponge', 'the sponge in the cube'],
  ['pair', 'two cube letters'],
  ['shadow', 'the sponge tower as a hexagon'],
];

const FIRST = { dim: 2, proj: 'cut', blocks: 6 };
const START = { 2: 'gasket', 3: 'sponge', 6: 'shadow' };

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const side = (value) => Math.min(16, Math.max(2, +value || 2));

const spell = (codes, numbers, bases) =>
  codes.map((code, i) => `c${code}${bases[i] === 2 ? '' : `.q${bases[i]}`}(${numbers[i]})`).join(' ');

function firstSlots(params, dim) {
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
  return TOWERS[START[dim]].letters.map(([code, base, number]) => ({ code, base, number }));
}

// BLOCKS

function count(dim, codes, numbers, bases, proj) {
  if (codes.length === 1) {
    const reps = dim === 3 ? [1, 1, 1] : [1, 1];
    const one = JSON.parse(m.tile_census(codes[0], numbers[0], 1, bases[0], dim, proj, reps, false));
    return {
      span: String(dim === 6 ? one.sheet[0] : one.side),
      cells: one.cells,
      fills: one.fills,
      exposed: one.exposed,
      ratio: one.ratio,
      wide: one.sheet[0],
    };
  }
  if (dim === 6) {
    const hex = JSON.parse(m.magic_hex_census(codes, numbers, bases, proj));
    return {
      span: String(hex.grid[0]),
      cells: String(hex.triangles),
      fills: String(hex.fills),
      exposed: String(hex.exposed),
      ratio: hex.ratio,
      wide: hex.grid[0],
    };
  }
  const census = JSON.parse(m.magic_census(codes, numbers, dim, bases));
  return {
    span: census.side,
    cells: census.cells,
    fills: census.fill,
    exposed: dim === 2 ? m.magic_perimeter(codes, numbers, bases) : m.magic_surface(codes, numbers, bases),
    ratio: census.ratio,
    wide: Number(census.side),
  };
}

function picture(dim, codes, numbers, bases, proj, scale) {
  const one = codes.length === 1;
  if (dim === 2) return { grid: one ? m.two_grid(codes[0], numbers[0], 1, 0, bases[0]) : m.magic_grid(codes, numbers, bases) };
  if (dim === 3) return { buffer: one ? m.three_faces(codes[0], numbers[0], 1, bases[0]) : m.magic_faces(codes, numbers, bases) };
  return { svg: one ? m.hex_svg(codes[0], numbers[0], 1, bases[0], proj, scale) : m.magic_hex(codes, numbers, bases, proj, scale) };
}

const density = (dim, block) => Number(block.exposed) / Number(block.span) ** DROP[dim];

function running(values) {
  const out = [];
  let sum = 0;
  for (const value of values) {
    sum += value;
    out.push(sum);
  }
  const last = out[out.length - 1] || 1;
  return out.map((value, i) => [(i + 0.5) / out.length, value / last]);
}

function App() {
  const seeds = useSeeds();
  const [q, set] = useQuery(FIRST);
  const dim = START[q.dim] ? q.dim : 2;
  const [slots, setSlots] = useState(() => firstSlots(new URLSearchParams(location.search), dim));
  const [preset, setPreset] = useState(START[FIRST.dim]);
  const [spin, setSpin] = useState(true);
  const live = useRef(null);

  const solid = dim !== 2;
  const codes = slots.map((slot) => slot.code.trim());
  const numbers = slots.map((slot) => side(slot.number));
  const bases = slots.map((slot) => (solid ? 2 : slot.base));
  const name = solid || bases.some((base) => base !== 2) ? '' : attempt(() => ({ text: m.magic_name(codes, numbers) })).text ?? '';
  const sig = `${dim}|${JSON.stringify(slots)}`;

  const cap = attempt(() => ({ top: m.magic_cap(numbers, solid ? 3 : 2, SIDE[dim]) })).top ?? 1;
  const depth = Math.max(1, Math.min(q.blocks || 1, cap));

  useEffect(() => {
    const values = { w: name || null };
    for (let i = 0; i < MAX_SLOTS; i++) {
      values[`l${i}code`] = i < slots.length ? codes[i] : null;
      values[`l${i}base`] = i < slots.length && !solid ? bases[i] : null;
      values[`l${i}n`] = i < slots.length ? numbers[i] : null;
    }
    stamp(values);
  }, [sig]);

  const word = useMemo(() => attempt(() => ({ census: JSON.parse(m.magic_census(codes, numbers, solid ? 3 : 2, bases)) })), [sig]);

  const tower = useMemo(() => {
    const blocks = [];
    let error = null;
    for (let k = 1; k <= depth; k++) {
      try {
        const cut = [codes.slice(0, k), numbers.slice(0, k), bases.slice(0, k)];
        const read = count(dim, ...cut, q.proj);
        if (dim === 3 && Number(read.fills) > CUBES) throw new Error(`block ${k} holds ${read.fills} cubes, more than this page draws; drop a letter or lower a side.`);
        if (dim === 6 && Number(read.cells) > TRIANGLES) throw new Error(`block ${k} holds ${read.cells} triangles, more than this page draws; drop a letter or lower a side.`);
        const scale = Math.max(1, Math.round(WIDE / read.wide));
        blocks.push({ k, word: spell(...cut), ...read, ...picture(dim, ...cut, q.proj, scale) });
      } catch (fault) {
        error = fault;
        break;
      }
    }
    return { blocks, error };
  }, [sig, dim, q.proj, depth]);

  const patch = (i, values) => setSlots(slots.map((slot, k) => (k === i ? { ...slot, ...values } : slot)));

  const load = (key) => {
    setPreset(key);
    setSlots(TOWERS[key].letters.map(([code, base, number]) => ({ code, base, number })));
    set({ dim: TOWERS[key].dim });
  };

  const shift = (value) => {
    seeds.drop();
    load(START[+value]);
  };

  const randomize = () => {
    const seed = seeds.next();
    const drawn = m.random_codes(solid ? 3 : 2, 2, seed, slots.length);
    const sides = roll(seed, slots.map(() => [2, solid ? 4 : 6]));
    setSlots(slots.map((slot, i) => ({ code: drawn[i], base: 2, number: sides[i] })));
  };

  const swap = (i) => {
    const next = [...slots];
    const at = (i + 1) % next.length;
    [next[i], next[at]] = [next[at], next[i]];
    setSlots(next);
  };

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  const onStage = (stage) => {
    live.current = stage;
    stage.clear();
    stage.spin = spin ? 0.004 : 0;
    const drawn = dim === 3 ? tower.blocks : [];
    drawn.forEach((block, i) => {
      const mesh = faces(block.buffer, ink.blue, 1);
      mesh.scale.setScalar(0.92 / drawn.length);
      mesh.position.set(-1 + (2 * i + 1) / drawn.length, 0, 0);
      stage.add(mesh);
    });
  };

  const drawChart = (canvas) => {
    const rows = tower.blocks;
    const b = board(canvas, 240, { left: 16, right: 16 });
    if (!rows.length) return;
    axis(b, [[0, 'block 1'], [1, `block ${rows.length}`]], { wall: true });
    bars(b, rows.map((row) => row.ratio), { color: ink.dim, inset: 4 });
    line(b, running(rows.map((row) => row.ratio)), ink.gold, { width: 1.8, dots: 3 });
    line(b, running(rows.map((row) => density(dim, row))), ink.blue, { width: 1.8, dots: 3 });
    tag(b, 'each running total against its own last value', ink.dim, 'right');
  };

  const art = (block) => {
    if (dim === 2) return <Grid grid={block.grid} on={ink.gold} role="img" aria-label={`block ${block.k}, ${block.word}`} />;
    if (dim === 6) return <Markup svg={block.svg} role="img" aria-label={`block ${block.k}, ${block.word}`} />;
    return null;
  };

  const census = word.census ?? {};
  const letters = census.letters ?? [];
  const shrinks = letters.length > 0 && letters.some((letter) => letter.fill !== letter.cells);
  const climbs = shrinks && census.dimension > DROP[dim];

  const controls = (
    <>
      <Group name="Tower">
        <Row>
          <Pick label="dimension" value={dim} options={DIMS} onChange={shift} />
          <Pick label="tower" value={preset} options={NAMES} onChange={load} />
          <Slider label="blocks" value={depth} min={1} max={cap} show={`${depth}/${cap}`} onChange={(v) => set({ blocks: v })} />
          <Btn onClick={randomize}>Randomize</Btn>
          {dim === 6 && <Pick label="projection" value={q.proj} options={PROJECTIONS} onChange={(v) => set({ proj: v })} />}
          {dim === 3 && <Check label="spin" checked={spin} onChange={turn} />}
        </Row>
      </Group>
      <Group name="Letters">
        {slots.map((slot, i) => (
          <Row key={i}>
            <span className="badge">letter {i + 1}</span>
            <span className="set">
              <Picker dimension={solid ? 3 : 2} bases={solid ? [2] : [2, 3]} code={slot.code} base={bases[i]} seeds={seeds} button={false} onChange={(values) => patch(i, values)} />
            </span>
            <label>side <input type="number" value={slot.number} min={2} max={16} onChange={(e) => patch(i, { number: e.target.value })} /></label>
            <button disabled={slots.length < 2} onClick={() => swap(i)}>swap next</button>
            <button disabled={slots.length < 3} onClick={() => setSlots(slots.filter((one, k) => k !== i))}>remove</button>
          </Row>
        ))}
        <Row>
          <button disabled={slots.length >= MAX_SLOTS} onClick={() => setSlots([...slots, { ...slots[slots.length - 1] }])}>add a letter</button>
        </Row>
      </Group>
    </>
  );

  return (
    <Page crumb="tower" title="Finite volume, infinite surface" controls={controls}
      sub="Gabriel's tower. The tile lays one design side by side on every axis; hold every axis but one to a single copy and let the word rise a letter per block, and the blocks stand at the same physical side while the design inside them deepens. A block's volume is its fill fraction, which falls by one letter's share at every step, so the tower's volume converges. Its surface is the exposed count over the side, and once a letter is under one and the design's dimension passes d - 1 that density climbs without bound. Finite volume, infinite surface, the fractal cousin of Gabriel's horn."
      foot={<>Every block is a prefix of the word, built in Rust before a pixel is drawn: the plane block is the word's grid, the cube block its exposed faces, the hexagon block the projected skin of the same cube word. Every printed number is a Rust number or a ratio of two Rust integers with both operands in view. The word, its letters and the products they multiply are on <a href="../words">the words</a>; the same design laid side by side on every axis instead of one is <a href="../tile">the tile</a>. The construction of a word and the fill law behind the geometric decay are written up in <a href="/research/magic/">the research note on magic words</a>.</>}>
      <p className="badge dim">{name ? `${name} · ` : ''}{spell(codes, numbers, bases)}</p>
      <Stage hidden={dim !== 3} role="img" aria-label="The tower" deps={[tower]} onStage={onStage} />
      <div className="tower">
        {tower.blocks.map((block) => (
          <div className="block" key={block.k}>
            {art(block)}
            <div className="stats">
              <span>block {block.k} <b>{block.word}</b></span>
              <Stat label={dim === 6 ? 'mesh wide' : 'side'}>{block.span}</Stat>
              <span>{dim === 6 ? 'inked' : 'fills'} / {dim === 6 ? 'triangles' : 'cells'} <b>{block.fills}</b> / <b>{block.cells}</b></span>
              <Stat label="volume">{block.ratio.toFixed(6)}</Stat>
              <Stat label="exposed">{block.exposed}</Stat>
            </div>
          </div>
        ))}
      </div>
      <Note error={tower.error ?? word.error} />
      <Stats>
        {name && <Stat label="name">{name}</Stat>}
        <Stat label="letters">{census.length}</Stat>
        <Stat label="blocks">{depth} of {cap}</Stat>
        <Stat label="word side">{census.side}</Stat>
        <Stat label="word cells">{census.cells}</Stat>
        <Stat label="word fill">{census.fill}</Stat>
        <Stat label="density">{census.ratio?.toFixed(6)}</Stat>
        <Stat label="dimension">{census.dimension?.toFixed(6)}</Stat>
        {dim !== 6 && <Stat label="d - 1">{DROP[dim]}</Stat>}
      </Stats>
      <Stats>
        {letters.map((letter, i) => (
          <span key={i} className="badge">{i + 1} <b>{letter.name}</b> side {letter.number} fill {letter.fill} / {letter.cells} dim {letter.dimension.toFixed(4)}</span>
        ))}
      </Stats>
      <Stats>
        {word.error ? null : <span className={`chip ${shrinks ? 'proved' : 'refuted'}`}>{shrinks ? 'a letter buys a fraction under one, so the volume converges' : 'every letter fills its cell, so the volume grows without bound'}</span>}
        {word.error || dim === 6 ? null : <span className={`chip ${climbs ? 'proved' : 'conjecture'}`}>{climbs ? 'a letter under one and dimension over d - 1, so the surface density diverges' : shrinks ? 'dimension at or under d - 1, so the surface density stays bounded' : 'every letter full, so the surface density stays constant'}</span>}
        {dim === 6 && <span>the hexagon is the shadow of the cube tower: its volume is the inked share of the mesh and its surface the boundary edges of that ink over the mesh width</span>}
        {dim === 6 && q.proj === 'iso' && <span className="chip conjecture">the isometric skin is the visible surface, so it inks every triangle it draws and the volume sits at one; take the middle slice or the facing sides to watch it fall</span>}
      </Stats>
      <Sketch draw={drawChart} deps={[tower, dim]} className="bars" role="img" aria-label="Volume and surface running totals by block" />
      <Stats>
        <span><span className="swatch" style={{ background: ink.dim }}></span> volume per block, the fill fraction the census gives</span>
        <span><span className="swatch" style={{ background: ink.gold }}></span> the volume running total, flattening</span>
        <span><span className="swatch" style={{ background: ink.blue }}></span> the surface running total, climbing</span>
      </Stats>
      <p className="sub">The bars are exported numbers, one fill fraction per block. The two curves are the running totals of those bars and of the exposed count over the side, each drawn against its own last value so the shapes can be read side by side: the volume bends over as its steps shrink by a constant factor, the surface bends up as its steps grow. Neither total is printed anywhere on the page, because a total is a sum and the sum would have to happen here rather than in Rust.</p>
    </Page>
  );
}

mount(<App />);
