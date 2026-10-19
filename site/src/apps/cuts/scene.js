import { tidy } from '../../lib/app/spec.js';
import { TINTS, rgb } from '../../lib/color.js';
import { canvas as pen, frame as box, hex, svg as sheet } from '#view';
import { design, memo, named, tag as title } from '../../lib/three/engine.js';
import { wasm } from '../../lib/units/index.js';

export const GASKET = '126';
export const NUMBER = 2;
export const REACH = { points: 512, section: 64 };
const SPAN = 512;
const PAD = 0.04;
const FAINT = 18;
const SHELF = 48;
const FIT = 0.9;
const TAU = Math.PI * 2;
const ROOT6 = Math.sqrt(6);

export const lid = (math, view) => math.bang.level_cap(NUMBER, 1, REACH[view] ?? REACH.points);

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'points', options: [['points', 'Points'], ['section', 'Section']], group: 'Cut' },
  { key: 'level', label: 'Level', kind: 'slider', def: 5, min: 1, max: (value, units) => units && lid(units.math, value.view), step: 1, group: 'Cut' },
  { key: 'both', label: 'Both central heights', kind: 'toggle', def: 0, group: 'Cut' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look', when: (value) => value.view === 'section' },
];

export const units = wasm('math');

export const name = (value) => `cuts-${String(value.code ?? '').trim() || GASKET}-l${value.level}-${value.view}`;

export const PAGE = {
  defaults: { base: 2, code: '126', height: '' },
  design: { dim: 3, base: 'base', code: 'code' },
  spec: SPEC,
  units,
  gestures: true,
  about: [
    'A plane x + y + z = s slides through the solid of code 126 and meets exactly 3^level cells at every height it meets at all.',
    'The binary digits of the height above the support say which corners each scale may use, so every cut is a Sierpinski gasket, and the two central heights together fall into six gaskets tiling a hexagon.',
    'Drag across the picture or press Left and Right to move the plane. A height is counted without building a cell, so the points reach level 9. More in [the cuts note](/research/cuts/).',
  ],
  name,
  keys: [
    { key: 'ArrowLeft', label: 'Height down', act: 'lower', menu: 'View' },
    { key: 'ArrowRight', label: 'Height up', act: 'higher', menu: 'View' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    lower: (scene) => scene.slide?.(-1),
    higher: (scene) => scene.slide?.(1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

/* STUDY */

const profiles = memo(6);
const drawings = memo(4);

export function profile(math, code, base, level) {
  return profiles(`${code} ${base} ${level}`, () => {
    const counts = math.three.profile(code, NUMBER, level, base).map(Number);
    const side = Number(math.counts.grid(NUMBER, 1, level));
    const support = math.three.support(counts);
    if (!support) return { counts, side, empty: true };
    const [low, high] = support;
    const span = counts.slice(low, high + 1);
    const live = span.filter((count) => count > 0);
    const least = Math.min(...live);
    const most = Math.max(...live);
    const mid = Math.floor((low + high) / 2);
    return { counts, side, low, high, span, nonempty: live.length, heights: span.length, least, most, central: [mid, Math.min(high, mid + 1)] };
  });
}

export function heights(cut, value) {
  if (cut.empty) return [];
  if (Number(value.both) === 1) return cut.central;
  const asked = String(value.height ?? '').trim();
  const at = asked === '' || !Number.isFinite(Number(asked)) ? cut.central[0] : Math.round(Number(asked));
  return [Math.min(cut.high, Math.max(cut.low, at))];
}

export function points(math, code, base, level, list) {
  return drawings(`${code} ${base} ${level} ${list}`, () => math.three.diagonal_svg(code, NUMBER, level, base, Uint32Array.from(list), Math.max(1, Math.round(SPAN / 2 ** level))));
}

export function section(math, code, base, level) {
  return drawings(`section ${code} ${base} ${level}`, () => math.six.cut_design(code, NUMBER, level, base));
}

export function study(math, value) {
  const { base, code } = design(math, { ...value, code: String(value.code ?? '').trim() || GASKET });
  const level = Math.min(value.level, lid(math, value.view));
  const cut = profile(math, code, base, level);
  const list = heights(cut, value);
  const counts = list.map((h) => cut.counts[h] ?? 0);
  const drawn = value.view === 'section' ? '-' : String(counts.reduce((a, b) => a + b, 0));
  const facts = cut.empty
    ? [['Name', title(named(math, code, base), code, base)], ['Side', cut.side], ['Support', 'none']]
    : [
        ['Name', title(named(math, code, base), code, base)],
        ['Height', list.join(' and ')],
        ['This slice', counts.join(' and ')],
        ['Offset in binary', list.map((h) => (h - cut.low).toString(2)).join(' and ')],
        ['Side', cut.side],
        ['Support', `[${cut.low}, ${cut.high}]`],
        ['Non-empty', `${cut.nonempty} of ${cut.heights}`],
        ['Min', cut.least],
        ['Max', cut.most],
        ['Points drawn', drawn],
      ];
  const art = cut.empty ? null : value.view === 'section' ? { mesh: section(math, code, base, level) } : { svg: points(math, code, base, level, list) };
  return { code, base, level, cut, heights: list, rows: facts, art, view: value.view };
}

/* SHELF */

export function shelf(cut, count = SHELF) {
  if (cut.empty) return [];
  const { low, high } = cut;
  if (high - low < count) return Array.from({ length: high - low + 1 }, (_, i) => low + i);
  return Array.from({ length: count }, (_, i) => low + Math.round((i * (high - low)) / (count - 1)));
}

export function near(list, at) {
  return list.reduce((best, one) => (Math.abs(one - at) < Math.abs(best - at) ? one : best), list[0]);
}

export function plot(math, code, base, level, height) {
  return math.three.diagonal_slice(code, NUMBER, level, base, height).map(([x, y, z]) => [(x - y) / Math.SQRT2, (x + y - 2 * z) / ROOT6]);
}

/* PAINT */

export function thumb(ctx, side, dots, edge, accent) {
  const reach = (edge - 1) * Math.sqrt(2 / 3);
  const scale = (FIT * side) / 2 / (reach + Math.SQRT1_2);
  const radius = Math.max(0.5, scale * Math.SQRT1_2 * FIT);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  ctx.fillStyle = accent;
  ctx.beginPath();
  for (const [u, v] of dots) {
    const x = side / 2 + u * scale;
    const y = side / 2 - v * scale;
    ctx.moveTo(x + radius, y);
    ctx.arc(x, y, radius, 0, TAU);
  }
  ctx.fill();
}

export const layout = (w, h) => box(0, 0, w, h).inset(Math.min(w, h) * PAD);

export function scrub(cut, x, w) {
  if (cut.empty) return null;
  const frac = Math.min(1, Math.max(0, x / Math.max(1, w)));
  return Math.round(cut.low + frac * (cut.high - cut.low));
}

export function fit(w, h, frame) {
  const scale = Math.min(frame.w / w, frame.h / h);
  return [frame.x + (frame.w - w * scale) / 2, frame.y + (frame.h - h * scale) / 2, w * scale, h * scale];
}

export function picture(page, frame, plan, paint, image) {
  if (plan.art?.mesh) {
    const faint = [...paint.accent.slice(0, 3), FAINT];
    hex.draw(page, frame, plan.art.mesh, 0, (type) => (type === paint.fill ? paint.accent : type === paint.void ? faint : null));
  } else if (image?.ctx && plan.art?.svg) {
    image.ctx.drawImage(image.img, ...fit(image.img.naturalWidth || 1, image.img.naturalHeight || 1, frame));
  }
}

/* SCENE */

const settle = (opts) => ({ ...tidy(SPEC, opts, opts.math && { math: opts.math }), seed: opts.seed, base: opts.base, code: opts.code, height: opts.height });

const drawable = () => typeof Image === 'function' && typeof Blob === 'function' && typeof URL?.createObjectURL === 'function';

export function make(canvas, view, opts = {}) {
  const { math, tell } = opts;
  if (!math) throw new Error('cuts: the scene wants @mrlyprod/mrly/math as opts.math');
  const ctx = canvas.getContext('2d');
  const codes = { fill: math.six.FILL(), void: math.six.VOID() };
  let value = settle(opts);
  let plan = study(math, value);
  let dirty = true;
  let img = null;
  let wanted = '';
  const paint = () => ({ ...codes, accent: [...rgb(view.look().accent), 255] });
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    picture(pen(ctx, view.w, view.h), layout(view.w, view.h), plan, paint(), img && { ctx, img });
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const fetch = () => {
    const svg = plan.art?.svg;
    if (!svg || svg === wanted || !drawable()) return;
    wanted = svg;
    const one = new Image();
    const link = URL.createObjectURL(new Blob([svg], { type: 'image/svg+xml' }));
    one.onload = () => {
      URL.revokeObjectURL(link);
      if (wanted !== svg) return;
      img = one;
      touch();
    };
    one.src = link;
  };
  const report = () => tell?.({ rows: plan.rows, cut: plan.cut, heights: plan.heights, code: plan.code, base: plan.base, level: plan.level });
  report();
  fetch();
  return {
    still: true,
    draw,
    size: touch,
    theme: touch,
    set: (next) => {
      value = settle({ ...next, math });
      plan = study(math, value);
      report();
      fetch();
      touch();
      return true;
    },
    seek: (x) => scrub(plan.cut, x, view.w),
    svg: async () => {
      if (plan.art?.svg) return plan.art.svg;
      const w = view.w / view.dpr;
      const h = view.h / view.dpr;
      const page = sheet(w, h);
      picture(page, layout(w, h), plan, paint(), null);
      return page.text();
    },
  };
}
