import { rgb, rng } from '../scene.js';
import { blend, fill, program, target, texture, tier } from './gl2.js';

const WIDE = 4096;
export const FLOOR = 1e-30;
const BITS = Math.max(128, Math.ceil(Math.log2(1 / FLOOR)) + 64);
const ESCAPE = 1e4;
const RATE = 0.5;
const GLIDE = 1500;
const HOLD = 2000;
export const FADE = 800;
const FRAMED = 0.4;
const EXTENT = 2.5;
export const SPAN = EXTENT / FRAMED;
const LAST = 12;
const BASE = 256;
const MORE = 96;
const LEASH = 2 ** 12;
const TILE = 8;
const GAP = 2;
const NEWTON = 64;
const VAR = 16;
const SHARP = 0.25;
const ZERO = 1e-8;
const NEAR = 2;
const REACH = 64;
const SOON = 24;
const CLOSE = 5;
const WINDOW = 12;
const AFAR = 8;
const LONG = 3;
const LATE = 30;
const HEAVY = 1 / 3;
const AGE = 2 ** 8;
const DRIFT = 1 / 64000;
const DIM = 0.14;
const LACE = 0.3;
const LIN = 1 / 128;
const REACHED = 0.08;
const JITTER = 0.4;
const SALT = 0x2c1b3c6d;
const GOLD = 0x9e3779b1;
const MIX = 0x85ebca6b;
const TIERS = {
  phone: { probe: 48, marks: 3, cap: 4096, budget: 800 },
  desk: { probe: 64, marks: 4, cap: 8192, budget: 2000 },
};
const HOME = { set: [-0.75, 0], julia: [0, 0] };
const RAMPS = {
  fire: ['#000000', '#9a2005', '#f06a12', '#ffe79a'],
  ice: ['#000000', '#0d4f8c', '#47c3ee', '#f2fdff'],
  mono: ['#000000', '#4a4a4a', '#a2a2a2', '#ffffff'],
};

/* NUMBERS */

export function fixed(x, bits = BITS) {
  if (!x || !Number.isFinite(x)) return 0n;
  let e = Math.floor(Math.log2(Math.abs(x))) - 52;
  if (Math.abs(x) / 2 ** e < 2 ** 52) e -= 1;
  const m = BigInt(Math.round(x / 2 ** e));
  const s = bits + e;
  return s >= 0 ? m << BigInt(s) : m >> BigInt(-s);
}

export const float = (v, bits = BITS) => Number(v) * 2 ** -bits;

const shift = (v, dx, dy) => [v[0] + fixed(dx), v[1] + fixed(dy)];

const apart = (a, b) => [float(a[0] - b[0]), float(a[1] - b[1])];

const WORD = BigInt(BITS);

const LIMIT = BigInt(ESCAPE) << WORD;

const smooth = (u) => {
  const x = Math.min(Math.max(u, 0), 1);
  return x * x * (3 - 2 * x);
};

/* ORBIT */

export function orbit({ c, z0 = null, bits = BITS, max = 8192 }) {
  const b = BigInt(bits);
  const rows = Math.max(1, Math.ceil(max / WIDE));
  const data = new Float32Array(rows * WIDE * 2);
  const wide = new Float64Array(max * 2);
  const limit = BigInt(ESCAPE) << b;
  const [cr, ci] = c;
  let [zr, zi] = z0 ?? [0n, 0n];
  let length = 0;
  let done = false;
  const put = () => {
    const x = float(zr, bits);
    const y = float(zi, bits);
    wide[2 * length] = x;
    wide[2 * length + 1] = y;
    data[2 * length] = x;
    data[2 * length + 1] = y;
    length += 1;
  };
  put();
  const self = { data, wide, rows, max, julia: Boolean(z0), length, done, crit: null };
  self.crit = self;
  self.step = (n = Infinity) => {
    for (let k = 0; k < n && !done; k++) {
      const rr = (zr * zr) >> b;
      const ii = (zi * zi) >> b;
      if (length >= max || rr + ii > limit) {
        done = true;
        break;
      }
      const ri = (zr * zi) >> (b - 1n);
      zr = rr - ii + cr;
      zi = ri + ci;
      put();
    }
    self.length = length;
    self.done = done;
    return done;
  };
  return self;
}

const walker = (ref, dc) => {
  const julia = ref.julia;
  const er = julia ? 0 : dc[0];
  const ei = julia ? 0 : dc[1];
  let o = ref;
  let m = 0;
  let dr = julia ? dc[0] : 0;
  let di = julia ? dc[1] : 0;
  return () => {
    const tr = 2 * o.wide[2 * m] + dr;
    const ti = 2 * o.wide[2 * m + 1] + di;
    const nr = tr * dr - ti * di + er;
    di = tr * di + ti * dr + ei;
    dr = nr;
    m += 1;
    const zr = o.wide[2 * m] + dr;
    const zi = o.wide[2 * m + 1] + di;
    if (Math.hypot(zr, zi) < Math.hypot(dr, di) || m >= o.length - 1) {
      o = o.crit;
      m = 0;
      dr = zr - o.wide[0];
      di = zi - o.wide[1];
    }
    return [zr, zi];
  };
};

const start = (ref, dc) => (ref.julia ? [ref.wide[0] + dc[0], ref.wide[1] + dc[1]] : [0, 0]);

export function period(ref, dc, max = ref.max) {
  const next = walker(ref, dc);
  let low = Infinity;
  let p = 0;
  for (let n = 1; n <= max; n++) {
    const [zr, zi] = next();
    const r = zr * zr + zi * zi;
    if (r > ESCAPE) break;
    if (r < low) {
      if (!p || n % p || r < SHARP * low) p = n;
      low = r;
    }
  }
  return p;
}

function exact(ref, dc, p) {
  const next = walker(ref, dc);
  const tiny = [];
  for (let n = 1; n <= p; n++) {
    const [zr, zi] = next();
    tiny.push(Math.hypot(zr, zi) < ZERO);
  }
  for (let q = 1; q <= p; q++) if (p % q === 0 && tiny[q - 1]) return q;
  return 0;
}

export function nucleus(ref, dc, p) {
  if (!(p >= 1)) return null;
  let [x, y] = dc;
  let tol = 0;
  let settled = 0;
  for (let it = 0; it < NEWTON; it++) {
    const next = walker(ref, [x, y]);
    let [zr, zi] = start(ref, [x, y]);
    let gr = ref.julia ? 1 : 0;
    let gi = 0;
    for (let n = 1; n <= p; n++) {
      const nr = 2 * (zr * gr - zi * gi) + (ref.julia ? 0 : 1);
      gi = 2 * (zr * gi + zi * gr);
      gr = nr;
      [zr, zi] = next();
    }
    const q = gr * gr + gi * gi;
    if (!(q > 0) || !Number.isFinite(zr + zi)) return null;
    const sr = (zr * gr + zi * gi) / q;
    const si = (zi * gr - zr * gi) / q;
    x -= sr;
    y -= si;
    const step = Math.hypot(sr, si);
    if (!Number.isFinite(x + y)) return null;
    if (!it) tol = 1e-12 * Math.max(step, Math.hypot(...dc));
    if (step <= tol) settled += 1;
    if (settled > 1) return [x, y];
  }
  return null;
}

function ball(ref, dc, radius, max = ref.max) {
  const next = walker(ref, dc);
  let [zr, zi] = start(ref, dc);
  let r = ref.julia ? radius : 0;
  for (let n = 1; n <= max; n++) {
    r = (2 * Math.hypot(zr, zi) + r) * r + (ref.julia ? 0 : radius);
    [zr, zi] = next();
    const a = Math.hypot(zr, zi);
    if (a < r) return n;
    if (a * a > ESCAPE || !Number.isFinite(r)) return 0;
  }
  return 0;
}

export function size(ref, dc, p) {
  const next = walker(ref, dc);
  if (ref.julia) {
    let [zr, zi] = start(ref, dc);
    let lr = 1;
    let li = 0;
    for (let n = 0; n < p; n++) {
      const nr = 2 * (zr * lr - zi * li);
      li = 2 * (zr * li + zi * lr);
      lr = nr;
      if (n < p - 1) [zr, zi] = next();
    }
    const q = lr * lr + li * li;
    return [lr / q, -li / q];
  }
  let lr = 1;
  let li = 0;
  let br = 1;
  let bi = 0;
  for (let n = 1; n < p; n++) {
    const [zr, zi] = next();
    const nr = 2 * (zr * lr - zi * li);
    li = 2 * (zr * li + zi * lr);
    lr = nr;
    const q = lr * lr + li * li;
    br += lr / q;
    bi -= li / q;
  }
  const l2r = lr * lr - li * li;
  const l2i = 2 * lr * li;
  const dr = br * l2r - bi * l2i;
  const di = br * l2i + bi * l2r;
  const q = dr * dr + di * di;
  return [dr / q, -di / q];
}

/* PROBE */

function decode(probe, w, h) {
  const n = w * h;
  const inside = new Uint8Array(n);
  const count = new Float32Array(n);
  const per = new Int32Array(n);
  for (let i = 0; i < n; i++) {
    const b = probe[4 * i + 2];
    inside[i] = b >= 128 ? 1 : 0;
    count[i] = ((probe[4 * i] << 8) | probe[4 * i + 1]) / 8;
    per[i] = ((b & 127) << 8) | probe[4 * i + 3];
  }
  const edge = new Uint8Array(n);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const i = y * w + x;
      for (const [nx, ny] of [[x + 1, y], [x, y + 1]]) {
        if (nx >= w || ny >= h) continue;
        const j = ny * w + nx;
        if (inside[i] !== inside[j] || (!inside[i] && Math.abs(count[i] - count[j]) > 2)) {
          edge[i] = 1;
          edge[j] = 1;
        }
      }
    }
  }
  return { inside, count, per, edge };
}

function islands(inside, per, w, h) {
  const seen = new Uint8Array(w * h);
  const found = [];
  for (let s = 0; s < w * h; s++) {
    if (!inside[s] || seen[s]) continue;
    const stack = [s];
    seen[s] = 1;
    let touch = false;
    let sx = 0;
    let sy = 0;
    let n = 0;
    const votes = new Map();
    while (stack.length) {
      const i = stack.pop();
      const x = i % w;
      const y = (i - x) / w;
      if (x === 0 || y === 0 || x === w - 1 || y === h - 1) touch = true;
      sx += x;
      sy += y;
      n += 1;
      votes.set(per[i], (votes.get(per[i]) ?? 0) + 1);
      for (const [nx, ny] of [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]) {
        if (nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
        const j = ny * w + nx;
        if (inside[j] && !seen[j]) {
          seen[j] = 1;
          stack.push(j);
        }
      }
    }
    if (touch) continue;
    let period = 0;
    let most = 0;
    for (const [p, k] of votes) {
      if (k > most || (k === most && p < period)) {
        most = k;
        period = p;
      }
    }
    if (period > 0 && most * 2 >= n && n <= TILE * TILE) found.push({ x: sx / n + 0.5, y: sy / n + 0.5, n, period });
  }
  return found;
}

export function score(probe, w, h) {
  const { inside, count, per, edge } = decode(probe, w, h);
  const pools = islands(inside, per, w, h);
  const list = [];
  for (let ty = 0; ty * TILE < h; ty++) {
    for (let tx = 0; tx * TILE < w; tx++) {
      let n = 0;
      let held = 0;
      let rim = 0;
      let sum = 0;
      let sq = 0;
      let best = -1;
      let top = -1;
      for (let y = ty * TILE; y < Math.min(h, (ty + 1) * TILE); y++) {
        for (let x = tx * TILE; x < Math.min(w, (tx + 1) * TILE); x++) {
          const i = y * w + x;
          n += 1;
          rim += edge[i];
          if (inside[i]) {
            held += 1;
            continue;
          }
          sum += count[i];
          sq += count[i] * count[i];
          if (edge[i] && (best < 0 || count[i] > count[best])) best = i;
          if (top < 0 || count[i] > count[top]) top = i;
        }
      }
      const out = n - held;
      const mean = out ? sum / out : 0;
      const variance = out > 1 ? Math.max(0, sq / out - mean * mean) : 0;
      if (held > 0.9 * n || variance < 0.5) continue;
      const pool = pools.find((one) => Math.floor(one.x - 0.5) >= tx * TILE && Math.floor(one.x - 0.5) < (tx + 1) * TILE && Math.floor(one.y - 0.5) >= ty * TILE && Math.floor(one.y - 0.5) < (ty + 1) * TILE);
      const b = rim / n;
      const v = variance / (variance + VAR);
      const share = out / n;
      const s = b + v + (share >= 0.2 && share <= 0.8 ? 0.5 : 0) + 0.5 * b * v + (pool ? 1 : 0);
      const at = best >= 0 ? best : top;
      list.push({
        x: pool ? pool.x : (at % w) + 0.5,
        y: pool ? pool.y : Math.floor(at / w) + 0.5,
        ex: (at % w) + 0.5,
        ey: Math.floor(at / w) + 0.5,
        s,
        kind: pool ? 'island' : b * v > 0.25 ? 'spiral' : 'edge',
        period: pool ? pool.period : per[at],
        n: pool ? pool.n : 0,
        tx,
        ty,
      });
    }
  }
  return list.sort((a, b) => b.s - a.s);
}

function spread(list, most, gap = GAP) {
  const kept = [];
  for (const one of list) {
    if (kept.length >= most) break;
    if (kept.some((k) => Math.max(Math.abs(k.tx - one.tx), Math.abs(k.ty - one.ty)) <= gap)) continue;
    kept.push(one);
  }
  return kept;
}

function edge(probe, w, h) {
  const { inside, count, per, edge: rim } = decode(probe, w, h);
  let best = -1;
  let fallback = -1;
  for (let i = 0; i < w * h; i++) {
    if (inside[i] || !rim[i]) continue;
    const x = i % w;
    const y = (i - x) / w;
    const touches = [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].some(([nx, ny]) => nx >= 0 && ny >= 0 && nx < w && ny < h && inside[ny * w + nx]);
    if (touches && (best < 0 || count[i] > count[best])) best = i;
    if (fallback < 0 || count[i] > count[fallback]) fallback = i;
  }
  const at = best >= 0 ? best : fallback;
  return at < 0 ? null : { x: (at % w) + 0.5, y: Math.floor(at / w) + 0.5, period: per[at] };
}

/* JULIA */

function constant(seed, j = 0) {
  const rand = rng((seed ^ SALT ^ Math.imul(j + 1, GOLD)) >>> 0);
  for (let tries = 0; tries < 2000; tries++) {
    const cr = -1.9 + 2.35 * rand();
    const ci = -1.1 + 2.2 * rand();
    let zr = 0;
    let zi = 0;
    let gone = false;
    for (let n = 0; n < 2000 && !gone; n++) {
      const t = zr * zr - zi * zi + cr;
      zi = 2 * zr * zi + ci;
      zr = t;
      gone = zr * zr + zi * zi > 4;
    }
    if (gone) continue;
    const [ar, ai] = [zr, zi];
    let q = 0;
    for (let n = 1; n <= 16 && !q; n++) {
      const t = zr * zr - zi * zi + cr;
      zi = 2 * zr * zi + ci;
      zr = t;
      if (Math.hypot(zr - ar, zi - ai) < 1e-9) q = n;
    }
    if (q < 2) continue;
    let lr = 1;
    let li = 0;
    for (let n = 0; n < q; n++) {
      const nr = 2 * (zr * lr - zi * li);
      li = 2 * (zr * li + zi * lr);
      lr = nr;
      const t = zr * zr - zi * zi + cr;
      zi = 2 * zr * zi + ci;
      zr = t;
    }
    const k = Math.hypot(lr, li);
    if (k > 0.55 && k < 0.95) return [cr, ci];
  }
  return [-0.1226, 0.7449];
}

/* SHADER */

const shader = (probe) => `#version 300 es
precision highp float;
precision highp int;
precision highp sampler2D;
uniform sampler2D uOrbit;
uniform int uLen;
uniform int uCrit;
uniform int uCritLen;
uniform int uMax;
uniform bool uJulia;
uniform vec2 uRes;
uniform vec2 uOff;
uniform vec2 uU;
uniform vec2 uV;
${probe ? '' : `uniform bool uRamp;
uniform vec3 uA;
uniform vec3 uB;
uniform vec3 uC;
uniform vec3 uD;
uniform vec3 uS[4];
uniform float uFreq;
uniform float uDrift;
uniform float uAlpha;
uniform float uPx;
uniform float uFall;
`}out vec4 o;
vec2 at(int k) { return texelFetch(uOrbit, ivec2(k & 4095, k >> 12), 0).xy; }
void main() {
  vec2 p = gl_FragCoord.xy - 0.5 * uRes;
  vec2 dc = uOff + p.x * uU + p.y * uV;
  vec2 d = uJulia ? dc : vec2(0.0);
  vec2 e = uJulia ? vec2(0.0) : dc;
  int base = 0;
  int len = uLen;
  int m = 0;
  vec2 Z = at(0);
  vec2 w = Z + d;
${probe ? '' : `  vec2 dz = uJulia ? vec2(uPx, 0.0) : vec2(0.0);
  vec2 kick = uJulia ? vec2(0.0) : vec2(uPx, 0.0);
`}  float r = 0.0;
  int n = 0;
  bool gone = false;
  float low = 3.0e38;
  int per = 0;
  for (int i = 0; i < uMax; i++) {
${probe ? '' : `    dz = 2.0 * vec2(w.x * dz.x - w.y * dz.y, w.x * dz.y + w.y * dz.x) + kick;
`}    vec2 t = 2.0 * Z + d;
    d = vec2(t.x * d.x - t.y * d.y, t.x * d.y + t.y * d.x) + e;
    m++;
    Z = at(base + m);
    vec2 z = Z + d;
    w = z;
    r = dot(z, z);
    n = i + 1;
    if (r > ${ESCAPE.toFixed(1)}) {
      gone = true;
      break;
    }
${probe ? `    if (r < low) {
      if (per == 0 || n % per != 0 || r < ${SHARP} * low) per = n;
      low = r;
    }
` : ''}    if (max(abs(z.x), abs(z.y)) < max(abs(d.x), abs(d.y)) || m >= len - 1) {
      d = z;
      m = 0;
      base = uCrit;
      len = uCritLen;
      Z = at(base);
    }
  }
  float sl = float(n) + 1.0 - log2(max(0.5 * log2(max(r, 1.0)), 1e-6));
${probe ? `  int q = gone ? int(clamp(sl * 8.0, 0.0, 65535.0)) : 0;
  per = min(per, 32767);
  o = vec4(float(q >> 8), float(q & 255), float((gone ? 0 : 128) + (per >> 8)), float(per & 255)) / 255.0;
` : `  if (!gone) {
    o = vec4(0.0, 0.0, 0.0, uAlpha);
    return;
  }
  float x = log(max(sl, 1.0)) + sl * ${LIN.toFixed(6)};
  vec3 col;
  if (uRamp) {
    float k = fract(x * uFreq + uDrift);
    k = (1.0 - abs(2.0 * k - 1.0)) * 3.0;
    col = k < 1.0 ? mix(uS[0], uS[1], k) : k < 2.0 ? mix(uS[1], uS[2], k - 1.0) : mix(uS[2], uS[3], k - 2.0);
  } else {
    col = uA + uB * cos(6.2831853 * (uC * x + uD + uDrift));
  }
  float de = 0.5 * sqrt(r) * log(r) / max(length(dz), 1e-30);
  col *= mix(${DIM.toFixed(2)}, 1.0, exp(-de * uFall));
  col = mix(col, mix(col, vec3(1.0), ${LACE.toFixed(2)}), 1.0 - smoothstep(0.2, 0.9, de));
  col = pow(clamp(col, 0.0, 1.0), vec3(2.2));
  o = vec4(col * uAlpha, uAlpha);
`}}
`;

/* COLOUR */

const unit = (hex) => rgb(hex).map((v) => v / 255);

function tones(name, seed) {
  if (RAMPS[name]) return () => ({ uRamp: true, uS: RAMPS[name].flatMap(unit) });
  if (name === 'accent') {
    return (accent) => {
      const a = unit(accent);
      return { uRamp: true, uS: [[0, 0, 0], a.map((v) => v * 0.55), a, a.map((v) => v + (1 - v) * 0.75)].flat() };
    };
  }
  const rand = rng((seed ^ MIX) >>> 0);
  const f = 0.6 + 0.6 * rand();
  const d0 = rand();
  const spread = 0.05 + 0.15 * rand();
  return () => ({ uRamp: false, uA: [0.5, 0.5, 0.5], uB: [0.45, 0.45, 0.45], uC: [f, f, f], uD: [d0, d0 + spread, d0 + 2 * spread] });
}

/* DEEP */

export function deep(gl, view, { kind = 'set', seed = 0, palette = '', speed = 1, auto = 1 } = {}) {
  const julia = kind === 'julia';
  const plan = TIERS[tier(view) === 'phone' ? 'phone' : 'desk'];
  const rate = (RATE * speed) / 1000;
  const deepest = Math.log2(SPAN / FLOOR);
  const side = plan.probe;
  const paint = program(gl, shader(false));
  const sense = program(gl, shader(true));
  const sight = target(gl, side, side);
  const pixels = new Uint8Array(side * side * 4);
  const tone = tones(palette, seed);
  const home = HOME[julia ? 'julia' : 'set'].map((v) => fixed(v));
  let dive = null;
  let cur = null;
  let next = null;
  let upcoming = null;
  let markers = [];
  let lit = -1;
  let chosen = false;
  let tint = null;
  let shades = null;
  const upload = (ref) => {
    if (ref.tex) return ref;
    const a = ref.o.data;
    const data = julia ? new Float32Array(a.length + ref.o.crit.data.length) : a;
    if (julia) {
      data.set(a);
      data.set(ref.o.crit.data, a.length);
    }
    ref.tex = texture(gl, { w: WIDE, h: julia ? ref.o.rows * 2 : ref.o.rows, format: 'rg32f', data });
    return ref;
  };
  const release = (ref) => {
    if (ref?.tex && !ref.keep) gl.deleteTexture(ref.tex);
    if (ref && !ref.keep) ref.tex = null;
  };
  const reference = (at, dv) => {
    const o = julia ? orbit({ c: dv.c, z0: at, max: plan.cap }) : orbit({ c: at, max: plan.cap });
    if (julia) o.crit = dv.crit;
    return { at, o, tex: null, keep: false };
  };
  const crit = (j) => {
    const cf = constant(seed, j);
    const c = cf.map((v) => fixed(v));
    return { j, cf, c, o: orbit({ c, z0: [0n, 0n], max: plan.cap }) };
  };
  const settled = julia ? null : { at: home, o: orbit({ c: home, max: plan.cap }), tex: null, keep: true };
  settled?.o.step();
  if (settled) upload(settled);
  const prepare = (j) => {
    if (!julia) return { j, c: null, cf: null, crit: null, base: settled };
    const made = upcoming?.j === j ? upcoming : crit(j);
    made.o.step();
    return { j, c: made.c, cf: made.cf, crit: made.o, base: upload({ at: [0n, 0n], o: made.o, tex: null, keep: false }) };
  };
  const begin = (prep, t0) => {
    for (const ref of [cur, next, dive?.base]) if (ref !== prep.base) release(ref);
    dive = { ...prep, t0, phase: 'dive', k: 0, glide: null, arrive: null, goal: null, scan: null, rim: null, floor: t0 + deepest / rate, frozen: null, heir: null, fade: 0 };
    cur = prep.base;
    next = null;
    upcoming = julia ? crit(prep.j + 1) : null;
    markers = [];
    lit = -1;
    chosen = false;
  };
  const centreAt = (dv, t) => {
    const g = dv.glide;
    if (!g) return home;
    const u = (t - g.t) / g.ms;
    if (u >= 1) return g.to;
    if (u <= 0) return g.from;
    const e = smooth(u);
    return shift(g.from, g.dx * e, g.dy * e);
  };
  const spanAt = (dv, t) => {
    const a = dv.arrive;
    if (!a || t <= a.t) return SPAN * 2 ** (-rate * (Math.min(t, dv.floor) - dv.t0));
    const s = Math.min(t, a.end) - a.t;
    let l;
    if (a.go <= 0) l = a.la + (a.le - a.la) * smooth(s / GLIDE);
    else if (s <= a.cruise) l = a.la - rate * s;
    else {
      const u = s - a.cruise;
      const left = a.go - rate * a.cruise;
      l = a.la - rate * a.cruise - rate * u + ((rate * rate) / (4 * left)) * u * u;
    }
    return 2 ** l;
  };
  const angleAt = (dv, t) => {
    const a = dv.arrive;
    if (!a || t <= a.t) return 0;
    return a.angle * smooth((t - a.t) / (a.end - a.t));
  };
  const viewAt = (dv, t) => (dv.phase === 'fade' ? dv.frozen : { centre: centreAt(dv, t), span: spanAt(dv, t), angle: angleAt(dv, t) });
  const homeView = () => ({ centre: home, span: SPAN, angle: 0 });
  const iterations = (v) => Math.min(plan.cap, Math.round(BASE + MORE * Math.max(0, Math.log2(SPAN / v.span))));
  const spans = (v) => {
    const short = Math.min(view.w, view.h);
    return [(v.span * view.w) / short, (v.span * view.h) / short];
  };
  const polish = (at, p, dv) => {
    let [nr, ni] = at;
    for (let round = 0; round < 2; round++) {
      let zr = julia ? nr : 0n;
      let zi = julia ? ni : 0n;
      const [cr, ci] = julia ? dv.c : [nr, ni];
      let gr = julia ? 1 : 0;
      let gi = 0;
      for (let n = 0; n < p; n++) {
        const fr = float(zr);
        const fi = float(zi);
        const t = 2 * (fr * gr - fi * gi) + (julia ? 0 : 1);
        gi = 2 * (fr * gi + fi * gr);
        gr = t;
        const rr = (zr * zr) >> WORD;
        const ii = (zi * zi) >> WORD;
        if (rr + ii > LIMIT) return [nr, ni];
        const ri = (zr * zi) >> (WORD - 1n);
        zr = rr - ii + cr;
        zi = ri + ci;
      }
      const fr = float(zr);
      const fi = float(zi);
      const q = gr * gr + gi * gi;
      if (!(q > 0)) break;
      nr -= fixed((fr * gr + fi * gi) / q);
      ni -= fixed((fi * gr - fr * gi) / q);
    }
    return [nr, ni];
  };
  const settle = (at, p, dv, reach = Infinity, fine = true) => {
    const dc = apart(at, cur.at);
    const n = p ? nucleus(cur.o, dc, p) : null;
    if (!n || Math.hypot(n[0] - dc[0], n[1] - dc[1]) > reach) return null;
    const q = exact(cur.o, n, p) || p;
    const rough = shift(cur.at, n[0], n[1]);
    return { at: fine ? polish(rough, q, dv) : rough, n, p: q };
  };
  const anchor = (at, radius, dv, fine = true) => settle(at, ball(cur.o, apart(at, cur.at), radius, plan.cap), dv, REACH * radius, fine);
  const renew = (at, radius, dv) => {
    const found = anchor(at, radius, dv);
    if (found) next = reference(found.at, dv);
  };
  const framing = (v, found) => {
    const s = size(cur.o, found.n, found.p);
    const span = Math.hypot(s[0], s[1]) * SPAN;
    return span >= FLOOR && span < 0.7 * v.span ? { at: found.at, s, span, p: found.p } : null;
  };
  const swap = () => {
    upload(next);
    release(cur);
    cur = next;
    next = null;
  };
  const ensure = (v) => {
    if (next?.o.done) return swap();
    const [dx, dy] = apart(v.centre, cur.at);
    if (Math.hypot(dx, dy) / v.span < LEASH) return;
    next ??= reference(anchor(v.centre, v.span, dive)?.at ?? v.centre, dive);
    next.o.step();
    swap();
  };
  const shade = (prog, ref, v, w, h, extra) => {
    const [sx, sy] = spans(v);
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    prog.set({
      uOrbit: ref.tex,
      uLen: ref.o.length,
      uCrit: julia ? ref.o.rows * WIDE : 0,
      uCritLen: ref.o.crit.length,
      uMax: iterations(v),
      uJulia: julia,
      uRes: [w, h],
      uOff: apart(v.centre, ref.at),
      uU: [(cos * sx) / w, (sin * sx) / w],
      uV: [(-sin * sy) / h, (cos * sy) / h],
      uPx: sx / w,
      uFall: 1 / (REACHED * Math.min(w, h)),
      ...extra,
    });
    fill(gl);
  };
  const locate = (v, x, y) => {
    const [sx, sy] = spans(v);
    const u = (x / side - 0.5) * sx;
    const w = (y / side - 0.5) * sy;
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    return shift(v.centre, u * cos - w * sin, u * sin + w * cos);
  };
  const see = (v) => {
    ensure(v);
    gl.bindFramebuffer(gl.FRAMEBUFFER, sight.fb);
    gl.viewport(0, 0, side, side);
    blend(gl, null);
    shade(sense, upload(cur), v, side, side, {});
    gl.readPixels(0, 0, side, side, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
    return pixels;
  };
  const steer = (dv, to, t, exactly = null, ms = GLIDE) => {
    const from = centreAt(dv, t);
    const [dx, dy] = apart(to, from);
    dv.glide = { from, to, t, ms, dx, dy };
    if (exactly) next = reference(exactly, dv);
    else renew(to, spanAt(dv, t) / 4, dv);
  };
  const resolve = (v, one) => {
    if (one.hit !== undefined) return one.hit;
    const px = spans(v)[0] / side;
    const near = (found) => found && Math.hypot(...apart(found.at, one.at)) <= NEAR * TILE * px;
    let found = settle(one.at, one.period, dive, Infinity, false);
    if (!near(found)) found = anchor(one.at, px * Math.max(0.5, Math.sqrt(one.n / Math.PI)), dive, false);
    one.hit = near(found) ? framing(v, found) : null;
    return one.hit;
  };
  function* aim(dv, v, picks, late) {
    let best = null;
    for (const at of [v.centre, ...picks.map((one) => one.at)]) {
      const found = anchor(at, v.span / 8, dv, false);
      if (found && found.p <= plan.cap * HEAVY) {
        const [dx, dy] = apart(found.at, v.centre);
        const far = Math.hypot(dx, dy) / v.span;
        const hit = far <= AFAR ? framing(v, found) : null;
        if (hit && (hit.span <= FLOOR * 2 ** WINDOW || late) && (!best || far < best.far)) best = { ...hit, far };
      }
      yield;
    }
    if (best) best.at = polish(best.at, best.p, dv);
    return best;
  }
  const arrive = (dv, t, hit) => {
    const [hr, hi] = HOME[julia ? 'julia' : 'set'];
    const [sr, si] = hit.s;
    const to = shift(hit.at, sr * hr - si * hi, sr * hi + si * hr);
    const la = Math.log2(spanAt(dv, t));
    const le = Math.log2(hit.span);
    const go = la - le;
    const cruise = go > 1 ? (go - 1) / rate : 0;
    const end = t + (go <= 0 ? GLIDE : cruise + (2 * Math.min(go, 1)) / rate);
    steer(dv, to, t, hit.at);
    dv.arrive = { t, la, le, go, cruise, end, angle: Math.atan2(si, sr), p: hit.p };
    dv.scan = null;
    dv.phase = 'arrive';
    markers = [];
    lit = -1;
  };
  const take = (dv, one, t) => {
    dv.goal = null;
    dv.scan = null;
    const hit = one.kind === 'island' ? resolve(viewAt(dv, t), one) : null;
    if (hit) return arrive(dv, t, { ...hit, at: polish(hit.at, hit.p, dv) });
    steer(dv, one.at, t);
  };
  const choose = (dv, picks, late, k) => {
    const rand = rng((seed ^ Math.imul(dv.j + 1, GOLD) ^ Math.imul(k + 1, MIX)) >>> 0);
    const want = late && picks.some((one) => one.kind === 'island');
    const any = !picks.some((one) => (one.kind === 'island') === want);
    let best = -1;
    let top = -Infinity;
    picks.forEach((one, i) => {
      const s = one.s * (1 - JITTER / 2 + JITTER * rand());
      if ((any || (one.kind === 'island') === want) && s > top) {
        top = s;
        best = i;
      }
    });
    return best;
  };
  const decide = (dv, t, { v, picks, list, rim, depth }) => {
    if (!list.length) {
      const to = rim ? locate(v, rim.x, rim.y) : dv.rim;
      if (to) steer(dv, to, t);
    } else if (auto && lit >= 0 && picks[lit].kind === 'island' && depth < deepest - LAST) {
      steer(dv, locate(v, picks[lit].ex, picks[lit].ey), t);
    } else if (auto && lit >= 0) take(dv, picks[lit], t);
  };
  const aimed = (dv, t, seen, hit) => {
    dv.scan = null;
    if (!hit) return decide(dv, t, seen);
    dv.goal = hit;
    lit = -1;
    steer(dv, hit.at, t, hit.at, GLIDE * Math.min(1 + hit.far, LONG));
  };
  const scan = (dv, t, whole) => {
    const one = dv.scan;
    if (!one) return;
    let r = one.run.next();
    while (whole && !r.done) r = one.run.next();
    if (r.done) aimed(dv, whole ? one.t : t, one, r.value);
  };
  const mark = (dv, t, k) => {
    const v = viewAt(dv, t);
    const probe = see(v);
    const list = score(probe, side, side);
    const depth = Math.log2(SPAN / v.span);
    const picks = spread(list, plan.marks).map((one, i) => ({ ...one, id: k * 8 + i, number: i + 1, at: locate(v, one.x, one.y) }));
    for (const one of picks) {
      if (one.kind !== 'island' || resolve(v, one)) continue;
      one.kind = 'edge';
      one.at = locate(v, one.ex, one.ey);
    }
    const rim = edge(probe, side, side);
    if (rim) dv.rim = locate(v, rim.x, rim.y);
    markers = picks;
    chosen = false;
    lit = -1;
    if (dv.goal && Math.log2(v.span / dv.goal.span) <= CLOSE) return arrive(dv, t, dv.goal);
    if (!dv.goal) {
      lit = choose(dv, picks, depth >= deepest - LAST, k);
      const seen = { v, picks, list, rim, depth };
      if (auto && depth >= SOON) dv.scan = { ...seen, t, run: aim(dv, v, picks, depth >= deepest - LATE) };
      else decide(dv, t, seen);
    }
    const [dx, dy] = apart(v.centre, cur.at);
    if (!next && dv.phase === 'dive' && Math.hypot(dx, dy) / v.span >= AGE) renew(v.centre, v.span, dv);
  };
  const fadeOut = (dv, t) => {
    dv.frozen = viewAt(dv, t);
    dv.scan = null;
    dv.phase = 'fade';
    dv.fade = t;
    dv.heir = prepare(dv.j + 1);
    markers = [];
    lit = -1;
  };
  const advance = (t) => {
    for (;;) {
      const dv = dive;
      if (dv.phase === 'dive') {
        const at = dv.t0 + dv.k / rate;
        if (dv.floor <= t && dv.floor <= at) {
          fadeOut(dv, dv.floor);
          continue;
        }
        if (at > t) {
          scan(dv, t, false);
          return;
        }
        scan(dv, at, true);
        dv.k += 1;
        mark(dv, at, dv.k - 1);
        continue;
      }
      if (dv.phase === 'arrive') {
        const out = dv.arrive.end + HOLD;
        if (out > t) return;
        fadeOut(dv, out);
        continue;
      }
      const out = dv.fade + FADE;
      if (out > t) return;
      begin(dv.heir, out);
    }
  };
  const work = () => {
    let left = plan.budget;
    if (next && !next.o.done) {
      const before = next.o.length;
      next.o.step(left);
      left -= next.o.length - before + 1;
    }
    if (left > 0 && upcoming && !upcoming.o.done) upcoming.o.step(left);
  };
  const draw = (into = null) => {
    const t = view.t;
    advance(t);
    work();
    const v = viewAt(dive, t);
    ensure(v);
    const w = into?.w ?? view.w;
    const h = into?.h ?? view.h;
    gl.bindFramebuffer(gl.FRAMEBUFFER, into?.fb ?? null);
    gl.viewport(0, 0, w, h);
    blend(gl, null);
    const accent = view.look().accent;
    if (accent !== tint) {
      tint = accent;
      shades = tone(accent);
    }
    const colour = { ...shades, uFreq: 1, uDrift: t * DRIFT };
    shade(paint, upload(cur), v, w, h, { ...colour, uAlpha: 1 });
    if (dive.phase === 'fade') {
      const k = smooth((t - dive.fade) / FADE);
      blend(gl, 'alpha');
      shade(paint, dive.heir.base, homeView(), w, h, { ...colour, uAlpha: k });
      blend(gl, null);
    }
  };
  const marks = () => {
    if (dive.phase !== 'dive') return [];
    const v = viewAt(dive, view.t);
    const [sx, sy] = spans(v);
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    return markers.map((one, i) => {
      const [dx, dy] = apart(one.at, v.centre);
      const x = ((dx * cos + dy * sin) / sx + 0.5) * view.w;
      const y = (0.5 - (-dx * sin + dy * cos) / sy) * view.h;
      return { id: one.id, x, y, label: String(one.number), number: one.number, on: i === lit, kind: one.kind };
    }).filter((one) => one.x >= 0 && one.y >= 0 && one.x <= view.w && one.y <= view.h);
  };
  const go = (id) => {
    if (dive.phase !== 'dive') return false;
    const i = markers.findIndex((one) => one.id === id);
    if (i < 0) return false;
    lit = i;
    chosen = true;
    take(dive, markers[i], view.t);
    return true;
  };
  const depth = () => viewAt(dive, view.t).span;
  const info = () => ({ centre: centreAt(dive, view.t).map(String), phase: dive.phase, dive: dive.j, c: dive.cf, lit: markers[lit]?.number ?? 0, chosen, goal: dive.goal?.p ?? dive.arrive?.p ?? 0, aiming: Boolean(dive.scan) });
  const drop = () => {
    for (const ref of [cur, next, dive?.base, dive?.heir?.base, settled]) {
      if (ref?.tex) gl.deleteTexture(ref.tex);
      if (ref) ref.tex = null;
    }
    paint.drop();
    sense.drop();
    sight.drop();
  };
  begin(prepare(0), 0);
  return { draw, marks, go, depth, info, drop };
}
