import { useMemo } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { board, axis, rules, tag } from '../../../lib/chart.js';
import { mount, Page, Group, Pick, Slider, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { useQuery } from '../../../lib/query.js';

const m = await ready();
const LOW = 6;
const HIGH = 16;
const CELLS = 720;
const LEAST = 48;
const STEPS = 1000;
const FLOOR = 0.7;
const FIRST = { level: 16, x: 14348906, focus: 12766858, zoom: 16, three: 15, four: 12 };

const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, Math.round(v)));
const whole = (n) => n.toLocaleString('en');
const down = (v) => (Math.floor(v * 1e6) / 1e6).toFixed(6);
const up = (v) => (Math.ceil(v * 1e6) / 1e6).toFixed(6);
const key = (row) => `${row.three},${row.four}`;
const kind = (row) => (row.clean ? 'clean' : row.copy ? 'gap copy' : 'mixed');

const attempt = (fn) => {
  try {
    return { value: fn(), error: null };
  } catch (error) {
    return { value: null, error };
  }
};

function paint(b, fills) {
  const { ctx } = b;
  const n = fills.length;
  const step = b.wide / n;
  const gap = step >= 6 ? 1 : step >= 3 ? 0.5 : 0;
  ctx.fillStyle = ink.deep;
  ctx.fillRect(b.x(0), b.roof, b.wide, b.tall);
  ctx.fillStyle = ink.blue;
  fills.forEach((f, i) => {
    if (!f) return;
    ctx.globalAlpha = f;
    ctx.fillRect(b.x(i / n) + gap / 2, b.roof, Math.max(step - gap, 0.5), b.tall);
  });
  ctx.globalAlpha = 1;
}

function App() {
  const [pick, set] = useQuery(FIRST);
  const level = clamp(pick.level, LOW, HIGH);
  const top = 3 ** level;
  const x = clamp(pick.x, 1, top);
  const zmax = Math.max(0, Math.floor(Math.log2((x + 1) / LEAST)));
  const zoom = clamp(pick.zoom, 0, zmax);
  const span = Math.max(Math.min(LEAST, x + 1), Math.floor((x + 1) / 2 ** zoom));
  const focus = clamp(pick.focus, 0, x);
  const lo = clamp(focus - Math.floor(span / 2), 0, x + 1 - span);
  const hi = lo + span;

  const read = useMemo(() => attempt(() => JSON.parse(m.sumset_read(level, x))), [level, x]);
  const envelope = useMemo(() => attempt(() => m.sumset_envelope(level, CELLS)), [level]);
  const census = useMemo(() => attempt(() => JSON.parse(m.sumset_pairs(level))), [level]);
  const pairs = census.value ?? [];
  const chosen = pairs.find((row) => row.three === pick.three && row.four === pick.four);
  const here = read.value;
  const gaps = pairs.filter((row) => row.gap);

  const choose = (row) => set({ three: row.three, four: row.four, x: row.gap ? row.gap[1] : row.largest, focus: row.largest });
  const logx = (f) => clamp(Math.exp(Math.max(0, Math.min(1, f)) * Math.log(top)), 1, top);

  const overview = (canvas) => {
    const b = board(canvas, 64, { top: 10, bottom: 10 });
    paint(b, m.sumset_strip(level, 0, x + 1, Math.max(1, Math.min(x + 1, Math.floor(b.wide)))));
    const a = b.x(lo / (x + 1)), c = b.x(hi / (x + 1));
    b.ctx.strokeStyle = ink.yellow;
    b.ctx.lineWidth = 2;
    b.ctx.strokeRect(a - 1, b.roof - 5, Math.max(2, c - a) + 2, b.tall + 10);
  };

  const closeup = (canvas) => {
    const b = board(canvas, 76, { top: 8, bottom: 24 });
    paint(b, m.sumset_strip(level, lo, hi, Math.max(1, Math.min(span, Math.floor(b.wide)))));
    axis(b, [[0, whole(lo)], [1, whole(hi - 1)]]);
  };

  const density = (canvas) => {
    const b = board(canvas, 320, { top: 24, bottom: 24 });
    const { ctx } = b;
    const fx = (v) => Math.log(v) / Math.log(top);
    const fy = (v) => (v - FLOOR) / (1 - FLOOR);
    for (const row of gaps) {
      const a = b.x(fx(row.gap[0])), c = b.x(fx(row.gap[1] + 1));
      ctx.fillStyle = row === chosen ? ink.yellow : ink.line;
      ctx.globalAlpha = row === chosen ? 0.35 : 1;
      ctx.fillRect(a, b.roof, Math.max(1.5, c - a), b.tall);
    }
    ctx.globalAlpha = 1;
    ctx.strokeStyle = ink.line;
    ctx.fillStyle = ink.dim;
    ctx.textAlign = 'right';
    for (const v of [0.8, 0.9, 1]) {
      ctx.beginPath();
      ctx.moveTo(b.x(0), b.y(fy(v)));
      ctx.lineTo(b.x(1), b.y(fy(v)));
      ctx.stroke();
      ctx.fillText(v.toFixed(1), b.x(1) - 2, b.y(fy(v)) - 3);
    }
    ctx.textAlign = 'left';
    const band = envelope.value;
    if (band) {
      const w = b.wide / CELLS;
      ctx.fillStyle = ink.blue;
      for (let i = 0; i < CELLS; i++) {
        const low = band[2 * i], high = band[2 * i + 1];
        if (Number.isNaN(low)) continue;
        const y0 = b.y(fy(high)), y1 = b.y(fy(Math.max(low, FLOOR)));
        ctx.fillRect(b.x(i / CELLS), y0, Math.max(1, w + 0.3), Math.max(1.5, y1 - y0));
      }
    }
    rules(b, [fx(x)], { color: ink.pink, width: 1.5 });
    if (here) {
      ctx.fillStyle = ink.pink;
      ctx.beginPath();
      ctx.arc(b.x(fx(x)), b.y(fy(Math.max(here.density, FLOOR))), 4, 0, Math.PI * 2);
      ctx.fill();
    }
    const marks = [];
    for (let k = 4; k < level; k += 4) marks.push([k / level, `3^${k}`]);
    axis(b, [[0, '1'], ...marks, [1, `3^${level}`]]);
    const edge = tag(b, 'D(x), least to greatest over each pixel', ink.blue);
    tag(b, 'gaps', ink.dim, 'left', edge + 14);
  };

  const energy = (canvas) => {
    const b = board(canvas, 240, { top: 24, bottom: 24 });
    const { ctx } = b;
    const n = pairs.length;
    if (!n) return;
    const ceiling = Math.ceil(Math.max(...pairs.map((row) => row.ratio)) * 4 + 1) / 4;
    const fx = (i) => (i + 0.5) / n;
    const fy = (q) => (q - 1) / (ceiling - 1);
    ctx.strokeStyle = ink.line;
    ctx.fillStyle = ink.dim;
    for (let q = 1; q <= ceiling; q += 0.5) {
      ctx.beginPath();
      ctx.moveTo(b.x(0), b.y(fy(q)));
      ctx.lineTo(b.x(1), b.y(fy(q)));
      ctx.stroke();
      if (q < ceiling) ctx.fillText(q.toFixed(1), b.x(0), b.y(fy(q)) - 3);
    }
    pairs.forEach((row, i) => {
      const px = b.x(fx(i)), py = b.y(fy(row.ratio));
      ctx.beginPath();
      ctx.arc(px, py, row === chosen ? 6 : 4.5, 0, Math.PI * 2);
      if (row.copy) {
        ctx.strokeStyle = ink.dim;
        ctx.lineWidth = 1.5;
        ctx.stroke();
      } else {
        ctx.fillStyle = row.clean ? ink.yellow : ink.blue;
        ctx.fill();
      }
      if (row === chosen) {
        ctx.strokeStyle = ink.fg;
        ctx.lineWidth = 1.5;
        ctx.beginPath();
        ctx.arc(px, py, 9, 0, Math.PI * 2);
        ctx.stroke();
      }
    });
    axis(b, [[fx(0), `k = ${pairs[0].three}`], [fx(n - 1), `k = ${pairs[n - 1].three}`]]);
    const one = tag(b, 'clean', ink.yellow);
    const two = tag(b, 'mixed', ink.blue, 'left', one + 12);
    tag(b, 'gap copy', ink.dim, 'left', two + 12);
  };

  const controls = (
    <>
      <Group name="Height">
        <Slider label="level" value={level} min={LOW} max={HIGH} show={`3^${level}`} onChange={(v) => set({ level: v, x: Math.min(x, 3 ** v), focus: Math.min(focus, 3 ** v) })} />
      </Group>
      <Group name="Cursor">
        <Slider label="x" value={Math.round((STEPS * Math.log(x)) / Math.log(top))} min={0} max={STEPS} show={whole(x)} onChange={(v) => set({ x: logx(v / STEPS) })} />
        <Pick label="gap" value={chosen?.gap ? key(chosen) : ''} options={[['', 'jump to a gap'], ...gaps.map((row) => [key(row), `(${row.three}, ${row.four}) at ${row.scale.toFixed(3)}`])]}
          onChange={(v) => { const row = gaps.find((r) => key(r) === v); if (row) choose(row); }} />
      </Group>
      <Group name="Zoom">
        <Slider label="zoom" value={zoom} min={0} max={zmax} show={`${whole(span)} integers`} onChange={(v) => set({ zoom: v })} />
      </Group>
    </>
  );

  return (
    <Page crumb="sumset" title="Three plus four: Erdos problem 125"
      sub={<>Add a number whose base-3 digits are all <code>0</code> or <code>1</code> to one whose base-4 digits are. Just below each near meeting of a power of 3 and a power of 4 the sums <code>S</code> miss a whole run of integers, so the share <code>D(x)</code> of lit integers up to <code>x</code> dips, and along ever closer meetings its lower limit is 0. Whether it returns above one fixed share at arbitrarily large <code>x</code> is open. Jump to a gap, zoom into the strip and watch.</>}
      controls={controls}
      foot={<>The strip is one bit per integer up to <code>3^{level}</code>, built in Rust: the members of <code>A</code> set directly, then each power of 4 folded in by one shift-or pass. Each strip pixel is shaded by the share of lit integers it covers, and the density chart draws the least and the greatest <code>D(x)</code> over each pixel of <code>log x</code>, so every dip shows at its true depth. The energy <code>E(k, m)</code> is summed over the <code>3^m</code> digit strings a difference in <code>B_m</code> can be, each weighted by <code>2</code> to the zero digits it has in base 4 and in balanced ternary. Every count, density, gap and energy comes out of the crates through wasm; the page only draws. The proofs and the census to <code>3^22</code> are on <a href="/research/cobham/">two bases</a>, section Object S.</>}>
      <p className="banner"><span className="chip conjecture">Open</span> Is the upper density of <code>S</code> positive: does <code>D(x)</code> return above one fixed share at arbitrarily large <code>x</code>?</p>
      <p className="sub"><span className="chip proved">Proved</span> No sum lands in the gap <code>(d, min(3^k, 4^m))</code>, <code>d = (3^k - 1)/2 + (4^m - 1)/3</code> the largest sum of <code>A_k + B_m</code>. Where <code>4^m/3^k</code> is near 1 that gap is a sixth of the scale, and iterated along ever closer coincidences it drives the lower density to 0, the answer to the question Erdos asked, checked in Lean on the problem page.</p>
      <div className="panel">
        <h2>S up to x <span>{`${whole(x + 1)} integers from 0, the yellow box the window below`}</span></h2>
        <Sketch draw={overview} deps={[level, x, lo, hi]} onSeek={(f) => set({ focus: clamp(f * x, 0, x) })} role="img" aria-label="The sumset up to x as a strip, each pixel shaded by the share of members" />
        <h2>the window <span>{`${whole(lo)} to ${whole(hi - 1)}, one cell an integer once they fit`}</span></h2>
        <Sketch draw={closeup} deps={[level, lo, hi]} role="img" aria-label="A window of the sumset, lit integers in blue" />
        {here && (
          <Stats>
            <Stat label="x">{whole(x)}</Stat>
            <Stat label="card(S meet [1, x])">{whole(here.count)}</Stat>
            <Stat label="D(x)">{down(here.density)}</Stat>
            <Stat label="x in S">{here.member ? 'yes' : 'no'}</Stat>
          </Stats>
        )}
        <Note error={read.error ?? envelope.error ?? census.error} />
      </div>
      <div className="arena">
        <div className="panel">
          <h2>the density <span>{`D(x) on a log scale from 1 to 3^${level}`}</span></h2>
          <Sketch draw={density} deps={[level, x, envelope.value, chosen]} onSeek={(f) => set({ x: logx(f) })} role="img" aria-label="The density of the sumset against log x, dipping at every shaded gap" />
          <p className="sub">Each grey band is a gap <code>(d, min(3^k, 4^m))</code>, open when <code>4^m/3^k</code> lies between about <code>3/4</code> and <code>3/2</code>. <code>D</code> falls through the band and bottoms out just below <code>min(3^k, 4^m)</code>; what it does after the last band drawn is the open question. Click to move <code>x</code>.</p>
        </div>
        <div className="panel">
          <h2>the energy ratio <span>{`Q(k, m) at ${pairs.length} pairs from k = 6`}</span></h2>
          <Sketch draw={energy} deps={[pairs, chosen]} onSeek={(f) => pairs.length && choose(pairs[clamp(f * pairs.length - 0.5, 0, pairs.length - 1)])} role="img" aria-label="The energy ratio Q at every pair of levels" />
          <p className="sub"><code>Q(k, m) = E(k, m) (d + 1)/4^(k+m)</code> weighs how often two sums of <code>A_k + B_m</code> collide against a flat spread, and Cauchy-Schwarz gives <code>card(A_k + B_m) &gt;= (d + 1)/Q</code>. <span className="chip proved">Proved</span> <code>Q</code> is unbounded, yet for every <code>eps &gt; 0</code> it is below <code>3^(eps k)</code> at every large <code>k</code> and every <code>m</code> with <code>1/3 &lt;= 4^m/3^k &lt; 4</code>, the window every pair here sits in. <span className="chip conjecture">Conjecture</span> <code>Q</code> stays bounded along one infinite chain of pairs, which would put the upper density at least <code>1/Q</code>. Click a dot to pick its pair.</p>
          {chosen && (
            <Stats>
              <Stat label="pair">{`(${chosen.three}, ${chosen.four}), ${kind(chosen)}`}</Stat>
              <Stat label="4^m/3^k">{up(chosen.scale)}</Stat>
              <Stat label="d">{whole(chosen.largest)}</Stat>
              <Stat label="gap">{chosen.gap ? `${whole(chosen.gap[0])} to ${whole(chosen.gap[1])}` : 'none'}</Stat>
              <Stat label="E">{chosen.energy}</Stat>
              <Stat label="Q">{up(chosen.ratio)}</Stat>
              <Stat label="1/Q">{down(chosen.bound)}</Stat>
              <Stat label="fill to d">{down(chosen.fill)}</Stat>
            </Stats>
          )}
        </div>
      </div>
    </Page>
  );
}

mount(<App />);
