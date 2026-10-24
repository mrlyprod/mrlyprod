import { tidy } from "../../lib/app/spec.js";
import { TINTS } from "../../lib/color.js";
import {
  cap,
  coords,
  GROW,
  hit,
  inks,
  LOOKS,
  lay,
  PAD,
  parse,
  picture,
  RINGS,
  reached,
  share,
  study,
  styleOf,
  THUMB,
  table,
  tones,
  turned,
  walk,
} from "./engine.js";

const plain = (value) => value.look === "plain";

export const picked = (value) => parse(value.click) !== null;

export const named = (value) =>
  [
    "gaussian",
    ...(value.ring === "gaussian" ? [] : [value.ring]),
    value.limit,
    ...(value.look === "fate" ? [] : [value.look]),
  ].join("-");

export const SPEC = [
  { key: "ring", label: "Ring", kind: "pick", def: "gaussian", options: RINGS, group: "Window" },
  {
    key: "limit",
    label: "Reach",
    kind: "slider",
    def: 40,
    min: 1,
    max: (value) => cap(value.ring),
    step: 1,
    group: "Window",
  },
  { key: "look", label: "Colour", kind: "segment", def: "fate", options: LOOKS, group: "Window" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Window", when: plain },
  { key: "faint", label: "Composites", kind: "toggle", def: 1, group: "Window" },
  { key: "units", label: "Show its units", kind: "toggle", def: 1, group: "Point" },
  { key: "click", label: "Point", kind: "text", def: "", when: () => false },
  {
    key: "cell",
    label: "Cell",
    kind: "slider",
    def: 0,
    min: 0,
    max: 32,
    step: 1,
    unit: "px",
    zero: "Fit",
    group: "Size",
  },
];

const ABOUT = [
  "The Gaussian integers a + bi and the Eisenstein integers a + bω are whole numbers in the plane, and they have primes of their own. Painted, the primes make a snowflake: four arms on the square lattice, six on the hexagonal one.",
  "The colour says what became of an ordinary prime p in the plane. Blue: it split into a point and its mirror image. Orange: it stayed prime, as p does when it is 3 mod 4 on the square lattice or 2 mod 3 on the hexagonal one. Pink: it ramified, 2 or 3, into a unit times a square. The units are green and the composites faint.",
  "The norm of a + bi is a² + b², of a + bω it is a² - ab + b², and norms multiply. The 4 or 6 units turn every prime into its associates, and with the mirror they give the picture its symmetry. On the square lattice r(n) = 4(d₁ - d₃) points have norm n, d₁ and d₃ counting the divisors of n that are 1 and 3 mod 4; it is zero exactly when an inert prime divides n to an odd power. The zeta functions of the two rings are in the [bases note](/research/bases/) of Research.",
  "Tap a point for its norm, its class and its factors; its associates are ringed and its conjugate dashed. The left and right arrows walk the primes, up and down turn the point by a unit, and Grow paints the window again norm by norm.",
];

export const PAGE = {
  spec: SPEC,
  name: named,
  gestures: true,
  record: true,
  about: ABOUT,
  keys: [
    { key: "Enter", label: "Grow", act: "grow", button: true, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Walk the primes", act: "walk", menu: "Edit" },
    { key: ["ArrowUp", "ArrowDown"], label: "Turn by a unit", act: "turn", when: picked },
  ],
  actions: {
    grow: (scene) => scene.grow?.(),
    walk: (scene, e) => scene.walk?.(e.key === "ArrowLeft" ? -1 : 1),
    turn: (scene, e) => scene.turn?.(e.key === "ArrowDown" ? -1 : 1),
  },
};

const TAU = Math.PI * 2;
const CHUNK = 4096;
const GAP = 0.12;
const SMALL = 6;
const RING = 3;
const THIN = 1.5;
const DASH = [4, 3];
const SOFT = 3;

/* CELLS */

const place = (plan, box, x, y) => [
  box.ox + (x * plan.ex[0] + y * plan.ey[0]) * box.k,
  box.oy - (x * plan.ex[1] + y * plan.ey[1]) * box.k,
];

const bulk = (k, dpr) => {
  const gap = k >= SMALL ? Math.max(dpr, GAP * k) : 0;
  return Math.max(0.5, (k - gap) / 2);
};

function mark(target, word, x, y, h) {
  if (word === "eisenstein") {
    target.moveTo(x + h, y);
    target.arc(x, y, h, 0, TAU);
    return;
  }
  const x0 = Math.round(x - h);
  const y0 = Math.round(y - h);
  target.rect(x0, y0, Math.max(1, Math.round(x + h) - x0), Math.max(1, Math.round(y + h) - y0));
}

export function sketch(num, set, ctx, w, h, dpr, look) {
  const value = tidy(SPEC, { ...set, limit: Math.min(set.limit, THUMB) });
  const plan = study(num, value.ring, value.limit);
  const box = lay(plan.frame, w, h, SOFT * dpr);
  const r = bulk(box.k, dpr);
  const style = Uint8Array.from({ length: plan.count }, (_, p) =>
    styleOf(value.look, value.faint, plan.fate[p], plan.norm[p], plan.top),
  );
  tones(inks(look)).forEach((color, s) => {
    if (!color) return;
    ctx.fillStyle = color;
    ctx.beginPath();
    for (let p = 0; p < plan.count; p++)
      if (style[p] === s) mark(ctx, plan.word, ...place(plan, box, plan.a[p], plan.b[p]), r);
    ctx.fill();
  });
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error("gaussian: the scene wants @mrlyprod/mrly/num as opts.num");
  const ctx = canvas.getContext("2d");
  let value = tidy(SPEC, opts);
  let plan = null;
  let style = null;
  let ink = null;
  let tone = [];
  let box = null;
  let layer = null;
  let pen = null;
  let drawn = -1;
  let origin = null;
  let pick = null;
  let dirty = true;
  const paint = () => {
    ink = inks(view.look());
    tone = tones(ink);
  };
  const restyle = () => {
    style = new Uint8Array(plan.count);
    for (let p = 0; p < plan.count; p++)
      style[p] = styleOf(value.look, value.faint, plan.fate[p], plan.norm[p], plan.top);
  };
  const restudy = () => {
    plan = study(num, value.ring, value.limit);
    box = null;
    restyle();
  };
  const choose = () => {
    const at = parse(value.click);
    pick = at ? hit(num, plan, at[0], at[1]) : null;
  };
  const fit = () => {
    box = lay(plan.frame, view.w, view.h, PAD * view.dpr, value.cell * view.dpr);
  };
  const spot = (x, y) => place(plan, box, x, y);
  const half = () => bulk(box.k, view.dpr);
  const lays = (target, from, to, h) => {
    tone.forEach((color, s) => {
      if (!color) return;
      let open = 0;
      const flush = () => {
        if (!open) return;
        target.fill();
        open = 0;
      };
      for (let p = from; p <= to; p++) {
        if (style[p] !== s) continue;
        if (!open) {
          target.fillStyle = color;
          target.beginPath();
        }
        const [x, y] = spot(plan.a[p], plan.b[p]);
        mark(target, plan.word, x, y, h);
        if (++open >= CHUNK) flush();
      }
      flush();
    });
  };
  const reset = () => {
    fit();
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext("2d");
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.clearRect(0, 0, view.w, view.h);
    drawn = -1;
  };
  const extend = (to) => {
    if (to <= drawn) return;
    lays(pen, drawn + 1, to, half());
    drawn = to;
  };
  const ring = (x, y, radius, width, color, dash) => {
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.setLineDash(dash);
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
  };
  const overlay = () => {
    const dpr = view.dpr;
    const radius = half() + RING * dpr;
    if (value.units && pick.norm > 1) {
      for (const [x, y] of pick.associates.slice(1)) ring(...spot(x, y), radius, THIN * dpr, ink.dim, []);
      ring(
        ...spot(pick.conjugate[0], pick.conjugate[1]),
        radius,
        THIN * dpr,
        ink.ramified,
        DASH.map((d) => d * dpr),
      );
    }
    ring(...spot(pick.a, pick.b), radius, RING * dpr, ink.fg, []);
  };
  const draw = () => {
    if (origin === null && !dirty && layer) return;
    dirty = false;
    const part = origin === null ? 1 : share(view.t - origin, GROW);
    if (part >= 1) origin = null;
    const i = reached(part, plan.count);
    if (!layer || i < drawn) reset();
    extend(i);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(layer, 0, 0);
    if (pick) overlay();
  };
  const touch = () => {
    box = null;
    layer = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    touch();
  };
  const set = (next) => {
    const was = value;
    value = tidy(SPEC, next);
    if (value.ring !== was.ring || value.limit !== was.limit) {
      restudy();
      origin = null;
      layer = null;
    } else if (value.look !== was.look || value.faint !== was.faint) {
      restyle();
      layer = null;
    }
    if (value.cell !== was.cell) touch();
    if (value.tint !== was.tint) theme();
    choose();
    dirty = true;
  };
  const grow = () => {
    origin = view.t;
    dirty = true;
    if (view.still) view.wake?.();
  };
  const at = (u, v) => {
    if (!box) fit();
    const x = (u * view.w - box.ox) / box.k;
    const y = (box.oy - v * view.h) / box.k;
    const [p, q] = num.gauss.Ring.nearest(plan.ring, x, y);
    return hit(num, plan, Number(p), Number(q));
  };
  const along = (dir) => {
    const next = walk(plan, pick ? pick.index : 0, dir);
    return next && coords(...next);
  };
  const turn = (dir) => {
    const next = pick && turned(pick, dir);
    return next && coords(...next);
  };
  const svg = () => picture(plan, style, tone);
  const csv = () => table(plan);
  restudy();
  choose();
  paint();
  return { draw, size: touch, theme, set, grow, at, walk: along, turn, svg, csv, still: true };
}
