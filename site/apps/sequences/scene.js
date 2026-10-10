import { tidy } from '../../lib/knobs.js';
import { rgb, veil } from '../../lib/scene.js';
import { KINDS, decade, diffs, digits as expand, label, negative, ratios, scale, sums, ticks } from './chart.js';
import { AXES, MEASURES } from './engine.js';

export const SPEC = [
  { key: 'dim', label: 'Space', kind: 'segment', def: 2, options: [[2, 'Plane'], [3, 'Cube']], group: 'Designs' },
  { key: 'base', label: 'Base', kind: 'segment', def: 2, options: (value) => (value.dim === 3 ? [[2, '2']] : [[2, '2'], [3, '3']]), group: 'Designs', when: (value) => value.dim !== 3 },
  { key: 'measure', label: 'Measure', kind: 'pick', def: '', options: [['', 'Any'], ...MEASURES.map((one) => [one.slug, one.label])], group: 'Reading' },
  { key: 'axis', label: 'Axis', kind: 'segment', def: '', options: [['', 'Both'], ...AXES.map((one) => [one.slug, one.label])], group: 'Reading' },
  { key: 'chart', label: 'Chart', kind: 'pick', def: 'pins', options: KINDS, group: 'Chart' },
  { key: 'digits', label: 'Digits', kind: 'segment', def: 2, options: [[2, '2'], [3, '3'], [10, '10']], group: 'Chart' },
  { key: 'terms', label: 'Terms', kind: 'slider', def: 12, min: 4, max: 24, step: 1, group: 'Chart' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: ['ArrowDown', 'ArrowUp'], label: 'Browse', act: 'browse' },
    { key: '/', label: 'Search', act: 'search' },
    { key: 'c', label: 'Chart', act: 'cycle' },
  ],
  actions: {
    browse: (scene, e) => scene.browse?.(e.key === 'ArrowUp' ? -1 : 1),
    search: (scene) => scene.find?.(),
    cycle: (scene) => scene.cycle?.(),
  },
};

const FACE = 'Noto Sans Mono';
const REVEAL = 900;
const TAIL = 4;
const FADE = 0.62;
const DEPTH = 6;
const SIDE = 3;

/* TONES */

export function mix(a, b, t) {
  const [x, y] = [rgb(a), rgb(b)];
  const k = Math.max(0, Math.min(1, t));
  return `rgb(${x.map((c, i) => Math.round(c + (y[i] - c) * k)).join(', ')})`;
}

export const shade = (accent, paper, i, m) => mix(accent, paper, m > 1 ? (i / (m - 1)) * FADE : 0);

const clamp = (v) => Math.max(0, Math.min(1, v));

const ease = (v) => 1 - (1 - clamp(v)) ** 3;

export const grow = (k, j, n) => ease((k * (n + TAIL) - j) / TAIL);

const measureOf = (slug) => MEASURES.find((one) => one.slug === slug)?.label ?? slug;

const axisOf = (slug) => AXES.find((one) => one.slug === slug) ?? AXES[0];

export const title = (row) => `${row.label ?? row.name} · ${measureOf(row.measure)} by ${row.axis}`;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const { chart, digits: base } = tidy(SPEC, opts);
  const feed = () => opts.live?.current ?? opts;
  let seen = null;
  let shown = -1;
  const paint = (k, rows) => {
    const { accent, paper } = view.look();
    const dpr = view.dpr;
    const px = Math.round(12 * dpr);
    const pad = Math.round(14 * dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.font = `${px}px "${FACE}", ui-monospace, monospace`;
    ctx.textBaseline = 'alphabetic';
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
    const dim = mix(accent, paper, 0.5);
    const hair = veil(accent, 0.14);
    const tones = { accent, paper, dim, hair, px, dpr };
    if (!rows.length) {
      ctx.fillStyle = dim;
      ctx.textAlign = 'center';
      ctx.fillText('pick a row', view.w / 2, view.h / 2);
      return;
    }
    const shades = rows.map((_, i) => shade(accent, paper, i, rows.length));
    const used = legend(ctx, rows, shades, tones, pad, view.w - 2 * pad);
    const gutter = chart === 'pins' || chart === 'steps' || chart === 'sums' || chart === 'ratios' ? Math.round(px * 3.6) : 0;
    const box = { x: pad + gutter, y: pad + used, w: Math.max(1, view.w - 2 * pad - gutter), h: Math.max(1, view.h - 2 * pad - used - Math.round(px * 1.8)) };
    KIND[chart](ctx, box, rows, shades, tones, k, base);
  };
  const draw = () => {
    const k = view.still ? 1 : clamp(view.t / REVEAL);
    const data = feed();
    const version = data.version ?? 0;
    if (!view.still && k === shown && version === seen) return;
    shown = k;
    seen = version;
    paint(k, Array.isArray(data.rows) ? data.rows : []);
  };
  const again = () => {
    shown = -1;
    draw();
  };
  return { draw, size: again, theme: again, font: FACE };
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

function legend(ctx, rows, shades, tones, pad, width) {
  const { px, dpr, hair } = tones;
  const line = Math.round(px * 1.6);
  const unit = Math.round(px * 0.3);
  const dot = unit * SIDE;
  let x = pad;
  let y = pad + px;
  ctx.textAlign = 'left';
  rows.forEach((row, i) => {
    const text = title(row);
    const shape = glyph(row.cells);
    const swatch = shape ? shape.width * unit : dot;
    const w = swatch + px * 0.6 + ctx.measureText(text).width;
    if (x > pad && x + w > pad + width) {
      x = pad;
      y += line;
    }
    ctx.fillStyle = shades[i];
    if (shape) mark(ctx, row.cells, x, y - dot, unit, shades[i], hair);
    else ctx.fillRect(x, y - dot, dot, dot);
    ctx.fillStyle = shades[i];
    ctx.fillText(text, x + swatch + px * 0.6, y);
    x += w + px * 1.4;
  });
  return y - pad + Math.round(px * 1.2) + dpr;
}

function index(ctx, box, rows, tones, n) {
  const { px, dim } = tones;
  const axes = new Set(rows.map((row) => row.axis));
  const axis = axes.size === 1 ? axisOf(rows[0].axis) : null;
  const marks = axis ? ticks(n, axis.start, axis.step) : ticks(n, 1, 1);
  const slot = box.w / n;
  ctx.fillStyle = dim;
  ctx.textAlign = 'center';
  for (const [i, text] of marks) ctx.fillText(text, box.x + (i + 0.5) * slot, box.y + box.h + px * 1.4);
  ctx.textAlign = 'left';
}

function rules(ctx, box, tones, sc) {
  const { px, dim, hair, dpr } = tones;
  ctx.strokeStyle = hair;
  ctx.lineWidth = dpr;
  ctx.fillStyle = dim;
  ctx.textAlign = 'right';
  const top = label(sc.max);
  const marks = sc.log ? Array.from({ length: sc.top - sc.floor + 1 }, (_, i) => [sc.at(`1${'0'.repeat(sc.floor + i)}`), decade(sc.floor + i)]) : top === '0' ? [[0, '0']] : [[0, '0'], [1, top]];
  for (const [f, text] of marks) {
    const y = box.y + box.h * (1 - f);
    ctx.beginPath();
    ctx.moveTo(box.x, y);
    ctx.lineTo(box.x + box.w, y);
    ctx.stroke();
    ctx.fillText(text, box.x - px * 0.5, y + px * 0.35);
  }
  ctx.textAlign = 'left';
}

function pins(ctx, box, rows, shades, tones, k) {
  const { px, dpr, accent } = tones;
  const n = Math.max(1, ...rows.map((row) => row.terms.length));
  const m = rows.length;
  const sc = scale(rows);
  rules(ctx, box, tones, sc);
  index(ctx, box, rows, tones, n);
  const slot = box.w / n;
  const spread = m > 1 ? (slot * 0.5) / (m - 1) : 0;
  const width = Math.max(1.5 * dpr, Math.min(4 * dpr, (slot * 0.14) / m));
  const head = Math.max(2.5 * dpr, Math.min(5 * dpr, (slot * 0.11) / m));
  const room = m === 1 && slot >= ctx.measureText(label(rows[0].terms[n - 1] ?? '')).width + px * 0.5;
  rows.forEach((row, i) => {
    ctx.strokeStyle = shades[i];
    ctx.fillStyle = shades[i];
    ctx.lineWidth = width;
    row.terms.forEach((t, j) => {
      const g = grow(k, j, n);
      if (g <= 0) return;
      const x = box.x + (j + 0.5) * slot + (i - (m - 1) / 2) * spread;
      const tall = Math.max(box.h * sc.at(t), head) * g;
      const y = box.y + box.h - tall;
      ctx.beginPath();
      ctx.moveTo(x, box.y + box.h);
      ctx.lineTo(x, y);
      ctx.stroke();
      ctx.beginPath();
      ctx.arc(x, y, head, 0, Math.PI * 2);
      if (negative(t)) {
        ctx.fillStyle = tones.paper;
        ctx.fill();
        ctx.fillStyle = shades[i];
        ctx.stroke();
      } else ctx.fill();
      if (room && g >= 1) {
        ctx.textAlign = 'center';
        ctx.fillStyle = accent;
        ctx.fillText(label(t), x, Math.max(box.y + px, y - head - px * 0.5));
        ctx.fillStyle = shades[i];
      }
    });
  });
  ctx.textAlign = 'left';
}

function steps(ctx, box, rows, shades, tones, k, running) {
  const { dpr } = tones;
  const series = rows.map((row) => ({ ...row, terms: running ? sums(row.terms) : row.terms }));
  const n = Math.max(1, ...series.map((row) => row.terms.length));
  const sc = scale(series);
  rules(ctx, box, tones, sc);
  index(ctx, box, rows, tones, n);
  const slot = box.w / n;
  ctx.save();
  ctx.beginPath();
  ctx.rect(box.x, box.y - dpr, box.w * k, box.h + 2 * dpr);
  ctx.clip();
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
  });
  ctx.restore();
}

function ratioPlot(ctx, box, rows, shades, tones, k) {
  const { px, dpr, dim, hair } = tones;
  const series = rows.map((row) => ratios(row.terms));
  const n = Math.max(1, ...series.map((one) => one.length));
  const all = series.flat().filter((v) => Number.isFinite(v));
  const lo = all.length ? Math.min(...all) : 0;
  const hi = all.length ? Math.max(...all) : 1;
  const gap = hi - lo || 1;
  const at = (v) => box.y + box.h * (1 - (v - lo + gap * 0.08) / (gap * 1.16));
  ctx.strokeStyle = hair;
  ctx.lineWidth = dpr;
  ctx.fillStyle = dim;
  ctx.textAlign = 'right';
  const whole = Math.ceil(hi) - Math.floor(lo) <= 10;
  const marks = whole ? Array.from({ length: Math.ceil(hi) - Math.floor(lo) + 1 }, (_, i) => Math.floor(lo) + i) : [lo, hi];
  for (const v of marks) {
    const y = at(v);
    ctx.beginPath();
    ctx.moveTo(box.x, y);
    ctx.lineTo(box.x + box.w, y);
    ctx.stroke();
    ctx.fillText(whole ? String(v) : v.toFixed(2), box.x - px * 0.5, y + px * 0.35);
  }
  ctx.textAlign = 'left';
  index(ctx, box, rows.map((row) => ({ ...row, axis: row.axis })), tones, n + 1);
  const slot = box.w / (n + 1);
  const shown = Math.ceil(k * n);
  series.forEach((one, i) => {
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
      ctx.arc(box.x + (j + 1.5) * slot, at(v), 3 * dpr, 0, Math.PI * 2);
      ctx.fill();
    });
  });
}

function bands(rows, box, px) {
  const gap = Math.round(px * 0.8);
  const h = (box.h - gap * (rows.length - 1)) / rows.length;
  return rows.map((_, i) => ({ y: box.y + i * (h + gap), h }));
}

function triangles(ctx, box, rows, shades, tones, k) {
  const { px, dpr, paper } = tones;
  const lanes = bands(rows, box, px);
  rows.forEach((row, i) => {
    const tri = diffs(row.terms, DEPTH);
    const n = Math.max(1, tri[0].length);
    const lane = lanes[i];
    const cell = Math.max(2 * dpr, Math.min(box.w / n, lane.h / tri.length, px * 2.4));
    const tops = tri.map((line) => scale([{ terms: line }]));
    const show = Math.floor(k * tri.length + 1e-9);
    tri.forEach((line, r) => {
      if (r >= show) return;
      const y = lane.y + r * cell;
      line.forEach((v, j) => {
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
}

function heat(ctx, box, rows, shades, tones, k, base) {
  const { px, dpr, hair } = tones;
  const lanes = bands(rows, box, px);
  rows.forEach((row, i) => {
    const lines = row.terms.map((t) => expand(t, base));
    const cols = Math.max(1, ...lines.map((one) => one.length));
    const lane = lanes[i];
    const cell = Math.max(2 * dpr, Math.min(box.w / cols, lane.h / Math.max(1, lines.length), px * 1.2));
    const show = Math.ceil(k * lines.length);
    const right = box.x + Math.min(box.w, cols * cell);
    lines.slice(0, show).forEach((line, j) => {
      const y = lane.y + j * cell;
      line.forEach((d, c) => {
        const x = right - (line.length - c) * cell;
        ctx.fillStyle = d ? veil(shades[i], 0.3 + (0.7 * d) / (base - 1)) : hair;
        ctx.fillRect(x, y, Math.max(1, cell - dpr), Math.max(1, cell - dpr));
      });
    });
  });
}

const KIND = {
  pins,
  steps: (ctx, box, rows, shades, tones, k) => steps(ctx, box, rows, shades, tones, k, false),
  sums: (ctx, box, rows, shades, tones, k) => steps(ctx, box, rows, shades, tones, k, true),
  ratios: ratioPlot,
  diffs: triangles,
  digits: heat,
};
