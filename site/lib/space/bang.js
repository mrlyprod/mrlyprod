import { rng } from '../scene.js';
import { project, uniforms } from './camera.js';
import { blend, fill, program, texture, tier } from './gl2.js';
import { add, cross, dot, len, mix, mul, norm, rot, sub } from './vec.js';

const DIRS = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

const HALF = [0.5, 0.5, 0.5];
const LOW = 0.4;
const HIGH = 0.9;
const WALLS = 2.2;
const SPEED = 1.15;
const STRAIGHT = 0.45;
const FRESH = 0.3;
const TRAIL = 0.1;
const JITTER = 0.4;
const MARKS = 4;
const AHEAD = 2;
const PRELUDE = 2;
const KEEP = 4;
const ROLL = 0.15;
const SWAY = 0.7;
const RISE = 0.35;
const FADE = 0.6;
const DIP = 0.7;
const CLOSE = 0.8;
const FOG = 0.24;
const SIGHT = 5;
const LOOKS = 160;
const SHY = 0.3;
const HUE = 0.17;
const DRIFT = 0.45;
const SALT = 0x2f6b8d31;
const GOLD = 0x9e3779b1;
const DESK = { steps: 200, depth: 12, taps: 8, shadow: 1 };
const PHONE = { steps: 96, depth: 8, taps: 2, shadow: 0 };
const RIDE = [0, -0.35, 1.6];
const SIDE = 0.22;
const LEVEL = 3;
const GROW = 0.6;
const RATE = 11;
const SPLIT = 0.6;
const SNAP = 0.15;
const LEAN = 0.3;
const PITCH = [0.3, 0.55];
const TURN = [0.3, 0.55];
const TAIL = 2.5;
const FLARE = 4;
const WIDE = 2.2;
const TOTEM = 0x7f4a7c15;
const TAU = Math.PI * 2;
const WHITE = [1, 1, 1];
const PALE = [0.8, 0.97, 1];
const FIRES = [[0.4, 0.8, 1.6], [1.5, 0.7, 0.3], [0.8, 0.5, 1.5], [0.5, 1.4, 0.9]];
const STAR = { dir: [-0.5, 0.75, -0.45], color: [1.5, 1.4, 1.25], ambient: [0.03, 0.035, 0.05] };
const BLUE = { color: 1.3, ambient: 0.3 };
const FLASH = { color: 3, ambient: 1.2 };

/* RULE */

const kept = (code, i, j, k) => (code >> ((i & 1) * 4 + (j & 1) * 2 + (k & 1))) & 1;

export function cells(code, n) {
  const out = new Uint8Array(n * n * n);
  for (let z = 0; z < n; z++) for (let y = 0; y < n; y++) for (let x = 0; x < n; x++) out[(z * n + y) * n + x] = kept(code, x, y, z);
  return out;
}

const index = (n, c) => (c[2] * n + c[1]) * n + c[0];

const unindex = (n, k) => [k % n, Math.floor(k / n) % n, Math.floor(k / (n * n))];

const read = (all, n, c) => (c.every((v) => v >= 0 && v < n) ? all[index(n, c)] : -1);

const entry = (g, n) => DIRS[g].map((d) => (d > 0 ? 0 : d < 0 ? n - 1 : (n - 1) / 2));

/* DOORS */

export function doors(all, n) {
  const open = DIRS.map((_, g) => read(all, n, entry(g, n)) === 0);
  const table = DIRS.map((_, g) => {
    if (!open[g]) return null;
    const start = entry(g, n);
    const from = new Map([[index(n, start), null]]);
    const queue = [start];
    const exits = [];
    const trail = (c) => {
      const list = [];
      for (let k = index(n, c); k !== null; k = from.get(k)) list.push(unindex(n, k));
      return list.reverse();
    };
    for (let q = 0; q < queue.length; q++) {
      const c = queue[q];
      DIRS.forEach((d, dir) => {
        const next = add(c, d);
        const v = read(all, n, next);
        if (v === 1 && open[dir]) exits.push({ cell: c, dir, trail: trail(c) });
        if (v === 0 && !from.has(index(n, next))) {
          from.set(index(n, next), index(n, c));
          queue.push(next);
        }
      });
    }
    return exits.length ? { start, exits } : null;
  });
  for (let changed = true; changed; ) {
    changed = false;
    table.forEach((one, g) => {
      if (!one) return;
      const exits = one.exits.filter((e) => table[e.dir]);
      if (exits.length === one.exits.length) return;
      changed = true;
      table[g] = exits.length ? { ...one, exits } : null;
    });
  }
  return table;
}

/* GATE */

function whole(all, n, full) {
  const first = all.indexOf(1);
  if (first < 0) return false;
  const seen = new Set([first]);
  const queue = [first];
  for (let q = 0; q < queue.length; q++) {
    const c = unindex(n, queue[q]);
    for (const d of DIRS) {
      const next = add(c, d);
      if (read(all, n, next) !== 1 || seen.has(index(n, next))) continue;
      seen.add(index(n, next));
      queue.push(index(n, next));
    }
  }
  return seen.size === full;
}

export function gate(code, n) {
  const all = cells(code, n);
  let full = 0;
  let empty = 0;
  let walls = 0;
  for (let k = 0; k < all.length; k++) {
    if (all[k]) {
      full++;
      continue;
    }
    empty++;
    const c = unindex(n, k);
    for (const d of DIRS) walls += Math.max(0, read(all, n, add(c, d)));
  }
  const share = full / all.length;
  if (share < LOW || share > HIGH || !empty || walls / empty < WALLS) return false;
  return whole(all, n, full) && doors(all, n).some(Boolean);
}

const gated = new Map();

export function codes(n) {
  if (!gated.has(n)) gated.set(n, Array.from({ length: 256 }, (_, code) => code).filter((code) => gate(code, n)));
  return gated.get(n);
}

/* BOXES */

function boxes(all, n) {
  const lo = new Uint8Array(n * n * n * 4);
  const hi = new Uint8Array(n * n * n * 4);
  const at = (x, y, z) => all[(z * n + y) * n + x];
  const empty = (x0, x1, y0, y1, z0, z1) => {
    for (let z = z0; z < z1; z++) for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) if (at(x, y, z)) return false;
    return true;
  };
  for (let z = 0; z < n; z++) {
    for (let y = 0; y < n; y++) {
      for (let x = 0; x < n; x++) {
        const i = ((z * n + y) * n + x) * 4;
        if (at(x, y, z)) {
          lo[i] = 255;
          continue;
        }
        const b = [x, x + 1, y, y + 1, z, z + 1];
        for (let grew = true; grew; ) {
          grew = false;
          if (b[0] > 0 && empty(b[0] - 1, b[0], b[2], b[3], b[4], b[5])) {
            b[0]--;
            grew = true;
          }
          if (b[1] < n && empty(b[1], b[1] + 1, b[2], b[3], b[4], b[5])) {
            b[1]++;
            grew = true;
          }
          if (b[2] > 0 && empty(b[0], b[1], b[2] - 1, b[2], b[4], b[5])) {
            b[2]--;
            grew = true;
          }
          if (b[3] < n && empty(b[0], b[1], b[3], b[3] + 1, b[4], b[5])) {
            b[3]++;
            grew = true;
          }
          if (b[4] > 0 && empty(b[0], b[1], b[2], b[3], b[4] - 1, b[4])) {
            b[4]--;
            grew = true;
          }
          if (b[5] < n && empty(b[0], b[1], b[2], b[3], b[5], b[5] + 1)) {
            b[5]++;
            grew = true;
          }
        }
        lo.set([b[0], b[2], b[4]], i + 1);
        hi.set([b[1], b[3], b[5]], i);
      }
    }
  }
  return { lo, hi };
}

const shapes = new Map();

function shape(code, n) {
  const key = `${code}:${n}`;
  if (!shapes.has(key)) {
    const all = cells(code, n);
    shapes.set(key, { all, table: doors(all, n), boxes: boxes(all, n) });
  }
  return shapes.get(key);
}

/* ROUTE */

const smooth = (x) => {
  const p = Math.min(Math.max(x, 0), 1);
  return p * p * (3 - 2 * p);
};

const flatten = (u, t) => norm(sub(u, mul(t, dot(u, t))));

const curve = (P, u) => {
  const u2 = u * u;
  const u3 = u2 * u;
  return mul(add(add(mul(P[0], -u3 + 2 * u2 - u), mul(P[1], 3 * u3 - 5 * u2 + 2)), add(mul(P[2], -3 * u3 + 4 * u2 + u), mul(P[3], u3 - u2))), 0.5);
};

const slope = (P, u) => {
  const u2 = u * u;
  return add(add(mul(P[0], -3 * u2 + 4 * u - 1), mul(P[1], 9 * u2 - 10 * u)), add(mul(P[2], -9 * u2 + 8 * u + 1), mul(P[3], 3 * u2 - 2 * u)));
};

const warp = (s, a, b) => {
  const s2 = s * s;
  const s3 = s2 * s;
  return (s3 - 2 * s2 + s) * a + (-2 * s3 + 3 * s2) + (s3 - s2) * b;
};

export function route(design, seed = 0) {
  const { n } = design;
  const { all, table, boxes: made } = shape(design.code, n);
  const ways = table.map((one, g) => (one ? g : -1)).filter((g) => g >= 0);
  if (!ways.length) throw new Error(`bang: code ${design.code} has no way in at n ${n}`);
  const cycles = [];
  const keys = [];
  let origin = 0;
  let asked = null;
  let eye = [0, 0, 0];
  const cycle = (j) => cycles.find((one) => one.j === j);
  const pending = () => cycles[cycles.length - 1];
  const toFrame = (key, frame) => {
    let p = key.p;
    for (let k = key.j; k > frame; k--) p = add(cycle(k - 1).child, mul(p, 1 / n));
    for (let k = key.j; k < frame; k++) p = mul(sub(p, cycle(k).child), n);
    return p;
  };
  const tangent = (m) => norm(sub(toFrame(keys[m + 1], keys[m].j), toFrame(keys[Math.max(m - 1, 0)], keys[m].j)));
  const orient = (m) => {
    const t = tangent(m);
    const prev = keys[m - 1];
    if (!prev?.up) {
      keys[m].up = flatten(Math.abs(t[1]) < 0.9 ? [0, 1, 0] : [0, 0, 1], t);
    } else {
      const axis = cross(prev.dir, t);
      const s = len(axis);
      keys[m].up = flatten(s > 1e-9 ? rot(prev.up, axis, Math.atan2(s, dot(prev.dir, t))) : prev.up, t);
    }
    keys[m].dir = t;
  };
  const push = (j, p, lev) => {
    const prev = keys[keys.length - 1];
    const time = prev ? prev.time + len(sub(p, toFrame(prev, j))) / SPEED : 0;
    keys.push({ j, p, lev, time });
    if (keys.length > 1) orient(keys.length - 2);
  };
  const open = (j, g) => {
    const info = table[g];
    const rand = rng((seed ^ SALT ^ Math.imul(j + 1, GOLD)) >>> 0);
    const recent = [cycle(j - 1)?.exit?.dir, cycle(j - 2)?.exit?.dir];
    const scored = info.exits.map((e) => ({ ...e, score: (1 + STRAIGHT * (e.dir === g) + FRESH * !recent.includes(e.dir) + TRAIL * e.trail.length) * (1 - JITTER / 2 + JITTER * rand()) }));
    const best = new Map();
    for (const e of scored) if (!best.has(e.dir) || e.score > best.get(e.dir).score) best.set(e.dir, e);
    const first = [...best.values()].sort((a, b) => b.score - a.score).slice(0, MARKS);
    const rest = scored.filter((e) => !first.includes(e)).sort((a, b) => b.score - a.score);
    const options = [...first, ...rest].slice(0, MARKS).sort((a, b) => a.dir - b.dir || index(n, a.cell) - index(n, b.cell));
    const auto = options.reduce((top, e, i) => (e.score > options[top].score ? i : top), 0);
    if (!keys.length) push(j, add(add(info.start, HALF), mul(DIRS[g], -0.5)), j);
    const one = { j, g, options, auto, pick: null, exit: null, child: null, door: keys[keys.length - 1].time };
    cycles.push(one);
    push(j, add(info.start, HALF), j);
  };
  const lock = (one) => {
    const exit = one.options[one.pick ?? one.auto];
    one.exit = exit;
    for (const c of exit.trail.slice(1)) push(one.j, add(c, HALF), one.j);
    push(one.j, add(add(exit.cell, HALF), mul(DIRS[exit.dir], 0.5)), one.j + 0.5);
    one.child = add(exit.cell, DIRS[exit.dir]);
    open(one.j + 1, exit.dir);
    const old = one.j - KEEP;
    while (cycles[0].j < old - 1) cycles.shift();
    while (keys[0].j < old) keys.shift();
  };
  open(0, ways[Math.floor(rng((seed ^ SALT) >>> 0)() * ways.length)]);
  while (pending().j <= PRELUDE) lock(pending());
  origin = cycle(PRELUDE).door;
  const near = (frame) => {
    let bits = 0;
    for (let i = 0; i < 27; i++) {
      let carry = [Math.floor(i / 9) - 1, (Math.floor(i / 3) % 3) - 1, (i % 3) - 1];
      let ok = true;
      for (let k = frame - 1; ok && carry.some(Boolean); k--) {
        const up = cycle(k);
        if (!up?.child) break;
        const cell = add(up.child, carry);
        carry = cell.map((v) => Math.floor(v / n));
        ok = read(all, n, cell.map((v) => ((v % n) + n) % n)) === 1;
      }
      if (ok) bits |= 1 << i;
    }
    return bits;
  };
  const probe = (p0, bits) => {
    let p = p0;
    const q = p.map((v) => Math.floor(v / n));
    if (q.some(Boolean)) {
      if (q.some((v) => Math.abs(v) > 1)) return [n, false];
      p = sub(p, mul(q, n));
      if (!((bits >> ((q[0] + 1) * 9 + (q[1] + 1) * 3 + q[2] + 1)) & 1)) return [Math.max(0, Math.min(...p, ...p.map((v) => n - v))), false];
    }
    let s = 1;
    for (let k = 0; k < SIGHT; k++) {
      const c = p.map((v) => Math.min(Math.max(Math.floor(v), 0), n - 1));
      const i = index(n, c);
      if (!all[i]) {
        const lo = [made.lo[i * 4 + 1], made.lo[i * 4 + 2], made.lo[i * 4 + 3]];
        const hi = [made.hi[i * 4], made.hi[i * 4 + 1], made.hi[i * 4 + 2]];
        return [Math.min(...sub(p, lo), ...sub(hi, p)) * s, false];
      }
      p = mul(sub(p, c), n);
      s /= n;
    }
    return [0, true];
  };
  const clear = (from, to, bits, scale) => {
    const span = len(sub(to, from));
    const dir = norm(sub(to, from));
    const end = span - scale * SHY;
    const lift = mul(HALF, n);
    for (let t = 0, i = 0; t < end && i < LOOKS; i++) {
      const [d, hit] = probe(add(add(from, lift), mul(dir, t)), bits);
      if (hit) return false;
      t += Math.max(d, scale * 1e-3);
    }
    return true;
  };
  const at = (tau) => {
    const t = tau + origin;
    asked = tau;
    while (pending().door <= t) lock(pending());
    const cur = pending().j - 1;
    const frame = cur - AHEAD;
    const i = keys.findIndex((key) => key.time > t);
    const K = [i - 2, i - 1, i, i + 1].map((m) => keys[Math.max(m, 0)]);
    const P = K.map((key) => toFrame(key, frame));
    const T = K[2].time - K[1].time;
    const before = K[1].time - K[0].time || T;
    const after = K[3].time - K[2].time || T;
    const u = warp((t - K[1].time) / T, (2 * T) / (before + T), (2 * T) / (T + after));
    const fwd = norm(slope(P, u));
    const lift = mix(K[1].up, K[2].up, u);
    const up = flatten(len(lift) > 1e-6 ? lift : K[2].up, fwd);
    const lev = mix(K[1].lev, K[2].lev, u);
    const here = cycle(cur);
    const span = pending().door - here.door;
    const into = (t - here.door) / span;
    const face = (one, j) => sub(toFrame({ j, p: add(add(one.exit.cell, HALF), mul(DIRS[one.exit.dir], 0.5)) }, frame), mul(HALF, n));
    const doors = [{ pos: face(here, cur), scale: n ** (frame - cur), glow: smooth(into / RISE) * (0.6 + 0.4 * into) * (1 - DIP * smooth((into - CLOSE) / (1 - CLOSE))) }];
    const last = cycle(cur - 1);
    if (last?.exit) doors.push({ pos: face(last, cur - 1), scale: n ** (frame - cur + 1), glow: (1 - DIP) * (1 - smooth((t - here.door) / FADE)) });
    eye = sub(curve(P, u), mul(HALF, n));
    return {
      pos: eye,
      fwd,
      up,
      roll: ROLL * Math.sin(SWAY * tau),
      frame,
      level: cur - PRELUDE,
      scale: n ** (frame - lev),
      fog: FOG,
      near: near(frame),
      hue: HUE + HUE * Math.sin(DRIFT * lev),
      doors,
      next: pending().door - origin,
      chosen: pending().pick !== null,
    };
  };
  const ahead = (tau) => {
    if (tau !== asked) at(tau);
    const one = pending();
    const frame = one.j - 1 - AHEAD;
    const on = one.pick ?? one.auto;
    const bits = near(frame);
    const scale = n ** (frame - one.j);
    return one.options.map((e, id) => {
      const pos = sub(toFrame({ j: one.j, p: add(add(e.cell, HALF), mul(DIRS[e.dir], 0.5)) }, frame), mul(HALF, n));
      return { id, number: id + 1, dir: e.dir, pos, on: id === on, chosen: one.pick !== null, seen: clear(eye, pos, bits, scale) };
    });
  };
  const choose = (exit) => {
    const one = pending();
    const id = typeof exit === 'number' ? exit : one.options.indexOf(exit);
    if (!(id >= 0 && id < one.options.length)) return false;
    one.pick = id;
    return true;
  };
  const level = (tau) => at(tau).level;
  const next = () => ({ at: pending().door - origin, chosen: pending().pick !== null });
  return { at, ahead, choose, level, next };
}

/* DRAW */

const MARCH = `#version 300 es
precision highp float;
precision highp sampler3D;
in vec2 v;
out vec4 o;
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform float uN;
uniform int uDepth;
uniform int uSteps;
uniform int uTaps;
uniform int uShadow;
uniform sampler3D uLo;
uniform sampler3D uHi;
uniform float uScale;
uniform float uFog;
uniform float uOut;
uniform vec4 uDoor[2];
uniform float uGlow[2];
uniform vec3 uAccent;
uniform vec3 uRamp[4];
uniform float uHue;
uniform int uNear;
const float LOD = 3.5;
const float FALL = 0.25;
const float KEY = 0.3;
const float MIST = 0.9;
const vec3 SUN = vec3(0.505, 0.808, 0.303);
float box(vec3 p, vec3 b) {
  vec3 q = abs(p) - b;
  return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}
vec3 cube(vec3 p, float foot) {
  vec3 h = vec3(uN * 0.5);
  if (box(p - h, h) > 0.0) {
    vec3 q = floor(p / uN);
    if (max(abs(q.x), max(abs(q.y), abs(q.z))) > 1.0) return vec3(box(p - h, h * 3.0), 0.0, 1.0);
    ivec3 iq = ivec3(q) + 1;
    p -= q * uN;
    if (((uNear >> (iq.x * 9 + iq.y * 3 + iq.z)) & 1) == 0) {
      vec3 a = min(p, vec3(uN) - p);
      return vec3(max(min(a.x, min(a.y, a.z)), 0.0), 0.0, 1.0);
    }
  }
  float s = 1.0;
  int top = int(uN) - 1;
  for (int k = 0; k < 12; k++) {
    if (k >= uDepth || s < foot) break;
    ivec3 c = clamp(ivec3(floor(p)), ivec3(0), ivec3(top));
    vec4 lo = texelFetch(uLo, c, 0);
    if (lo.x < 0.5) {
      vec3 a = p - lo.yzw * 255.0;
      vec3 b = texelFetch(uHi, c, 0).xyz * 255.0 - p;
      return vec3(min(min(min(a.x, b.x), min(a.y, b.y)), min(a.z, b.z)) * s, 0.0, s);
    }
    p = (p - vec3(c)) * uN;
    s /= uN;
  }
  return vec3(box(p - h, h) * s, 1.0, s);
}
vec3 ramp(float x) {
  float y = clamp(x, 0.0, 1.0) * 3.0;
  int i = int(min(floor(y), 2.0));
  return mix(uRamp[i], uRamp[i + 1], y - float(i));
}
vec3 normal(vec3 c, float e, float foot) {
  vec2 k = vec2(1.0, -1.0);
  return normalize(k.xyy * cube(c + k.xyy * e, foot).x + k.yyx * cube(c + k.yyx * e, foot).x + k.yxy * cube(c + k.yxy * e, foot).x + k.xxx * cube(c + k.xxx * e, foot).x);
}
float occlusion(vec3 c, vec3 nr, float lod, float foot) {
  int taps = max(uTaps / 2, 1);
  float near = 0.0;
  float wide = 0.0;
  float sum = 0.0;
  float w = 1.0;
  for (int i = 1; i <= 4; i++) {
    if (i > taps) break;
    float a = lod * 0.7 * float(i);
    float b = uScale * 0.3 * float(i);
    near += (a - cube(c + nr * a, foot).x) / a * w;
    wide += (b - cube(c + nr * b, foot).x) / b * w;
    sum += w;
    w *= 0.65;
  }
  float k = 2.3 / sum;
  return clamp(1.0 - near * k * 0.35, 0.0, 1.0) * clamp(1.0 - wide * k * 0.3, 0.0, 1.0);
}
float shade(vec3 c, vec3 l, float far, float foot) {
  float t = foot * 4.0;
  for (int i = 0; i < 24; i++) {
    if (t > far) break;
    vec3 r = cube(c + l * t, foot * 3.0);
    if (r.y > 0.5) return 0.0;
    t += max(r.x, foot * 2.0);
  }
  return 1.0;
}
void main() {
  float span = min(uRes.x, uRes.y);
  vec2 uv = (gl_FragCoord.xy - 0.5 * uRes) / span * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  vec3 ro = uPos;
  vec3 h = vec3(uN * 0.5);
  float pix = 2.0 * uTan / span;
  float far = uOut > 0.5 ? uN * 5.0 : uScale * 5.5 / max(uFog * 0.25, 1e-3);
  float t = 0.0;
  float lod = 1.0;
  float foot = 0.0;
  bool hit = false;
  bool lost = false;
  if (uOut > 0.5) {
    vec3 inv = 1.0 / mix(rd, vec3(1e-6), vec3(lessThan(abs(rd), vec3(1e-6))));
    vec3 a = (-h - ro) * inv;
    vec3 b = (h - ro) * inv;
    vec3 lo = min(a, b);
    vec3 hi = max(a, b);
    float enter = max(max(lo.x, lo.y), lo.z);
    float leave = min(min(hi.x, hi.y), hi.z);
    lost = leave < max(enter, 0.0);
    t = max(enter, 0.0);
  }
  for (int i = 0; i < 200; i++) {
    if (i >= uSteps || lost) break;
    foot = max(t * pix, 1e-7);
    vec3 r = cube(ro + rd * t + h, foot * LOD);
    lod = r.z;
    if (r.y > 0.5) {
      if (-r.x > foot && t > 0.0) {
        t = max(t + r.x + foot * 0.5, 0.0);
        continue;
      }
      hit = true;
      break;
    }
    t += max(r.x, foot * 0.5);
    lost = t > far;
  }
  hit = hit || !lost;
  vec3 col = vec3(0.0);
  if (!hit && uOut < 0.5) {
    vec3 bound = vec3(uN * 1.5);
    vec3 inv = 1.0 / mix(rd, vec3(1e-6), vec3(lessThan(abs(rd), vec3(1e-6))));
    vec3 hi = max((-bound - ro) * inv, (bound - ro) * inv);
    float tx = min(hi.x, min(hi.y, hi.z));
    vec3 axis = vec3(lessThanEqual(hi, vec3(tx)));
    float kind = axis.x > 0.5 ? 0.0 : axis.y > 0.5 ? 2.0 : 1.0;
    col = ramp(uHue + kind * 0.22) * MIST * (0.45 + 0.55 * max(dot(-sign(rd) * axis, SUN), 0.0)) * exp(-tx / uScale * uFog * 0.15);
  }
  float end = hit ? t : 1e9;
  if (hit) {
    vec3 p = ro + rd * t;
    vec3 c = p + h;
    vec3 nr = normal(c, foot * 0.5, foot * LOD);
    float ao = occlusion(c, nr, lod, foot * LOD);
    vec3 an = abs(nr);
    float face = an.x > an.y && an.x > an.z ? 0.0 : an.y > an.z ? 2.0 : 1.0;
    vec3 alb = ramp(uHue + face * 0.22);
    float cells = t / uScale;
    float head = max(dot(nr, -rd), 0.0) / (1.0 + cells * cells * FALL);
    vec3 light = vec3(0.95, 0.97, 1.0) * head * 1.7 + vec3(0.035, 0.04, 0.05);
    if (uOut > 0.5) light += vec3(1.0, 0.95, 0.88) * max(dot(nr, SUN), 0.0) * 1.3;
    for (int i = 0; i < 2; i++) {
      if (uGlow[i] <= 0.0) continue;
      vec3 l = uDoor[i].xyz - p;
      float d = max(length(l), 1e-6);
      float r = d / (uDoor[i].w * 0.7);
      float lit = max(dot(nr, l / d), 0.0) * uGlow[i] / (1.0 + r * r);
      if (uShadow > 0 && lit > 0.01) lit *= shade(c, l / d, d - foot * 3.0, foot);
      light += uAccent * lit * 2.6;
    }
    float key = uOut > 0.5 ? 0.0 : KEY * (0.4 + 0.6 * max(dot(nr, SUN), 0.0)) * exp(-cells * uFog * 0.25);
    col = alb * ao * (light * exp(-cells * uFog) + key);
  }
  for (int i = 0; i < 2; i++) {
    if (uGlow[i] <= 0.0) continue;
    vec3 q = uDoor[i].xyz - ro;
    float tc = dot(q, rd);
    if (tc <= 0.0 || tc > end) continue;
    float d2 = max(dot(q, q) - tc * tc, 0.0);
    float r = uDoor[i].w * 0.16;
    col += uAccent * uGlow[i] * exp(-d2 / (r * r)) * 0.9 * exp(-tc / uScale * uFog) * smoothstep(0.0, 1.5, length(q) / uDoor[i].w);
  }
  o = vec4(col, 1.0);
}
`;

export function bang(gl, view, design) {
  const { n } = design;
  const made = shape(design.code, n).boxes;
  const budget = tier(view) === 'phone' ? PHONE : DESK;
  const lo = texture(gl, { w: n, h: n, d: n, data: made.lo, filter: 'nearest' });
  const hi = texture(gl, { w: n, h: n, d: n, data: made.hi, filter: 'nearest' });
  const prog = program(gl, MARCH);
  prog.set({ uN: n, uDepth: budget.depth, uSteps: budget.steps, uTaps: budget.taps, uShadow: budget.shadow, uLo: lo, uHi: hi });
  const draw = (cam, frame, look) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const doors = frame.doors ?? [];
    const pad = (k) => doors[k] ?? { pos: [0, 0, 0], scale: 1, glow: 0 };
    blend(gl, null);
    prog.set({
      ...uniforms({ w: port[2], h: port[3] }, cam),
      uScale: frame.scale,
      uFog: frame.fog ?? 0,
      uOut: frame.outside ? 1 : 0,
      uDoor: [0, 1].flatMap((k) => [...pad(k).pos, pad(k).scale]),
      uGlow: [0, 1].map((k) => pad(k).glow),
      uAccent: look.accent,
      uRamp: look.ramp.flat(),
      uHue: frame.hue ?? 0.2,
      uNear: frame.near ?? 0,
    });
    fill(gl);
  };
  const drop = () => {
    prog.drop();
    gl.deleteTexture(lo);
    gl.deleteTexture(hi);
  };
  return { draw, drop };
}

/* TOTEM */

const OBJECT = `#version 300 es
precision highp float;
precision highp sampler3D;
out vec4 o;
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform vec2 uOrigin;
uniform float uN;
uniform int uDepth;
uniform int uSteps;
uniform int uShadow;
uniform sampler3D uLo;
uniform sampler3D uHi;
uniform vec3 uAt;
uniform mat3 uTurn;
uniform vec3 uAxis;
uniform float uStretch;
uniform float uCell;
uniform float uSize;
uniform vec3 uDir;
uniform vec3 uColor;
uniform vec3 uAmbient;
uniform vec3 uTail;
uniform float uPlume;
uniform float uLength;
uniform vec3 uFire;
const float LOD = 1.5;
float box(vec3 p, vec3 b) {
  vec3 q = abs(p) - b;
  return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}
vec3 cube(vec3 p, float foot) {
  vec3 h = vec3(uN * 0.5);
  float away = box(p - h, h);
  if (away > 0.0) return vec3(away, 0.0, 1.0);
  float s = 1.0;
  int top = int(uN) - 1;
  for (int k = 0; k < 4; k++) {
    if (k >= uDepth || s < foot) break;
    ivec3 c = clamp(ivec3(floor(p)), ivec3(0), ivec3(top));
    vec4 lo = texelFetch(uLo, c, 0);
    if (lo.x < 0.5) {
      vec3 a = p - lo.yzw * 255.0;
      vec3 b = texelFetch(uHi, c, 0).xyz * 255.0 - p;
      return vec3(max(min(min(min(a.x, b.x), min(a.y, b.y)), min(a.z, b.z)), 0.0) * s, 0.0, s);
    }
    p = (p - vec3(c)) * uN;
    s /= uN;
  }
  return vec3(box(p - h, h) * s, 1.0, s);
}
vec3 leaf(vec3 p, float foot) {
  float s = 1.0;
  int top = int(uN) - 1;
  for (int k = 0; k < 4; k++) {
    if (k >= uDepth || s < foot) break;
    ivec3 c = clamp(ivec3(floor(p)), ivec3(0), ivec3(top));
    p = (p - vec3(c)) * uN;
    s /= uN;
  }
  return p;
}
vec3 squeeze(vec3 v) {
  return v + (1.0 / uStretch - 1.0) * dot(v, uAxis) * uAxis;
}
float shade(vec3 p, vec3 l, float foot) {
  vec3 inv = 1.0 / mix(l, vec3(1e-6), vec3(lessThan(abs(l), vec3(1e-6))));
  vec3 far = max(-p * inv, (vec3(uN) - p) * inv);
  float leave = min(min(far.x, far.y), far.z);
  float t = foot * 2.0;
  for (int i = 0; i < 32; i++) {
    if (t > leave) return 1.0;
    vec3 r = cube(p + l * t, foot * LOD);
    if (r.y > 0.5) return 0.0;
    t += max(r.x, foot);
  }
  return 1.0;
}
float occlusion(vec3 p, vec3 nr, float foot) {
  float dark = 0.0;
  float w = 1.0;
  for (int i = 1; i <= 3; i++) {
    float a = 0.14 * float(i * i);
    dark += clamp((a - cube(p + nr * a, foot * LOD).x) / a, 0.0, 1.0) * w;
    w *= 0.6;
  }
  return clamp(1.0 - dark * 0.45, 0.0, 1.0);
}
float trace(vec3 so, vec3 sd, float sdl, float base, float hi, float pix, out float foot) {
  vec3 inv = 1.0 / mix(sd, vec3(1e-6), vec3(lessThan(abs(sd), vec3(1e-6))));
  vec3 a = -so * inv;
  vec3 b = (vec3(uN) - so) * inv;
  vec3 near = min(a, b);
  vec3 far = max(a, b);
  float t = max(max(near.x, near.y), max(near.z, 0.0));
  float leave = min(min(min(far.x, far.y), far.z), hi);
  foot = 0.0;
  if (leave <= t) return -1.0;
  for (int i = 0; i < 200; i++) {
    if (i >= uSteps) return t;
    foot = max((base + t / sdl) * pix / uCell, 1e-6);
    vec3 r = cube(so + sd * t, foot * LOD);
    if (r.y > 0.5) return t;
    t += max(r.x, foot * 0.5);
    if (t > leave) return -1.0;
  }
  return t;
}
void main() {
  float span = min(uRes.x, uRes.y);
  vec2 uv = (gl_FragCoord.xy - uOrigin - 0.5 * uRes) / span * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  vec3 ro = uPos;
  float pix = 2.0 * uTan / span;
  mat3 back = transpose(uTurn);
  vec3 h = vec3(uN * 0.5);
  float along = dot(rd, uAxis);
  float tc = along > 1e-6 ? max(dot(uAt - ro, uAxis) / along, 0.0) : 1e9;
  vec3 so = back * (ro - uAt) / uCell + h;
  vec3 sd = back * rd / uCell;
  float sdl = length(sd);
  sd /= sdl;
  float foot = 0.0;
  float t = trace(so, sd, sdl, 0.0, tc * sdl, pix, foot);
  bool hit = t >= 0.0;
  bool pulled = false;
  float tw = hit ? t / sdl : 1e9;
  vec3 p = so + sd * max(t, 0.0);
  if (!hit && tc < 1e8) {
    vec3 sb = back * (ro + rd * tc - uAt) / uCell + h;
    vec3 db = back * squeeze(rd) / uCell;
    float dl = length(db);
    db /= dl;
    t = trace(sb, db, dl, tc, 1e9, pix, foot);
    hit = t >= 0.0;
    pulled = hit;
    tw = hit ? tc + t / dl : 1e9;
    p = sb + db * max(t, 0.0);
    sd = db;
  }
  vec3 col = vec3(0.0);
  if (hit) {
    vec3 q = leaf(p, foot * LOD);
    vec3 e = min(q, vec3(uN) - q);
    vec3 axis = e.x < e.y && e.x < e.z ? vec3(1.0, 0.0, 0.0) : e.y < e.z ? vec3(0.0, 1.0, 0.0) : vec3(0.0, 0.0, 1.0);
    vec3 nl = -axis * sign(dot(axis, sd));
    vec3 nw = normalize(pulled ? squeeze(uTurn * nl) : uTurn * nl);
    vec3 ll = normalize(back * (pulled ? squeeze(uDir) : uDir));
    float dif = max(dot(nw, uDir), 0.0);
    float lit = uShadow > 0 && dif > 0.0 ? shade(p + nl * foot * 2.0, ll, foot) : 1.0;
    float ao = occlusion(p + nl * foot, nl, foot);
    vec3 alb = vec3(0.8, 0.81, 0.84) * (0.9 + 0.1 * dot(axis, vec3(1.0, 0.6, 0.3)));
    float spec = pow(max(dot(nw, normalize(uDir - rd)), 0.0), 40.0) * 0.35 * lit * step(0.001, dif);
    float rim = pow(1.0 - max(dot(nw, -rd), 0.0), 3.0) * 0.25;
    col = alb * (uColor * dif * lit + uAmbient) * ao + uColor * (spec + rim * ao);
  }
  if (uPlume > 0.001) {
    for (int i = 0; i < 14; i++) {
      float fi = (float(i) + 0.5) / 14.0;
      vec3 s = uTail - uAxis * uLength * fi;
      float ts = dot(s - ro, rd);
      if (ts < 0.0 || ts > tw) continue;
      float dd = length(ro + rd * ts - s);
      float w = uSize * (0.3 - 0.22 * fi);
      col += uFire * uPlume * exp(-dd * dd / (w * w)) * 0.07 * (1.0 - fi);
    }
  }
  o = vec4(col, hit ? 1.0 : 0.0);
}
`;

const grow = (x) => 1 + GROW * (Math.exp(RATE * Math.min(Math.max(x, 0), 1)) - 1);

const axes = (cam) => {
  const m = cam.rot;
  return [[m[0], m[1], m[2]], [m[3], m[4], m[5]], [m[6], m[7], m[8]]];
};

const lerp = (a, b, k) => ({ dir: norm(mix(a.dir, b.dir, k)), color: mix(a.color, b.color, k), ambient: mix(a.ambient, b.ambient, k) });

export function ride(trip, cam, sun = STAR) {
  const { name, k } = trip.at();
  const flow = trip.flow();
  const [, up, fwd] = axes(cam);
  const flick = 1 + 0.35 * (flow.flicker ?? 0);
  const tip = flow.tint ?? WHITE;
  const blue = { dir: norm(add(mul(up, 0.6), mul(fwd, -0.8))), color: mul(tip, BLUE.color * flick), ambient: mul(flow.halo ?? tip, BLUE.ambient * flick) };
  const flash = { dir: norm(add(mul(up, 0.3), mul(fwd, -1))), color: mul(PALE, FLASH.color), ambient: mul(PALE, FLASH.ambient) };
  const star = { ...sun, dir: norm(sun.dir) };
  const ease = smooth(k);
  if (name === 'stretch') return [lerp(star, blue, 0.5 * ease), grow(SPLIT * k)];
  if (name === 'pile') return [lerp(star, blue, 0.5 + 0.5 * k), grow(SPLIT + (1 - SPLIT) * k)];
  if (name === 'flash') return [lerp(flash, blue, ease), grow(1 - k / SNAP)];
  if (name === 'tunnel') return [blue, 1];
  if (name === 'bloom') return [lerp(blue, flash, k * k), 1];
  if (name === 'white') return [flash, 1];
  if (name === 'decay') return [lerp(flash, star, ease), grow(1 - k)];
  return [star, 1];
}

export function totem(gl, view, { code = -1, n = 3, seed = 0 } = {}) {
  if (!Number.isInteger(code) || code < 0 || code > 255) return null;
  const all = cells(code, n);
  if (!all.some(Boolean)) return null;
  const budget = tier(view) === 'phone' ? PHONE : DESK;
  const prog = program(gl, OBJECT);
  const made = boxes(all, n);
  const lo = texture(gl, { w: n, h: n, d: n, data: made.lo, filter: 'nearest' });
  const hi = texture(gl, { w: n, h: n, d: n, data: made.hi, filter: 'nearest' });
  const rand = rng((seed ^ TOTEM) >>> 0);
  const pitch = PITCH[0] + (PITCH[1] - PITCH[0]) * rand();
  const lean = LEAN * (rand() * 2 - 1);
  const rate = (TURN[0] + (TURN[1] - TURN[0]) * rand()) * (rand() < 0.5 ? -1 : 1);
  const phase = rand() * TAU;
  const fire = FIRES[Math.floor(rand() * FIRES.length)];
  prog.set({ uN: n, uDepth: LEVEL, uSteps: budget.steps, uShadow: budget.shadow, uLo: lo, uHi: hi, uFire: fire, uSize: SIDE, uCell: SIDE / n });
  const pose = (cam, S) => {
    const [right, up, fwd] = axes(cam);
    const spin = phase + (rate * view.t) / 1000;
    const turn = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].map((e) => {
      const v = rot(rot(rot(e, [0, 1, 0], spin), [1, 0, 0], -pitch), [0, 0, 1], lean);
      return add(add(mul(right, v[0]), mul(up, v[1])), mul(fwd, v[2]));
    });
    const reach = (SIDE / 2) * turn.reduce((sum, v) => sum + Math.abs(dot(v, fwd)), 0);
    const at = add(add(add(cam.pos, mul(right, RIDE[0])), mul(up, RIDE[1])), mul(fwd, RIDE[2]));
    const tail = sub(at, mul(fwd, reach));
    const corners = Array.from({ length: 8 }, (_, i) => {
      const off = turn.reduce((sum, v, j) => add(sum, mul(v, ((i >> j) & 1 ? 0.5 : -0.5) * SIDE)), [0, 0, 0]);
      return [add(at, off), add(at, add(off, mul(fwd, (S - 1) * Math.max(dot(off, fwd), 0))))];
    }).flat();
    return { turn, at, tail, fwd, right, up, corners };
  };
  const frame = (port, cam, points) => {
    const [x0, y0, w, h] = port;
    const seen = points.map((p) => project({ w, h }, cam, p));
    if (seen.some((p) => !p)) return [x0, y0, w, h];
    const xs = seen.map((p) => p[0]);
    const ys = seen.map((p) => h - p[1]);
    const left = Math.max(0, Math.floor(Math.min(...xs)) - 2);
    const right = Math.min(w, Math.ceil(Math.max(...xs)) + 2);
    const low = Math.max(0, Math.floor(Math.min(...ys)) - 2);
    const top = Math.min(h, Math.ceil(Math.max(...ys)) + 2);
    return right > left && top > low ? [x0 + left, y0 + low, right - left, top - low] : null;
  };
  const draw = (cam, light = STAR, stretch = 1) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const S = Math.max(stretch, 1);
    const flame = Math.min(1, Math.log(S) / FLARE) * (0.9 + 0.1 * Math.sin(view.t * 0.04));
    const length = SIDE * TAIL * (0.4 + flame);
    const { turn, at, tail, fwd, right, up, corners } = pose(cam, S);
    const plume = flame > 0.001 ? [0, 0.5, 1].flatMap((f) => {
      const c = sub(tail, mul(fwd, length * f));
      const r = WIDE * SIDE * (0.3 - 0.22 * f);
      return [add(c, mul(right, r)), sub(c, mul(right, r)), add(c, mul(up, r)), sub(c, mul(up, r))];
    }) : [];
    const box = frame(port, cam, [...corners, ...plume]);
    if (!box) return;
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(...box);
    blend(gl, 'alpha');
    prog.set({
      ...uniforms({ w: port[2], h: port[3] }, cam),
      uOrigin: [port[0], port[1]],
      uAt: at,
      uTurn: turn.flat(),
      uAxis: fwd,
      uStretch: S,
      uDir: norm(light.dir),
      uColor: light.color,
      uAmbient: light.ambient ?? [0, 0, 0],
      uTail: tail,
      uPlume: flame,
      uLength: length,
    });
    fill(gl);
    blend(gl, null);
    gl.disable(gl.SCISSOR_TEST);
  };
  const drop = () => {
    prog.drop();
    gl.deleteTexture(lo);
    gl.deleteTexture(hi);
  };
  return { draw, drop };
}
