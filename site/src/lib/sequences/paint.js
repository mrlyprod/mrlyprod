import { blend, rgb, veil } from '../color.js';
import { clamp01 } from '../gl/vec.js';
import { decade, diffs, digits as expand, label, negative, ratios, scale, sums, ticks } from './chart.js';
import { axisOf, measureOf } from './engine.js';

export const FACE = 'Noto Sans Mono';

export const GROW = 300;

const FADE = 0.62;

const DEPTH = 6;

const SIDE = 3;

const RIBBON = 4;

/* TONES */

export const shade = (accent, paper, i, m) => blend(accent, paper, m > 1 ? (i / (m - 1)) * FADE : 0);

export const ease = (v) => 1 - (1 - clamp01(v)) ** 3;

export function tones(view) {
  const { accent, paper } = view.look();
  const dpr = view.dpr;
  const px = Math.round(13 * dpr);
  const edge = { x: Math.round(16 * dpr), top: Math.round(16 * dpr), bottom: Math.round(16 * dpr) };
  return { accent, paper, dim: blend(accent, paper, 0.5), hair: veil(accent, 0.14), px, dpr, edge };
}

export const font = (tones, k = 1, weight = '') => `${weight ? `${weight} ` : ''}${Math.round(tones.px * k)}px "${FACE}", ui-monospace, monospace`;

export function clear(ctx, view, tones) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, view.w, view.h);
  ctx.font = font(tones);
  ctx.textBaseline = 'alphabetic';
  ctx.textAlign = 'left';
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
}

export const title = (row) => row.title ?? `${row.label ?? row.name} · ${measureOf(row.measure)?.label ?? row.measure} by ${row.axis}`;

/* GROWTH */

export function growth(view) {
  const seen = new Map();
  const at = (g) => g.from + (g.to - g.from) * ease((view.t - g.t) / GROW);
  const shown = (row) => {
    const n = row.terms.length;
    if (view.still) return n;
    let g = seen.get(row.key);
    if (!g) {
      g = { from: 0, to: n, t: view.t };
      seen.set(row.key, g);
    } else if (g.to !== n) {
      g.from = Math.min(at(g), n);
      g.to = n;
      g.t = view.t;
    }
    return at(g);
  };
  const busy = (rows) => !view.still && rows.some((row) => {
    const g = seen.get(row.key);
    return !g || g.to !== row.terms.length || view.t - g.t < GROW;
  });
  return { shown, busy };
}

/* TEXT */

export function wrap(ctx, text, width) {
  const lines = [];
  let line = '';
  for (const word of String(text).split(/\s+/).filter(Boolean)) {
    const next = line ? `${line} ${word}` : word;
    if (line && ctx.measureText(next).width > width) {
      lines.push(line);
      line = word;
    } else line = next;
  }
  if (line) lines.push(line);
  return lines;
}

export function fit(ctx, text, width) {
  if (ctx.measureText(text).width <= width) return text;
  let cut = text;
  while (cut.length > 1 && ctx.measureText(`${cut}…`).width > width) cut = cut.slice(0, -1);
  return `${cut}…`;
}

/* PARTS */

export const glyph = (cells) => {
  const slices = cells?.length === 27 ? 3 : cells?.length === 9 ? 1 : 0;
  return slices ? { slices, width: slices * SIDE + (slices - 1) } : null;
};

function mark(ctx, cells, x, y, unit, shade, hair) {
  const size = Math.max(1, unit - Math.max(1, unit / 6));
  cells.forEach((v, i) => {
    const slice = Math.floor(i / (SIDE * SIDE));
    const row = Math.floor(i / SIDE) % SIDE;
    const col = i % SIDE;
    ctx.fillStyle = v ? shade : hair;
    ctx.fillRect(x + (slice * (SIDE + 1) + col) * unit, y + row * unit, size, size);
  });
}

const said = (hit) => (hit ? `${hit.id}${hit.shift ? ` ${hit.shift > 0 ? '+' : ''}${hit.shift}` : ''} · ${hit.status === 'Collision' ? 'a collision with ' : ''}${hit.name}` : 'no curated record holds these terms');

export function legend(ctx, rows, shades, tones, x0, y0, width, notes = true) {
  const { px, dpr, hair, dim } = tones;
  const line = Math.round(px * 1.7);
  const unit = Math.round(px * 0.32);
  const dot = unit * SIDE;
  const hits = [];
  let x = x0;
  let y = y0 + px;
  ctx.textAlign = 'left';
  ctx.font = font(tones, 1.1, '500');
  rows.forEach((row, i) => {
    const text = title(row);
    const shape = glyph(row.cells);
    const swatch = shape ? shape.width * unit : dot;
    const w = Math.min(width, swatch + px * 0.6 + ctx.measureText(text).width);
    if (x > x0 && x + w > x0 + width) {
      x = x0;
      y += line;
    }
    if (shape) mark(ctx, row.cells, x, y - dot, unit, shades[i], hair);
    else {
      ctx.fillStyle = shades[i];
      ctx.fillRect(x, y - dot, dot, dot);
    }
    ctx.fillStyle = shades[i];
    ctx.fillText(fit(ctx, text, width - swatch - px * 0.6), x + swatch + px * 0.6, y);
    hits.push({ i, x0: x, y0: y - line * 0.8, x1: x + w, y1: y + line * 0.3 });
    x += w + px * 1.4;
  });
  ctx.font = font(tones);
  if (notes && rows.length === 1 && 'hit' in rows[0]) {
    y += Math.round(px * 1.5);
    ctx.fillStyle = dim;
    ctx.fillText(fit(ctx, said(rows[0].hit), width), x0, y);
  }
  return { height: y - y0 + Math.round(px * 0.9) + dpr, hits };
}

export function caption(ctx, text, x, y, width, tones, align = 'center') {
  ctx.font = font(tones, 0.95);
  ctx.fillStyle = tones.dim;
  ctx.textAlign = align;
  ctx.fillText(fit(ctx, text, width), align === 'center' ? x + width / 2 : x, y);
  ctx.textAlign = 'left';
  ctx.font = font(tones);
}

const marks = (rows, n) => {
  const axes = new Set(rows.map((row) => row.axis));
  const axis = axes.size === 1 && rows[0].axis ? axisOf(rows[0].axis) : null;
  return { marks: axis ? ticks(n, axis.start, axis.step) : ticks(n, rows[0]?.start ?? 1, 1), say: axis ? axis.say : (rows[0]?.say ?? 'term index') };
};

export function index(ctx, box, rows, tones, n, lead = 0) {
  const { px, dim } = tones;
  const { marks: at, say } = marks(rows, n - lead);
  const slot = box.w / n;
  ctx.fillStyle = dim;
  ctx.textAlign = 'center';
  for (const [i, text] of at) ctx.fillText(text, box.x + (i + lead + 0.5) * slot, box.y + box.h + px * 1.4);
  ctx.textAlign = 'right';
  ctx.fillText(say, box.x + box.w, box.y + box.h + px * 2.9);
  ctx.textAlign = 'left';
}

function rules(ctx, box, tones, sc) {
  const { px, dim, hair, dpr } = tones;
  ctx.strokeStyle = hair;
  ctx.lineWidth = dpr;
  ctx.fillStyle = dim;
  ctx.textAlign = 'right';
  const top = label(sc.max);
  const lines = sc.log ? Array.from({ length: sc.top - sc.floor + 1 }, (_, i) => [sc.at(`1${'0'.repeat(sc.floor + i)}`), decade(sc.floor + i)]) : top === '0' ? [[0, '0']] : [[0, '0'], [1, top]];
  for (const [f, text] of lines) {
    const y = box.y + box.h * (1 - f);
    ctx.beginPath();
    ctx.moveTo(box.x, y);
    ctx.lineTo(box.x + box.w, y);
    ctx.stroke();
    ctx.fillText(text, box.x - px * 0.5, y + px * 0.35);
  }
  ctx.textAlign = 'left';
  ctx.fillText(sc.log ? 'log scale' : 'linear scale', box.x - px * 3.4, box.y - px * 1.05);
}

function band(ctx, box, slot, at, tones) {
  if (at < 0) return;
  ctx.fillStyle = veil(tones.accent, 0.08);
  ctx.fillRect(box.x + at * slot, box.y, slot, box.h);
}

/* CHARTS */

function pins(ctx, box, rows, shades, tones, opts) {
  const { px, dpr, accent } = tones;
  const n = Math.max(1, ...rows.map((row) => row.terms.length));
  const m = rows.length;
  const sc = scale(rows, opts.kind === 'log');
  const slot = box.w / n;
  band(ctx, box, slot, opts.at, tones);
  rules(ctx, box, tones, sc);
  index(ctx, box, rows, tones, n);
  const spread = m > 1 ? (slot * 0.5) / (m - 1) : 0;
  const width = Math.max(1.5 * dpr, Math.min(4 * dpr, (slot * 0.14) / m));
  const head = Math.max(2.5 * dpr, Math.min(5 * dpr, (slot * 0.11) / m));
  const room = m === 1 && slot >= ctx.measureText(label(rows[0].terms[n - 1] ?? '')).width + px * 0.5;
  rows.forEach((row, i) => {
    const have = opts.shown(i);
    ctx.strokeStyle = shades[i];
    ctx.fillStyle = shades[i];
    ctx.lineWidth = width;
    row.terms.forEach((t, j) => {
      const g = clamp01(have - j);
      if (g <= 0) return;
      const x = box.x + (j + 0.5) * slot + (i - (m - 1) / 2) * spread;
      const tall = Math.max(box.h * sc.at(t), head) * ease(g);
      const y = box.y + box.h - tall;
      ctx.beginPath();
      ctx.moveTo(x, box.y + box.h);
      ctx.lineTo(x, y);
      ctx.stroke();
      ctx.beginPath();
      ctx.arc(x, y, j === opts.at && i === 0 ? head * 1.6 : head, 0, Math.PI * 2);
      if (negative(t)) {
        ctx.fillStyle = tones.paper;
        ctx.fill();
        ctx.fillStyle = shades[i];
        ctx.stroke();
      } else ctx.fill();
      if ((room || (j === opts.at && i === 0)) && g >= 1) {
        ctx.textAlign = 'center';
        ctx.fillStyle = accent;
        ctx.fillText(label(t), x, Math.max(box.y - px * 0.9, y - head - px * 0.5));
        ctx.fillStyle = shades[i];
      }
    });
  });
  ctx.textAlign = 'left';
  return { x: box.x, slot, n, lead: 0 };
}

function steps(ctx, box, rows, shades, tones, opts, running) {
  const { dpr } = tones;
  const series = rows.map((row) => ({ ...row, terms: running ? sums(row.terms) : row.terms }));
  const n = Math.max(1, ...series.map((row) => row.terms.length));
  const sc = scale(series);
  const slot = box.w / n;
  band(ctx, box, slot, opts.at, tones);
  rules(ctx, box, tones, sc);
  index(ctx, box, rows, tones, n);
  const trace = (row) => {
    ctx.beginPath();
    row.terms.forEach((t, j) => {
      const y = box.y + box.h * (1 - sc.at(t));
      if (j === 0) ctx.moveTo(box.x, y);
      else ctx.lineTo(box.x + j * slot, y);
      ctx.lineTo(box.x + (j + 1) * slot, y);
    });
  };
  series.forEach((row, i) => {
    ctx.save();
    ctx.beginPath();
    ctx.rect(box.x, box.y - dpr, Math.min(box.w, opts.shown(i) * slot), box.h + 2 * dpr);
    ctx.clip();
    trace(row);
    ctx.lineTo(box.x + row.terms.length * slot, box.y + box.h);
    ctx.lineTo(box.x, box.y + box.h);
    ctx.closePath();
    ctx.fillStyle = veil(shades[i], 0.12);
    ctx.fill();
    trace(row);
    ctx.strokeStyle = shades[i];
    ctx.lineWidth = 2 * dpr;
    ctx.stroke();
    ctx.restore();
  });
  return { x: box.x, slot, n, lead: 0 };
}

function ratioPlot(ctx, box, rows, shades, tones, opts) {
  const { px, dpr, dim, hair } = tones;
  const series = rows.map((row) => ratios(row.terms));
  const n = Math.max(1, ...series.map((one) => one.length));
  const all = series.flat().filter((v) => Number.isFinite(v));
  const lo = all.length ? Math.min(...all) : 0;
  const hi = all.length ? Math.max(...all) : 1;
  const gap = hi - lo || 1;
  const at = (v) => box.y + box.h * (1 - (v - lo + gap * 0.08) / (gap * 1.16));
  const slot = box.w / (n + 1);
  band(ctx, box, slot, opts.at, tones);
  ctx.strokeStyle = hair;
  ctx.lineWidth = dpr;
  ctx.fillStyle = dim;
  ctx.textAlign = 'right';
  const whole = Math.ceil(hi) - Math.floor(lo) <= 10;
  const lines = whole ? Array.from({ length: Math.ceil(hi) - Math.floor(lo) + 1 }, (_, i) => Math.floor(lo) + i) : [lo, hi];
  for (const v of lines) {
    const y = at(v);
    ctx.beginPath();
    ctx.moveTo(box.x, y);
    ctx.lineTo(box.x + box.w, y);
    ctx.stroke();
    ctx.fillText(whole ? String(v) : v.toFixed(2), box.x - px * 0.5, y + px * 0.35);
  }
  ctx.textAlign = 'left';
  ctx.fillText('each term over the one before', box.x - px * 3.4, box.y - px * 1.05);
  index(ctx, box, rows, tones, n + 1);
  series.forEach((one, i) => {
    const shown = Math.ceil(opts.shown(i)) - 1;
    ctx.strokeStyle = shades[i];
    ctx.fillStyle = shades[i];
    ctx.lineWidth = 2 * dpr;
    ctx.beginPath();
    let open = false;
    one.slice(0, shown).forEach((v, j) => {
      if (!Number.isFinite(v)) {
        open = false;
        return;
      }
      const x = box.x + (j + 1.5) * slot;
      if (open) ctx.lineTo(x, at(v));
      else ctx.moveTo(x, at(v));
      open = true;
    });
    ctx.stroke();
    one.slice(0, shown).forEach((v, j) => {
      if (!Number.isFinite(v)) return;
      ctx.beginPath();
      ctx.arc(box.x + (j + 1.5) * slot, at(v), (j + 1 === opts.at && i === 0 ? 4.5 : 3) * dpr, 0, Math.PI * 2);
      ctx.fill();
    });
  });
  return { x: box.x, slot, n: n + 1, lead: 0 };
}

function bands(rows, box, px) {
  const gap = Math.round(px * 0.8);
  const h = (box.h - gap * (rows.length - 1)) / rows.length;
  return rows.map((_, i) => ({ y: box.y + i * (h + gap), h }));
}

function triangles(ctx, box, rows, shades, tones, opts) {
  const { px, dpr, paper } = tones;
  const lanes = bands(rows, box, px);
  let cellOf = 0;
  rows.forEach((row, i) => {
    const tri = diffs(row.terms, DEPTH);
    const n = Math.max(1, tri[0].length);
    const lane = lanes[i];
    const cell = Math.max(2 * dpr, Math.min(box.w / n, lane.h / tri.length, px * 2.6));
    if (i === 0) cellOf = cell;
    const tops = tri.map((line) => scale([{ terms: line }]));
    const have = opts.shown(i);
    tri.forEach((line, r) => {
      const y = lane.y + r * cell;
      line.forEach((v, j) => {
        if (j + r >= have) return;
        const x = box.x + (j + r / 2) * cell;
        const f = tops[r].at(v);
        ctx.fillStyle = veil(shades[i], 0.2 + 0.75 * f);
        if (negative(v)) {
          ctx.fillStyle = paper;
          ctx.fillRect(x, y, Math.max(1, cell - dpr), Math.max(1, cell - dpr));
          ctx.strokeStyle = veil(shades[i], 0.3 + 0.7 * f);
          ctx.lineWidth = dpr;
          ctx.strokeRect(x + dpr / 2, y + dpr / 2, Math.max(1, cell - 2 * dpr), Math.max(1, cell - 2 * dpr));
        } else ctx.fillRect(x, y, Math.max(1, cell - dpr), Math.max(1, cell - dpr));
        const text = label(v);
        if (cell >= ctx.measureText(text).width + px * 0.4) {
          ctx.fillStyle = f > 0.5 && !negative(v) ? paper : shades[i];
          ctx.textAlign = 'center';
          ctx.fillText(text, x + cell / 2, y + cell / 2 + px * 0.35);
        }
      });
    });
  });
  ctx.textAlign = 'left';
  ctx.fillStyle = tones.dim;
  ctx.fillText('each row the differences of the row above', box.x, box.y + box.h + px * 1.4);
  return { x: box.x, slot: cellOf || 1, n: Math.max(1, ...rows.map((row) => row.terms.length)), lead: 0 };
}

function heat(ctx, box, rows, shades, tones, opts) {
  const { px, dpr, hair } = tones;
  const base = opts.base;
  const lanes = bands(rows, box, px);
  rows.forEach((row, i) => {
    const lines = row.terms.map((t) => expand(t, base));
    const cols = Math.max(1, ...lines.map((one) => one.length));
    const lane = lanes[i];
    const cell = Math.max(2 * dpr, Math.min(box.w / cols, lane.h / Math.max(1, lines.length), px * 1.3));
    const show = Math.ceil(opts.shown(i));
    const right = box.x + Math.min(box.w, cols * cell);
    lines.slice(0, show).forEach((line, j) => {
      const y = lane.y + j * cell;
      if (j === opts.at && i === 0) {
        ctx.fillStyle = veil(tones.accent, 0.1);
        ctx.fillRect(box.x, y, right - box.x, cell);
      }
      line.forEach((d, c) => {
        const x = right - (line.length - c) * cell;
        ctx.fillStyle = d ? veil(shades[i], 0.3 + (0.7 * d) / (base - 1)) : hair;
        ctx.fillRect(x, y, Math.max(1, cell - dpr), Math.max(1, cell - dpr));
      });
    });
  });
  ctx.fillStyle = tones.dim;
  ctx.fillText(`every term spelt in base ${base}, one row a term`, box.x, box.y + box.h + px * 1.4);
  return null;
}

export const KIND = {
  pins,
  log: pins,
  steps: (ctx, box, rows, shades, tones, opts) => steps(ctx, box, rows, shades, tones, opts, false),
  sums: (ctx, box, rows, shades, tones, opts) => steps(ctx, box, rows, shades, tones, opts, true),
  ratios: ratioPlot,
  diffs: triangles,
  digits: heat,
};

export const gutter = (kind, tones) => (kind === 'diffs' || kind === 'digits' ? 0 : Math.round(tones.px * 3.8));

export function chart(ctx, box, rows, shades, tones, opts) {
  const room = gutter(opts.kind, tones);
  const inner = { x: box.x + room, y: box.y + Math.round(tones.px * 1.4), w: Math.max(1, box.w - room), h: Math.max(1, box.h - Math.round(tones.px * 4.4)) };
  const slots = (KIND[opts.kind] ?? pins)(ctx, inner, rows, shades, tones, opts);
  return slots && { ...slots, y0: inner.y, y1: inner.y + inner.h + tones.px * 2 };
}

export function slotAt(slots, x, y) {
  if (!slots || y < slots.y0 || y > slots.y1) return -1;
  const j = Math.floor((x - slots.x) / slots.slot);
  return j >= 0 && j < slots.n ? j : -1;
}

/* RIBBON */

function items(ctx, row, tones, short) {
  const axis = row.axis ? axisOf(row.axis) : { start: row.start ?? 1, step: 1 };
  return row.terms.map((t, j) => {
    const text = short ? label(t) : t;
    ctx.font = font(tones, 0.75);
    const head = String(axis.start + j * axis.step);
    const wide = ctx.measureText(head).width;
    ctx.font = font(tones);
    return { j, head, text, w: Math.max(wide, ctx.measureText(text).width) + tones.px * 0.9, prime: row.marks?.[j] };
  });
}

function lay(list, width) {
  const lines = [[]];
  let x = 0;
  for (const one of list) {
    if (x > 0 && x + one.w > width) {
      lines.push([]);
      x = 0;
    }
    lines.at(-1).push({ ...one, x });
    x += one.w;
  }
  return lines;
}

export function ribbon(ctx, row, tones, width, most = RIBBON) {
  let lines = lay(items(ctx, row, tones, false), width);
  if (lines.length > most) lines = lay(items(ctx, row, tones, true), width);
  const cut = lines.length > most;
  const kept = lines.slice(0, most);
  const line = Math.round(tones.px * 2.3);
  return { lines: kept, cut, height: kept.length * line, line };
}

export function drawRibbon(ctx, laid, x, y, tones, at = -1, color = tones.accent) {
  const { px } = tones;
  laid.lines.forEach((line, r) => {
    const top = y + r * laid.line;
    for (const one of line) {
      const on = one.j === at;
      ctx.font = font(tones, 0.75);
      ctx.fillStyle = tones.dim;
      ctx.fillText(one.head, x + one.x, top + px * 0.8);
      ctx.font = font(tones, 1, on ? '700' : '');
      ctx.fillStyle = one.prime ? blend(color, '#f5b400', 0.85) : on ? color : blend(color, tones.paper, 0.15);
      ctx.fillText(one.text, x + one.x, top + px * 1.9);
    }
  });
  if (laid.cut) {
    ctx.fillStyle = tones.dim;
    ctx.fillText('…', x, y + laid.height + px * 0.6);
  }
  ctx.font = font(tones);
}
