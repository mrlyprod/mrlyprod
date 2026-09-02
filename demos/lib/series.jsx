import { Sketch } from './draw.jsx';
import { ink, rgb } from './mrly.js';
import { board, line, axis, tag } from './chart.js';

const HEAD = 15;
const CHAR = 6.7;

// NUMBERS

export function mix(from, to, at) {
  const [a, b] = [rgb(from), rgb(to)];
  const f = Math.max(0, Math.min(1, at));
  return `rgb(${a.map((c, k) => Math.round(c + (b[k] - c) * f)).join(',')})`;
}

function text(t) {
  return String(t ?? '').trim();
}

function bare(t) {
  const s = text(t);
  return (s.startsWith('-') ? s.slice(1) : s).replace(/^0+(?=\d)/, '');
}

function negative(t) {
  return text(t).startsWith('-');
}

function log10(t) {
  const d = bare(t);
  if (/^\d+$/.test(d)) return d === '0' ? -Infinity : Math.log10(Number(d.slice(0, HEAD))) + Math.max(0, d.length - HEAD);
  const v = Math.abs(Number(t));
  return v > 0 ? Math.log10(v) : -Infinity;
}

function span(values) {
  const tops = values.map(log10).filter(Number.isFinite);
  return { hi: tops.length ? Math.max(...tops) : 0, lo: tops.length ? Math.min(...tops) : 0 };
}

function share(t, hi) {
  const l = log10(t);
  return l === -Infinity ? 0 : Math.min(1, 10 ** (l - hi));
}

function widest(values) {
  return values.reduce((w, v) => Math.max(w, text(v).length), 0);
}

function ticks(n, start) {
  const every = Math.max(1, Math.ceil(n / 12));
  const out = [];
  for (let i = 0; i < n; i++) if (i % every === 0) out.push([(i + 0.5) / n, String(start + i)]);
  return out;
}

function running(terms) {
  try {
    let total = 0n;
    return terms.map((t) => (total += BigInt(text(t))).toString());
  } catch {
    return null;
  }
}

function expand(t, base) {
  const d = bare(t);
  if (!/^\d+$/.test(d)) return [];
  const digits = base === 10 ? d : BigInt(d).toString(base);
  return [...digits].map((c) => parseInt(c, 36));
}

// PINS

export function Pins({ terms, start = 0, log = 'auto', height = 240, label = '', note, marks, hue = ink.blue, className = 'bars', style, ...rest }) {
  const draw = (canvas) => {
    const n = terms.length;
    const room = n > 0 && (canvas.clientWidth - 28) / n >= widest(terms) * CHAR + 10;
    const b = board(canvas, height, { top: room ? 36 : 26 });
    if (!n) return;
    const { hi, lo } = span(terms);
    const scaled = log === 'auto' ? hi - lo >= 2 : !!log;
    const at = (t) => {
      const l = log10(t);
      if (!scaled) return share(t, hi);
      return l === -Infinity ? 0 : (l - lo + 1) / (hi - lo + 1);
    };
    const step = b.wide / n;
    terms.forEach((t, i) => {
      const f = (i + 0.5) / n;
      const y = Math.max(0.015, at(t));
      const color = marks?.[i] ? ink.gold : negative(t) ? ink.orange : hue;
      line(b, [[f, 0], [f, y]], color, { width: Math.max(1, Math.min(3, step * 0.16)) });
      line(b, [[f, y]], color, { dots: Math.max(2, Math.min(4, step * 0.14)) });
      if (room) tag(b, text(t), ink.fg, 'center', b.x(f), Math.max(b.roof - 8, b.y(y) - 8));
    });
    axis(b, ticks(n, start));
    tag(b, `${scaled ? 'log scale' : 'linear scale'}${label ? `, ${label}` : ''}`, ink.dim);
    tag(b, note ?? `last ${text(terms.at(-1))}`, ink.fg, 'right');
  };
  return <Sketch className={className} style={{ height, ...style }} draw={draw} deps={[terms, log, start, marks, height]} {...rest} />;
}

// STAIRCASE

export function Staircase({ terms, start = 0, sums = false, height = 220, label = '', hue = ink.green, className = 'bars', style, ...rest }) {
  const draw = (canvas) => {
    const b = board(canvas, height);
    const values = (sums && running(terms)) || terms;
    const n = values.length;
    if (!n) return;
    const { hi } = span(values);
    const points = [];
    values.forEach((v, i) => {
      const f = share(v, hi);
      points.push([i / n, f], [(i + 1) / n, f]);
    });
    line(b, points, hue, { width: 1.6, fill: 0.16 });
    axis(b, ticks(n, start));
    const kind = values === terms ? 'step function' : 'partial sums';
    tag(b, `${kind}${values.some(negative) ? ', magnitudes' : ''}${label ? `, ${label}` : ''}`, ink.dim);
  };
  return <Sketch className={className} style={{ height, ...style }} draw={draw} deps={[terms, sums, start, height]} {...rest} />;
}

// DIGITS

export function Digits({ terms, base = 2, start = 0, cell = 13, label = '', className = 'bars', style, ...rest }) {
  const draw = (canvas) => {
    const q = Math.max(2, Math.min(36, Math.round(base)));
    const rows = terms.map((t) => expand(t, q));
    const cols = Math.max(1, rows.reduce((w, r) => Math.max(w, r.length), 0));
    const gutter = 42;
    const free = Math.max(40, canvas.clientWidth - gutter - 10);
    const size = Math.max(2, Math.min(cell, free / cols));
    const b = board(canvas, Math.round(40 + rows.length * size), { left: gutter, right: 10, top: 30, bottom: 10 });
    rows.forEach((digits, i) => {
      const y = b.roof + i * size;
      if (size >= 8) tag(b, String(start + i), ink.dim, 'right', gutter - 8, y + size - 3);
      digits.forEach((d, j) => {
        b.ctx.fillStyle = d === 0 ? ink.line : mix(ink.blue, ink.gold, q > 2 ? d / (q - 1) : 1);
        b.ctx.fillRect(gutter + (cols - digits.length + j) * size, y, Math.max(1, size - 1), Math.max(1, size - 1));
      });
    });
    tag(b, `${label ? `${label}, ` : ''}base ${q}, least significant at the right`, ink.dim);
  };
  return <Sketch className={className} draw={draw} deps={[terms, base, start, cell]} style={style} {...rest} />;
}

// RATIOS

export function Ratios({ values, start = 0, height = 96, label = '', className = 'bars', style, ...rest }) {
  const draw = (canvas) => {
    const b = board(canvas, height, { top: 26, bottom: 20 });
    const n = values.length;
    if (!n) return;
    const nums = values.map(Number).filter(Number.isFinite);
    const lo = Math.min(...nums), hi = Math.max(...nums);
    const step = b.wide / n;
    const room = step >= widest(values) * CHAR + 8;
    values.forEach((v, i) => {
      const at = hi > lo ? (Number(v) - lo) / (hi - lo) : 0.5;
      b.ctx.fillStyle = mix(ink.blue, ink.pink, Number.isFinite(at) ? at : 0.5);
      b.ctx.fillRect(b.x(i / n), b.roof, Math.max(1, step - 1), b.floor - b.roof);
      if (room) tag(b, text(v), ink.deep, 'center', b.x((i + 0.5) / n), (b.roof + b.floor) / 2 + 4);
    });
    axis(b, ticks(n, start));
    tag(b, label || 'each term against the one before', ink.dim);
  };
  return <Sketch className={className} style={{ height, ...style }} draw={draw} deps={[values, start, height]} {...rest} />;
}

// DIFFERENCES

export function Differences({ rows, cell = 26, label = '', className = 'bars', style, ...rest }) {
  const draw = (canvas) => {
    const cols = Math.max(1, rows[0]?.length ?? 0);
    const gutter = 42;
    const free = Math.max(40, canvas.clientWidth - gutter - 10);
    const size = Math.max(6, Math.min(cell, free / cols));
    const b = board(canvas, Math.round(40 + rows.length * size), { left: gutter, right: 10, top: 30, bottom: 10 });
    rows.forEach((row, r) => {
      const { hi } = span(row);
      const y = b.roof + r * size;
      if (size >= 10) tag(b, r ? `d${r}` : 'terms', ink.dim, 'right', gutter - 8, y + size - 6);
      row.forEach((v, i) => {
        const x = gutter + (i + r / 2) * size;
        b.ctx.globalAlpha = 0.25 + 0.7 * share(v, hi);
        b.ctx.fillStyle = log10(v) === -Infinity ? ink.dim : negative(v) ? ink.orange : ink.blue;
        b.ctx.fillRect(x, y, Math.max(1, size - 1), Math.max(1, size - 1));
        b.ctx.globalAlpha = 1;
        if (size >= text(v).length * CHAR + 4) tag(b, text(v), ink.deep, 'center', x + size / 2, y + size - 7);
      });
    });
    tag(b, label || 'the difference triangle, each row the differences of the row above', ink.dim);
  };
  return <Sketch className={className} draw={draw} deps={[rows, cell]} style={style} {...rest} />;
}

// TERMS

export function Terms({ terms, start = 0, marks, capped, tight, empty = '', onPick, label }) {
  if (!terms.length && empty) return <div className="ribbon"><span className="cap">{empty}</span></div>;
  return (
    <div className={tight ? 'ribbon tight' : 'ribbon'}>
      {label ? <span className="tag">{label}</span> : null}
      {terms.map((t, i) => (
        <span key={i} className={marks?.[i] ? 'gold' : undefined} role={onPick ? 'button' : undefined} onClick={onPick ? () => onPick(t, start + i) : undefined}>
          {tight ? null : <i>{start + i}</i>}
          <b>{text(t)}</b>
        </span>
      ))}
      {capped ? <span className="cap">to the budget</span> : null}
    </div>
  );
}
