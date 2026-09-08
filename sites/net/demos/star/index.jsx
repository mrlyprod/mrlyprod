import { useMemo, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Stats, Stat, Note } from '../../lib/app.jsx';
import { Pixels, Signs, Sketch } from '../../lib/draw.jsx';
import { board, line, axis, tag } from '../../lib/chart.js';
import { useQuery } from '../../lib/query.js';
import { Ramp } from '../../lib/select.jsx';

const m = await ready();
const CARPET = '23';
const SIZES = [128, 192, 256, 320, 384];
const hues = () => ({ plus: ink.yellow, minus: ink.blue, empty: ink.deep });

const fixed = (value, places) => (Number.isFinite(value) ? value.toFixed(places) : 'none');

function Arm({ read }) {
  const rows = read.rows;
  const draw = (canvas) => {
    const b = board(canvas, 240);
    const n = rows.length;
    const at = (i) => (i + 0.5) / n;
    const step = Math.max(1, Math.ceil(n / 12));
    axis(b, rows.map((row, i) => [at(i), String(row.n)]).filter((_, i) => i % step === 0), { wall: true });
    b.ctx.strokeStyle = ink.line;
    b.ctx.lineWidth = 1;
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(0), b.y(0.5));
    b.ctx.lineTo(b.x(1), b.y(0.5));
    b.ctx.stroke();
    line(b, rows.map((row, i) => [at(i), row.law]), ink.blue, { width: 1.5 });
    line(b, rows.map((row, i) => [at(i), row.ink]), ink.yellow, { width: 0, dots: 2.6 });
    tag(b, 'the counted ink of the band, one dot a layer', ink.yellow);
    tag(b, '1/2 + chi_8(n)/(2n)', ink.blue, 'right');
  };
  return <Sketch className="bars" draw={draw} deps={[read]} role="img" aria-label="The band ink at every odd layer against the closed form" />;
}

function Fall({ read }) {
  const draw = (canvas) => {
    const b = board(canvas, 240, { left: 46 });
    const walk = read.walk;
    if (walk.length < 2) {
      tag(b, 'two rungs are needed before the decay reads', ink.dim);
      return;
    }
    const model = walk.map(([count]) => [count, -Math.log(count) / 4 + read.constant]);
    const lows = walk.map(([, value]) => value).concat(model.map(([, value]) => value));
    const low = Math.min(...lows), high = Math.max(...lows);
    const span = Math.max(high - low, 1e-9);
    const first = Math.log(walk[0][0]), last = Math.log(walk[walk.length - 1][0]);
    const reach = Math.max(last - first, 1e-9);
    const place = ([count, value]) => [(Math.log(count) - first) / reach, (value - low) / span];
    line(b, model.map(place), ink.blue, { width: 1.5, dash: [4, 4] });
    line(b, walk.map(place), ink.yellow, { width: 1.8 });
    axis(b, [[0, String(walk[0][0])], [0.5, 'L'], [1, String(walk[walk.length - 1][0])]], { wall: true });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    b.ctx.fillText(fixed(high, 3), b.left - 6, b.y(1) + 4);
    b.ctx.fillText(fixed(low, 3), b.left - 6, b.y(0) + 4);
    b.ctx.textAlign = 'left';
    tag(b, 'L times the excess, read against ln L', ink.yellow);
    tag(b, read.slope === null ? 'the L/2 window mixes the parities here' : `slope ${read.slope.toFixed(6)} against ${read.target.toFixed(6)}`, ink.blue, 'right');
  };
  return <Sketch className="bars" draw={draw} deps={[read]} role="img" aria-label="The scaled excess against the log of the layer count" />;
}

function App() {
  const [pick, set] = useQuery({ layers: 28, half: 0, size: 256 });
  const [look, setLook] = useState({ ramp: 'fire', levels: 24, invert: false });

  let error = null;
  let view = null;
  let arm = null;
  let fall = null;
  try {
    const field = m.star_field(CARPET, pick.layers, pick.size);
    let low = Infinity, high = -Infinity;
    for (const value of field) if (!Number.isNaN(value)) { low = Math.min(low, value); high = Math.max(high, value); }
    view = {
      pixels: m.paint_span(field, pick.size, low, high, look.ramp, look.levels, look.invert),
      band: m.star_band(CARPET, pick.layers, pick.size, pick.half),
      low,
      high,
    };
    arm = JSON.parse(m.star_layers(CARPET, pick.layers, pick.half));
    fall = JSON.parse(m.star_decay(CARPET, pick.layers, pick.half));
  } catch (fault) {
    error = fault;
  }

  const ribbon = useMemo(() => (arm ? arm.rows.slice(0, 12) : []), [arm]);
  const deepest = arm ? arm.rows[arm.rows.length - 1] : null;

  const controls = (
    <>
      <section>
        <h3>The stack</h3>
        <Row>
          <Slider label="layers L" value={pick.layers} min={4} max={128} step={2} onChange={(v) => set({ layers: v })} />
          <Pick label="size" value={pick.size} options={SIZES.map((v) => [v, v])} onChange={(v) => set({ size: +v })} />
        </Row>
      </section>
      <section>
        <h3>The band</h3>
        <Row>
          <Slider label="half-width W" value={pick.half} min={0} max={16} step={1} show={`${pick.half} cells`} onChange={(v) => set({ half: v })} />
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
    <Page crumb="star" title="The ghost star at the hexagon's centre"
      sub="Stack the hexagonal cuts of a carpet cube, one per odd side, and a six-armed star stands at the centre. It is not there in the limit: it is the gap between an arm that carries no 1/(2n) term and a background that does, and it fades like (ln L)/L. How fast depends on how you measure it."
      controls={controls}
      foot={<>The picture resamples every layer onto one square, which is the ideal frame; the numbers below never resample, reading each layer on the cube's own cells, which is the cell frame. That difference is the whole point: the same star decays at <b>-0.1807</b> in the ideal frame, <b>-0.1252</b> in the lattice frame and exactly <b>-1/4</b> on the arm in the cell frame, and widening the band walks the coefficient again. Only the order <code>(ln L)/L</code>, the sign, and the cell frame's <code>-1/4</code> survive the change of frame, and only <code>-1/4</code> has a closed form beside it. At odd <code>L</code> the constant shifts by <code>1/8</code>, so the slider steps by two and stays even. That same character term traps the sliding window: it cancels between <code>L/2</code> and <code>L</code> only when <code>L</code> is divisible by four, so at <code>L = 2 mod 4</code> the slope is refused rather than reported wrong, and what you read instead is the other branch of the <code>1/L^2</code> term. The stack the layers come from is <a href="../moire">moire</a> and <a href="../volume">volume</a>, the mesh they are counted on is <a href="../slices">slices</a>, and the ink law the background uses is read back from the corners in <a href="../spectrometer">spectrometer</a>. The width readout beside the slope is the closed form <code>-(K + b)/(4(2K + 1))</code> at <code>K = floor(W/2)</code>, so an odd width is never a new band: <code>x - y</code> is even on the cut, and <code>W</code> and <code>W - 1</code> read one point set and one coefficient. The mathematics, its proofs and the frames it was measured in are on <a href="/research/hexagon/">the hexagon page</a>. Every number here is computed in Rust; the page only draws.</>}>
      <div className="arena">
        <div className="panel">
          <h2>The stack <span>{pick.layers} cut layers on one square, the mean ink at every sample</span></h2>
          {view && <Pixels data={view.pixels} />}
          <Stats>
            <Stat label="layers">{pick.layers}</Stat>
            <Stat label="deepest side">{fall?.deepest}</Stat>
            <Stat label="ink range">{view && `${fixed(view.low, 4)} to ${fixed(view.high, 4)}`}</Stat>
          </Stats>
        </div>
        <div className="panel">
          <h2>The band <span>yellow where the star is read, blue the background it is read against</span></h2>
          {view && <Signs grid={view.band} hues={hues()} />}
          <Stats>
            <Stat label="half-width">{`${pick.half} cells`}</Stat>
            <Stat label="arm">{fall?.arm ? 'the exact diameter x = y' : 'a band about it'}</Stat>
          </Stats>
        </div>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The arm <span>the band's ink at every odd layer against the closed form 1/2 + chi_8(n)/(2n)</span></h2>
        {arm && <Arm read={arm} />}
        <div className="ribbon">
          {ribbon.map((row) => (
            <span key={row.n}>
              <i>n {row.n}</i>
              <b className={row.exact ? 'yellow' : undefined}>{`${row.numer}/${row.denom}`}</b>
              <i>{`law ${row.lawNumer}/${row.lawDenom}`}</i>
            </span>
          ))}
        </div>
        <Stats>
          <Stat label="exact">{arm && `${arm.exact} of ${arm.layers} layers`}</Stat>
          <Stat label="chi_8 at the deepest">{deepest && `${deepest.chi > 0 ? '+' : ''}${deepest.chi}`}</Stat>
          <Stat label="arm ink">{deepest && fixed(deepest.ink, 6)}</Stat>
          <Stat label="hexagon ink">{deepest && fixed(deepest.hex, 6)}</Stat>
          <Stat label="excess">{deepest && fixed(deepest.excess, 6)}</Stat>
        </Stats>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The decay <span>L times the mean excess, which is -(ln L)/4 + C on the arm in the cell frame</span></h2>
        {fall && <Fall read={fall} />}
        <Stats>
          <Stat label="excess * L">{fall && fixed(fall.scaled, 6)}</Stat>
          <Stat label="plus (ln L)/4">{fall && fixed(fall.logged, 10)}</Stat>
          <Stat label="C">{fall && fixed(fall.constant, 10)}</Stat>
          <Stat label="slope against ln L">{fall && (fall.slope === null ? 'needs L divisible by four' : fixed(fall.slope, 6))}</Stat>
          <Stat label="the width law">{fall && fixed(fall.target, 6)}</Stat>
          <Stat label="L mod 4">{fall?.branch}</Stat>
          <Stat label="residual * L^2">{fall && fixed(fall.residual, 8)}</Stat>
          <Stat label="predicted">{fall && (fall.predicted === null ? 'odd L has none' : fixed(fall.predicted, 8))}</Stat>
        </Stats>
        <div className="ribbon tight">
          {fall && fall.rows.map((row) => (
            <span key={row.layers}>
              <i>L {row.layers}</i>
              <b>{fixed(row.scaled, 6)}</b>
              <i>{fixed(row.logged, 8)}</i>
            </span>
          ))}
        </div>
      </div>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
