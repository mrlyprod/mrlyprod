import { useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note } from '../../lib/app.jsx';
import { Pixels, Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, roll, Ramp, Sources, readSource, seedSource, SOURCE_FIRST } from '../../lib/select.jsx';
import { board, bars, axis, tag } from '../../lib/chart.js';

const m = await ready();
const OUT = 512;
const ORDERS = 48;
const RINGS = 160;
const BLENDS = ['mean', 'sum', 'union', 'meet', 'parity', 'difference'];
const FIRST = { ...SOURCE_FIRST, copies: 6, step: 60, blend: 'mean' };

const drawn = (seed) => {
  const [copies, at] = roll(seed, [[1, 36], [0, BLENDS.length - 1]]);
  let k = at;
  while (BLENDS[k] === 'meet') k = (k + 1) % BLENDS.length;
  return { copies, blend: BLENDS[k] };
};

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ ...seedSource(s, FIRST), ...(s.get() ? drawn(s.get()) : null) });
  const [full, setFull] = useState(() => !new URLSearchParams(location.search).has('step'));
  const [samples, setSamples] = useState(2);
  const [look, setLook] = useState({ ramp: 'fire', levels: 64, invert: false });
  const shown = useRef(null);

  const step = full ? +m.full_turn(pick.copies).toFixed(3) : pick.step;

  let error = null;
  try {
    const src = readSource(pick);
    const stack = m.radial(src.field, src.size, OUT, pick.copies, step, pick.blend, samples);
    const pixels = m.sheet(stack, OUT, look.ramp, look.levels, look.invert);
    const power = m.harmonics(src.field, src.size, RINGS, ORDERS);
    const order = m.turns(power);
    shown.current = {
      pixels, name: src.name, side: src.size, order, copies: pick.copies, full, blend: pick.blend,
      step, power: Array.from(power), share: m.radial_share(power),
      petals: full ? (order ? m.petals(pick.copies, order) : 'none') : 'partial orbit',
    };
  } catch (fault) {
    error = fault;
  }

  const chart = (canvas) => {
    const view = shown.current;
    if (!view) return;
    const b = board(canvas, 300);
    const lives = (k) => (view.full ? k % view.copies === 0 : k === 0);
    bars(b, view.power, { color: (k) => (k === 0 ? ink.gold : lives(k) ? ink.blue : ink.line) });
    axis(b, view.power.map((_, k) => [(k + 0.5) / view.power.length, k]).filter(([, k]) => k % 4 === 0));
    tag(b, `order 0 carries ${view.share.toFixed(1)}% of the power`, ink.gold);
    tag(b, view.order ? `live orders are multiples of ${view.order}` : 'no live order', ink.pink, 'right');
  };

  const view = shown.current;

  const controls = (
    <>
      <section>
        <h3>Source</h3>
        <Row>
          <Sources value={pick} onChange={set} seeds={s} onSeed={(seed) => { set(drawn(seed)); setFull(true); }} />
        </Row>
      </section>
      <section>
        <h3>Stack</h3>
        <Row>
          <Slider label="copies" value={pick.copies} min={1} max={36} onChange={(v) => set({ copies: v })} />
          <Check label="full turn" checked={full} onChange={(v) => { setFull(v); if (!v) set({ step }); }} />
          <label>step <input type="number" min={0} max={360} step={0.5} value={step} disabled={full} onChange={(e) => set({ step: +e.target.value })} /> degrees</label>
          <Pick label="blend" value={pick.blend} options={BLENDS} onChange={(v) => set({ blend: v })} />
          <Pick label="samples" value={samples} options={[[1, 1], [2, 2], [3, 3]]} onChange={(v) => setSamples(+v)} />
        </Row>
      </section>
      <section>
        <h3>Colour</h3>
        <Row>
          <Ramp value={look} onChange={(patch) => setLook({ ...look, ...patch })} />
        </Row>
      </section>
    </>
  );

  return (
    <Page crumb="radial" title="Stack a design around its centre"
      sub={<>Copy a design, turn it, lay it on the first, and repeat. A full turn shared out over <code>q</code> copies is what a screen shows when a spinning design advances <code>p/q</code> of a turn per frame: it keeps exactly the circular harmonics whose order is a multiple of <code>q</code>, and a design of rotation order <code>g</code> shows <code>lcm(q, g)</code> petals. The spectrum on the right says which harmonics the design has to give.</>}
      controls={controls}
      foot={<>Each copy is the source turned about its centre; a pixel of the stack averages a few points, each point reading every copy at once and merging them by the blend: their mean, their sum, their union, their meet, their parity, or what the first copy keeps that no other has. The harmonics are exact: every circle about the centre is cut at the grid lines it crosses and the arc integrals of <code>e^(-i m theta)</code> are taken in closed form. Two copies at 45 degrees is the same picture as an afterglow at 900 rpm on a 60 Hz screen with the phase slipping by an eighth of a turn. Which harmonics a turned stack keeps, and the exact circle mean behind them, are in <a href="/research/spin/">the spin note</a>.</>}>
      <div className="arena">
        <div className="panel">
          <h2>The stack <span>{view && `${view.copies} ${view.copies === 1 ? 'copy' : 'copies'}, ${view.blend}`}</span></h2>
          {view && <Pixels data={view.pixels} role="img" aria-label="The stack" />}
        </div>
        <div className="panel">
          <h2>Spin spectrum <span>circular-harmonic power by order</span></h2>
          <Sketch draw={chart} deps={[view]} className="bars" style={{ height: 300 }} role="img" aria-label="Spin spectrum" />
          <p className="foot" style={{ marginTop: 12, paddingTop: 10 }}>Gold is order zero, the infinite spin. Blue orders survive this stack, dim ones are cancelled by it. The rotation order of the source is the gcd of its live orders.</p>
        </div>
      </div>
      <Stats>
        <Stat label="name">{view?.name}</Stat>
        <Stat label="side">{view?.side}</Stat>
        <Stat label="copies">{view?.copies}</Stat>
        <Stat label="step">{view && `${view.step}°`}</Stat>
        <Stat label="rotation order">{view && (view.order || 'round')}</Stat>
        <Stat label="petals">{view?.petals}</Stat>
      </Stats>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
