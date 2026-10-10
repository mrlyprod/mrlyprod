import { useEffect, useMemo, useState } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { board } from '../../../lib/chart.js';
import { demo, Page, Group, Pick, Slider, Text, Btn, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { Terms, mix } from '../../../lib/series.jsx';
import { useQuery } from '../../../lib/query.js';

const m = await ready();

const PRIMES = Array.from(m.kummer_primes());
const TOP = m.kummer_top();
const PAIRS = [
  ['2,1', '(2, 1), the half interval'],
  ['3,1', "(3, 1), the theorem's set"],
  ['4,1', '(4, 1)'],
  ['5,2', '(5, 2)'],
];
const FIRST = { p: 7, a: 3, b: 1, level: 3, k: '18', at: -1 };
const PAD = 24;
const GAP = 4;
const STEP = 650;
const APART = { marginTop: 8 };

const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, Math.round(Number.isFinite(+v) ? +v : lo)));

const attempt = (fn) => {
  try {
    return { value: fn(), error: null };
  } catch (error) {
    return { value: null, error };
  }
};

const nearest = (p) => PRIMES.reduce((best, q) => (Math.abs(q - p) < Math.abs(best - p) ? q : best), PRIMES[0]);

const written = (digits, p) => `${[...digits].reverse().join(p > 10 ? '·' : '')}_${p}`;

const named = (state, bottom) => (bottom === 1 ? `carry ${state[1]}` : `(${state[0]}, ${state[1]})`);

const hue = (s, n) => (s === 0 ? ink.blue : n <= 2 ? ink.orange : mix(ink.orange, ink.yellow, (s - 1) / (n - 2)));

const spans = (runs, states, bottom) =>
  runs.length ? runs.map(([lo, hi, to]) => `${lo === hi ? lo : `${lo}..${hi}`} to ${named(states[to], bottom)}`).join(', ') : 'nothing';

function head(b, caption, items) {
  const { ctx } = b;
  let x = b.x(1);
  ctx.textAlign = 'right';
  for (const [text, color] of [...items].reverse()) {
    ctx.fillStyle = color;
    ctx.fillText(text, x, 14);
    x -= ctx.measureText(text).width + 14;
  }
  ctx.textAlign = 'left';
  if (b.x(0) + ctx.measureText(caption).width + 4 > x) return;
  ctx.fillStyle = ink.dim;
  ctx.fillText(caption, b.x(0), 14);
}

function keys(read, bottom) {
  const n = read.states.length;
  if (n <= 3) return read.states.map((state, s) => [named(state, bottom), hue(s, n)]);
  return [['no carry', hue(0, n)], ['a carry', hue(1, n)], ['more carry', hue(n - 1, n)]];
}

function App() {
  const [q, put] = useQuery(FIRST);
  const [playing, setPlaying] = useState(false);
  const p = nearest(+q.p);
  const top = clamp(q.a, 2, TOP);
  const bottom = clamp(q.b, 1, top - 1);
  const cap = m.kummer_cap(p);
  const level = clamp(q.level, 1, cap);

  const view = useMemo(() => attempt(() => JSON.parse(m.kummer_read(p, top, bottom))), [p, top, bottom]);
  const read = view.value;
  const cells = useMemo(() => attempt(() => m.kummer_board(p, top, bottom, level)).value, [p, top, bottom, level]);
  const traced = useMemo(() => attempt(() => JSON.parse(m.kummer_trace(q.k || '0', p, top, bottom))), [q.k, p, top, bottom]);
  const trace = traced.value;
  const n = trace ? trace.columns.length : 0;
  const at = Number.isFinite(q.at) && q.at >= 0 && q.at <= n ? Math.round(q.at) : n;
  const done = at >= n;
  const shown = Math.max(0, Math.min(at, n - 1));

  useEffect(() => {
    if (!playing) return undefined;
    if (done) {
      setPlaying(false);
      return undefined;
    }
    const timer = setTimeout(() => put({ at: at + 1 }), STEP);
    return () => clearTimeout(timer);
  }, [playing, at, done]);

  const pick = (k) => {
    setPlaying(false);
    put({ k: String(k), at: -1 });
  };
  const play = () => {
    put({ at: 0 });
    setPlaying(true);
  };

  const S = read ? read.states.length : 1;
  const path = trace ? trace.digits : [];
  const band = level <= 3 ? 44 : level <= 5 ? 32 : 24;

  const tree = (canvas) => {
    const b = board(canvas, 30 + level * (band + GAP) + 10, { left: PAD, right: PAD, top: 30, bottom: 14 });
    if (!read || !cells || b.wide <= 0) return;
    const { ctx } = b;
    let offset = 0;
    for (let r = 1; r <= level; r++) {
      const count = p ** r;
      const w = b.wide / count;
      const y = b.roof + (r - 1) * (band + GAP);
      const gap = w >= 4 ? 1 : 0;
      ctx.fillStyle = ink.dim;
      ctx.textAlign = 'right';
      ctx.fillText(String(r), b.left - 8, y + band / 2 + 4);
      ctx.textAlign = 'left';
      let j = 0;
      while (j < count) {
        const t = cells[offset + j];
        let e = j + 1;
        if (!gap) while (e < count && cells[offset + e] === t) e++;
        if (t) {
          ctx.globalAlpha = read.closes[t - 1] ? 1 : 0.35;
          ctx.fillStyle = hue(t - 1, S);
          ctx.fillRect(b.x(j / count), y, Math.max(0.75, (e - j) * w - gap), band);
        }
        j = e;
      }
      ctx.globalAlpha = 1;
      offset += count;
    }
    let place = 0;
    for (let r = 1; r <= level; r++) {
      place = place * p + (path[r - 1] ?? 0);
      const count = p ** r;
      const w = b.wide / count;
      const y = b.roof + (r - 1) * (band + GAP);
      ctx.strokeStyle = done || r <= at ? ink.fg : ink.dim;
      ctx.lineWidth = 1.5;
      ctx.strokeRect(b.x(place / count) - 1.5, y - 1.5, Math.max(w, 2) + 3, band + 3);
    }
    head(b, `row r: the k below ${p}^r, units digit first`, keys(read, bottom));
  };

  const seek = (f) => {
    const count = p ** level;
    let rest = clamp(Math.floor(Math.max(0, Math.min(1, f)) * count), 0, count - 1);
    let k = 0;
    for (let i = level - 1; i >= 0; i--) {
      k += (rest % p) * p ** i;
      rest = Math.floor(rest / p);
    }
    pick(k);
  };

  const machine = (canvas) => {
    const row = S <= 2 ? 34 : S <= 6 ? 22 : 14;
    const b = board(canvas, 30 + S * row + 24, { left: 70, right: 14, top: 30, bottom: 24 });
    if (!read || b.wide <= 0) return;
    const { ctx } = b;
    const w = b.wide / p;
    const gap = w >= 5 ? 1.5 : 0;
    read.moves.forEach((moves, s) => {
      const y = b.roof + s * row;
      ctx.fillStyle = ink.dim;
      ctx.textAlign = 'right';
      ctx.fillText(named(read.states[s], bottom), b.left - 8, y + row / 2 + 4);
      ctx.textAlign = 'left';
      moves.forEach((to, d) => {
        const x = b.x(d / p) + gap / 2;
        if (to === null) {
          ctx.fillStyle = ink.line;
          ctx.fillRect(x, y + row * 0.42, Math.max(0.5, w - gap), row * 0.16);
          return;
        }
        ctx.globalAlpha = read.closes[to] ? 1 : 0.35;
        ctx.fillStyle = hue(to, S);
        ctx.fillRect(x, y + 2, Math.max(0.5, w - gap), row - 4);
        ctx.globalAlpha = 1;
      });
    });
    const cur = trace?.columns[shown];
    if (cur && cur.digit < p) {
      ctx.strokeStyle = ink.fg;
      ctx.lineWidth = 2;
      ctx.strokeRect(b.x(cur.digit / p) - 1, b.roof + cur.state * row, Math.max(w, 3) + 2, row);
    }
    const every = Math.max(1, Math.ceil(16 / w));
    ctx.fillStyle = ink.dim;
    ctx.textAlign = 'center';
    for (let d = 0; d < p; d += every) ctx.fillText(String(d), b.x((d + 0.5) / p), b.h - 8);
    ctx.textAlign = 'left';
    head(b, 'digit read, and the state it moves to', [...keys(read, bottom).map(([text, color]) => [`to ${text}`, color]), ['refused', ink.dim]]);
  };

  const adder = (canvas) => {
    const row = 24;
    const b = board(canvas, 34 + 5 * row + 8, { left: 76, right: 14, top: 34, bottom: 8 });
    if (!trace || b.wide <= 0) return;
    const { ctx } = b;
    const cols = Math.max(trace.digits.length, trace.low.length, trace.high.length, n);
    const cw = Math.min(40, b.wide / cols);
    const right = Math.min(b.x(1), b.x(0.5) + (cols * cw) / 2 + 40);
    const cx = (i) => right - (i + 0.5) * cw;
    const cy = (r) => b.roof + r * row;
    const size = cw >= 18 ? 13 : cw >= 11 ? 11 : 9;
    if (!done) {
      ctx.fillStyle = ink.line;
      ctx.globalAlpha = 0.6;
      ctx.fillRect(cx(at) - cw / 2, cy(0) - 4, cw, 5 * row);
      ctx.globalAlpha = 1;
    }
    const labels = ['state', 'k', `${bottom}k`, `${top - bottom}k`, 'column sum'];
    ctx.fillStyle = ink.dim;
    ctx.textAlign = 'right';
    labels.forEach((text, r) => ctx.fillText(text, right - cols * cw - 10, cy(r) + row / 2 + 4));
    ctx.strokeStyle = ink.line;
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(right - cols * cw, cy(4));
    ctx.lineTo(right, cy(4));
    ctx.stroke();
    ctx.font = `${size}px ${b.mono}`;
    ctx.textAlign = 'center';
    const digit = (value, i, r, color) => {
      ctx.fillStyle = color;
      ctx.fillText(String(value), cx(i), cy(r) + row / 2 + size / 3);
    };
    for (let i = 0; i < cols; i++) {
      const seen = i < at;
      const column = trace.columns[i];
      digit(trace.digits[i] ?? 0, i, 1, i < trace.digits.length ? (seen ? ink.fg : ink.dim) : ink.line);
      if (i < trace.low.length) digit(trace.low[i], i, 2, seen ? ink.fg : ink.dim);
      if (i < trace.high.length) digit(trace.high[i], i, 3, seen ? ink.fg : ink.dim);
      if (!column || i > at) continue;
      const y = cy(0) + row / 2;
      ctx.fillStyle = hue(column.state, S);
      ctx.beginPath();
      ctx.arc(cx(i), y, Math.max(1, Math.min(6, cw * 0.22)), 0, Math.PI * 2);
      ctx.fill();
      if (!seen) continue;
      digit(column.left + column.right, i, 4, column.next === null ? ink.pink : ink.fg);
      if (column.next !== null && i + 1 < cols) {
        ctx.strokeStyle = hue(column.next, S);
        ctx.lineWidth = 1.5;
        ctx.beginPath();
        ctx.moveTo(cx(i) - cw * 0.28, y);
        ctx.lineTo(cx(i + 1) + cw * 0.28, y);
        ctx.stroke();
      }
      if (column.next === null) {
        ctx.strokeStyle = ink.pink;
        ctx.lineWidth = 1.5;
        ctx.strokeRect(cx(i) - cw / 2 + 1, cy(1) - 2, cw - 2, 4 * row);
      }
    }
    ctx.font = `11px ${b.mono}`;
    ctx.textAlign = 'left';
    head(b, `base ${p}, units at the right, read right to left`, [[done ? (trace.member ? 'no column reaches p' : 'a column reaches p: it carries') : `column ${at + 1} of ${n}`, done && !trace.member ? ink.pink : ink.fg]]);
  };

  const pair = `${top},${bottom}`;
  const known = PAIRS.some(([v]) => v === pair);
  const choose = (v) => {
    if (v === 'custom') return;
    const [a, b] = v.split(',').map(Number);
    setPlaying(false);
    put({ a, b, at: -1 });
  };
  const scrub = Math.min(Number(q.k) || 0, p ** level - 1);
  const w = read?.witness;
  const binomial = trace ? `C(${trace.product}, ${trace.chosen})` : '';

  const controls = (
    <>
      <Group name="The set">
        <Pick label="pair" value={known ? pair : 'custom'} onChange={choose} options={[...PAIRS, ...(known ? [] : [['custom', `(${top}, ${bottom})`]])]} />
        <Slider label="prime p" value={PRIMES.indexOf(p)} min={0} max={PRIMES.length - 1} show={p} onChange={(i) => put({ p: PRIMES[i], at: -1 })} />
        <Slider label="a" value={top} min={2} max={TOP} onChange={(v) => put({ a: v, b: Math.min(bottom, v - 1), at: -1 })} />
        <Slider label="b" value={bottom} min={1} max={top - 1} onChange={(v) => put({ b: v, at: -1 })} />
      </Group>
      <Group name="The board">
        <Slider label="level" value={level} min={1} max={cap} onChange={(v) => put({ level: v })} />
      </Group>
      <Group name="The reader">
        <Text label="k" value={q.k} onChange={(v) => pick(v.replace(/\D/g, ''))} />
        <Slider label="scrub k" value={scrub} min={0} max={p ** level - 1} onChange={pick} />
        <Slider label="digits read" value={Math.min(at, n)} min={0} max={n} onChange={(v) => { setPlaying(false); put({ at: v }); }} />
        <Btn primary onClick={play}>Read it</Btn>
      </Group>
    </>
  );

  return (
    <Page title="The Kummer sets" controls={controls}
      sub="Keep the k for which a prime p does not divide C(a k, b k). By Kummer, p divides it exactly when the addition b k + (a - b) k carries somewhere in base p. At (2, 1) that asks only that every digit of k sit at most (p - 1)/2, a digit design. At (3, 1) and p from 5 on, a digit is kept or refused by the carry the doubling 2k brings up from the digit below, so the set is read by a two-state automaton from the units digit up, and no list of digits writes it. Pick a pair and a prime, then type, scrub or click a k and read it."
      foot={<>Every number on this page comes from the crate through wasm: the automaton, the board, the count below p^L, the witness, the columns read for k and the valuation of C(a k, b k) by Legendre's digit sums; the page only draws. At every prime p above a each state keeps exactly (p + 1)/2 digits, so the count below p^L is ((p + 1)/2)^L whatever the pair; at p up to a the carries can outlive the digits, faded cells end in a carry the zeros cannot spend, and there the law fails. Nearby: <a href="../dissection/">the dissection</a> proves the Mobius bound on the half interval this page opens beside, and <a href="../memory/">the memory dial</a> keeps a word by windows of its digits.</>}>
      <Note error={view.error ?? traced.error} />
      <Sketch draw={tree} deps={[read, cells, level, path, at, done, p]} onSeek={seek} pad={PAD} role="img" aria-label={read ? `the board of the Kummer set (${top}, ${bottom}) at ${p}, levels 1 to ${level}` : 'the board'} />
      {read && (
        <Stats>
          <Stat label={`count below ${p}^${level}`}>{read.counts[level - 1]}</Stat>
          <Stat label={`((p + 1)/2)^${level}`}>{read.law[level - 1]}</Stat>
          {read.counts[level - 1] === read.law[level - 1] ? (
            <span className="chip proved">{read.holds ? 'p > a, so the count meets the law' : 'the count meets the law here'}</span>
          ) : (
            <span className="chip refuted">{`p = ${p} <= a = ${top}, and the count misses the law`}</span>
          )}
        </Stats>
      )}
      {read && <Terms terms={read.counts} start={1} marks={read.counts.map((c, i) => c !== read.law[i])} label="count by level" />}
      {read && <Terms terms={read.law} start={1} label={`${read.fill}^L`} />}
      <Sketch draw={machine} deps={[read, trace, shown]} style={APART} role="img" aria-label="the carry automaton: for each state, the digits it keeps and the state each one moves to" />
      {read && (
        <Stats>
          {read.states.map((state, s) => (
            <Stat key={s} label={`${named(state, bottom)} keeps ${read.kept[s]}`}>{spans(read.runs[s], read.states, bottom)}</Stat>
          ))}
        </Stats>
      )}
      {read && (w ? (
        <div className="ribbon">
          <span className="tag">no digit design: u and v p are in, u + v p is out</span>
          {[[w.u, w.digits[0], 'in'], [w.high, w.digits[1], 'in'], [w.sum, w.digits[2], 'out']].map(([k, digits, side]) => (
            <span key={side + k} role="button" className={side === 'out' ? 'yellow' : undefined} onClick={() => pick(k)}>
              <i>{side}</i><b>{k}</b><i>{written(digits, p)}</i>
            </span>
          ))}
        </div>
      ) : (
        <Stats><Stat label="witness">{read.states.length === 1 ? 'none: one state, so every digit is kept or refused on its own and the set is a digit design' : 'none: no digits u and v with u and v p in the set and u + v p out'}</Stat></Stats>
      ))}
      <Sketch draw={adder} deps={[trace, at, p, done]} style={APART} role="img" aria-label={trace ? `k = ${trace.k} read column by column` : 'the reader'} />
      {trace && (
        <Stats>
          <Stat label="k">{`${trace.k} = ${written(trace.digits, p)}`}</Stat>
          <Stat label={`v_${p}(${binomial})`}>{trace.valuation}</Stat>
          {done ? (
            <>
              <span className="chip verified">{trace.member ? `${p} does not divide ${binomial}` : `${p} divides ${binomial}`}</span>
              <span className="chip proved">{trace.member === (trace.valuation === 0) ? 'the automaton meets the valuation' : 'the automaton misses the valuation'}</span>
            </>
          ) : (
            <Stat label="reading">{`column ${at + 1} of ${n}`}</Stat>
          )}
        </Stats>
      )}
    </Page>
  );
}

export const { mount, unmount } = demo(<App />);
