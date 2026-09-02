import { useEffect, useMemo, useReducer, useRef, useState } from 'react';
import { ready, ink, fit } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Text, Check, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid, Sketch } from '../../lib/draw.jsx';
import { Picker, useSeeds } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const FLAT = 96;
const LINE = 320;
const ROWS = 200;
const LIMIT = 512;
const TRACE = 320;

const FIRST = { dim: 2, code: '7', side: 3, level: 1, birth: '3', survive: '23', wrap: true, seed: 'soup', density: 0.3, bseq: '', sseq: '' };

const NAMES = [...m.life_sequences()];

const SEEDS = [['soup', 'soup'], ['single', 'one cell'], ['blank', 'blank']];

const PRESETS = [
  ['Conway', { dim: 2, code: '7', side: 3, level: 1, birth: '3', survive: '23', bseq: '', sseq: '' }],
  ['Cantor-Life', { dim: 1, code: '1', side: 3, level: 3, birth: '3', survive: '23', bseq: '', sseq: '' }],
  ['Menger row', { dim: 1, code: '3', side: 3, level: 2, birth: '3', survive: '23', bseq: '', sseq: '' }],
  ['nine-cell XOR', { dim: 2, code: '15', side: 3, level: 1, birth: '1357', survive: '02468', bseq: '', sseq: '' }],
  ['rule 150', { dim: 1, code: '1', side: 3, level: 1, birth: '1', survive: '02', bseq: '', sseq: '' }],
];

const attempt = (fn) => {
  try {
    return { read: fn(), error: null };
  } catch (error) {
    return { read: null, error };
  }
};

function parse(text) {
  const clean = String(text).trim();
  const parts = /[^0-9]/.test(clean) ? clean.split(/[^0-9]+/) : [...clean];
  const seen = new Set();
  for (const part of parts) if (part !== '') seen.add(+part);
  return Uint32Array.from([...seen].sort((a, b) => a - b));
}

function spell(counts, sequence) {
  if (sequence) return sequence;
  if (counts.some((c) => c > 9)) return null;
  return counts.join('');
}

const alive = (types) => types.reduce((a, b) => a + b, 0);

function ruleName(birth, survive, bseq, sseq, wrap) {
  const b = spell(birth, bseq);
  const s = spell(survive, sseq);
  if (b === null || s === null) return null;
  return `mrly_rule_b${b}_s${s}${wrap ? '_w' : ''}`;
}

// THE WORLD

function sow(dim, kind, density, tap) {
  const width = dim === 2 ? FLAT : LINE;
  const height = dim === 2 ? FLAT : 1;
  if (kind === 'soup') return m.life_noise(width, height, density, tap || 1);
  const types = new Uint8Array(width * height);
  if (kind === 'single') types[dim === 2 ? (height >> 1) * width + (width >> 1) : width >> 1] = 1;
  return types;
}

function born(dim, kind, density, tap) {
  const types = sow(dim, kind, density, tap);
  const sheet = dim === 2 ? null : new Uint8Array(LINE * ROWS);
  if (sheet) sheet.set(types, 0);
  return { types, sheet, used: 1, pops: [alive(types)] };
}

function advance(world, dim, next) {
  world.types = next;
  if (dim === 1) {
    if (world.used < ROWS) {
      world.sheet.set(next, world.used * LINE);
      world.used += 1;
    } else {
      world.sheet.copyWithin(0, LINE);
      world.sheet.set(next, (ROWS - 1) * LINE);
    }
  }
  world.pops.push(alive(next));
  if (world.pops.length > TRACE) world.pops.shift();
}

// THE TRACE

const trace = (pops) => (canvas) => {
  const [ctx, w, h] = fit(canvas, 120);
  ctx.clearRect(0, 0, w, h);
  const top = Math.max(1, ...pops);
  ctx.strokeStyle = ink.blue;
  ctx.lineWidth = 1.5;
  ctx.beginPath();
  pops.forEach((p, i) => {
    const x = 8 + (pops.length < 2 ? 0 : (i / (pops.length - 1)) * (w - 16));
    const y = h - 8 - (p / top) * (h - 20);
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.stroke();
  ctx.fillStyle = ink.dim;
  ctx.font = '11px ui-monospace, monospace';
  ctx.fillText(String(top), 8, 12);
};

function App() {
  const taps = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [playing, setPlaying] = useState(false);
  const [age, setAge] = useState(0);
  const [verdict, setVerdict] = useState(null);
  const [error, setError] = useState(null);
  const [, force] = useReducer((x) => x + 1, 0);

  const dim = pick.dim === 1 ? 1 : 2;
  const cap = Math.min(3, m.level_cap(pick.side, 1, dim === 2 ? 27 : 81));
  const level = Math.max(1, Math.min(pick.level, cap));

  const mask = useMemo(() => attempt(() => m.life_mask(dim, pick.code.trim(), pick.side, level)), [dim, pick.code, pick.side, level]);
  const index = useMemo(() => (mask.read ? attempt(() => m.life_mask_index(mask.read.types, mask.read.width, mask.read.height)) : { read: null, error: null }), [mask.read]);
  const budget = mask.read ? alive(mask.read.types) : 8;

  const world = useRef(null);
  world.current ??= born(dim, pick.seed, pick.density, taps.get());

  const birth = parse(pick.birth);
  const survive = parse(pick.survive);
  const name = ruleName(birth, survive, pick.bseq, pick.sseq, pick.wrap);

  const reseed = (patch = {}) => {
    const next = { ...pick, ...patch };
    set(patch);
    setPlaying(false);
    setAge(0);
    setVerdict(null);
    setError(null);
    world.current = born(next.dim === 1 ? 1 : 2, next.seed, next.density, taps.get());
    force();
  };

  const step = () => {
    if (!mask.read) return;
    try {
      const width = dim === 2 ? FLAT : LINE;
      const height = dim === 2 ? FLAT : 1;
      const next = m.life_next_masked(world.current.types, width, height, birth, survive, mask.read.types, mask.read.width, mask.read.height, pick.wrap);
      advance(world.current, dim, next);
      setAge((g) => g + 1);
      setError(null);
      force();
    } catch (thrown) {
      setPlaying(false);
      setError(thrown);
    }
  };

  useEffect(() => {
    if (!playing) return;
    const timer = setInterval(step, 60);
    return () => clearInterval(timer);
  }, [playing, dim, pick.code, pick.side, level, pick.birth, pick.survive, pick.wrap, mask.read]);

  const fate = () => {
    if (!mask.read) return;
    try {
      const width = dim === 2 ? FLAT : LINE;
      const height = dim === 2 ? FLAT : 1;
      const run = JSON.parse(m.life_run_masked(world.current.types, width, height, birth, survive, mask.read.types, mask.read.width, mask.read.height, pick.wrap, LIMIT));
      setVerdict(run);
      setError(null);
    } catch (thrown) {
      setError(thrown);
    }
  };

  const toggle = (event) => {
    const box = event.target.getBoundingClientRect();
    const width = dim === 2 ? FLAT : LINE;
    const x = Math.floor((event.clientX - box.left) / box.width * width);
    if (dim === 2) {
      const y = Math.floor((event.clientY - box.top) / box.height * FLAT);
      world.current.types[y * FLAT + x] ^= 1;
    } else {
      world.current.types[x] ^= 1;
      world.current.sheet.set(world.current.types, (world.current.used - 1) * LINE);
    }
    setVerdict(null);
    force();
  };

  const sequenced = (which, key, sequence) => {
    const patch = { [key]: sequence };
    if (sequence) patch[which] = [...m.life_sequence(sequence, budget)].join(' ');
    set(patch);
  };

  const preset = (values) => {
    const patch = { ...values, level: values.level ?? 1 };
    reseed(patch);
  };

  const wears = (values) => Object.entries(values).every(([key, value]) => (key === 'dim' ? dim : key === 'level' ? level : pick[key]) === value);

  const controls = (
    <>
      <Group name="Run">
        <Btn primary onClick={() => setPlaying(!playing)}>{playing ? 'Pause' : 'Play'}</Btn>
        <Btn onClick={step}>Step</Btn>
        <Btn onClick={fate}>Run to fate</Btn>
        <Btn onClick={() => reseed()}>Reset</Btn>
        <Check label="wrap" checked={pick.wrap} onChange={(v) => set({ wrap: v })} />
      </Group>
      <Group name="The mask">
        <Pick label="dimension" value={dim} options={[[1, '1'], [2, '2']]} onChange={(v) => reseed({ dim: +v, code: +v === 1 ? '1' : '7' })} />
        <Picker dimension={dim} code={pick.code} seeds={taps} onChange={(patch) => set(patch)} />
        <Pick label="side" value={pick.side} options={[[3, 3], [5, 5], [7, 7], [9, 9]]} onChange={(v) => set({ side: +v, level: Math.min(level, Math.min(3, m.level_cap(+v, 1, dim === 2 ? 27 : 81))) })} />
        <Pick label="level" value={level} options={Array.from({ length: cap }, (_, i) => [i + 1, i + 1])} onChange={(v) => set({ level: +v })} />
      </Group>
      <Group name="The rule">
        <Pick label="birth from" value={pick.bseq} options={[['', 'by hand'], ...NAMES]} onChange={(v) => sequenced('birth', 'bseq', v)} />
        <Text label="birth counts" value={pick.birth} onChange={(v) => set({ birth: v, bseq: '' })} />
        <Pick label="survive from" value={pick.sseq} options={[['', 'by hand'], ...NAMES]} onChange={(v) => sequenced('survive', 'sseq', v)} />
        <Text label="survive counts" value={pick.survive} onChange={(v) => set({ survive: v, sseq: '' })} />
      </Group>
      <Group name="The seed">
        <Pick label="seed" value={pick.seed} options={SEEDS} onChange={(v) => reseed({ seed: v })} />
        <Slider label="density" value={pick.density} min={0.05} max={0.95} step={0.01} onChange={(v) => reseed({ seed: 'soup', density: v })} />
        <Btn onClick={() => { taps.next(); reseed({ seed: 'soup' }); }}>Randomize</Btn>
      </Group>
      <Group name="Presets">
        {PRESETS.map(([label, values]) => <Btn key={label} on={wears(values)} onClick={() => preset(values)}>{label}</Btn>)}
      </Group>
    </>
  );

  const sheet = dim === 2
    ? { width: FLAT, height: FLAT, types: world.current.types }
    : { width: LINE, height: world.current.used, types: world.current.sheet.subarray(0, world.current.used * LINE) };

  const reading = index.read === null ? 'unread' : index.read === 0 ? 'index 0: the offsets span no lattice' : index.read === 1 ? 'index 1: one lattice, nothing splits' : `index ${index.read}: ${index.read} interleaved copies`;

  return (
    <Page crumb="mrlylife" title="mrlylife"
      sub="Life is one point of a family: pick the neighbourhood as a design rather than a ring, pick the birth and survival counts by hand or from a named sequence, and run it in one dimension or two. The mask is the object; the rule reads only how many of its cells are alive."
      foot={<>The kind is LIFE, outer-totalistic: a dead cell is born when its live neighbour count is in the birth list, a live cell stays when its count is in the survival list, and the mask says which cells are neighbours. Conway is the level-1 carpet `mrly_bang_d2_7` with its centre popped, drawn plain on <a href="../life">the Life page</a>; the one-dimensional two-state radius-one masks are the elementary rules on <a href="../wolfram">the Wolfram page</a>. Menger-Life proper lives at `D = 3` on the 20 offsets of `mrly_bang_d3_23` and is not drawn here, so the Menger chip reads that tile one dimension down. The masks, the indices and every generation come out of the crates through wasm. The research page is <a href="/research/automata/">automata</a>.</>}
      controls={controls}>

      <div className="arena">
        <div className="panel">
          <h2>the mask <span>a design, centre popped</span></h2>
          <Note error={mask.error} />
          {mask.read && <Grid grid={mask.read} on={ink.gold} style={{ maxWidth: 200 }} aria-label="The neighbourhood mask" />}
          <Stats>
            <Stat label="mask">{`${dim === 1 ? 'd1' : 'd2'} code ${pick.code.trim()} side ${pick.side} level ${level}`}</Stat>
            <Stat label="cells">{budget}</Stat>
            <Stat label="lattice">{reading}</Stat>
          </Stats>
          <p className="sub">The index is the decoupling index: the sublattice the mask offsets and the centre generate inside the whole lattice. An index above one means the board never mixes - it is that many interleaved copies of the same automaton, each blind to the others.</p>
        </div>
        <div className="panel">
          <h2>the rule <span>kind LIFE on this mask</span></h2>
          <Stats>
            <Stat label="name">{name ?? `B${[...birth].join(',')}/S${[...survive].join(',')}`}</Stat>
            <Stat label="birth">{[...birth].join(', ') || 'none'}</Stat>
            <Stat label="survive">{[...survive].join(', ') || 'none'}</Stat>
            {name === null && <span className="chip conjecture">above 9 the digits run out, so this rule has no name</span>}
          </Stats>
          <p className="sub">Counts run from 0 to {budget}, the cell count of the mask. Type them as digits, as `1 3 5 7`, or draw them from a named sequence cut at the budget; a sequence side spells itself in the name, so `mrly_rule_bprimes_s23` is a rule and not a description of one.</p>
        </div>
      </div>

      <div className="panel">
        <h2>{dim === 2 ? 'the board' : 'the space-time diagram'} <span>{dim === 2 ? `${FLAT} by ${FLAT}` : `${LINE} cells, newest row at the bottom`}</span></h2>
        <Grid grid={sheet} on={ink.green} style={{ maxWidth: 720 }} onClick={toggle} aria-label={dim === 2 ? 'The board, click a cell to toggle it' : 'The space-time diagram, click the newest row to toggle a cell'} />
        <Stats>
          <Stat label="generation">{age}</Stat>
          <Stat label="population">{world.current.pops[world.current.pops.length - 1]}</Stat>
          {verdict && <Stat label="fate">{verdict.fate} after {verdict.count}{verdict.loop ? `, loop ${verdict.loop}` : ''}</Stat>}
        </Stats>
        <Note error={error} />
      </div>

      <div className="panel">
        <h2>the population <span>the last {TRACE} generations</span></h2>
        <Sketch draw={trace(world.current.pops)} deps={[age, dim]} className="bars" aria-label="The population against generation" />
        <p className="sub">Run to fate replays the board for at most {LIMIT} generations and reports death, a frozen board, a loop with its period, or a timeout. Every value on this page is a link: the dimension, the code, the side, the level, the two count lists, the wrap and the seed all live in the address bar.</p>
      </div>
    </Page>
  );
}

mount(<App />);
