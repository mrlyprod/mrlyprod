import { useEffect, useReducer, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Text, Check, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Grid } from '../lib/draw.jsx';
import { useSeeds } from '../lib/select.jsx';

const m = await ready();
const W = 96, H = 96, BUDGET = 8;
const SEEDS = [['noise', 'noise'], ['7', 'carpet'], ['14', 'net'], ['9', 'void'], ['blank', 'blank']];
const NAMES = [...m.life_sequences()];

const parse = (text) => Uint32Array.from(text.replace(/\D/g, ''), (d) => +d);

function App() {
  const s = useSeeds();
  const [pick, setPick] = useState({ seed: 'noise', density: 0.35, birth: '3', survive: '23', wrap: true, birthSeq: '', surviveSeq: '' });
  const [running, setRunning] = useState(false);
  const [generation, setGeneration] = useState(0);
  const [verdict, setVerdict] = useState(null);
  const [error, setError] = useState(null);
  const [, force] = useReducer((x) => x + 1, 0);
  const grid = useRef(null);
  grid.current ??= m.life_noise(W, H, pick.density, s.get() || 1);

  const rule = () => [parse(pick.birth), parse(pick.survive), pick.wrap];

  const reseed = (patch = {}) => {
    const next = { ...pick, ...patch };
    setPick(next);
    setGeneration(0);
    setVerdict(null);
    if (next.seed === 'noise') {
      grid.current = m.life_noise(W, H, next.density, s.get() || 1);
    } else if (next.seed === 'blank') {
      grid.current = new Uint8Array(W * H);
    } else {
      const cell = m.two_grid(next.seed, 3, 3, 0, 2);
      grid.current = new Uint8Array(W * H);
      const off = Math.floor((W - cell.width) / 2);
      for (let y = 0; y < cell.height; y++) {
        grid.current.set(cell.types.subarray(y * cell.width, (y + 1) * cell.width), (y + off) * W + off);
      }
    }
  };

  const step = () => {
    try {
      grid.current = m.life_next(grid.current, W, H, ...rule());
      setGeneration((g) => g + 1);
      setError(null);
    } catch (error) {
      setRunning(false);
      setError(error);
    }
  };

  useEffect(() => {
    if (!running) return;
    const timer = setInterval(step, 40);
    return () => clearInterval(timer);
  }, [running, pick.birth, pick.survive, pick.wrap]);

  const fate = () => {
    try {
      const run = JSON.parse(m.life_run(grid.current, W, H, ...rule(), 512));
      setVerdict(<span>fate <b>{run.fate}</b> after <b>{run.count}</b> generations{run.loop ? <span> in a loop of <b>{run.loop}</b></span> : null}</span>);
    } catch (error) {
      setError(error);
    }
  };

  const toggle = (event) => {
    const box = event.target.getBoundingClientRect();
    const x = Math.floor((event.clientX - box.left) / box.width * W), y = Math.floor((event.clientY - box.top) / box.height * H);
    grid.current[y * W + x] ^= 1;
    force();
  };

  const sequenced = (side, name) => {
    const patch = { [`${side}Seq`]: name };
    if (name) patch[side] = Array.from(m.life_sequence(name, BUDGET)).join('');
    setPick({ ...pick, ...patch });
  };

  return (
    <Page crumb="life" title="Life, ruled by sequences"
      sub="A cell is born or survives when its neighbor count sits in a list, and the lists can be spelled by hand or drawn from a named sequence: the primes, the Fibonacci numbers, the fills of a carpet. Seed a grid, pick the rule, and run it to its fate."
      foot="The Moore neighborhood holds eight cells, so a sequence is cut at eight: primes give 2, 3, 5, 7 and Fibonacci gives 1, 2, 3, 5, 8. Every generation is stepped in Rust, and the run to fate replays the grid until it dies, freezes, loops or hits 512 generations.">
      <Row>
        <Pick label="seed" value={pick.seed} options={SEEDS} onChange={(v) => reseed({ seed: v })} />
        <Slider label="density" value={pick.density} min={0.05} max={0.95} step={0.01} onChange={(v) => reseed({ density: v })} />
        <Pick label="birth" value={pick.birthSeq} options={[['', 'by hand'], ...NAMES]} onChange={(v) => sequenced('birth', v)} />
        <Text label="" value={pick.birth} onChange={(v) => setPick({ ...pick, birth: v, birthSeq: '' })} />
        <Pick label="survive" value={pick.surviveSeq} options={[['', 'by hand'], ...NAMES]} onChange={(v) => sequenced('survive', v)} />
        <Text label="" value={pick.survive} onChange={(v) => setPick({ ...pick, survive: v, surviveSeq: '' })} />
        <Check label="wrap" checked={pick.wrap} onChange={(v) => setPick({ ...pick, wrap: v })} />
      </Row>
      <Row>
        <Btn primary onClick={() => setRunning(!running)}>{running ? 'Pause' : 'Play'}</Btn>
        <Btn onClick={step}>Step</Btn>
        <Btn onClick={fate}>Run to fate</Btn>
        <Btn onClick={() => { s.next(); reseed({ seed: 'noise' }); }}>Randomize</Btn>
        <span className="num dim">generation {generation}</span>
      </Row>
      <Grid grid={{ width: W, height: H, types: grid.current }} on={ink.green} style={{ maxWidth: 640 }} onClick={toggle} />
      <Stats>
        <Stat label="rule">{`B${Array.from(parse(pick.birth)).join('')}/S${Array.from(parse(pick.survive)).join('')}`}</Stat>
        {verdict}
      </Stats>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
