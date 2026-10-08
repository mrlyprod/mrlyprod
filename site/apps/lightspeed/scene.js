import { palette } from '../../kit/theme/palette.js';
import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, rng, veil } from '../../lib/scene.js';

export const SPEC = [
  { key: 'max', label: 'Jump speed', kind: 'slider', def: 14, min: 4, max: 30, step: 1, group: 'Jump' },
  { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005, group: 'Jump' },
  { key: 'stars', label: 'Density', kind: 'slider', def: 1, min: 0.25, max: 3, step: 0.25, group: 'Sky' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Sky' },
  { key: 'hyper', label: 'Hyperspace', kind: 'segment', def: 'mix', options: [['no', 'No'], ['yes', 'Yes'], ['mix', 'Mix']], group: 'Saver' },
];

export const PAGE = {
  spec: SPEC,
  keys: [{ key: 'Enter', label: 'Jump', does: 'Again in hyperspace to exit', act: 'jump', button: true }],
  actions: { jump: (scene) => (scene.phase() === 'cruise' ? scene.trigger() : scene.exit()) },
};

const TIMES = { wind: 0.28, run: 1, punch: 0.45, snap: 0.5, flash: 0.25, fade: 1 };

const HYPER = { yes: [12, 12], mix: [6, 30] };

const STAY = [4, 10];

const TAU = Math.PI * 2;
const AREA = 1100;
const FEW = 400;
const MANY = 1400;
const LIFT = 0.3;
const WASH = 0.55;
const BURST = 0.82;
const CALM = 0.6;
const RINGS = 9;
const DEEP = 3;
const EPS = 1e-3;
const SALT = 0x5bd1e995;

const clamp = (v, a, b) => Math.min(Math.max(v, a), b);

const ease = (v) => v * v * (3 - 2 * v);

const number = (v) => (String(v ?? '').trim() ? Number(v) : NaN);

const mix = (a, b, k) => {
  const from = rgb(a);
  const to = rgb(b);
  return `rgb(${from.map((v, i) => Math.round(v + (to[i] - v) * k)).join(', ')})`;
};

const bright = (color) => {
  const [r, g, b] = rgb(color);
  return 0.299 * r + 0.587 * g + 0.114 * b > 127.5;
};

function times(opts) {
  const read = ([key, fallback]) => {
    const v = number(opts?.[key]);
    return [key, Number.isFinite(v) && v >= 0 ? v : fallback];
  };
  return Object.fromEntries(Object.entries(TIMES).map(read));
}

export function make(canvas, view, opts) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const { idle, max, stars: dense, hyper: mode } = tidy(SPEC, opts);
  const { wind, flash, ...rest } = times(opts);
  const run = Math.max(rest.run, EPS);
  const punch = Math.max(rest.punch, EPS);
  const snap = Math.max(rest.snap, EPS);
  const fade = Math.max(rest.fade, EPS);
  const up = (wind + run) * 1000;
  const top = idle + max;
  const hyper = HYPER[mode] ?? null;
  const gaps = rng(Number.isFinite(opts?.seed) ? (opts.seed ^ SALT) >>> 0 : undefined);
  const span = ([a, b]) => (a + gaps() * (b - a)) * 1000;
  const later = () => (hyper && !view.still ? view.t + span(hyper) : Infinity);
  const rnd = (a, b) => a + rand() * (b - a);
  let w = 1;
  let h = 1;
  let edge = 0;
  let roll = 0;
  let last = view.t;
  let from = null;
  let out = null;
  let stay = Infinity;
  let base = idle;
  let reach = 0;
  let lift = 0;
  let next = later();
  const jumps = [];
  const done = [];
  let stars = [];
  const spawn = (s, far) => {
    const sx = rnd(-0.55, 0.55) * w;
    const sy = rnd(-0.55, 0.55) * h;
    const c = Math.cos(roll);
    const n = Math.sin(roll);
    s.z = far ? rnd(1, 1.15) : rnd(0.08, 1);
    s.x = (sx * c + sy * n) * s.z;
    s.y = (sy * c - sx * n) * s.z;
    s.m = rnd(0.25, 1);
    s.r = 0.5 + rand() ** 3 * 2.2;
    s.f = rnd(0.5, 2.5);
    s.p = rnd(0, TAU);
    s.tint = rand() < 0.35;
    return s;
  };
  const size = () => {
    w = view.w / view.dpr;
    h = view.h / view.dpr;
    edge = Math.hypot(w, h) * 0.54;
    stars = Array.from({ length: Math.round(dense * clamp((w * h) / AREA, FEW, MANY)) }, () => spawn({}, false));
  };
  const sense = (now) => {
    if (from === null) return { speed: idle, p: 0, k: 0, x: -1 };
    if (out === null || now < out) {
      const e = (now - from) / 1000;
      if (e < wind) return { speed: idle - 0.4 * Math.sin((Math.PI * e) / wind), p: 0, k: 0, x: -1 };
      const p = (e - wind) / run;
      if (p < 1) return { speed: idle + max * p ** 2.5, p, k: 0, x: -1 };
      const k = ease(Math.min((e - wind - run) / punch, 1));
      return { speed: top * (1 - CALM * k), p: 1, k, x: -1 };
    }
    const x = (now - out) / 1000;
    if (x >= snap) return { speed: idle + max * Math.max(0, 1 - (x - snap) * 4), p: 0, k: 0, x };
    const f = x / snap;
    return { speed: base + (top - base) * f * f, p: reach + (1 - reach) * f, k: lift * (1 - f), x };
  };
  const leave = (at) => {
    const now = sense(at);
    base = now.speed;
    reach = now.p;
    lift = now.k;
    if (from === null) from = at;
    out = at;
  };
  const phase = () => {
    if (from === null) return 'cruise';
    if (out !== null && view.t >= out) return 'exit';
    const e = view.t - from;
    return e < wind * 1000 ? 'wind' : e < up ? 'jump' : 'hyper';
  };
  const streak = (tx, ty, hx, hy, width, color, alpha) => {
    const dx = hx - tx;
    const dy = hy - ty;
    ctx.globalAlpha = alpha;
    ctx.fillStyle = color;
    ctx.beginPath();
    if (Math.hypot(dx, dy) < width) ctx.arc(hx, hy, width / 2, 0, TAU);
    else {
      const ang = Math.atan2(dy, dx);
      ctx.moveTo(tx, ty);
      ctx.arc(hx, hy, width / 2, ang + Math.PI / 2, ang - Math.PI / 2, true);
    }
    ctx.fill();
  };
  const bloom = (x, y, radius, alpha, ink, glint, accent) => {
    const g = ctx.createRadialGradient(x, y, 0, x, y, radius);
    g.addColorStop(0, ink);
    g.addColorStop(0.25, veil(glint, 0.8));
    g.addColorStop(0.65, veil(accent, 0.35));
    g.addColorStop(1, veil(accent, 0));
    ctx.globalAlpha = alpha;
    ctx.fillStyle = g;
    ctx.fillRect(0, 0, w, h);
  };
  const ground = (x, y, k, glint, accent) => {
    const g = ctx.createRadialGradient(x, y, 0, x, y, edge * 1.2);
    g.addColorStop(0, veil(glint, 0.7));
    g.addColorStop(0.06, veil(accent, 0.4));
    g.addColorStop(0.35, veil(accent, 0.14));
    g.addColorStop(1, veil(accent, 0.06));
    ctx.globalAlpha = k;
    ctx.fillStyle = g;
    ctx.fillRect(0, 0, w, h);
  };
  const tunnel = (k, travel, glint) => {
    const side = Math.min(w, h) * 0.3;
    ctx.strokeStyle = glint;
    for (let i = 0; i < RINGS; i++) {
      const z = (DEEP * (i + 1 - (travel % 1))) / RINGS;
      const half = side / z;
      if (half > edge * 1.5) continue;
      ctx.globalAlpha = k * 0.3 * (1 - z / DEEP);
      ctx.lineWidth = Math.min(3, 0.8 / z);
      ctx.strokeRect(-half, -half, half * 2, half * 2);
    }
    ctx.globalAlpha = k * 0.12;
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (const [sx, sy] of [[-1, -1], [1, -1], [1, 1], [-1, 1]]) {
      ctx.moveTo(sx * edge, sy * edge);
      ctx.lineTo(0, 0);
    }
    ctx.stroke();
  };
  const fire = (list) => {
    for (const call of list.splice(0)) call();
  };
  const draw = () => {
    if (from === null && view.t >= next) {
      from = view.t;
      stay = span(STAY);
    }
    if (from !== null && out === null && view.t >= from + up + stay) leave(from + up + stay);
    const dt = Math.min((view.t - last) / 1000, 0.05);
    last = view.t;
    const { speed, p, k, x } = sense(view.t);
    const life = from === null ? -1 : (view.t - from - up) / 1000;
    if (from !== null && (life >= 0 || out !== null)) fire(jumps);
    if (x >= snap) fire(done);
    if (x >= snap + flash + fade) {
      from = null;
      out = null;
      next = later();
    }
    const { paper, accent } = view.look();
    const light = bright(paper);
    const ink = light ? palette.black : palette.white;
    const glint = mix(accent, ink, LIFT);
    const tail = p > 0 ? Math.max(speed, k * top) * (0.04 + 0.1 * p) : 0;
    const q = clamp((p - 0.75) / 0.25, 0, 1) ** 2 * (1 - 0.5 * k);
    const glow = (1 + 0.8 * p) * (1 + 0.12 * k * Math.sin(life * 9));
    const shake = p > 0.5 && !view.still ? 3 * ((p - 0.5) * 2) ** 2 * (1 - k) : 0;
    const jolt = () => (shake ? rnd(-shake, shake) : 0);
    const now = view.t / 1000;
    const cx = w / 2 + k * w * 0.07 * Math.sin(life * 0.37);
    const cy = h / 2 + k * h * 0.06 * Math.sin(life * 0.29 + 1.3);
    roll += dt * 0.012;
    ctx.setTransform(view.dpr, 0, 0, view.dpr, 0, 0);
    ctx.globalCompositeOperation = 'source-over';
    ctx.globalAlpha = 1;
    ctx.fillStyle = paper;
    ctx.fillRect(0, 0, w, h);
    if (k > 0) ground(cx, cy, k, glint, accent);
    ctx.translate(cx + jolt(), cy + jolt());
    ctx.rotate(roll + k * 0.25 * Math.sin(life * 0.7));
    ctx.globalCompositeOperation = light ? 'multiply' : 'lighter';
    if (k > 0) tunnel(k, life * 1.4, glint);
    for (const s of stars) {
      s.z -= speed * dt;
      if (s.z < 0.04) spawn(s, true);
      const hx = s.x / s.z;
      const hy = s.y / s.z;
      if (hx * hx + hy * hy > edge * edge) {
        spawn(s, true);
        continue;
      }
      const zt = Math.min(s.z + tail, 2);
      const near = 1 - Math.min(s.z, 1);
      const tw = 0.75 + 0.25 * Math.sin(now * s.f + s.p);
      const a = Math.min(1, s.m * (0.3 + 0.7 * near) * tw * glow);
      const r = Math.min(s.r * (0.5 + 1.5 * near), 5);
      streak(s.x / zt, s.y / zt, hx, hy, r * (3.2 + 6 * q), s.tint ? accent : glint, a * (0.22 + 0.4 * q));
      streak(s.x / zt, s.y / zt, hx, hy, r, ink, a);
    }
    ctx.setTransform(view.dpr, 0, 0, view.dpr, 0, 0);
    ctx.globalCompositeOperation = 'source-over';
    if (x >= snap) {
      const u = clamp((x - snap - flash) / fade, 0, 1);
      ctx.globalAlpha = (1 - u) ** 2;
      ctx.fillStyle = ink;
      ctx.fillRect(0, 0, w, h);
      if (u > 0 && u < 1) bloom(w / 2, h / 2, edge * (0.15 + 1.2 * (1 - u)), 1 - u, ink, glint, accent);
    } else if (x >= 0) {
      const f = x / snap;
      bloom(cx, cy, edge * (0.35 + 1.3 * f), Math.min(1, 3 * f * f), ink, glint, accent);
      ctx.globalAlpha = clamp((f - WASH) / (1 - WASH), 0, 1) ** 2;
      ctx.fillStyle = ink;
      ctx.fillRect(0, 0, w, h);
    } else if (p > BURST && k === 0) {
      const f = (p - BURST) / (1 - BURST);
      bloom(cx, cy, edge * (0.25 + 0.6 * f), 0.7 * f * f, ink, glint, accent);
    } else if (life >= 0 && life < punch) {
      ctx.globalAlpha = 0.6 * (1 - life / punch) ** 2;
      ctx.fillStyle = ink;
      ctx.fillRect(0, 0, w, h);
    }
  };
  const trigger = (onDone, onJump) => {
    if (onJump) jumps.push(onJump);
    if (onDone) done.push(onDone);
    if (view.still) {
      from = view.t - up - (punch + 1) * 1000;
      out = null;
      stay = Infinity;
      draw();
      fire(done);
      return;
    }
    if (from !== null) return;
    from = view.t;
    stay = Infinity;
  };
  const exit = (onDone) => {
    if (onDone) done.push(onDone);
    if (view.still) {
      from = null;
      out = null;
      draw();
      fire(jumps);
      fire(done);
      return;
    }
    if (out === null) leave(view.t);
  };
  const keep = () => {
    if (view.still) return;
    if (from === null) trigger();
    else if (out === null) stay = Infinity;
  };
  const stop = () => {
    fire(jumps);
    fire(done);
  };
  size();
  return { draw, size, trigger, exit, hold: keep, windDown: exit, phase, stop };
}
