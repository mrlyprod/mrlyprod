import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Row, Slider, Pick, Check, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { board, line, rules, axis, tag } from '../../lib/chart.js';
import { useQuery } from '../../lib/query.js';

const m = await ready();
const BASES = [2, 3, 4, 5, 6, 7, 8, 9, 10];
const TOP = 64;

const fixed = (value, places) => (Number.isFinite(value) ? value.toFixed(places) : 'none');
const full = (base) => (1 << base) - 1;
const setOf = (mask, base) => Array.from({ length: base }, (_, d) => d).filter((d) => mask & (1 << d));

function span(rows) {
  let low = Infinity, high = -Infinity;
  for (const row of rows) for (const value of row) { if (value < low) low = value; if (value > high) high = value; }
  return [low, high];
}

function Meter({ view }) {
  const draw = (canvas) => {
    const b = board(canvas, 260, { left: 44 });
    const { logx, meter, echo, rest, read, split } = view;
    const drawn = split && read.sieve ? [meter, echo, rest] : [meter];
    const [low, high] = span(drawn);
    const reach = Math.max(high - low, 1e-9);
    const first = logx[0], last = logx[logx.length - 1];
    const wide = Math.max(last - first, 1e-9);
    const step = Math.max(1, Math.round(logx.length / 1400));
    const place = (series) => {
      const out = [];
      for (let i = 0; i < series.length; i += step) out.push([(logx[i] - first) / wide, (series[i] - low) / reach]);
      return out;
    };
    const ticks = [];
    const gap = Math.ceil(read.depth / 8);
    for (let j = 0; j <= read.depth; j += gap) {
      const at = (j * Math.log(read.base) - first) / wide;
      if (at >= 0 && at <= 1) ticks.push([at, `${read.base}^${j}`]);
    }
    axis(b, ticks, { wall: true });
    b.ctx.strokeStyle = ink.line;
    b.ctx.lineWidth = 1;
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(0), b.y(-low / reach));
    b.ctx.lineTo(b.x(1), b.y(-low / reach));
    b.ctx.stroke();
    if (drawn.length > 1) {
      line(b, place(echo), ink.blue, { width: 1.2 });
      line(b, place(rest), ink.pink, { width: 1.2 });
    }
    line(b, place(meter), ink.yellow, { width: 1.4 });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    b.ctx.fillText(fixed(high, 3), b.left - 6, b.y(1) + 4);
    b.ctx.fillText(fixed(low, 3), b.left - 6, b.y(0) + 4);
    b.ctx.textAlign = 'left';
    tag(b, 'the meter', ink.yellow);
    if (drawn.length > 1) {
      tag(b, 'the echo', ink.blue, 'left', b.x(0) + 76);
      tag(b, 'the residual', ink.pink, 'left', b.x(0) + 152);
    }
    tag(b, read.sieve ? '' : 'past the sieve cap, no echo here', ink.dim, 'right');
  };
  return <Sketch className="bars" draw={draw} deps={[view]} role="img" aria-label="The scaled design Mobius meter against the log of x" />;
}

function Spectrum({ view }) {
  const draw = (canvas) => {
    const b = board(canvas, 260, { left: 44 });
    const { gamma, score, read } = view;
    const at = (value) => value / TOP;
    let peak = 10;
    for (let i = 0; i < gamma.length; i += 1) if (gamma[i] > read.band[0] && gamma[i] < read.band[1] && score[i] > peak) peak = score[i];
    const roof = Math.log10(peak);
    const lift = (value) => Math.log10(Math.max(value, 1)) / roof;
    rules(b, read.lattice.filter((g) => g < TOP).map(at), { color: ink.pink, dash: [3, 4] });
    rules(b, read.zeros.filter((g) => g < TOP).map(at), { color: ink.yellow });
    const points = [];
    for (let i = 0; i < gamma.length && gamma[i] <= TOP; i += 1) points.push([at(gamma[i]), lift(score[i])]);
    axis(b, [0, 10, 20, 30, 40, 50, 60].map((g) => [at(g), String(g)]), { wall: true });
    line(b, [[0, lift(read.threshold)], [1, lift(read.threshold)]], ink.line, { width: 1, dash: [2, 4] });
    line(b, points, ink.blue, { width: 1.2 });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    b.ctx.fillText(fixed(peak, 1), b.left - 6, b.y(1) + 4);
    b.ctx.fillText('1', b.left - 6, b.y(0) + 4);
    b.ctx.textAlign = 'left';
    tag(b, 'the zeta ordinates', ink.yellow);
    tag(b, `the pole lattice 2 pi j / log ${read.base}`, ink.pink, 'right');
  };
  return <Sketch className="bars" draw={draw} deps={[view]} role="img" aria-label="The power spectrum of the meter against the zeta ordinates and the pole lattice" />;
}

function App() {
  const [pick, set] = useQuery({ base: 10, mask: 511, depth: 5, split: true, sub: false });

  let error = null;
  let view = null;
  let caps = null;
  let depth = pick.depth;
  try {
    caps = JSON.parse(m.echo_caps(pick.base, pick.mask));
    depth = Math.min(Math.max(pick.depth, caps.least), caps.deepest);
    const got = m.echo_read(pick.base, pick.mask, depth, pick.sub);
    view = {
      logx: got.logx,
      meter: got.meter,
      echo: got.echo,
      rest: got.rest,
      gamma: got.gamma,
      score: got.score,
      read: JSON.parse(got.read),
      split: pick.split,
    };
  } catch (fault) {
    error = fault;
  }

  const read = view?.read;
  const digits = setOf(pick.mask, pick.base);
  const toggle = (d) => {
    const next = pick.mask ^ (1 << d);
    if (setOf(next, pick.base).filter((v) => v > 0).length < 1 || setOf(next, pick.base).length < 2) return;
    set({ mask: next });
  };
  const rebase = (base) => {
    const kept = pick.mask & full(base);
    const ok = setOf(kept, base).length >= 2 && setOf(kept, base).some((d) => d > 0);
    set({ base, mask: ok ? kept : full(base) });
  };

  const controls = (
    <>
      <section>
        <h3>The design</h3>
        <Row>
          <Pick label="base q" value={pick.base} options={BASES.map((q) => [q, q])} onChange={(v) => rebase(+v)} />
          <Btn on={pick.mask === full(pick.base)} onClick={() => set({ mask: full(pick.base) })}>full set</Btn>
        </Row>
        <Row>
          {Array.from({ length: pick.base }, (_, d) => (
            <Check key={d} label={String(d)} checked={(pick.mask & (1 << d)) !== 0} onChange={() => toggle(d)} />
          ))}
        </Row>
      </section>
      <section>
        <h3>The depth</h3>
        <Row>
          <Slider label="digits L" value={depth} min={caps?.least ?? 3} max={caps?.deepest ?? 3} onChange={(v) => set({ depth: v })} />
        </Row>
      </section>
      <section>
        <h3>The split</h3>
        <Row>
          <Check label="echo and residual" checked={pick.split} onChange={(v) => set({ split: v })} />
          <Check label="subtract the echo" checked={pick.sub} onChange={(v) => set({ sub: v })} />
        </Row>
      </section>
    </>
  );

  return (
    <Page crumb="echo" title="The meter that echoes the zeros"
      sub="The Mobius meter of a digit design, read uniformly in log x, oscillates at the ordinates of the Riemann zeta zeros and never at the design's own pole lattice. It is the classical Mertens function heard through the design's density: split the meter into that echo and a residual, and the zeros leave with the echo. The echo dies against the meter's own yardstick at x to the minus half of the smaller of alpha and one less alpha, so the deeper the read the quieter it gets."
      controls={controls}
      foot={<>Every number here is computed in Rust and the page only draws. The elements of <code>S_F</code> are the whole numbers whose base-<code>q</code> digits all lie in the set, the meter is <code>M_F(x)</code>, the sum of <code>mu(n)</code> over those elements up to <code>x</code>, and the yardstick is <code>x^(alpha/2)</code> at <code>alpha = log_q k</code>. The spectrum is that series resampled on 4096 points uniform in <code>log x</code>, mean-removed, Hann-windowed and read as <code>gamma = 2 pi j</code> over the log range against a 101-bin running median floor, so a peak is a power over its own neighbourhood and the threshold is 8. Resolution comes from the log range and not from the element count, so the bin is <code>2 pi / (L log q)</code> and a browser that stops at <code>q^L &lt; 2^27</code> stops at a bin near a third. Read the hit counts against the chance rates beside them: with 13 ordinates and 20 lattice lines in the band a peak lands on one by luck often enough that a single peak proves nothing, which is why the full set is here as the control and the residual as the null. The echo needs the Mobius values of every whole number up to <code>q^L</code>, so it is refused past <code>2^24</code> and the page says so. The lab reads the same designs four digits deeper, where base 3 <code>{'{0, 1}'}</code> carries ten peaks and its pole lattice scores below its own null; at the depth a browser affords, that design is still under the floor and the base-10 designs are not. The critical line itself is <a href="../zeta">zeta</a>, the Mertens sum against the square root is one dial of <a href="../formulas">formulas</a>, and the mathematics is on <a href="/research/mobius/">the Mobius page</a>.</>}>
      <div className="panel">
        <h2>The meter <span>M_F(x) over x^(alpha/2), drawn against log x</span></h2>
        {view && <Meter view={view} />}
        <Stats>
          <Stat label="alpha">{read && fixed(read.alpha, 6)}</Stat>
          <Stat label="digits">{`{${digits.join(', ')}}`}</Stat>
          <Stat label="elements A_F">{read?.count}</Stat>
          <Stat label="M_F">{read?.last}</Stat>
          <Stat label="max |M_F|">{read?.peak}</Stat>
          <Stat label="thetamax">{read && fixed(read.theta, 4)}</Stat>
          <Stat label="echo share">{read && (read.share === null ? 'past the cap' : fixed(read.share, 4))}</Stat>
          <Stat label="residual share">{read && (read.residual === null ? 'past the cap' : fixed(read.residual, 4))}</Stat>
          <Stat label="decay rate">{read && `x^${fixed(read.rate, 6)}`}</Stat>
        </Stats>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The spectrum <span>{pick.sub ? 'the residual, echo taken out' : 'the meter'} over its local median floor, yellow the zeta ordinates, pink the pole lattice</span></h2>
        {view && <Spectrum view={view} />}
        <Stats>
          <Stat label="log range">{read && fixed(read.span, 4)}</Stat>
          <Stat label="bin">{read && fixed(read.bin, 4)}</Stat>
          <Stat label="peaks over 8">{read?.found}</Stat>
          <Stat label="at a zeta zero">{read && `${read.hits} of ${read.peaks.length}`}</Stat>
          <Stat label="by chance">{read && fixed(read.chance, 3)}</Stat>
          <Stat label="at the lattice">{read && `${read.lines} of ${read.peaks.length}`}</Stat>
          <Stat label="by chance">{read && fixed(read.chanceLines, 3)}</Stat>
        </Stats>
        <div className="ribbon tight">
          {read?.peaks.map((row) => (
            <span key={row.gamma}>
              <i>{fixed(row.gamma, 3)}</i>
              <b className={row.zeta <= read.bin ? 'yellow' : undefined}>{fixed(row.score, 1)}</b>
              <i>{`zeta ${fixed(row.zeta, 3)}`}</i>
              <i>{`lattice ${fixed(row.lattice, 3)}`}</i>
            </span>
          ))}
          {read?.peaks.length === 0 && <span><i>no peak clears the floor at this depth</i></span>}
        </div>
      </div>
      <Note error={error}>{caps && `depth ${caps.least} to ${caps.deepest} here; the echo is sieved through depth ${caps.sieved}, past which q^L leaves 2^24.`}</Note>
    </Page>
  );
}

mount(<App />);
