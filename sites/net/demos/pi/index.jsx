import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { useQuery } from '../../lib/query.js';
import { mount, Page, Row, Slider, Pick, Check, Stats, Stat } from '../../lib/app.jsx';
import { Pixels, Sketch } from '../../lib/draw.jsx';
import { board, line, axis, tag } from '../../lib/chart.js';

const m = await ready();

const SIDE = 720;
const STOPS = 220;
const FLOOR = 8;
const REACH = 500;
const DIMS = [2, 3, 4, 5, 6];

const whole = (value, low, high, fallback) =>
  (Number.isFinite(value) ? Math.min(high, Math.max(low, Math.round(value))) : fallback);

function App() {
  const [q, set] = useQuery({ n: 100, d: 2, layers: true });
  const n = whole(q.n, FLOOR, REACH, 100);
  const d = whole(q.d, DIMS[0], DIMS[DIMS.length - 1], 2);
  const read = useMemo(() => JSON.parse(m.visible_read(n, d)), [n, d]);
  const walk = useMemo(() => m.visible_walk(n, d, STOPS), [n, d]);
  const sheet = useMemo(() => m.visible_pixels(n, SIDE, q.layers), [n, q.layers]);

  const stops = [];
  for (let k = 0; k < walk.length; k += 2) if (walk[k] >= FLOOR) stops.push([walk[k], walk[k + 1]]);

  const chart = (canvas) => {
    const b = board(canvas, 220, { pad: 26, top: 24, bottom: 30 });
    const span = Math.max(...stops.map(([, v]) => Math.abs(v - read.truth)), 1e-12) * 1.2;
    const width = Math.log(n) - Math.log(FLOOR);
    const across = (w) => (width > 0 ? (Math.log(w) - Math.log(FLOOR)) / width : 0);
    const up = (v) => 0.5 + (v - read.truth) / (2 * span);
    axis(b, [[0, String(FLOOR)], [1, String(n)]]);
    line(b, [[0, 0.5], [1, 0.5]], ink.dim, { width: 1, dash: [4, 4] });
    line(b, stops.map(([w, v]) => [across(w), up(v)]), ink.blue, { width: 1.4 });
    tag(b, `${read.name} recovered as the window grows`, ink.blue);
    tag(b, read.truth.toFixed(9), ink.dim, 'right');
  };

  const controls = (
    <Row>
      <Slider label="window n" value={n} min={FLOOR} max={REACH} onChange={(v) => set({ n: v })} />
      <Pick label="dimension" value={String(d)} onChange={(v) => set({ d: +v })} options={DIMS.map((v) => [String(v), String(v)])} />
      <Check label="shade the stack layers" checked={q.layers} onChange={(v) => set({ layers: v })} />
    </Row>
  );

  return (
    <Page crumb="pi" title="Pi out of the grid"
      sub="Stand at the corner and light every lattice point nothing hides: the pairs whose coordinates share no divisor. They take six over pi squared of the window, so counting dots hands pi back, and counting in d dimensions hands back zeta of d instead."
      controls={controls}
      foot={<>Every point (a, b) is a scaled copy g * (a/g, b/g) of a lit one, g the divisor they share, so the grid is the lit set stacked at every scale and the lit share is one over zeta. The count is 2 sum phi(k) - 1, sieved in Rust; the picture, the limit and the constant come out of the same crate call and the page only paints them. Why one carpet cannot hold pi is in <a href="/research/pi/">the pi note</a>, and the same stack drawn as fractions is <a href="../farey/">the Farey stack</a>.</>}>
      <div className="arena">
        <div className="panel">
          <h2>The window <span>{`${n} by ${n}, ${read.lit} lit`}</span></h2>
          <Pixels data={sheet} role="img" aria-label="The corner window of the grid, the points visible from the origin lit, the hidden ones shaded by the scale that hides them" />
        </div>
        <div className="panel">
          <h2>The approach <span>{`windows ${FLOOR} to ${n}, dimension ${d}`}</span></h2>
          <Sketch draw={chart} deps={[n, d]} className="bars" role="img" aria-label="The constant recovered at every window, against the value it walks to" />
          <Stats>
            <Stat label="lit points">{read.lit}</Stat>
            <Stat label="of">{read.total}</Stat>
            <Stat label="density">{read.density.toFixed(10)}</Stat>
          </Stats>
          <Stats>
            <Stat label={`limit 1/zeta(${d})`}>{read.limit.toFixed(10)}</Stat>
            <Stat label={read.name}>{read.constant.toFixed(8)}</Stat>
            <Stat label="off by">{read.error.toExponential(1)}</Stat>
          </Stats>
        </div>
      </div>
    </Page>
  );
}

mount(<App />);
