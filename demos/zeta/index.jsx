import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, fit } from '../lib/mrly.js';
import { stamp, useQuery } from '../lib/query.js';
import { mount, Page, Row, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, line, axis, tag } from '../lib/chart.js';

const m = await ready();
const REACH = 200;
const DENSITY = 30;
const LISTED = 8;
const FIRST = { t: 30, speed: 4, zeros: 10, x: 100 };
const SPANS = [[10, 150], [1, 60], [50, 500]];
const ZEROS = m.zeta_zeros(100);
const [JOIN, SEAM] = m.zeta_seam(REACH, 1000);

const fixed = (v) => (typeof v === 'number' ? v.toFixed(4) : '');

function drawn(seed) {
  if (!seed) return FIRST;
  const [t, zeros, x] = roll(seed, SPANS);
  return { ...FIRST, t, zeros, x };
}

function App() {
  const s = useSeeds();
  const first = useRef(null);
  first.current ??= drawn(s.get());
  const [pick, set] = useQuery(first.current);
  const [head, setHead] = useState(pick.t);
  const [playing, setPlaying] = useState(false);
  const [error, setError] = useState(null);
  const at = useRef(pick.t);
  const now = useRef(pick);
  const path = useRef(null);
  path.current ??= m.zeta_line(0, pick.t, Math.max(1, Math.ceil(pick.t * DENSITY)));
  now.current = pick;

  const settle = () => stamp({ t: at.current.toFixed(2), speed: now.current.speed, zeros: now.current.zeros, x: now.current.x });

  const trace = (to) => {
    at.current = to;
    setHead(to);
    try {
      path.current = m.zeta_line(0, to, Math.max(1, Math.ceil(to * DENSITY)));
      setError(null);
    } catch (error) {
      setError(error);
    }
  };

  useEffect(() => settle(), []);

  useEffect(() => {
    if (!playing) return;
    let live = true;
    let last = 0;
    const frame = (clock) => {
      if (!live) return;
      requestAnimationFrame(frame);
      const dt = last ? Math.min(0.1, (clock - last) / 1000) : 0;
      last = clock;
      if (!dt) return;
      const to = Math.min(REACH, at.current + now.current.speed * dt);
      try {
        const grown = m.zeta_line(at.current, to, Math.max(1, Math.ceil((to - at.current) * DENSITY)));
        const longer = new Float64Array(path.current.length + grown.length - 4);
        longer.set(path.current);
        longer.set(grown.subarray(4), path.current.length);
        path.current = longer;
        at.current = to;
        setHead(to);
      } catch (error) {
        live = false;
        setError(error);
        setPlaying(false);
        settle();
        return;
      }
      if (to >= REACH) {
        live = false;
        setPlaying(false);
        settle();
      }
    };
    requestAnimationFrame(frame);
    return () => { live = false; };
  }, [playing]);

  const look = useMemo(() => {
    try {
      const [re, im, z, theta] = m.zeta_at(head);
      return { re, im, z, theta, count: m.zeta_count(head), error: null };
    } catch (error) {
      return { count: 0, error };
    }
  }, [head]);

  const fold = useMemo(() => {
    try {
      return {
        stair: m.psi_stair(pick.x),
        some: m.psi_formula(pick.x, ZEROS.subarray(0, pick.zeros), 500),
        none: m.psi_formula(pick.x, ZEROS.subarray(0, 0), 500),
        gap: m.psi_gap(pick.x, ZEROS.subarray(0, pick.zeros)),
        error: null,
      };
    } catch (error) {
      return { error };
    }
  }, [pick.x, pick.zeros]);

  const walk = (canvas) => {
    const [ctx, w, h] = fit(canvas, canvas.clientWidth);
    ctx.fillStyle = ink.deep;
    ctx.fillRect(0, 0, w, h);
    let reach = 2;
    for (let k = 0; k < path.current.length; k += 4) reach = Math.max(reach, Math.abs(path.current[k + 1]), Math.abs(path.current[k + 2]));
    const scale = (Math.min(w, h) / 2 - 14) / reach;
    const px = (re) => w / 2 + re * scale, py = (im) => h / 2 - im * scale;
    ctx.strokeStyle = ink.line;
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(0, py(0));
    ctx.lineTo(w, py(0));
    ctx.moveTo(px(0), 0);
    ctx.lineTo(px(0), h);
    ctx.stroke();
    ctx.setLineDash([3, 5]);
    ctx.beginPath();
    ctx.arc(px(0), py(0), scale, 0, Math.PI * 2);
    ctx.stroke();
    ctx.setLineDash([]);
    const draw = (from, color, width) => {
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.beginPath();
      for (let k = from; k < path.current.length; k += 4) {
        if (k === from) ctx.moveTo(px(path.current[k + 1]), py(path.current[k + 2]));
        else ctx.lineTo(px(path.current[k + 1]), py(path.current[k + 2]));
      }
      ctx.stroke();
    };
    if (path.current.length) {
      draw(0, ink.blue, 1.2);
      draw(Math.max(0, path.current.length - 4 * 2 * DENSITY), ink.fg, 2);
    }
    ctx.strokeStyle = ink.gold;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(px(0), py(0), 5, 0, Math.PI * 2);
    ctx.stroke();
    if (path.current.length) {
      ctx.fillStyle = ink.orange;
      ctx.beginPath();
      ctx.arc(px(path.current.at(-3)), py(path.current.at(-2)), 4, 0, Math.PI * 2);
      ctx.fill();
    }
    const mono = getComputedStyle(document.body).getPropertyValue('--mono');
    ctx.font = `11px ${mono}`;
    ctx.fillStyle = ink.dim;
    ctx.fillText('1', px(1) + 4, py(0) - 5);
    ctx.fillText('i', px(0) + 5, py(1) - 4);
  };

  const chart = (canvas) => {
    const b = board(canvas, 220);
    const span = Math.max(head, 1);
    let peak = 1e-9;
    for (let k = 3; k < path.current.length; k += 4) peak = Math.max(peak, Math.abs(path.current[k]));
    b.ctx.strokeStyle = ink.line;
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(0), b.y(0.5));
    b.ctx.lineTo(b.x(1), b.y(0.5));
    b.ctx.stroke();
    b.ctx.strokeStyle = ink.gold;
    for (const zero of ZEROS) {
      if (zero > head) break;
      b.ctx.beginPath();
      b.ctx.moveTo(b.x(zero / span), b.floor);
      b.ctx.lineTo(b.x(zero / span), b.floor - 10);
      b.ctx.stroke();
    }
    const points = [];
    for (let k = 0; k < path.current.length; k += 4) points.push([path.current[k] / span, 0.5 + 0.5 * path.current[k + 3] / peak]);
    if (points.length > 1) line(b, points, ink.blue);
    axis(b, [[0, '0'], [1, `t = ${head.toFixed(2)}`]]);
    tag(b, 'Z(t), real on the line', ink.blue);
    tag(b, `${look.count} ${look.count === 1 ? 'zero' : 'zeros'}`, ink.gold, 'right');
  };

  const stairs = (canvas) => {
    if (fold.error) return;
    const b = board(canvas, 260);
    const x = pick.x, k = pick.zeros;
    let peak = fold.stair.at(-1);
    for (let i = 1; i < fold.some.length; i += 2) peak = Math.max(peak, fold.some[i]);
    peak *= 1.04;
    const fx = (u) => (u - 1) / (x - 1);
    const steps = [];
    for (let n = 1; n <= x; n++) {
      steps.push([fx(n), fold.stair[n - 1] / peak]);
      if (n < x) steps.push([fx(n + 1), fold.stair[n - 1] / peak]);
    }
    const curve = (flat) => {
      const pts = [];
      for (let i = 0; i < flat.length; i += 2) pts.push([fx(flat[i]), flat[i + 1] / peak]);
      return pts;
    };
    b.ctx.save();
    b.ctx.beginPath();
    b.ctx.rect(b.left, b.roof - 4, b.wide, b.tall + 4);
    b.ctx.clip();
    line(b, curve(fold.none), ink.pink, { dash: [4, 4], width: 1 });
    line(b, curve(fold.some), ink.blue);
    line(b, steps, ink.gold, { width: 2 });
    b.ctx.restore();
    axis(b, [[0, '1'], [1, `x = ${x}`]]);
    let spot = tag(b, 'psi(x)', ink.gold);
    spot = tag(b, `formula with ${k} ${k === 1 ? 'zero' : 'zeros'}`, ink.blue, 'left', spot + 14);
    tag(b, 'no zeros', ink.pink, 'left', spot + 14);
  };

  const play = () => {
    if (playing) {
      setPlaying(false);
      settle();
      return;
    }
    if (at.current >= REACH) trace(0);
    setPlaying(true);
  };

  const shuffle = () => {
    setPlaying(false);
    const [t, zeros, x] = roll(s.next(), SPANS);
    set({ zeros, x });
    trace(t);
    stamp({ t: t.toFixed(2), speed: now.current.speed });
  };

  const next = ZEROS.find((zero) => zero > head);
  const list = Array.from(ZEROS.subarray(0, LISTED), (zero, i) => `${String(i + 1).padStart(2)}  ${zero.toFixed(6)}${zero <= head ? '  passed' : ''}`).join('\n');

  return (
    <Page crumb="zeta" title="Walking the critical line"
      sub={<>The zeta function is a curve you can walk. Put <code>s = 1/2 + it</code> and let <code>t</code> grow: the point loops around the plane and every so often passes straight through the origin, one zero per pass. Those zeros know where the primes are: the staircase below counts the prime powers, and adding the zeros one by one folds a smooth guess into it.</>}
      foot={<>Two engines share the line. Below <code>t = {JOIN}</code> every point is the complex Euler-Maclaurin sum, <code>t</code> plus ten terms closed by seven Bernoulli corrections, good to ten decimals; above it Z(t) comes from the Riemann-Siegel formula, the main sum of <code>floor(sqrt(t / 2pi))</code> cosines and the first four correction terms, with the kernel derivatives taken by central differences, and <code>zeta = Z e^(-i theta)</code>. On this page the two engines never differ by more than {SEAM.toExponential(1)} in Z beyond the join, so the seam is invisible. <code>theta(t)</code> is the argument of <code>Gamma(1/4 + it/2)</code> less <code>t ln(pi) / 2</code>, by Stirling's series after a shift of ten. The zeros are sign changes of Z between Gram points, refined by bisection on the Euler-Maclaurin engine to a billionth, so the six decimals listed are exact; the count below <code>t</code> is the same scan. <code>psi(x)</code> is exact: the sieve adds <code>ln p</code> at every prime power. The blue curve is the von Mangoldt explicit formula <code>x - sum x^rho / rho - ln 2pi - ln(1 - x^-2) / 2</code> cut off at the chosen zeros, each paired with its mirror; the pink curve keeps none of them. At a jump the full formula lands on the midpoint, and more zeros sharpen every step. The zeros come back to the page as numbers, so both curves are Rust; the page only draws. The same primes are sieved on <a href="../primes">primes</a>.</>}>
      <Row>
        <Slider label="t" value={head} min={0} max={REACH} step={0.05} show={head.toFixed(2)} onChange={(v) => { trace(v); settle(); }} />
        <Slider label="speed" value={pick.speed} min={1} max={20} onChange={(v) => set({ speed: v })} />
        <Btn onClick={play}>{playing ? 'Pause' : 'Play'}</Btn>
        <Btn onClick={shuffle}>Randomize</Btn>
      </Row>
      <Row>
        <Slider label="zeros in the formula" value={pick.zeros} min={0} max={100} onChange={(v) => set({ zeros: v })} />
        <Slider label="x" value={pick.x} min={10} max={1000} onChange={(v) => set({ x: v })} />
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The walk <span>{`t = ${head.toFixed(2)}, ${look.count} ${look.count === 1 ? 'pass' : 'passes'} through the origin`}</span></h2>
          <Sketch draw={walk} deps={[head]} />
        </div>
        <div className="panel">
          <h2>Z(t) <span>the signed distance from the origin, zeros in gold</span></h2>
          <Sketch className="bars" draw={chart} deps={[head]} />
          <pre>{list}</pre>
        </div>
      </div>
      <div className="arena">
        <div className="panel">
          <h2>The prime staircase <span>{`${pick.zeros} of ${ZEROS.length} zeros folded in`}</span></h2>
          <Sketch className="bars" draw={stairs} deps={[fold]} />
        </div>
      </div>
      <Stats>
        <Stat label="re">{fixed(look.re)}</Stat>
        <Stat label="im">{fixed(look.im)}</Stat>
        <Stat label="Z(t)">{fixed(look.z)}</Stat>
        <Stat label="theta">{fixed(look.theta)}</Stat>
        <Stat label="zeros below t">{look.count}</Stat>
        <Stat label="next zero">{next === undefined ? 'past the list' : next.toFixed(6)}</Stat>
        <Stat label="psi(x)">{fold.error ? '' : fold.stair.at(-1).toFixed(4)}</Stat>
        <Stat label="formula">{fold.error ? '' : fold.some.at(-1).toFixed(4)}</Stat>
        <Stat label="gap">{fold.error ? '' : fold.gap.toFixed(4)}</Stat>
      </Stats>
      <Note error={error ?? look.error ?? fold.error} />
    </Page>
  );
}

mount(<App />);
