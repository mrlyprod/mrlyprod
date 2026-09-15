import { useEffect, useRef, useState } from 'react';
import { ready } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Text, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Pixels } from '../../lib/draw.jsx';
import { useQuery, stamp } from '../../lib/query.js';

const m = await ready();

const SIZES = [256, 512, 1024];
const SCHEDULES = [
  ['unspun', 'unspun'],
  ['degrees', 'layer k at k increments'],
  ['golden', 'golden angle'],
  ['primes', 'layer k at the k-th prime'],
  ['random', 'random, seeded'],
  ['gaussian', 'the Gaussian angle of n'],
];
const SETS = ['odd', 'primes', 'squarefree', 'prime powers'];
const WEIGHTS = ['plain', 'mobius', 'harmonic'];
const MODES = ['cells', 'edges', 'corners'];
const BLENDS = ['mean', 'sum', 'union', 'meet', 'parity', 'difference'];
const EYES = JSON.parse(m.tourbillon_eyes(12));
const STEP = 0.05;
const SWEEP = 6;
const BEAT = 1000;
const TIGHT = 1e-9;

const place = (value, places) => (Number.isFinite(value) ? value.toFixed(places) : '-');

const trim = (value) => String(Number(value.toFixed(6)));

const eyeAt = (value) => EYES.findIndex(([angle]) => Math.abs(angle - value) < TIGHT);

const label = ([angle, numer, denom]) => (denom === 1 ? trim(angle) : `${trim(angle)} = ${numer}/${denom}`);

function App() {
  const [q, setQ] = useQuery({
    n: 55, r: 512, sched: 'degrees', inc: 1, set: 'odd',
    weights: 'plain', mode: 'cells', blend: 'mean', seed: 1,
  });
  const [inc, setInc] = useState(q.inc);
  const [typed, setTyped] = useState(trim(q.inc));
  const [playing, setPlaying] = useState(false);
  const live = useRef(inc);
  const held = useRef(null);
  live.current = inc;

  let error = null;
  try {
    const clock = performance.now();
    const field = m.tourbillon(q.n, q.r, q.sched, inc, q.set, q.weights, q.mode, q.blend, q.seed);
    const read = JSON.parse(m.tourbillon_stats(field, q.r, q.n, q.sched, inc, q.set, q.weights, q.blend, q.seed));
    const signed = read.weighted && q.weights === 'mobius';
    const reach = Math.max(Math.abs(read.low), Math.abs(read.high));
    const pixels = m.paint_span(field, q.r, signed ? -reach : read.low, signed ? reach : read.high,
      signed ? 'diverge' : 'fire', read.layers + 1, false);
    held.current = { pixels, read, ms: performance.now() - clock };
  } catch (fault) {
    error = fault;
  }

  useEffect(() => {
    if (!playing) return;
    let id = 0;
    let last = 0;
    const frame = (now) => {
      id = requestAnimationFrame(frame);
      const step = last ? Math.min(0.25, (now - last) / 1000) : 0;
      last = now;
      if (step > 0) setInc((old) => (old + SWEEP * step) % 360);
    };
    id = requestAnimationFrame(frame);
    const beat = setInterval(() => stamp({ inc: place(live.current, 2) }), BEAT);
    return () => {
      cancelAnimationFrame(id);
      clearInterval(beat);
    };
  }, [playing]);

  const settle = (next) => {
    const value = Math.min(360, Math.max(0, Number(next.toFixed(6))));
    setPlaying(false);
    setInc(value);
    setTyped(trim(value));
    setQ({ inc: value });
  };

  const write = (text) => {
    setPlaying(false);
    setTyped(text);
    const value = Number(text);
    if (!text.trim() || !Number.isFinite(value) || value < 0 || value > 360) return;
    setInc(value);
    setQ({ inc: value });
  };

  const sweep = () => {
    if (playing) {
      settle(live.current);
      return;
    }
    setQ({ sched: 'degrees' });
    setPlaying(true);
  };

  const view = held.current;
  const read = view?.read;
  const angles = read ? read.angles.map((value) => place(value, 1)).join(' ') : '';
  const peaks = read
    ? read.peaks.map(([x, y, value]) => `  ${place(x, 4)} ${place(y, 4)} ${place(value, 4)}`).join('\n')
    : '';
  const aside = read && !read.weighted && q.weights !== 'plain' ? '   weights unused' : '';
  const eye = eyeAt(inc);

  const controls = (
    <>
      <section>
        <h3>The stack</h3>
        <Row>
          <Slider label="scales up to" value={q.n} min={3} max={99} step={2} onChange={(v) => setQ({ n: v })} />
          <Pick label="layers" value={q.set} options={SETS} onChange={(v) => setQ({ set: v })} />
          <Pick label="weights" value={q.weights} options={WEIGHTS} onChange={(v) => setQ({ weights: v })} />
          <Pick label="size" value={q.r} options={SIZES.map((v) => [v, v])} onChange={(v) => setQ({ r: +v })} />
        </Row>
      </section>
      <section>
        <h3>The spin</h3>
        <Row>
          <Pick label="schedule" value={q.sched} options={SCHEDULES} onChange={(v) => setQ({ sched: v })} />
          <Slider label="increment" value={inc} min={0} max={360} step={0.5} show={`${place(inc, 2)}°`}
            onChange={settle} />
          <Text label="degrees" value={playing ? place(inc, 2) : typed} onChange={write} />
          <Btn onClick={() => settle(inc - STEP)}>-</Btn>
          <Btn onClick={() => settle(inc + STEP)}>+</Btn>
          <Btn primary on={playing} onClick={sweep}>{playing ? 'Stop' : 'Play the increment'}</Btn>
          <Btn onClick={() => setQ({ sched: 'random', seed: q.seed + 1 })}>Randomize</Btn>
        </Row>
      </section>
      <section>
        <h3>The eyes</h3>
        <Row>
          <Pick label="quarter turn" value={eye < 0 ? '' : String(eye)}
            options={[['', 'off the lattice'], ...EYES.map((row, at) => [String(at), label(row)])]}
            onChange={(v) => { if (v !== '') { setQ({ sched: 'degrees' }); settle(EYES[+v][0]); } }} />
        </Row>
      </section>
      <section>
        <h3>The draw</h3>
        <Row>
          <Pick label="mode" value={q.mode} options={MODES} onChange={(v) => setQ({ mode: v })} />
          <Pick label="blend" value={q.blend} options={BLENDS} onChange={(v) => setQ({ blend: v })} />
        </Row>
      </section>
    </>
  );

  return (
    <Page crumb="tourbillon" title="The tourbillon"
      sub="The odd parity carpet at the scales 1, 3, 5 and on, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer. Turn the layers as far as you like: the centre is the one point every rotation fixes, so its value never moves. The carpet is unchanged by a quarter turn, so at an increment of 90 a over q the layers fall into q angle classes and the spin goes dead."
      controls={controls}
      foot={<>The layer schedule, the selection, the weights, the quarter-turn lattice and the angle classes are all computed in Rust, the disc is masked and the layers merged there, and the page only paints the pixels it is handed. The unspun stack is the one the <a href="../moire">moire</a> page shows; spinning it is what breaks the shared grid, and the exact nodes that survive a turn are the Gaussian-integer angles. What the stack is, and why an address is not a construction, is in <a href="/research/stack/">the stack note</a>.</>}>
      {view && <Pixels data={view.pixels} style={{ maxWidth: 640 }} role="img" aria-label="The spun carpet stack inside its disc" />}
      <Stats>
        <Stat label="layers">{read?.layers}</Stat>
        <Stat label="pixels">{read && `${q.r} by ${q.r}`}</Stat>
        <Stat label="draw">{view && `${view.ms.toFixed(0)} ms`}</Stat>
      </Stats>
      {read && <pre>{`layers ${read.layers}   scales ${read.scales.join(' ')}\nangles ${angles}\nmean ${place(read.mean, 4)}   rms contrast ${place(read.rms, 4)}   rms * sqrt(L) ${place(read.faded, 4)}   centre ${place(read.centre, 4)}${aside}\nangle classes mod 90 ${read.classes}   increments to a quarter turn ${read.period ?? 'none'}   layer pairs sharing a class ${read.pairs}\ntop three maxima (x, y, value)\n${peaks}`}</pre>}
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
