import { tidy } from '../../lib/app/spec.js';
import { KINDS } from '../../lib/sequences/chart.js';
import { MEASURES, TERMS, applies, print } from '../../lib/sequences/engine.js';
import { FACE, chart, clear, drawRibbon, fit, font, growth, legend, ribbon, shade, slotAt, tones } from '../../lib/sequences/paint.js';
import { ARG, OPS, PAIRED, TAKES, decimal } from './engine.js';

const measures = (value) => MEASURES.filter((one) => applies(one, value.dim ?? 2, value.base ?? 2)).map((one) => [one.slug, one.label]);

const mixed = (value) => Number(value.mix) === 1;

export const SPEC = [
  { key: 'measure', label: 'Measure', kind: 'pick', def: 'surface', options: measures, group: 'Sequence' },
  { key: 'axis', label: 'Axis', kind: 'segment', def: 'level', options: [['level', 'Level'], ['side', 'Odd side']], group: 'Sequence' },
  { key: 'terms', label: 'Terms', kind: 'slider', def: 12, min: TERMS.min, max: TERMS.max, step: 1, group: 'Sequence' },
  { key: 'chart', label: 'Chart', kind: 'pick', def: 'pins', options: KINDS, group: 'View' },
  { key: 'digits', label: 'Digits', kind: 'segment', def: 2, options: [[2, '2'], [3, '3'], [10, '10']], group: 'View', when: (value) => value.chart === 'digits' },
  { key: 'mix', label: 'Mix a second', kind: 'toggle', def: 0, group: 'Mix' },
  { key: 'op', label: 'Operation', kind: 'pick', def: 'hadamard', options: OPS, group: 'Mix', when: mixed },
  { key: 'scode', label: 'Second code', kind: 'text', def: '3', group: 'Mix', when: (value) => mixed(value) && PAIRED.includes(value.op) },
  { key: 'smeasure', label: 'Its measure', kind: 'pick', def: 'fills', options: measures, group: 'Mix', when: (value) => mixed(value) && PAIRED.includes(value.op) },
  { key: 'arg', label: 'Argument', kind: 'slider', def: 2, min: (value) => (value.op === 'scale' ? ARG.min : value.op === 'decimate' ? 2 : 1), max: ARG.max, step: 1, group: 'Mix', when: (value) => mixed(value) && TAKES.includes(value.op) },
];

export const filed = (value) => {
  const dim = Number(value.dim);
  const row = print({ dim, base: dim === 3 ? 2 : value.base, code: String(value.code ?? '').replace(/\W+/g, ''), measure: value.measure, axis: value.axis });
  return ['plot', row, ...(mixed(value) ? [value.op] : []), value.chart].join('-');
};

export const PAGE = {
  spec: SPEC,
  name: filed,
  about: [
    'One sequence of the ledger drawn large. Pick a design, a measure and an axis, and the terms stand up as pins, a log plot, steps, partial sums, ratios, a difference triangle or a heatmap of their digits.',
    'Under the chart is the rule: the smallest linear recurrence every term satisfies, its characteristic polynomial and its largest real root, the growth. A rule needs about twice its order plus two terms before it can be trusted, so a short read may find none where a longer one would.',
    'The mix runs a term operation over this sequence and a second one from the same space and axis: add, subtract, multiply, convolve, or shift, thin, difference, sum and scale the first alone. Sequences with linear recurrences are closed under every one of these, so the mix has a rule of its own.',
    'Terms are exact. The rule and the mix work within 128 bits and say where they stop.',
  ],
  keys: [
    { key: 'Enter', label: 'Mix', act: 'mix', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Design', act: 'design' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Terms', act: 'terms' },
    { key: 'c', label: 'Chart', act: 'cycle' },
  ],
  actions: {
    mix: (scene) => scene.mix?.(),
    design: (scene, e) => scene.design?.(e.key === 'ArrowLeft' ? -1 : 1),
    terms: (scene, e) => scene.terms?.(e.key === 'ArrowDown' ? -1 : 1),
    cycle: (scene) => scene.cycle?.(),
  },
};

/* RULE */

export function lines(rule) {
  if (rule.error) return [`no rule read: ${rule.error}`, ''];
  if (rule.order === null) return ['no linear rule fits these terms', `read from ${rule.read} terms`];
  const growth = Number.isFinite(rule.growth) ? `growth ${decimal(rule.growth)}, ${decimal(rule.exponent)} decades a term` : rule.from;
  return [rule.recurrence, `${rule.polynomial} · ${growth} · read from ${rule.read} terms`];
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  let { chart: kind, digits: base } = tidy(SPEC, opts);
  const feed = () => opts.live?.current ?? opts;
  const grown = growth(view);
  let seen = null;
  let dirty = true;
  let moving = false;
  let picked = [];
  let hits = [];
  const panel = (one, box, i, t, most) => {
    const { px } = t;
    const row = one.row;
    const color = shade(t.accent, t.paper, i, 2);
    const top = legend(ctx, [row], [color], t, box.x, box.y, box.w, true);
    const laid = ribbon(ctx, row, t, box.w, most);
    const rule = lines(one.rule);
    const tail = laid.height + px * 4.4;
    const area = { x: box.x, y: box.y + top.height, w: box.w, h: Math.max(px * 6, box.h - top.height - tail) };
    const at = picked[i] ?? -1;
    const slots = row.terms.length ? chart(ctx, area, [row], [color], t, { kind, base, at, shown: () => grown.shown(row) }) : null;
    if (!row.terms.length) {
      ctx.fillStyle = t.dim;
      ctx.fillText(one.note || 'no terms read', area.x, area.y + px * 2);
    }
    const y = area.y + area.h + px * 0.6;
    drawRibbon(ctx, laid, box.x, y, t, at, color);
    ctx.font = font(t, 1.05, '500');
    ctx.fillStyle = t.accent;
    ctx.fillText(fit(ctx, rule[0], box.w), box.x, y + laid.height + px * 1.6);
    ctx.font = font(t, 0.9);
    ctx.fillStyle = t.dim;
    ctx.fillText(fit(ctx, rule[1], box.w), box.x, y + laid.height + px * 3.1);
    if (one.note && row.terms.length) ctx.fillText(fit(ctx, one.note, box.w), box.x, y + laid.height + px * 4.3);
    ctx.font = font(t);
    return slots;
  };
  const paint = (panels) => {
    const t = tones(view);
    clear(ctx, view, t);
    const pad = Math.round(16 * view.dpr);
    const gap = Math.round(t.px * 2);
    if (!panels.length) {
      ctx.fillStyle = t.dim;
      ctx.textAlign = 'center';
      ctx.fillText('reading', view.w / 2, view.h / 2);
      ctx.textAlign = 'left';
      hits = [];
      return;
    }
    const inner = { x: pad, y: pad, w: view.w - 2 * pad, h: view.h - 2 * pad };
    const two = panels.length > 1;
    const across = two && inner.w > inner.h * 1.3;
    const boxes = panels.map((_, i) => {
      if (!two) return inner;
      if (across) return { x: inner.x + i * ((inner.w + gap) / 2), y: inner.y, w: (inner.w - gap) / 2, h: inner.h };
      return { x: inner.x, y: inner.y + i * ((inner.h + gap) / 2), w: inner.w, h: (inner.h - gap) / 2 };
    });
    hits = panels.map((one, i) => panel(one, boxes[i], i, t, two ? 2 : 3));
  };
  const draw = () => {
    const data = feed();
    const panels = Array.isArray(data.panels) ? data.panels : [];
    const version = data.version ?? 0;
    const rows = panels.map((one) => one.row);
    const busy = grown.busy(rows);
    if (!view.still && !dirty && version === seen && !busy && !moving) return;
    moving = busy;
    if (version !== seen) picked = picked.map((at, i) => (at < (rows[i]?.terms.length ?? 0) ? at : -1));
    seen = version;
    dirty = false;
    paint(panels);
  };
  const again = () => {
    dirty = true;
    draw();
  };
  const set = (value) => {
    ({ chart: kind, digits: base } = tidy(SPEC, value));
    dirty = true;
  };
  const tap = (x, y) => {
    for (let i = 0; i < hits.length; i++) {
      const j = slotAt(hits[i], x, y);
      if (j >= 0) {
        picked[i] = picked[i] === j ? -1 : j;
        again();
        return { panel: i, term: picked[i] };
      }
    }
    return null;
  };
  return { draw, size: again, theme: again, set, tap, font: FACE };
}
