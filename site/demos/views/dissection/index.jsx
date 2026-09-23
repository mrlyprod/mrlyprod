import { useMemo } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { mount, Page, Row, Slider, Pick, Btn, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { board, line, rules, axis, tag } from '../../../lib/chart.js';
import { useQuery } from '../../../lib/query.js';

const m = await ready();
const CHAIN = m.dissection_chain();
const FIRST = 3;
const ROOF = 0.75;
const FRIENDS = [[10, 7], [10, 5], [10, 0], [3, 1], [17, 8], [33, 16]];
const NAMES = ['A', 'B', 'C1', 'C2'];
const LANES = [3, 2, 1, 0];
const hues = () => [ink.blue, ink.dim, ink.orange, ink.yellow];

const fixed = (value, places) => (Number.isFinite(value) ? value.toFixed(places) : 'none');
const clamp = (value, low, high) => Math.min(Math.max(value, low), high);

function logTicks(b, read) {
  const first = read.logx[0], last = read.logx[read.logx.length - 1];
  const wide = Math.max(last - first, 1e-9);
  const ticks = [];
  for (let j = 1; j <= read.level; j += 1) {
    const at = (j * Math.log(read.base) - first) / wide;
    if (at >= 0 && at <= 1.0001) ticks.push([Math.min(at, 1), `${read.base}^${j}`]);
  }
  axis(b, ticks, { wall: true });
  return (i) => (read.logx[i] - first) / wide;
}

function Meter({ tally, by }) {
  const draw = (canvas) => {
    const b = board(canvas, 240, { left: 44 });
    const series = by === 'mass' ? tally.mass : tally.root;
    const reach = by === 'mass' ? 1 : Math.max(1.25, tally.read.peak * 1.1);
    const at = logTicks(b, tally);
    const lift = (v) => 0.5 + v / (2 * reach);
    line(b, [[0, lift(1)], [1, lift(1)]], ink.line, { width: 1, dash: [3, 4] });
    line(b, [[0, lift(-1)], [1, lift(-1)]], ink.line, { width: 1, dash: [3, 4] });
    line(b, [[0, 0.5], [1, 0.5]], ink.line, { width: 1 });
    line(b, Array.from(series, (v, i) => [at(i), lift(v)]), ink.yellow, { width: 1.4 });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    b.ctx.fillText(fixed(reach, 2), b.left - 6, b.y(1) + 4);
    b.ctx.fillText(fixed(-reach, 2), b.left - 6, b.y(0) + 4);
    b.ctx.textAlign = 'left';
    tag(b, by === 'mass' ? 'M_F(x) / A_F(x)' : 'M_F(x) / A_F(x)^(1/2)', ink.yellow);
    tag(b, 'dashed: plus and minus one', ink.dim, 'right');
  };
  return <Sketch className="bars" draw={draw} deps={[tally, by]} role="img" aria-label="The meter of the set over its yardstick against log x" />;
}

function Primes({ tally }) {
  const draw = (canvas) => {
    const b = board(canvas, 200, { left: 44 });
    let high = 1.5;
    for (const v of tally.primes) if (v > high) high = v;
    const at = logTicks(b, tally);
    const lift = (v) => v / high;
    line(b, [[0, lift(1)], [1, lift(1)]], ink.line, { width: 1, dash: [3, 4] });
    line(b, Array.from(tally.primes, (v, i) => [at(i), lift(v)]), ink.green, { width: 1.4 });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    b.ctx.fillText(fixed(high, 2), b.left - 6, b.y(1) + 4);
    b.ctx.fillText('1', b.left - 6, b.y(lift(1)) + 4);
    b.ctx.fillText('0', b.left - 6, b.y(0) + 4);
    b.ctx.textAlign = 'left';
    tag(b, 'psi_F(x) / (kappa_F A_F(x))', ink.green);
  };
  return <Sketch className="bars" draw={draw} deps={[tally]} role="img" aria-label="The prime count of the set over its main term against log x" />;
}

function Lanes({ grid }) {
  const draw = (canvas) => {
    const b = board(canvas, 250, { left: 34 });
    const inks = hues();
    const { region, weight } = grid;
    const y = region.length;
    const lane = b.tall / 4;
    const decades = 4;
    const tall = (w) => Math.max(1.5, (lane - 6) * clamp(1 + Math.log10(Math.max(w, 1e-12)) / decades, 0, 1));
    const cols = Math.max(1, Math.floor(b.wide));
    const sparse = y * 3 <= cols;
    const peak = LANES.map(() => new Float32Array(sparse ? y : cols).fill(-1));
    for (let a = 0; a < y; a += 1) {
      const slot = sparse ? a : Math.min(cols - 1, Math.floor((a / y) * cols));
      const row = peak[region[a]];
      if (weight[a] > row[slot]) row[slot] = weight[a];
    }
    LANES.forEach((r, k) => {
      const floor = b.roof + lane * (k + 1);
      b.ctx.fillStyle = ink.line;
      b.ctx.fillRect(b.left, floor - 0.5, b.wide, 1);
      b.ctx.fillStyle = inks[r];
      const row = peak[r];
      const step = b.wide / row.length;
      for (let s = 0; s < row.length; s += 1) {
        if (row[s] < 0) continue;
        const h = tall(row[s]);
        const x = b.left + s * step;
        b.ctx.fillRect(x, floor - h, sparse ? Math.max(1, step - 1) : Math.max(1, step), h);
      }
      b.ctx.fillStyle = inks[r];
      b.ctx.textAlign = 'right';
      b.ctx.fillText(NAMES[r], b.left - 8, floor - lane / 2 + 4);
      b.ctx.textAlign = 'left';
    });
    axis(b, [[0, '0'], [0.25, '1/4'], [0.5, '1/2'], [0.75, '3/4'], [1, '1']]);
    tag(b, `a / ${grid.read.base}^${grid.read.level}, bar height |hat F(a/y)| / fill^level on four decades`, ink.dim);
  };
  return <Sketch className="bars" draw={draw} deps={[grid]} role="img" aria-label="The frequency grid cut into the regions A, B, C1 and C2, each frequency weighed by the digit transform" />;
}

function Wall({ read }) {
  const draw = (canvas) => {
    const b = board(canvas, 260, { left: 44 });
    const low = Math.log(FIRST), high = Math.log(FIRST + CHAIN.length - 1);
    const across = (q) => (Math.log(q) - low) / (high - low);
    const lift = (v) => clamp(v / ROOF, 0, 1);
    const [barA, barB] = read.bars;
    const walls = [[read.walls.digit, 'digit'], [read.walls.window, 'window'], [read.walls.chain, 'chain']];
    rules(b, walls.map(([q]) => across(q)), { color: ink.line, dash: [2, 4] });
    line(b, [[0, lift(barA)], [1, lift(barA)]], ink.pink, { width: 1.2 });
    line(b, [[0, lift(barB)], [1, lift(barB)]], ink.pink, { width: 1, dash: [4, 4] });
    const curve = [];
    for (let i = 0; i < CHAIN.length; i += 1) curve.push([across(FIRST + i), lift(CHAIN[i])]);
    line(b, curve, ink.blue, { width: 1.6 });
    line(b, [[across(read.base), lift(read.chain)]], ink.blue, { dots: 5 });
    line(b, [[across(read.base), lift(read.reading)]], ink.yellow, { dots: 5 });
    axis(b, [[0, String(FIRST)], [across(10), '10'], [across(100), '100'], [across(1000), '1000'], [1, String(FIRST + CHAIN.length - 1)]], { wall: true });
    b.ctx.fillStyle = ink.dim;
    b.ctx.textAlign = 'right';
    for (const v of [barA, barB, 0.5, ROOF]) b.ctx.fillText(fixed(v, 2), b.left - 6, b.y(lift(v)) + 4);
    b.ctx.textAlign = 'center';
    for (const [q, name] of walls) b.ctx.fillText(`${name} ${q}`, b.x(across(q)), b.roof - 4 + (name === 'window' ? -10 : 0));
    b.ctx.textAlign = 'left';
    tag(b, 'the chain alpha_1 by base', ink.blue, 'left', b.x(0), b.roof + 14);
  };
  return <Sketch className="bars" draw={draw} deps={[read]} role="img" aria-label="The chain certificate exponent against the base, the bars one fifth and one quarter, and the walls" />;
}

function reachLine(read) {
  const { walls, base, digit } = read;
  const where = {
    proof: `Base ${base} missing ${digit} is reached by proof: the chain clears 1/5 at every base from ${walls.chain}.`,
    certificate: `Base ${base} missing ${digit} is reached by certificate: a verified shifted-grid bound below 1/5 holds for this set, short of the chain's proof from ${walls.chain}.`,
    none: `Base ${base} missing ${digit} is not reached: no certificate below 1/5 is known for this set, so the theorem says nothing here and the curves above are readings.`,
  }[read.reach];
  return `${where} The chain proves every base from ${walls.chain}; window certificates cover ${walls.window} to ${walls.chain - 1} and per-digit certificates every set from ${walls.digit}, and base ${walls.first} missing 0 is the first set certified below 1/5.`;
}

function App() {
  const [pick, set] = useQuery({ base: 10, digit: 7, depth: 6, level: 3, z: 8, by: 'mass' });
  const digit = Math.min(pick.digit, pick.base - 1);

  let error = null;
  const read = useMemo(() => {
    try {
      return JSON.parse(m.dissection_read(pick.base, digit));
    } catch (fault) {
      return { fault };
    }
  }, [pick.base, digit]);
  if (read.fault) error = read.fault;
  const depth = read.depths ? clamp(pick.depth, read.depths[0], read.depths[1]) : pick.depth;
  const level = read.grids ? clamp(pick.level, read.grids[0], read.grids[1]) : pick.level;

  const tally = useMemo(() => {
    if (read.fault) return null;
    try {
      const got = m.dissection_tally(pick.base, digit, depth);
      return { logx: got.logx, mass: got.mass, root: got.root, primes: got.primes, read: JSON.parse(got.read), base: pick.base, level: depth };
    } catch (fault) {
      return { fault };
    }
  }, [pick.base, digit, depth, read]);
  const grid = useMemo(() => {
    if (read.fault) return null;
    try {
      const got = m.dissection_grid(pick.base, digit, level, pick.z);
      return { region: got.region, weight: got.weight, read: { ...JSON.parse(got.read), base: pick.base, level } };
    } catch (fault) {
      return { fault };
    }
  }, [pick.base, digit, level, pick.z, read]);
  error = error ?? tally?.fault ?? grid?.fault ?? null;
  const good = !error;

  const controls = (
    <>
      <section>
        <h3>The set</h3>
        <Row>
          <Slider label="base" value={pick.base} min={3} max={128} onChange={(v) => set({ base: v, digit: Math.min(digit, v - 1) })} />
          <Slider label="missing" value={digit} min={0} max={pick.base - 1} onChange={(v) => set({ digit: v })} />
        </Row>
        <Row>
          {FRIENDS.map(([q, e]) => (
            <Btn key={`${q}-${e}`} on={pick.base === q && digit === e} onClick={() => set({ base: q, digit: e })}>{`${q} less ${e}`}</Btn>
          ))}
        </Row>
      </section>
      <section>
        <h3>The meter</h3>
        <Row>
          <Slider label="digits of x" value={depth} min={read.depths?.[0] ?? 3} max={read.depths?.[1] ?? 3} onChange={(v) => set({ depth: v })} />
          <Pick label="against" value={pick.by} options={[['mass', 'A_F(x)'], ['root', 'A_F(x)^(1/2)']]} onChange={(v) => set({ by: v })} />
        </Row>
      </section>
      <section>
        <h3>The grid</h3>
        <Row>
          <Slider label="level" value={level} min={read.grids?.[0] ?? 1} max={read.grids?.[1] ?? 1} onChange={(v) => set({ level: v })} />
          <Slider label="Z" value={pick.z} min={2} max={64} onChange={(v) => set({ z: v })} />
        </Row>
      </section>
    </>
  );

  const t = tally?.read;
  const g = grid?.read;
  return (
    <Page crumb="dissection" title="The dissection and its wall"
      sub="Keep the positive integers whose digits in base q avoid one digit. Their Mobius sum M_F(x) is o(A_F(x)), with no hypothesis, once the digit transform's l^1 exponent alpha_1 sits below 1/5, and the prime count follows its main term kappa_F A_F(x). That happens from base 584 by proof. Pick a small base, where everything fits in a browser, and see what the theorem is about and whether it reaches the set you picked."
      controls={controls}
      foot={<>Every number here is computed in Rust by <code>mrlyrs::num::dissection</code> and the page only draws. The set <code>S_F</code> holds the integers whose base-<code>q</code> digits all lie in <code>F</code>, the base less one digit; <code>A_F(x)</code> counts it up to <code>x</code>, <code>M_F(x)</code> sums <code>mu</code> over it and <code>psi_F(x)</code> sums the von Mangoldt weight <code>log p</code> over its prime powers, against <code>kappa_F = (q/phi(q)) #&#123;f in F : gcd(f, q) = 1&#125;/fill</code>. The grid is the <code>y = q^level</code> frequencies <code>a/y</code>, each given its last continued-fraction convergent <code>l/d</code> with <code>d &lt;= Q = y^(3/5)</code> and its height <code>h = |ad - ly|</code>: region A is <code>d &gt;= y^(2/5)</code>, the minor arcs; C1 and C2 have <code>d &lt; Z</code> and <code>h &lt; Z</code>, C2 when <code>d</code> divides a power of <code>q</code>; B is the rest. The cut never sees the digits; the bar heights, <code>|hat F_level(a/y)|</code>, are what each region is paid against. Region A pays the whole <code>l^1</code> mass against the minor-arc bound <code>x^(4/5)</code>, which is why the bar is <code>1/5</code>; region B asks only <code>1/4</code>. The blue curve is the chain certificate, one number per base good at every missing digit; the yellow dot is the reading <code>log_q(c_j/(fill c_(j-1)))</code> of the unshifted masses at the deepest level inside <code>2^21</code> frequencies, a reading and never a bound. The mathematics is <a href="/research/mobius/">the Mobius page</a>, section The unconditional dissection, and <a href="/papers/unconditional-mertens-at-large-base/">the paper</a>.</>}>
      <div className="panel">
        <h2>The meter <span>{pick.by === 'mass' ? (read.reach === 'none' ? 'M_F(x) over its mass A_F(x): a reading, the theorem does not reach this set' : 'M_F(x) over its mass A_F(x): the theorem sends this to zero') : 'M_F(x) over A_F(x)^(1/2): square-root size, the open conjecture'}</span></h2>
        {good && tally && <Meter tally={tally} by={pick.by} />}
        <Stats>
          <Stat label="digits">{read.digits && `${read.fill} of ${read.base}, ${digit} missing`}</Stat>
          <Stat label="A_F(x)">{t?.count}</Stat>
          <Stat label="M_F(x)">{t?.meter}</Stat>
          <Stat label="M_F / A_F">{t && fixed(t.mass, 6)}</Stat>
          <Stat label="M_F / A_F^(1/2)">{t && fixed(t.root, 4)}</Stat>
          <Stat label="max |M_F| / A_F^(1/2)">{t && fixed(t.peak, 4)}</Stat>
          <Stat label="x up to">{t && `${pick.base}^${depth}`}</Stat>
        </Stats>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The prime count <span>psi_F(x) over its main term kappa_F A_F(x)</span></h2>
        {good && tally && <Primes tally={tally} />}
        <Stats>
          <Stat label="kappa_F">{read.kappa && `${read.kappa[0]}/${read.kappa[1]}`}</Stat>
          <Stat label="psi_F(x)">{t && fixed(t.psi, 2)}</Stat>
          <Stat label="psi_F / (kappa_F A_F)">{t && fixed(t.primes, 6)}</Stat>
          <Stat label="two consecutive digits">{read.digits && (read.consecutive ? 'yes' : 'no, so no main term')}</Stat>
        </Stats>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The grid <span>the frequencies a/y cut into four regions by their Dirichlet fraction</span></h2>
        {good && grid && <Lanes grid={grid} />}
        <Stats>
          <Stat label="y">{g?.y}</Stat>
          <Stat label="Q">{g?.cap}</Stat>
          <Stat label="y^(2/5)">{g && fixed(g.low, 2)}</Stat>
          {NAMES.map((name, r) => (
            <Stat key={name} label={name}>{g && `${g.counts[r]}, ${fixed(100 * g.shares[r], 1)}% of l^1`}</Stat>
          ))}
        </Stats>
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The wall <span>alpha_1 against 1/5 and 1/4, blue the chain by base, yellow this set's reading</span></h2>
        {good && read.chain !== undefined && <Wall read={read} />}
        <Stats>
          <Stat label="chain alpha_1">{read.chain !== undefined && fixed(read.chain, 6)}</Stat>
          <Stat label="reading alpha_1">{read.reading !== undefined && `${fixed(read.reading, 6)} at level ${read.level}`}</Stat>
          <Stat label="bars">{read.bars && `${read.bars[0]} and ${read.bars[1]}`}</Stat>
        </Stats>
        <p style={{ marginTop: 10, fontSize: 13 }}>{good && read.walls && reachLine(read)}</p>
      </div>
      <Note error={error}>{good && g && `Z = ${g.z}; the regions partition the grid once Z sits below y^(2/5), and at level ${level} that is ${fixed(g.low, 2)}.`}</Note>
    </Page>
  );
}

mount(<App />);
