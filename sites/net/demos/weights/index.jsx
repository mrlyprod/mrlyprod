import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { board, line, axis, rules, tag } from '../../lib/chart.js';
import { mount, Page, Group, Pick, Slider, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Pixels, Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const SIDES = [2, 3, 5];
const DEEPEST = 6;
const CELLS = 729;
const SLIDERS = 9;
const CAP = 32;
const SAMPLES = 241;
const MOMENTS = 161;
const REACH = 4;
const PAD = 14;

const cap = (side) => Math.min(DEEPEST, m.level_cap(side, 1, CELLS));
const sum = (list) => list.reduce((a, b) => a + b, 0);

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({
    code: seeded(s, 2, 3, '69'), side: 3, base: 3, level: 4,
    w: '3,2,3', gamma: 35, curve: 'spectrum',
  });

  const code = pick.code.trim();
  const level = Math.max(1, Math.min(pick.level, cap(pick.side)));
  const span = pick.side ** level;

  const corners = useMemo(() => {
    try {
      return { list: Array.from(m.weights_corners(code, pick.side, pick.base)), error: null };
    } catch (error) {
      return { list: [], error };
    }
  }, [code, pick.side, pick.base]);

  const k = corners.list.length / 2;
  const shares = useMemo(() => {
    const read = pick.w.split(',').map((v) => Math.max(1, Math.min(CAP, Math.round(+v) || 1)));
    return read.length === k ? read : Array(k).fill(1);
  }, [pick.w, k]);
  const key = shares.join(',');
  const whole = sum(shares);

  const read = useMemo(() => {
    if (!k) return { error: corners.error };
    try {
      const w = Float64Array.from(shares);
      return {
        dims: m.weights_dims(code, pick.side, pick.base, w),
        point: m.weights_point(code, pick.side, pick.base, w, 1),
        spectrum: m.weights_spectrum(code, pick.side, pick.base, w, SAMPLES),
        pressure: m.weights_pressure(code, pick.side, pick.base, w, -REACH, REACH, MOMENTS),
        name: m.name_of(code, 2, pick.base),
        error: null,
      };
    } catch (error) {
      return { error };
    }
  }, [code, pick.side, pick.base, key]);

  const drawn = useMemo(() => {
    if (!k || read.error) return null;
    const w = Float64Array.from(shares);
    const mass = m.weights_mass(code, pick.side, level, pick.base, w);
    let least = Infinity;
    for (const v of mass) if (v > 0 && v < least) least = v;
    return {
      sheet: m.weights_pixels(code, pick.side, level, pick.base, w, pick.gamma / 100),
      peak: Math.max(...mass),
      least,
      cells: mass.reduce((count, v) => count + (v > 0 ? 1 : 0), 0),
    };
  }, [code, pick.side, pick.base, level, key, pick.gamma]);

  const curve = (canvas) => {
    if (read.error || !read.dims) return;
    const b = board(canvas, 340, { pad: PAD, top: 20, bottom: 22 });
    const [low, high, one, roof] = Array.from(read.dims);
    if (pick.curve === 'pressure') {
      const xs = [], ys = [];
      for (let i = 0; i < read.pressure.length; i += 2) {
        xs.push(read.pressure[i]);
        ys.push(read.pressure[i + 1]);
      }
      const floor = Math.min(...ys), top = Math.max(...ys);
      const pad = (top - floor) * 0.08 || 0.05;
      const fx = (v) => (v + REACH) / (2 * REACH);
      const fy = (v) => (v - floor + pad) / (top - floor + 2 * pad);
      rules(b, [fx(0), fx(1)], { dash: [2, 4] });
      line(b, [[0, fy(0)], [1, fy(0)]], ink.line, { width: 1 });
      line(b, xs.map((v, i) => [fx(v), fy(ys[i])]), ink.blue, { width: 1.6 });
      line(b, [[fx(0), fy(roof)]], ink.gold, { dots: 3.6 });
      line(b, [[fx(1), fy(0)]], ink.gold, { dots: 3.6 });
      axis(b, [[0, `s ${-REACH}`], [0.5, '0'], [1, `${REACH}`]], { wall: true });
      const edge = tag(b, 'tau(s) = log_q sum w^s', ink.blue);
      tag(b, `tau(0) ${roof.toFixed(9)}, tau(1) 0`, ink.gold, 'left', edge + 12);
      return;
    }
    const xs = [], ys = [];
    for (let i = 0; i < read.spectrum.length; i += 2) {
      xs.push(read.spectrum[i]);
      ys.push(read.spectrum[i + 1]);
    }
    const wide = high - low || 1;
    const fx = (v) => 0.04 + 0.92 * (v - low) / wide;
    const fy = (v) => 0.05 + 0.9 * v / (roof || 1);
    rules(b, [fx(one)], { dash: [2, 4] });
    line(b, [[0, fy(0)], [1, fy(0)]], ink.line, { width: 1 });
    line(b, xs.map((v, i) => [fx(v), fy(ys[i])]), ink.blue, { width: 1.6 });
    line(b, [[fx(read.point[2]), fy(read.point[3])]], ink.gold, { dots: 4 });
    axis(b, [[0, `alpha ${low.toFixed(6)}`], [1, high.toFixed(6)]], { wall: true });
    const edge = tag(b, 'f(alpha)', ink.blue);
    tag(b, `alpha(1) ${one.toFixed(9)}`, ink.gold, 'left', edge + 12);
  };

  const spread = (make) => set({ w: make().join(',') });
  const flat = () => Array(k).fill(1);
  const rim = (v) => v === 0 || v === pick.side - 1;
  const heavy = () => Array.from({ length: k }, (_, i) => (rim(corners.list[2 * i]) && rim(corners.list[2 * i + 1]) ? 8 : 1));
  const drawWeights = () => Array.from(m.random_between(s.next(), Array(k).fill(1), Array(k).fill(CAP)));

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={2} bases={[3, 2]} code={pick.code} base={pick.base} seeds={s} onChange={set} />
        <Pick label="side" value={pick.side} options={SIDES.map((v) => [v, v])} onChange={(v) => set({ side: +v, level: Math.min(level, cap(+v)) })} />
        <Slider label="level" value={level} min={1} max={cap(pick.side)} show={`${level}, ${span} by ${span}`} onChange={(v) => set({ level: v })} />
      </Group>
      <Group name={`Weights over ${whole}`}>
        {k <= SLIDERS && shares.map((share, i) => (
          <Slider key={i} label={`(${corners.list[2 * i]},${corners.list[2 * i + 1]})`} value={share} min={1} max={CAP}
            show={`${share}/${whole}`} onChange={(v) => spread(() => shares.map((held, at) => (at === i ? v : held)))} />
        ))}
        <Btn primary onClick={() => spread(flat)}>equal</Btn>
        {k > SLIDERS && <Btn onClick={() => spread(heavy)}>corner-heavy</Btn>}
        {k > SLIDERS && <Btn onClick={() => spread(drawWeights)}>seeded random</Btn>}
      </Group>
      <Group name="Reading">
        <Slider label="gamma" value={pick.gamma} min={10} max={100} show={(pick.gamma / 100).toFixed(2)} onChange={(v) => set({ gamma: v })} />
        <Pick label="curve" value={pick.curve} options={[['spectrum', 'f(alpha)'], ['pressure', 'tau(s)']]} onChange={(v) => set({ curve: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="weights" title="The mass side of a weighted design"
      sub={<>Give every filled corner of a design a weight and the level-<code>L</code> cell reached by the digit word <code>f_1 ... f_L</code> carries the mass <code>w_(f_1) ... w_(f_L)</code>. The support never moves, the mass does: drag one weight and the picture darkens on one corner and lights on another while the design stays where it is. The curve beside it is the whole multifractal spectrum, and it is a closed form in the weights alone.</>}
      controls={controls}
      foot={<>The design is the picker's plane code at side <code>q</code> and its residue base; the filled cells of its level-one tile are the corners the sliders weight, listed row by row, and the vector is normalised to sum to one inside the crate. The picture is the level-<code>L</code> mass field, one cell a pixel, read at <code>(mass / peak)^gamma</code> through the ground-blue-gold ramp, so the gamma is display alone and never a printed number. The pressure is summed with the largest exponent factored out, which is why the curve stays exact out to <code>s = 30</code> where the spectrum's two tails are sampled. Equal weights are the 0/1 design itself: the spectrum collapses to the single point <code>(log_q k, log_q k)</code>, which is the box dimension the support has at every weighting. The geometry these weights leave alone is the same design fattened on <a href="../tube">the tube</a>, cropped on <a href="../crop">the crop</a> and laid over the torus on <a href="../modes">the modes</a>. Every mass, every exponent and every point of the curve comes out of the crates through wasm; the page only draws.</>}>
      <p><span className="chip verified">Verified</span> At contraction <code>1/q</code> under the open set condition, which every design satisfies with the unit cell, the pressure equation closes in one line: <code>tau(s) = log_q sum_f w_f^s</code>, with <code>f(alpha) = inf_s (alpha s + tau(s))</code> attained at <code>alpha(s) = -tau'(s)</code>, so the spectrum is explicit in the weights and nothing is fitted. The proof it is read from and the exact tables are on <a href="../../research/weights/">the weights page</a>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The mass field <span>{`level ${level}, ${span} by ${span}`}</span></h2>
          {drawn && <Pixels data={drawn.sheet} role="img" aria-label="The level-L mass field of the weighted design" />}
        </div>
        <div className="panel">
          <h2>{pick.curve === 'pressure' ? 'The pressure' : 'The spectrum'} <span>{pick.curve === 'pressure' ? `tau against s, ${MOMENTS} moments` : `f against alpha, ${SAMPLES} moments`}</span></h2>
          <Sketch draw={curve} deps={[read, pick.curve]} />
        </div>
      </div>
      <Stats>
        <Stat label="design">{read.name ?? ''}</Stat>
        <Stat label="base">{pick.base}</Stat>
        <Stat label="side q">{pick.side}</Stat>
        <Stat label="corners k">{k}</Stat>
        <Stat label="weights">{`${shares.join(' ')} over ${whole}`}</Stat>
        <Stat label="cells">{drawn ? `${drawn.cells} of ${span * span}` : ''}</Stat>
      </Stats>
      <Stats>
        <Stat label="alpha_min">{read.dims ? read.dims[0].toFixed(9) : ''}</Stat>
        <Stat label="alpha_max">{read.dims ? read.dims[1].toFixed(9) : ''}</Stat>
        <Stat label="alpha(1)">{read.dims ? read.dims[2].toFixed(9) : ''}</Stat>
        <Stat label="tau(0)">{read.dims ? read.dims[3].toFixed(9) : ''}</Stat>
        <Stat label="heaviest cell">{drawn ? drawn.peak.toExponential(6) : ''}</Stat>
        <Stat label="lightest cell">{drawn ? drawn.least.toExponential(6) : ''}</Stat>
      </Stats>
      <Note error={read.error} />
    </Page>
  );
}

mount(<App />);
