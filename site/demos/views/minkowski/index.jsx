import { useMemo } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { board, line, axis, rules, tag } from '../../../lib/chart.js';
import { mount, Page, Group, Slider, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Signs, Sketch } from '../../../lib/draw.jsx';
import { useQuery } from '../../../lib/query.js';

const m = await ready();
const PERIODS = 8;
const STEPS = 72;
const SIDE = 486;
const PLANES = 162;
const DOT = 0.75 / SIDE;
const PAD = 14;
const walk = JSON.parse(m.minkowski_walk(PERIODS, STEPS));
const COUNT = walk.u.length;
const START = walk.start;
const END = walk.start + PERIODS * walk.period;
const OPEN = Math.log(walk.cover / walk.edge);
const band = () => ({ plus: ink.fg, minus: ink.blue, empty: ink.deep });
const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, Math.round(v)));
const fixed = (v, n = 9) => (v === null || v === undefined ? 'no closed form' : v.toFixed(n));

function App() {
  const [pick, set] = useQuery({ at: 140, plane: 81 });
  const at = clamp(pick.at, 0, COUNT - 1);
  const plane = clamp(pick.plane, 0, PLANES / 2);
  const radius = walk.radius[at];
  const u = walk.u[at];

  const slice = useMemo(() => {
    try {
      return { dist: m.minkowski_slice(plane / PLANES, SIDE), error: null };
    } catch (error) {
      return { dist: null, error };
    }
  }, [plane]);

  const read = useMemo(() => JSON.parse(m.minkowski_read(radius)), [radius]);

  const cells = useMemo(() => {
    if (!slice.dist) return null;
    return { width: SIDE, height: SIDE, types: Uint8Array.from(slice.dist, (d) => (d <= DOT ? 0 : d <= radius ? 1 : 2)) };
  }, [slice.dist, radius]);

  const open = read.profile === null;
  const phase = (((u - START) / walk.period) % 1) * 100;

  const chart = (canvas) => {
    const b = board(canvas, 340, { pad: PAD, top: 20, bottom: 22 });
    const spread = walk.high - walk.low;
    const floor = walk.low - 1.6 * spread, roof = walk.high + 0.35 * spread;
    const fx = (v) => (v - START) / (END - START);
    const fy = (v) => (v - floor) / (roof - floor);
    b.ctx.fillStyle = ink.line;
    for (let k = 0; k < PERIODS; k += 1) {
      const a = b.x(fx(START + k * walk.period)), c = b.x(fx(START + k * walk.period + OPEN));
      b.ctx.fillRect(a, b.roof, c - a, b.floor - b.roof);
    }
    const marks = [];
    for (let k = 1; k < PERIODS; k += 1) marks.push(fx(START + k * walk.period));
    rules(b, marks, { dash: [2, 4] });
    const runs = (values) => {
      const out = [];
      let run = [];
      values.forEach((v, i) => {
        if (v === null || v < floor) {
          if (run.length > 1) out.push(run);
          run = [];
          return;
        }
        run.push([fx(walk.u[i]), fy(Math.min(v, roof))]);
      });
      if (run.length > 1) out.push(run);
      return out;
    };
    for (const run of runs(walk.profile)) line(b, run, ink.yellow, { width: 1.8 });
    for (const run of runs(walk.reading)) line(b, run, ink.blue, { width: 1.6 });
    rules(b, [fx(u)], { color: ink.pink });
    axis(b, [[0, `ln 1/r ${START.toFixed(2)}`], [1, END.toFixed(2)]], { wall: true });
    const edge = tag(b, 'reading', ink.blue);
    tag(b, `profile p, swing ${walk.swing.toFixed(4)}%`, ink.yellow, 'left', edge + 12);
  };

  const seek = (f) => set({ at: clamp(f * COUNT - 0.5, 0, COUNT - 1) });

  const controls = (
    <>
      <Group name="Radius">
        <Slider label="r" value={at} min={0} max={COUNT - 1} show={`${radius.toPrecision(5)}, 1/${(1 / radius).toFixed(1)}`} onChange={(v) => set({ at: v })} />
      </Group>
      <Group name="Slice">
        <Slider label="z" value={plane} min={0} max={PLANES / 2} show={`${plane}/${PLANES}`} onChange={(v) => set({ plane: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="minkowski" title="The tube of the Menger sponge and the wave it never leaves"
      sub={<>Fatten the sponge by a radius <code>r</code> and weigh what it swallows. Multiply by <code>r^(D-3)</code>, the power its dimension <code>D</code> asks for, and you get the Minkowski reading, which would settle down if the sponge had a size in its own dimension. Drag the radius: the reading climbs onto a wave that repeats at every factor of 3 and never goes flat.</>}
      controls={controls}
      foot={<>The slice is the plane <code>z = {plane}/{PLANES}</code> through the unit cube, {SIDE} pixels a side, every pixel the exact distance from its centre to the sponge, computed in Rust: the digits of the point descend while each triple holds at most one <code>1</code>, and then the distance is the one to the twelve edges of the centre cube or to the four carpets walling an arm of the plus. Blue is every pixel within <code>r</code>; the sponge itself is the pixels within three quarters of a pixel width of it. <code>T(r)</code> is the tube inside the plus of seven removed cubes, in closed form on <code>(0, 1/6]</code>: the edge cylinders of the centre cube plus arcsine sums over the holes of the 24 wall carpets, with the paper's term <code>24 Deep</code> left out, <code>Deep</code> the corner volume beyond both wall tubes, at most <code>24 x 3.842e-5 &lt; 9.23e-4</code> at <code>1/6</code> and <code>5.81e-9</code> at <code>1/12</code>; so the curves are the upper edge of the certificate, and only its two certified phases, <code>1/12</code> and <code>1/6</code>, carry the proof. Each of the 20 kept subcubes holds the whole picture at a third of the scale, so the volume inside the cube is <code>sum_k (20/27)^k T(3^k r)</code>, and the reading climbs onto the profile <code>p</code>, periodic in <code>ln 1/r</code> with period <code>ln 3</code>. The shaded phases are the radii <code>(1/6, sqrt(2)/6]</code> up to a power of 3, where some <code>T(3^k r)</code> has no closed form yet; there both curves stop, and that third of every period is the paper's open problem. The flat version is <a href="../tube/">the tube</a> of the carpet, and the sponge itself grows in <a href="../sponge/">the sponge</a>. Every distance, volume and profile value comes out of the crates through wasm; the page only draws.</>}>
      <p><span className="chip proved">Proved</span> The Menger sponge is not Minkowski measurable. By the criterion of Kombrink, Pearse and Winter it would be exactly when <code>p</code> is constant, and an interval-arithmetic certificate puts <code>p(1/12)</code> in <code>[2.122718, 2.122723]</code> and <code>p(1/6)</code> in <code>[2.135019, 2.136794]</code>, a swing of at least <code>0.5792%</code>. The proof is <a href="/papers/sponge-measurability/">the paper</a>; the theorem sits on <a href="/research/dimensions/">the dimensions page</a>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The tube in a slice <span>{`z = ${plane}/${PLANES}, radius ${radius.toPrecision(4)}`}</span></h2>
          {cells && <Signs grid={cells} hues={band()} role="img" aria-label="A plane slice of the Menger sponge with the points within the radius in blue" />}
        </div>
        <div className="panel">
          <h2>The reading and its profile <span>{`${PERIODS} factors of 3, shaded where no closed form reaches`}</span></h2>
          <Sketch draw={chart} deps={[at]} onSeek={seek} role="img" aria-label="The Minkowski reading climbing onto the periodic profile against ln 1/r" />
        </div>
      </div>
      <Stats>
        <Stat label="radius r">{radius.toPrecision(9)}</Stat>
        <Stat label="ln 1/r">{u.toFixed(6)}</Stat>
        <Stat label="phase">{`${phase.toFixed(1)}% of the period`}</Stat>
        <Stat label="window">{open ? 'open, no closed form' : 'closed form'}</Stat>
      </Stats>
      <Stats>
        <Stat label="T in the plus">{fixed(read.tube)}</Stat>
        <Stat label="volume in the cube">{fixed(read.volume)}</Stat>
        <Stat label="reading">{fixed(read.reading)}</Stat>
        <Stat label="profile p">{fixed(read.profile)}</Stat>
      </Stats>
      <Stats>
        <Stat label="dimension D">{walk.dimension.toFixed(9)}</Stat>
        <Stat label="p over the walk">{`${walk.low.toFixed(6)} to ${walk.high.toFixed(6)}`}</Stat>
        <Stat label="swing">{`${walk.swing.toFixed(4)}%`}</Stat>
      </Stats>
      <Note error={slice.error} />
    </Page>
  );
}

mount(<App />);
