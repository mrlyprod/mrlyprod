import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { board, line, axis, rules, tag } from '../../lib/chart.js';
import { mount, Page, Group, Pick, Slider, Check, Stats, Stat, Note } from '../../lib/app.jsx';
import { Signs, Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();
const SIDES = [2, 3, 5];
const DEEPEST = 6;
const CELLS = 729;
const SAMPLES = 200;
const SWEEP = 2001;
const PAD = 14;
const BAND = { plus: ink.fg, minus: ink.blue, empty: ink.deep };

const cap = (side) => Math.min(DEEPEST, m.level_cap(side, 1, CELLS));

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({
    code: seeded(s, 2, 3, '495'), side: 3, base: 3, level: 5, eps: 7, closed: true,
  });

  const code = pick.code.trim();
  const level = Math.max(1, Math.min(pick.level, cap(pick.side)));
  const span = pick.side ** level;
  const reach = Math.max(1, span / pick.side);
  const eps = Math.max(1, Math.min(pick.eps, reach));

  const built = useMemo(() => {
    try {
      return {
        dist: m.tube_distance(code, pick.side, level, pick.base),
        pairs: m.tube_profile(code, pick.side, level, pick.base, SAMPLES),
        digits: m.modes_digits(code, pick.side, pick.base),
        d: m.dimension(code, pick.side, 2, pick.base),
        inside: m.tube_class(code, pick.side, pick.base),
        name: m.name_of(code, 2, pick.base),
        error: null,
      };
    } catch (error) {
      return { dist: null, pairs: null, digits: 0, d: 0, inside: false, name: '', error };
    }
  }, [code, pick.side, level, pick.base]);

  const limit = useMemo(() => {
    if (!built.inside) return null;
    const gap = 1 - 1 / pick.side;
    let low = Infinity, high = -Infinity;
    for (let i = 0; i < SWEEP; i += 1) {
      const g = m.tube_closed(pick.side, built.digits, 1 / pick.side + (gap * i) / (SWEEP - 1));
      low = Math.min(low, g);
      high = Math.max(high, g);
    }
    return { low, high, swing: (100 * (high - low)) / low };
  }, [built.inside, built.digits, pick.side]);

  const cells = useMemo(() => {
    if (!built.dist) return null;
    return { width: span, height: span, types: Uint8Array.from(built.dist, (v) => (v === 0 ? 0 : v <= eps ? 1 : 2)) };
  }, [built.dist, eps]);

  const volume = useMemo(
    () => (built.dist ? m.tube_volume(code, pick.side, level, pick.base, eps) : null),
    [built.dist, eps],
  );

  const gold = useMemo(() => {
    if (!limit || !built.pairs || !pick.closed) return null;
    const out = [];
    for (let k = 0; k < built.pairs.length; k += 2) out.push(m.tube_closed(pick.side, built.digits, Math.exp(-built.pairs[k])));
    return out;
  }, [limit, built.pairs, pick.closed]);

  const profile = (canvas) => {
    const pairs = built.pairs;
    if (!pairs || pairs.length < 4) return;
    const b = board(canvas, 340, { pad: PAD, top: 20, bottom: 22 });
    const us = [], ms = [];
    for (let k = 0; k < pairs.length; k += 2) {
      us.push(pairs[k]);
      ms.push(pairs[k + 1]);
    }
    const [u0, u1] = [us[0], us.at(-1)];
    const seen = gold ? ms.concat(gold) : ms;
    const floor = Math.min(...seen), roof = Math.max(...seen);
    const pad = (roof - floor) * 0.08 || 0.05;
    const fx = (u) => (u - u0) / (u1 - u0);
    const fy = (v) => (v - floor + pad) / (roof - floor + 2 * pad);
    const step = Math.log(pick.side);
    const marks = [];
    for (let n = Math.ceil(u0 / step); n * step <= u1; n += 1) marks.push(fx(n * step));
    rules(b, marks, { dash: [2, 4] });
    rules(b, [fx(Math.log(span / eps))], { color: ink.pink });
    if (gold) {
      line(b, [[0, fy(limit.low)], [1, fy(limit.low)]], ink.dim, { width: 1, dash: [3, 5] });
      line(b, [[0, fy(limit.high)], [1, fy(limit.high)]], ink.dim, { width: 1, dash: [3, 5] });
      line(b, us.map((u, i) => [fx(u), fy(gold[i])]), ink.gold, { width: 1.6 });
    }
    line(b, us.map((u, i) => [fx(u), fy(ms[i])]), ink.blue, { width: 1.6 });
    axis(b, [[0, `ln 1/eps ${u0.toFixed(2)}`], [1, u1.toFixed(2)]], { wall: true });
    const edge = tag(b, 'M measured', ink.blue);
    if (gold) tag(b, `G limit, swing ${limit.swing.toFixed(5)}%`, ink.gold, 'left', edge + 12);
  };

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={2} bases={[3, 2]} code={pick.code} base={pick.base} seeds={s} onChange={set} />
        <Pick label="side" value={pick.side} options={SIDES.map((v) => [v, v])} onChange={(v) => set({ side: +v, level: Math.min(level, cap(+v)) })} />
        <Slider label="level" value={level} min={1} max={cap(pick.side)} show={`${level}, ${span} by ${span}`} onChange={(v) => set({ level: v })} />
      </Group>
      <Group name="Radius">
        <Slider label="eps" value={eps} min={1} max={reach} show={`${eps} cells, ${(eps / span).toFixed(6)} of the side`} onChange={(v) => set({ eps: v })} />
      </Group>
      <Group name="Limit">
        <Check label="closed form" checked={pick.closed} onChange={(v) => set({ closed: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="tube" title="The inner tube of a design and what it refuses to settle on"
      sub={<>Fatten a design by <code>eps</code> and measure the area it swallows: that is the inner tube <code>V(eps)</code>. Divide by the power the dimension asks for and you get the Minkowski content reading <code>M(eps)</code>, which should settle down if the design has a length in its own dimension. Drag the radius and watch it not settle: it circles the same profile forever, once per factor of <code>q</code>.</>}
      controls={controls}
      foot={<>The left panel is an exact Euclidean distance transform of the level-<code>L</code> grid in Rust, the two-pass lower envelope of parabolas, so every cell carries its true distance in cell widths to the nearest filled cell; the band is that field thresholded at the radius, and the tube area is the field read again, each cell carrying the share of itself the radius reaches. No hole lemma enters, so every design the picker offers gets a tube and a profile. The gold curve is the other route and applies only to designs whose holes are isolated interior squares with their boundaries in the set: there the complement splits level by level into <code>k^(m-1)</code> open squares of side <code>q^(-m)</code>, the inner parallel area of a square of side <code>s</code> is <code>4 eps s - 4 eps^2</code> until <code>2 eps</code> passes <code>s</code> and <code>s^2</code> after, and the two geometric tails close in the form the page draws. The measured curve sits below the limit by about <code>(4/5) eps^(d-1)</code>, the cost of a finite grid, and climbs onto it as the radius shrinks. The dashed rules are the powers of <code>q</code>, one period of the profile apart. The same design counted inside a shape rather than fattened is <a href="../crop">the crop</a>, and its mask laid over the torus is <a href="../modes">the modes</a>. Every distance, area and profile value comes out of the crates through wasm; the page only draws.</>}>
      <p><span className="chip proved">Proved</span> The Sierpinski carpet is not Minkowski measurable: its tube is the exact hole sum, <code>M(eps)</code> runs onto a log-periodic <code>G(t)</code> with <code>G(1/3) = G(1) = 379/280</code>, and the swing between its maximum and its minimum is <code>0.36625%</code>, above zero, so no limit exists. The proof, the class it opens and the sponge it does not reach are on <a href="/research/dimensions/">the dimensions page</a>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The design and its tube <span>{`level ${level}, radius ${eps} of ${span} cells`}</span></h2>
          {cells && <Signs grid={cells} hues={BAND} role="img" aria-label="The design in the foreground with its inner tube band in blue" />}
        </div>
        <div className="panel">
          <h2>The Minkowski profile <span>{built.pairs && built.pairs.length ? `M against ln 1/eps, ${SAMPLES} radii` : 'the grid is too coarse to resolve a range of radii'}</span></h2>
          <Sketch draw={profile} deps={[built.pairs, gold, eps, span]} />
        </div>
      </div>
      <Stats>
        <Stat label="design">{built.name}</Stat>
        <Stat label="base">{pick.base}</Stat>
        <Stat label="side q">{pick.side}</Stat>
        <Stat label="digits k">{built.digits}</Stat>
        <Stat label="dimension d">{built.d ? built.d.toFixed(9) : ''}</Stat>
        <Stat label="grid">{`${span} by ${span}`}</Stat>
      </Stats>
      <Stats>
        <Stat label="eps">{`${eps} cells, ${(eps / span).toFixed(9)}`}</Stat>
        <Stat label="V in cells">{volume === null ? '' : volume.toFixed(3)}</Stat>
        <Stat label="V of the square">{volume === null ? '' : (volume / (span * span)).toFixed(9)}</Stat>
        <Stat label="closed form">{built.inside ? 'the design is in the class' : 'outside the class, measured only'}</Stat>
        <Stat label="G band">{limit ? `${limit.low.toFixed(9)} to ${limit.high.toFixed(9)}` : ''}</Stat>
        <Stat label="swing">{limit ? `${limit.swing.toFixed(6)}%` : ''}</Stat>
      </Stats>
      <Note error={built.error} />
    </Page>
  );
}

mount(<App />);
