import { useMemo } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { useQuery } from '../lib/query.js';
import { mount, Page, Row, Pick, Slider, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { Pins, Ratios, Terms, mix } from '../lib/series.jsx';
import { board, line, axis, tag } from '../lib/chart.js';

const m = await ready();
const BASES = [3, 5];
const CAPS = new Map(BASES.map((base) => [base, m.carry_cap(base)]));
const STRIP = Math.max(...CAPS.values());
const LANES = [[3, 'three'], [5, 'five']];
const SIGNS = JSON.parse(m.carry_signs(STRIP));
const CHAR = 6.7;

function shown(value) {
  return value === null || value === undefined ? 'none' : Number(value).toFixed(6);
}

function App() {
  const [pick, set] = useQuery({ base: 3, dimension: 3, levels: 8 });

  const cap = CAPS.get(pick.base) ?? 2;
  const dimension = Math.min(cap, Math.max(2, pick.dimension));

  const read = useMemo(() => {
    try {
      return { row: JSON.parse(m.carry_block(pick.base, dimension, pick.levels)), error: null };
    } catch (error) {
      return { row: null, error };
    }
  }, [pick.base, dimension, pick.levels]);

  const row = read.row;

  const block = (canvas) => {
    if (!row) return;
    const grid = row.block;
    const n = grid.length;
    const b = board(canvas, 62 + n * 34, { top: 26, bottom: 12 });
    const size = Math.min(b.wide / n, (b.floor - b.roof) / n);
    const left = b.x(0) + (b.wide - size * n) / 2;
    const peak = Math.max(...grid.flat(), 1);
    grid.forEach((cells, r) => cells.forEach((value, c) => {
      const x = left + c * size;
      const y = b.roof + r * size;
      b.ctx.fillStyle = mix(ink.line, ink.gold, value / peak);
      b.ctx.fillRect(x, y, size - 2, size - 2);
      const text = String(value);
      if (size >= text.length * CHAR + 8) tag(b, text, ink.deep, 'center', x + size / 2 - 1, y + size / 2 + 3);
    }));
    tag(b, `the reflection-even block, ${n} by ${n}, rows the carry out`, ink.dim);
  };

  const parity = (canvas) => {
    const b = board(canvas, 210, { top: 30, bottom: 24 });
    const n = SIGNS.length;
    const step = b.wide / n;
    const mid = (b.roof + b.floor) / 2;
    const tall = (b.floor - b.roof) / 2 / LANES.length - 3;
    SIGNS.forEach((each, i) => {
      const x = b.x(i / n) + 1;
      const wide = Math.max(1, step - 2);
      const odd = each.dimension % 2 === 1;
      LANES.forEach(([base, key], lane) => {
        const sign = each[key]?.sign;
        const y = odd ? mid - (lane + 1) * (tall + 3) : mid + 3 + lane * (tall + 3);
        b.ctx.fillStyle = sign === undefined ? ink.line : sign > 0 ? ink.orange : ink.blue;
        b.ctx.fillRect(x, y, wide, tall);
        if (base === pick.base && each.dimension === dimension) {
          b.ctx.strokeStyle = ink.fg;
          b.ctx.lineWidth = 1.5;
          b.ctx.strokeRect(x - 0.5, y - 0.5, wide + 1, tall + 1);
        } else if (each.open) {
          b.ctx.strokeStyle = ink.gold;
          b.ctx.lineWidth = 1;
          b.ctx.strokeRect(x + 0.5, y + 0.5, wide - 1, tall - 1);
        }
      });
    });
    line(b, [[0, 0.5], [1, 0.5]], ink.dim, { width: 1, dash: [4, 4] });
    axis(b, SIGNS.map((each, i) => [(i + 0.5) / n, each.dimension]));
    const next = tag(b, 'odd D above, the slice exponent over the codimension', ink.orange);
    tag(b, 'even D below, under it', ink.blue, 'left', next + 16);
    tag(b, 'gold: the open class D = 1 mod 3', ink.gold, 'right');
  };

  return (
    <Page crumb="carry" title="One carry decides how many past terms a diagonal slice needs"
      sub={<>Keep the base-<code>q</code> cells whose digit vector has at most one middle digit and you have a sponge in every dimension. Cut it through the centre, square to the main diagonal, and count the cells the cut meets at each level. Refining by one level adds one digit per coordinate, and all the cut remembers is a carry - a small whole number that a contraction and a reflection squeeze into <code>ceil(D/2)</code> states. That is the whole rule: <code>D</code> dimensions, <code>ceil(D/2)</code> past terms, and a growth exponent that misses the generic value by a hair, above it at odd <code>D</code> and below it at even <code>D</code>.</>}
      foot={<>The integers here are the crate's exact arithmetic; the root, the logs, the gap and the spectral ratio are its floating-point readings. The digit polynomial is the level-one census by digit sum, <code>A(t)^(D-1)(A(t) + D t^m)</code> with <code>A</code> the sum of every digit power but the middle one; at base three it factors as <code>(1 + t^2)^(D-1)(1 + D t + t^2)</code>. The carry map <code>c -&gt; (c + mD - s)/q</code> contracts onto <code>|c| &lt;= (D-1)/2</code>, the polynomial's palindromic symmetry folds that window in half, and the even block left over is the matrix drawn above: its characteristic polynomial is the recurrence, its Perron root the growth, and the order the counts exhibit is read back independently by the recurrence hunter that <a href="../plot">the plot</a> uses. The sign is a comparison of whole numbers, not a rounding: the characteristic polynomial is evaluated at <code>f_D/q</code> with the denominators cleared, so a hundred and twenty-eight bits set the dimensions the strip can reach. At <code>D = 3</code> the ladder is 1, 6, 42, 306, 2250 - the same count the sponge's diagonal profile gives on <a href="../slices">the slices</a> and <a href="../spectrometer">the spectrometer</a>, reached there by building the cut and here without one. The research page is <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/slices.md">slices</a>; the order theorem and the odd half of the sign law at base 3, away from the open class, are the shelf paper <a href="https://github.com/carlomitchener/carlomitchener/tree/main/research/slice-recurrence-order">slice-recurrence-order</a>, and the even half, at base three and base five, is <a href="https://github.com/carlomitchener/carlomitchener/tree/main/research/slice-sign-even-half">slice-sign-even-half</a>. The odd class <code>D = 1 mod 3</code> is still open.</>}>
      <Row>
        <Pick label="base" value={pick.base} options={BASES} onChange={(value) => set({ base: +value, dimension: Math.min(CAPS.get(+value) ?? 2, dimension) })} />
        <Slider label="dimension" value={dimension} min={2} max={cap} onChange={(value) => set({ dimension: value })} show={`D ${dimension} of ${cap}`} />
        <Slider label="levels" value={pick.levels} min={1} max={24} onChange={(value) => set({ levels: value })} />
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The carry block <span>{row ? `order ${row.order}, ceil(D/2)` : 'nothing read'}</span></h2>
          <Sketch draw={block} deps={[row]} className="bars" />
          {row && <Terms terms={row.digits.map(String)} tight label="P(t)" />}
          {row && (
            <Stats>
              <Stat label="characteristic">{row.polynomial}</Stat>
              <Stat label="trace">{row.trace}</Stat>
              <Stat label="determinant">{row.determinant}</Stat>
              <Stat label="Perron root">{shown(row.read.root)}</Stat>
              <Stat label="fill">{row.fill}</Stat>
              <Stat label="spectral ratio">{shown(row.spectral)}</Stat>
            </Stats>
          )}
        </div>
        <div className="panel">
          <h2>The exponent <span>{row ? `sign ${row.read.sign > 0 ? 'plus one' : 'minus one'}` : 'nothing read'}</span></h2>
          {row && (
            <Stats>
              <Stat label={`log_${pick.base} of the root`}>{shown(row.read.log_root)}</Stat>
              <Stat label={`log_${pick.base} of the fill less one`}>{shown(row.read.log_fill)}</Stat>
              <Stat label="root against fill over q">{`${shown(row.read.root)} against ${row.fill} / ${pick.base}`}</Stat>
              <Stat label="gap">{shown(row.read.gap)}</Stat>
              <Stat label="sign read">{row.read.sign}</Stat>
              <Stat label="the law">{row.law}</Stat>
              <Stat label="agree">{row.read.sign === row.law ? 'yes' : 'no'}</Stat>
              <Stat label="class">{row.open ? 'the open odd class 1 mod 3' : dimension % 2 === 0 ? 'even half, proved on the shelf at bases 3 and 5' : pick.base === 3 ? 'odd half, proved on the shelf at base 3' : 'odd half at base 5, verified on this range and proved nowhere'}</Stat>
            </Stats>
          )}
          <h2>The order <span>{row ? (row.fits ? 'the terms agree' : 'not enough terms') : ''}</span></h2>
          {row && (
            <Stats>
              <Stat label="proved order">{row.order}</Stat>
              <Stat label="order the terms exhibit">{row.found ?? (row.fits ? 'none fits these terms' : 'too few terms to read')}</Stat>
              <Stat label="free bound">2D + 1</Stat>
              <Stat label="terms in hand">{row.terms.length}</Stat>
              <Stat label="held to">{row.capped ? 'the exact integers' : 'the levels asked for'}</Stat>
            </Stats>
          )}
        </div>
      </div>
      <div className="panel">
        <h2>The ladder <span>{row ? `a_${dimension}(L), base ${pick.base}` : ''}</span></h2>
        {row && <Pins terms={row.terms} start={0} label={`level L from 0, base ${pick.base}, dimension ${dimension}`} />}
        {row && <Terms terms={row.terms} start={0} capped={row.capped} />}
        {row && <Ratios values={row.ratios} start={1} label={`each count against the one before, falling to the Perron root ${shown(row.read.root)}`} />}
      </div>
      <div className="panel">
        <h2>The sign law <span>{`dimensions 2 to ${STRIP}, both bases`}</span></h2>
        <Sketch draw={parity} deps={[pick.base, dimension]} className="bars" />
        <Stats>
          <Stat label="lanes">base 3 nearest the line, base 5 outside it</Stat>
          <Stat label="grey">past that base's exact cap</Stat>
          <Stat label="base 3 cap">{CAPS.get(3)}</Stat>
          <Stat label="base 5 cap">{CAPS.get(5)}</Stat>
        </Stats>
      </div>
      <Note error={read.error} />
    </Page>
  );
}

mount(<App />);
