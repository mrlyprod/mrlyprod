import { useEffect, useReducer, useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Check, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid } from '../../lib/draw.jsx';
import { useSeeds } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const W = 96;
const H = 96;
const LIMIT = 512;
const BIRTH = Uint32Array.from([3]);
const SURVIVE = Uint32Array.from([2, 3]);

const FIRST = { seed: 'soup', density: 0.3, wrap: true };

const SEEDS = [['soup', 'soup'], ['glider', 'glider'], ['pentomino', 'R-pentomino'], ['blinker', 'blinker'], ['block', 'block'], ['blank', 'blank']];

const SHAPES = {
  glider: [[1, 0], [2, 1], [0, 2], [1, 2], [2, 2]],
  pentomino: [[1, 0], [2, 0], [0, 1], [1, 1], [1, 2]],
  blinker: [[0, 0], [1, 0], [2, 0]],
  block: [[0, 0], [1, 0], [0, 1], [1, 1]],
};

function sow(kind, density, tap) {
  if (kind === 'soup') return m.life_noise(W, H, density, tap || 1);
  const types = new Uint8Array(W * H);
  const cells = SHAPES[kind];
  if (!cells) return types;
  const ox = (W >> 1) - 1;
  const oy = (H >> 1) - 1;
  for (const [x, y] of cells) types[(oy + y) * W + ox + x] = 1;
  return types;
}

const alive = (types) => types.reduce((a, b) => a + b, 0);

function App() {
  const taps = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [playing, setPlaying] = useState(false);
  const [age, setAge] = useState(0);
  const [verdict, setVerdict] = useState(null);
  const [error, setError] = useState(null);
  const [, force] = useReducer((x) => x + 1, 0);
  const board = useRef(null);
  board.current ??= sow(pick.seed, pick.density, taps.get());

  const reseed = (patch = {}) => {
    const next = { ...pick, ...patch };
    set(patch);
    setPlaying(false);
    setAge(0);
    setVerdict(null);
    setError(null);
    board.current = sow(next.seed, next.density, taps.get());
    force();
  };

  const step = () => {
    try {
      board.current = m.life_next(board.current, W, H, BIRTH, SURVIVE, pick.wrap);
      setAge((g) => g + 1);
      setError(null);
    } catch (thrown) {
      setPlaying(false);
      setError(thrown);
    }
  };

  useEffect(() => {
    if (!playing) return;
    const timer = setInterval(step, 40);
    return () => clearInterval(timer);
  }, [playing, pick.wrap]);

  const fate = () => {
    try {
      const run = JSON.parse(m.life_run(board.current, W, H, BIRTH, SURVIVE, pick.wrap, LIMIT));
      setVerdict(run);
      setError(null);
    } catch (thrown) {
      setError(thrown);
    }
  };

  const toggle = (event) => {
    const box = event.target.getBoundingClientRect();
    const x = Math.floor((event.clientX - box.left) / box.width * W);
    const y = Math.floor((event.clientY - box.top) / box.height * H);
    board.current[y * W + x] ^= 1;
    setVerdict(null);
    force();
  };

  const controls = (
    <>
      <Group name="Run">
        <Btn primary onClick={() => setPlaying(!playing)}>{playing ? 'Pause' : 'Play'}</Btn>
        <Btn onClick={step}>Step</Btn>
        <Btn onClick={fate}>Run to fate</Btn>
        <Btn onClick={() => reseed()}>Reset</Btn>
        <Check label="wrap" checked={pick.wrap} onChange={(v) => set({ wrap: v })} />
      </Group>
      <Group name="The seed">
        <Pick label="seed" value={pick.seed} options={SEEDS} onChange={(v) => reseed({ seed: v })} />
        <Slider label="density" value={pick.density} min={0.05} max={0.95} step={0.01} onChange={(v) => reseed({ seed: 'soup', density: v })} />
        <Btn onClick={() => { taps.next(); reseed({ seed: 'soup' }); }}>Randomize</Btn>
      </Group>
    </>
  );

  return (
    <Page crumb="life" title="Conway's Life"
      sub="One rule on the eight cells around: a dead cell with exactly three live neighbours is born, a live cell with two or three stays, everything else dies. Drop a soup or a glider, then play it, step it, or run it to its fate."
      foot={<>The neighbourhood is not a hand-drawn ring: it is the side-3 carpet tile `mrly_bang_d2_7` with its centre popped, the same eight offsets a design writes at level one. Every generation and every fate below is stepped in Rust through wasm and the page only draws. Life is one point of a much larger family - any mask, any birth and survival list, one or two dimensions - and that family is <a href="../mrlylife">mrlylife</a>.</>}
      controls={controls}>
      <Grid grid={{ width: W, height: H, types: board.current }} on={ink.green} style={{ maxWidth: 640 }} onClick={toggle} aria-label="The grid, click a cell to turn it on or off" />
      <Stats>
        <Stat label="rule">{`mrly_rule_b3_s23${pick.wrap ? '_w' : ''}`}</Stat>
        <Stat label="generation">{age}</Stat>
        <Stat label="population">{alive(board.current)}</Stat>
        {verdict && <Stat label="fate">{verdict.fate} after {verdict.count}{verdict.loop ? `, loop ${verdict.loop}` : ''}</Stat>}
      </Stats>
      <p className="sub">Click any cell to turn it on or off, then play from there. Run to fate replays the board from here for at most {LIMIT} generations and reports whether it dies, freezes, loops, or is still moving when the count runs out. The board is {W} by {H}; with wrap on it is a torus, with wrap off the outside is dead ground.</p>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
