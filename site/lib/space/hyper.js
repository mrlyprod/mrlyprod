import { rgb, rng } from '../scene.js';
import { ease, look as aim, project, shake } from './camera.js';
import { clock, flown } from './clock.js';
import { blend, fill, governor, hdr, program, target, texture, tier } from './gl2.js';
import { post } from './post.js';
import { sky as field } from './sky.js';
import { add, mul, norm } from './vec.js';

export const FILM = { wind: 300, stretch: 900, pile: 600, flash: 300, tunnel: 1200, bloom: 400, white: 200, decay: 900, snap: 200 };

const OLD = { wind: [['wind', 1]], run: [['stretch', 0.6], ['pile', 0.4]], punch: [['flash', 1]], snap: [['bloom', 1]], flash: [['white', 1]], fade: [['decay', 9 / 11], ['snap', 2 / 11]] };

const JUMP = ['wind', 'stretch', 'pile', 'flash', 'tunnel'];
const EXIT = ['bloom', 'white', 'decay', 'snap'];
const BUSY = ['flash', 'tunnel', 'bloom'];
const EARLY = ['wind', 'stretch', 'pile'];
const HOST = { wind: 'wind', stretch: 'jump', pile: 'jump', flash: 'jump', tunnel: 'hyper', bloom: 'exit', white: 'exit', decay: 'exit', snap: 'exit' };
const GAPS = { yes: [12, 12], mix: [6, 30] };
const STAY = [4, 10];
const SALT = 0x5bd1e995;
const EXTRA = 0x3c6ef372;
const GRAIN = 0x27d4eb2f;
const NOTCHES = { phone: [0.5, 0.4, 0.33, 0], desk: [0.75, 0.6, 0.5, 0], big: [0.5, 0.4, 0.33, 0] };
const TOP = 14;
const SHUTTER = 0.11;
const FLOW = 0.3;
const LEAD = 120;
const PEAK = 40;
const STILL = 1000;
const HOT = 1.8;
const CORE = 0.08;
const DRIFT = 0.05;
const ROLL = 0.2;
const NOISE = 256;
const OCTAVES = [8, 16, 32, 64];
const WHITE = [1, 1, 1];
const BLACK = [0, 0, 0];
const BLOOM = 0.8;
const FLARE = 12;
const NEUTRAL = [0.46, 0.76, 0.9];
const FILMIC = { deep: '#0c1869', mid: '#2d62d5', tip: '#b3e1f3', pale: '#e1fdff' };
const AHEAD = aim([0, 0, 0], [0, 0, 1]);
const PATHS = {
  wind: '#040a14',
  lift: '#081428',
  pile: '#0f2c44',
  decay: [[0, '#b2eefb'], [0.11, '#699ae8'], [0.44, '#6298e5'], [0.56, '#4675d2'], [0.67, '#132e78'], [0.78, '#101e42'], [0.89, '#0c1838'], [1, '#0a0d17']],
  streak: [[0, '#b8ecfb'], [0.44, '#b7edf8'], [0.56, '#7ebaf0'], [0.67, '#5a8ad8'], [1, '#4a6fc0']],
};

/* TIMES */

const number = (v) => (String(v ?? '').trim() ? Number(v) : NaN);

export function film(opts = {}) {
  const out = { ...FILM };
  for (const [key, parts] of Object.entries(OLD)) {
    const v = number(opts?.[key]);
    if (!Number.isFinite(v) || v < 0) continue;
    for (const [name, share] of parts) out[name] = Math.round(v * 1000 * share);
  }
  return out;
}

/* COLOUR */

const aces = (x) => Math.min(Math.max((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0), 1);

const unshade = (c) => {
  const want = Math.min(Math.max(c, 0), 0.995) ** 2.2;
  let lo = 0;
  let hi = 16;
  for (let i = 0; i < 36; i++) {
    const mid = (lo + hi) / 2;
    if (aces(mid) < want) lo = mid;
    else hi = mid;
  }
  return (lo + hi) / 2;
};

const shown = (hex) => rgb(hex).map((c) => c / 255);

const linear = (display) => display.map(unshade);

const blendTo = (a, b, k) => a.map((v, i) => v + (b[i] - v) * k);

const peakOf = (c) => c.map((v) => v / Math.max(...c));

const along = (stops, k) => {
  const i = Math.max(0, stops.findIndex(([at]) => at >= k) - 1);
  const [a, from] = stops[i];
  const [b, to] = stops[Math.min(i + 1, stops.length - 1)];
  return blendTo(from, to, b > a ? smooth(Math.min(Math.max((k - a) / (b - a), 0), 1)) : 0);
};

const shade = (hue, c) => {
  const lum = 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
  return lum < 0.5 ? blendTo(BLACK, hue, lum * 2) : blendTo(hue, WHITE, lum * 2 - 1);
};

const grey = (c) => {
  const lum = 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
  return [lum, lum, lum];
};

function trails(tint, plain = false) {
  const hue = tint ? shown(tint) : null;
  const tone = (hex) => linear(plain ? grey(shown(hex)) : hue ? shade(hue, shown(hex)) : shown(hex));
  const lift = (hex) => (plain ? BLACK : tone(hex));
  return { wind: lift(PATHS.wind), lift: lift(PATHS.lift), pile: lift(PATHS.pile), decay: PATHS.decay.map(([k, hex]) => [k, tone(hex)]), streak: PATHS.streak.map(([k, hex]) => [k, tone(hex)]) };
}

function ramp(tint) {
  if (!tint) return Object.fromEntries(Object.entries(FILMIC).map(([key, hex]) => [key, shown(hex)]));
  const hue = shown(tint);
  return { deep: blendTo(BLACK, hue, 0.3), mid: hue, tip: blendTo(hue, WHITE, 0.7), pale: blendTo(hue, WHITE, 0.88) };
}

/* NOISE */

const smooth = (x) => x * x * (3 - 2 * x);

function grain(seed) {
  const rand = rng((seed ^ GRAIN) >>> 0);
  const acc = new Float32Array(NOISE * NOISE);
  let amp = 1;
  for (const cells of OCTAVES) {
    const grid = Float32Array.from({ length: cells * cells }, () => rand());
    for (let y = 0; y < NOISE; y++) {
      const fy = (y / NOISE) * cells;
      const iy = Math.floor(fy);
      const ty = smooth(fy - iy);
      const y0 = (iy % cells) * cells;
      const y1 = ((iy + 1) % cells) * cells;
      for (let x = 0; x < NOISE; x++) {
        const fx = (x / NOISE) * cells;
        const ix = Math.floor(fx);
        const tx = smooth(fx - ix);
        const x0 = ix % cells;
        const x1 = (ix + 1) % cells;
        const top = grid[y0 + x0] + (grid[y0 + x1] - grid[y0 + x0]) * tx;
        const low = grid[y1 + x0] + (grid[y1 + x1] - grid[y1 + x0]) * tx;
        acc[y * NOISE + x] += amp * (top + (low - top) * ty);
      }
    }
    amp *= 0.5;
  }
  let lo = Infinity;
  let hi = -Infinity;
  for (const v of acc) {
    lo = Math.min(lo, v);
    hi = Math.max(hi, v);
  }
  return Uint8Array.from(acc, (v) => Math.round(((v - lo) / (hi - lo || 1)) * 255));
}

function wobble(seed) {
  const rand = rng((seed ^ GRAIN ^ SALT) >>> 0);
  const table = Float32Array.from({ length: 256 }, () => rand() * 2 - 1);
  const at = (x) => {
    const i = Math.floor(x);
    const f = smooth(x - i);
    const a = table[i & 255];
    return a + (table[(i + 1) & 255] - a) * f;
  };
  return (t) => 0.6 * at((t / 1000) * 4.3) + 0.4 * at((t / 1000) * 6.1 + 97);
}

/* SHADERS */

const TUNNEL = `#version 300 es
precision highp float;
out vec4 o;
uniform sampler2D uNoise;
uniform vec2 uEye;
uniform float uUnit;
uniform float uRoll;
uniform float uTime;
uniform float uFlow;
uniform float uCloud;
uniform float uFan;
uniform float uReach;
uniform float uCore;
uniform float uHot;
uniform float uMilk;
uniform vec3 uDeep;
uniform vec3 uMid;
uniform vec3 uTip;
uniform vec3 uPale;
const float TAU = 6.28318531;
const float STRETCH = 8.0;
float tap(vec2 q, float k) {
  vec2 gx = dFdx(q);
  vec2 gy = dFdy(q);
  gx.x -= k * floor(gx.x / k + 0.5);
  gy.x -= k * floor(gy.x / k + 0.5);
  float a = textureGrad(uNoise, q, gx, gy).r;
  float b = textureGrad(uNoise, q * 2.0 + vec2(0.31, 0.57), gx * 2.0, gy * 2.0).r;
  return a * 0.62 + b * 0.38;
}
float layer(float a, float depth, float k, float scale, float flow, float phase) {
  float u = k * (a + 0.08 * sin(TAU * a + phase) + 0.01 * flow * uTime);
  float v = (depth + uFlow * flow) * scale / STRETCH + phase;
  return tap(vec2(u, v), k);
}
void main() {
  vec2 p = (gl_FragCoord.xy - uEye) / uUnit;
  float c = cos(uRoll);
  float s = sin(uRoll);
  p = vec2(c * p.x - s * p.y, s * p.x + c * p.y);
  float r = max(length(p), 1e-4);
  float depth = 0.35 / r;
  float a = (atan(p.y, p.x) + 0.15 * sin(0.3 * uTime + depth * 0.5)) / TAU;
  float n = 0.5 * layer(a, depth, 1.0, 1.0, 1.0, 1.3) + 0.3 * layer(a, depth, 2.0, 2.1, 1.6, 4.1) + 0.2 * layer(a, depth, 4.0, 4.3, 2.4, 2.2);
  n = smoothstep(0.2, 0.92, n);
  vec3 top = mix(uMid, uTip, 0.15 + 0.85 * uMilk);
  vec3 tone = n < 0.5 ? mix(uDeep, uMid, n * 2.0) : mix(uMid, top, n * 2.0 - 1.0);
  float hole = 1.0 - exp(-r * r / (2.25 * uCore * uCore));
  vec3 col = tone * (0.21 + 0.35 * exp(-r * 1.8)) * (1.0 + 0.5 * uMilk) * uCloud * hole;
  col += uTip * uMilk * uMilk * 0.15 * uCloud * exp(-r * 0.9);
  col += uMid * uFan * (0.25 + 1.5 * n) * smoothstep(uReach, uReach * 0.15, r);
  col += uPale * uHot * (0.4 * exp(-r * r / (uCore * uCore)) + (0.06 + 0.19 * uMilk) * exp(-r / 0.25));
  o = vec4(col, 1.0);
}
`;

const COPY = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform sampler2D uSrc;
void main() {
  o = vec4(texture(uSrc, v).rgb, 1.0);
}
`;

const FLAT = `#version 300 es
precision highp float;
out vec4 o;
uniform vec3 uColor;
void main() {
  o = vec4(uColor, 1.0);
}
`;

/* LOOK */

const lerp = (a, b, k) => a + (b - a) * k;

const quiet = () => ({ tint: 0, hue: null, glass: null, here: 1, extra: 0, there: 0, ev: 0, fov: 1, jolt: 0, aberration: 0, vignette: 0.3, bloom: 0.8, flare: 0, field: null, cloud: 0, milk: 0, fan: 0, reach: 0, core: 1, hot: 0, steer: 0, dest: 1, approach: 1 });

/* HYPER */

export function hyper(gl, view, { seed = 0, sky = {}, tint = '', look = 'cloud', speed = 1, idle = 0.015, film: times = FILM, audio = null, mode = null } = {}) {
  const T = { ...FILM, ...times };
  const top = TOP * speed;
  const full = top * SHUTTER;
  const tunnelAt = T.wind + T.stretch + T.pile + T.flash;
  const peak = Math.min(PEAK, T.flash * 0.3);
  const density = sky.density ?? 1;
  const dust = sky.dust ?? 1;
  const roam = Boolean(sky.next);
  const lit = Object.fromEntries(Object.entries(ramp(tint)).map(([key, c]) => [key, linear(c)]));
  const films = trails(tint);
  const greys = trails('', true);
  const bright = linear(grey(shown(FILMIC.pale)));
  const streak = tint ? blendTo(peakOf(lit.tip), WHITE, 0.35) : peakOf(lit.tip);
  const glow = peakOf(lit.mid);
  const core = blendTo(peakOf(lit.mid), peakOf(lit.pale), 0.35).map((v) => v * 3);
  const budget = governor(view, NOTCHES[tier(view)]);
  const flick = wobble(seed);
  const jolts = (seed ^ EXTRA) >>> 0;
  const swing = rng((seed ^ GRAIN) >>> 0);
  const turns = Array.from({ length: 5 }, () => swing() * Math.PI * 2);
  const chain = post(gl, view);
  const gaps = rng((seed ^ SALT) >>> 0);
  const saver = GAPS[mode] ?? null;
  const span = ([a, b]) => (a + gaps() * (b - a)) * 1000;
  const later = (t) => (saver && !view.still ? t + span(saver) : Infinity);
  let n = 0;
  let here = field(gl, view, { seed: seed >>> 0, density, dust });
  let there = null;
  let extra = null;
  let kit = null;
  let trip = null;
  let rest = { s: 0, t: 0 };
  let next = later(view.t);
  let shot = { s: 0, shutter: 0, axis: [0, 0, 1], k: 1, flicker: 0 };
  let gone = false;
  let classic = look === 'classic';
  let plain = classic;
  const jumps = [];
  const done = [];
  const calls = [];
  const prepare = () => {
    extra ??= field(gl, view, { seed: (seed ^ EXTRA) >>> 0, density, dust: 0 });
    if (roam && !there) there = field(gl, view, { seed: (seed + n + 1) >>> 0, density, dust });
    if (kit || classic) return;
    let tunnel = null;
    try {
      tunnel = program(gl, TUNNEL);
      const copy = program(gl, COPY);
      const noise = texture(gl, { w: NOISE, h: NOISE, format: 'r8', data: grain(seed), filter: 'mip', wrap: 'repeat' });
      tunnel.set({ uNoise: noise, uDeep: lit.deep, uMid: lit.mid, uTip: lit.tip, uPale: core });
      kit = { tunnel, copy, noise, layer: target(gl, view.w, view.h, { hdr: hdr(gl) }) };
    } catch {
      tunnel?.drop();
      classic = true;
    }
  };
  const flat = program(gl, FLAT);
  const goal = () => there ?? here;
  const speeds = {
    jump: { wind: [idle, -0.4 * idle, 1], stretch: [-0.4 * idle, top, 2.5], pile: top, flash: [top, 0.4 * top, 1], tunnel: 0.4 * top },
    exit: (from) => ({ bloom: [from, top, 2], white: top, decay: [top, 0.15 * top, 0.5], snap: [0.15 * top, idle, 1] }),
  };
  const where = (t) => {
    if (!trip) return { name: 'cruise', k: 0, since: t - rest.t };
    const leg = trip.out === null || t < trip.out ? trip.jump : trip.exit;
    const now = leg.at(t);
    return now.name ? now : { name: 'cruise', k: 0, since: now.since };
  };
  const pace = (t) => {
    if (!trip) return idle;
    const [leg, table] = trip.out === null || t < trip.out ? [trip.jump, speeds.jump] : [trip.exit, trip.after];
    const { name, k } = leg.at(t);
    const c = table[name];
    if (c === undefined) return idle;
    return typeof c === 'number' ? c : c[0] + (c[1] - c[0]) * k ** c[2];
  };
  const distance = (t) => {
    if (!trip) return rest.s + (idle * (t - rest.t)) / 1000;
    const until = trip.out === null ? t : Math.min(t, trip.out);
    let s = trip.s0 + flown(trip.jump, speeds.jump, until);
    if (trip.out !== null && t > trip.out) s += flown(trip.exit, trip.after, t);
    return s;
  };
  const state = (t) => {
    const { name, k, since } = where(t);
    const o = { stage: name, k, since, s: distance(t), shutter: pace(t) * SHUTTER, flicker: flick(t), ...quiet() };
    const b = trip?.base;
    const sm = ease.smooth(k);
    const kept = b && EARLY.includes(b.stage) ? (b.stage === 'pile' ? 1 - b.k : 1) : 0;
    const paths = plain ? greys : films;
    const pale = plain ? bright : lit.pale;
    if (name === 'wind') {
      o.ev = -0.3 * sm;
      o.fov = 1 - 0.03 * sm;
      o.tint = 0.3 * sm;
      o.glass = blendTo(BLACK, paths.wind, sm);
    } else if (name === 'stretch') {
      o.tint = 0.3 + 0.7 * ease.smooth(k * 2.5);
      o.ev = -0.3 + 0.8 * k * k;
      o.fov = 0.97 + 0.15 * sm;
      o.jolt = 2 * ease.smooth((k - 0.5) * 2);
      o.here = 0.75 + 0.25 * k;
      o.shutter = full * k * k;
      o.glass = blendTo(paths.wind, paths.lift, sm);
      o.aberration = 0.001 * k;
    } else if (name === 'pile') {
      o.tint = 1;
      o.ev = 0.5 + 1.5 * k ** 1.5;
      o.fov = 1.12;
      o.jolt = 2;
      o.shutter = full;
      o.here = 1 + 0.2 * sm;
      o.extra = sm;
      o.fan = ease.in(k, 1.5);
      o.reach = 0.2 + 2 * k;
      o.glass = blendTo(paths.lift, paths.pile, sm);
      o.aberration = 0.001 + 0.002 * k;
      o.vignette = 0.3 - 0.05 * k;
      o.dest = 1 - k;
    } else if (name === 'flash') {
      const fall = (since - peak) / Math.max(T.flash - peak, 1);
      o.tint = 1;
      o.ev = since < peak ? 2 + (3 * since) / peak : 0.5 + 4.5 * (1 - fall) ** 3;
      o.field = pale.map((c) => (c / 32) * (1 - sm));
      o.cloud = k * k;
      o.milk = k;
      o.fan = 1 - 0.7 * k;
      o.reach = 2.2;
      o.hot = HOT * k * k;
      o.here = 1.2 - 0.9 * k;
      o.extra = 1 - k;
      o.shutter = full * (1 - 0.6 * k);
      o.fov = 1.12 - 0.12 * sm;
      o.jolt = 2 * (1 - k);
      o.aberration = 0.003 + 0.0005 * k;
      o.vignette = 0.45 * sm;
      o.steer = sm;
      o.dest = 0;
    } else if (name === 'tunnel') {
      const settle = ease.smooth(since / 400);
      o.tint = 1;
      o.ev = 0.5 * (1 - settle);
      o.cloud = 1 + 0.35 * o.flicker;
      o.milk = 0.3 + 0.7 * (1 - ease.smooth(since / 1500));
      o.core = 1 + 0.3 * o.flicker;
      o.hot = HOT;
      o.fan = 0.3 * (1 - settle);
      o.reach = 2.2;
      o.here = 0.08;
      o.shutter = 0.25 * full;
      o.aberration = 0.0035;
      o.vignette = 0.45;
      o.steer = 1;
      o.dest = 0;
    } else if (name === 'bloom') {
      o.tint = lerp(b.tint, 1, sm);
      o.ev = lerp(b.ev, BLOOM, ease.in(k, 2));
      o.cloud = b.cloud * (1 - k);
      o.milk = b.milk * (1 - k);
      o.core = b.core * (1 + 2 * k ** 3);
      o.hot = lerp(b.hot, 2 * HOT, k ** 3) / (1 + 2 * k ** 3);
      o.fan = b.fan * (1 - k);
      o.reach = b.reach;
      o.here = b.here * (1 - k);
      o.extra = b.extra * (1 - k);
      o.shutter = lerp(b.shutter, full, sm);
      o.flare = FLARE * k ** 3;
      o.steer = b.steer * (1 - sm);
      o.fov = lerp(b.fov, 1, sm);
      o.jolt = b.jolt * (1 - k);
      o.aberration = lerp(b.aberration, 0.0035, sm);
      o.vignette = lerp(b.vignette, 0.45, sm);
      o.bloom = 1;
      o.dest = kept;
    } else if (name === 'white') {
      const w = Math.max(0, (k - 0.25) / 0.75);
      o.tint = 1;
      o.ev = BLOOM + (5 - BLOOM) * w ** 3;
      o.field = paths.decay[0][1].map((c) => (c / 32) * w ** 3);
      o.cloud = 0;
      o.core = b.core * (3 + 2.5 * ease.smooth(k * 2));
      o.hot = ((2 * HOT) / 3) * (1 - w);
      o.here = 0;
      o.there = ease.smooth(w) ** 2;
      o.extra = ease.smooth(w) ** 2;
      o.shutter = full;
      o.flare = lerp(FLARE, 1, w);
      o.aberration = 0.0035;
      o.vignette = 0.45 * (1 - sm);
      o.bloom = 1;
      o.dest = kept * (1 - w);
    } else if (name === 'decay') {
      o.tint = 1;
      o.ev = 0.4 + 2.2 * (1 - k) ** 2 + 2.4 * (1 - k) ** 12;
      o.glass = along(paths.decay, k);
      o.hue = along(paths.streak, k);
      o.here = 0;
      o.there = 1;
      o.extra = 1 - k;
      o.shutter = full * 0.15 ** k;
      o.flare = 1 - sm;
      o.aberration = 0.0035 - 0.0015 * k;
      o.vignette = 0.3 * ease.smooth(k * 2);
      o.bloom = 0.6 + 0.4 * k;
      o.dest = ease.smooth((k - 0.45) / 0.55);
      o.approach = 1.4 - 0.4 * ease.out((k - 0.45) / 0.55);
    } else if (name === 'snap') {
      o.tint = 1 - sm;
      o.hue = along(paths.streak, 1);
      o.ev = 0.4 * (1 - sm);
      o.glass = blendTo(along(paths.decay, 1), BLACK, sm);
      o.here = 0;
      o.there = 1;
      o.shutter = Math.max(o.shutter, 0.15 * full * (1 - k) ** 2);
      o.aberration = 0.002 * (1 - sm);
    }
    if (o.glass) o.field = o.glass.map((c) => c / 2 ** o.ev);
    if (plain) {
      o.cloud = 0;
      o.fan = 0;
      o.hot = 0;
      o.tint = 0;
      if (name === 'tunnel') {
        o.here = 0.8;
        o.shutter = 0.6 * full;
        o.ev = 0;
      }
      if (name === 'bloom') {
        o.here = b.here;
        o.extra = b.extra;
        o.field = pale.map((c) => (c / 32) * k ** 3);
      }
      if (name === 'white') {
        o.here = b.here * (1 - sm);
        o.extra = Math.max(b.extra * (1 - sm), sm);
      }
    }
    return o;
  };
  const cue = (t, name, args) => {
    if (audio) trip.cues.push([t, name, args]);
  };
  const begin = (at, stay) => {
    prepare();
    const s0 = distance(at);
    const jump = clock(JUMP.map((name) => [name, name === 'tunnel' ? Infinity : T[name]]));
    jump.start(at);
    trip = { from: at, jump, s0, out: null, exit: null, after: null, stay, base: null, cues: [] };
    cue(at, 'riser', { dur: (T.wind + T.stretch + T.pile) / 1000 });
    cue(at + T.wind + T.stretch + T.pile, 'hit', {});
    cue(at + tunnelAt, 'drone', { dur: Math.max(T.tunnel, Number.isFinite(stay) ? stay : 4000) / 1000 });
  };
  const leave = (at) => {
    if (!trip) begin(at, Infinity);
    trip.base = state(at);
    const from = pace(at);
    trip.out = at;
    trip.exit = clock(EXIT.map((name) => [name, T[name]]));
    trip.exit.start(at);
    trip.after = speeds.exit(from);
    trip.cues = trip.cues.filter(([t]) => t < at);
    cue(at, 'swell', { dur: T.bloom / 1000 });
    cue(at + T.bloom, 'hit', { white: true });
  };
  const arrive = (end) => {
    rest = { s: distance(end), t: end };
    trip = null;
    if (roam && there) {
      here.drop();
      here = there;
      there = null;
      n += 1;
    }
    next = later(end);
  };
  const settle = (t) => {
    for (let guard = 0; guard < 64; guard++) {
      if (!trip) {
        if (t < next) return;
        begin(next, span(STAY));
        continue;
      }
      if (trip.out === null && Number.isFinite(trip.stay) && t >= trip.from + tunnelAt + trip.stay) {
        leave(trip.from + tunnelAt + trip.stay);
        continue;
      }
      if (t >= trip.from + tunnelAt || (trip.out !== null && t >= trip.out)) calls.push(...jumps.splice(0));
      if (trip.out === null || t < trip.out) return;
      if (t >= trip.out + T.bloom) calls.push(...done.splice(0));
      const end = trip.out + T.bloom + T.white + T.decay + T.snap;
      if (t < end) return;
      arrive(end);
    }
  };
  const fire = () => {
    for (const call of calls.splice(0)) call();
  };
  const sound = (t) => {
    if (!audio || !trip) return;
    const due = trip.cues.filter(([at]) => at - LEAD <= t);
    trip.cues = trip.cues.filter(([at]) => at - LEAD > t);
    for (const [at, name, args] of due) audio.cue(name, at, args);
  };
  const aimed = (cam, o, t) => {
    const m = cam.rot;
    const right = [m[0], m[1], m[2]];
    const up = [m[3], m[4], m[5]];
    const fwd = [m[6], m[7], m[8]];
    const fov = cam.fov * o.fov;
    const focal = Math.min(view.w, view.h) / (2 * Math.tan(fov / 2));
    const [jx, jy] = o.jolt && !view.still ? shake(jolts, t, (o.jolt * (view.dpr || 1)) / focal) : [0, 0];
    const sec = t / 1000;
    const dx = DRIFT * o.steer * (0.6 * Math.sin(0.83 * sec + turns[0]) + 0.4 * Math.sin(1.71 * sec + turns[1]));
    const dy = DRIFT * o.steer * (0.6 * Math.sin(0.67 * sec + turns[2]) + 0.4 * Math.sin(1.37 * sec + turns[3]));
    const yaw = jx + (dx * view.w) / focal;
    const pitch = jy + (dy * view.w) / focal;
    const roll = ROLL * o.steer * Math.sin(0.41 * sec + turns[4]);
    const ahead = add(add(fwd, mul(right, yaw)), mul(up, pitch));
    const out = aim(cam.pos, add(cam.pos, norm(ahead)), up, roll);
    out.fov = fov;
    return { cam: out, axis: fwd, roll };
  };
  const tunnel = (o, lens, scale, t) => {
    const layer = kit.layer.size(view.w * scale, view.h * scale);
    const eye = project(view, lens.cam, add(lens.cam.pos, lens.axis)) ?? [view.w / 2, view.h / 2];
    gl.bindFramebuffer(gl.FRAMEBUFFER, layer.fb);
    gl.viewport(0, 0, layer.w, layer.h);
    blend(gl, null);
    kit.tunnel.set({
      uEye: [eye[0] * (layer.w / view.w), (view.h - eye[1]) * (layer.h / view.h)],
      uUnit: Math.min(layer.w, layer.h) / 2 / Math.tan(lens.cam.fov / 2),
      uRoll: lens.roll,
      uTime: t / 1000,
      uFlow: o.s * FLOW,
      uCloud: Math.max(o.cloud, 0),
      uFan: o.fan,
      uReach: Math.max(o.reach, 1e-3),
      uCore: CORE * o.core,
      uHot: o.hot,
      uMilk: o.milk,
    });
    fill(gl);
    return layer;
  };
  const stars = (one, cam, flow, k) => {
    if (k > 0) one.draw(cam, { ...flow, k });
  };
  const draw = (cam = AHEAD, beneath = null, over = null, scale = 1) => {
    if (gone) return;
    const t = view.t;
    settle(t);
    const notch = BUSY.includes(where(t).name) ? budget.tick() : budget.scale();
    plain = classic || notch === 0;
    const o = state(t);
    sound(t);
    const lens = aimed(cam, o, t);
    const hue = o.hue ? peakOf(o.hue) : streak;
    const base = { s: o.s, shutter: Math.max(o.shutter, 0), axis: lens.axis };
    const flow = plain ? { ...base, halo: WHITE, tint: WHITE } : { ...base, halo: o.tint > 0 ? blendTo(NEUTRAL, glow, o.tint) : undefined, tint: blendTo(WHITE, hue, o.tint) };
    shot = { ...flow, k: o.here, flicker: o.flicker };
    const soft = !plain && kit && (o.cloud > 0 || o.fan > 0 || o.hot > 0);
    const layer = soft ? tunnel(o, lens, notch, t) : null;
    chain.begin(scale);
    if (goal() === here) stars(here, lens.cam, flow, o.here + o.there);
    else {
      stars(here, lens.cam, flow, o.here);
      stars(there, lens.cam, flow, o.there);
    }
    if (extra) stars(extra, lens.cam, flow, o.extra);
    if (o.field) {
      blend(gl, 'add');
      flat.set({ uColor: o.field });
      fill(gl);
    }
    if (layer) {
      blend(gl, 'add');
      kit.copy.set({ uSrc: layer });
      fill(gl);
    }
    blend(gl, null);
    if (beneath && o.dest > 0) beneath(o.dest, o.approach);
    if (over) {
      over(lens.cam);
      blend(gl, null);
    }
    chain.end({ ev: o.ev, bloom: o.bloom, aberration: o.aberration, vignette: o.vignette, flare: plain || tint ? 0 : o.flare, fade: 1 });
    fire();
    if (!trip && !gone) prepare();
  };
  const freeze = (at) => {
    trip = null;
    rest = { s: distance(at), t: at };
    begin(at - tunnelAt - STILL, Infinity);
    trip.cues = [];
  };
  const land = (...late) => {
    for (const call of late) call?.();
  };
  const trigger = (onDone, onJump) => {
    if (gone) return land(onJump, onDone);
    if (onJump) jumps.push(onJump);
    if (onDone) done.push(onDone);
    if (view.still) {
      freeze(view.t);
      draw();
      calls.push(...jumps.splice(0), ...done.splice(0));
      fire();
      return;
    }
    if (!trip) begin(view.t, Infinity);
  };
  const calm = () => {
    if (trip) arrive(view.t);
    draw();
    calls.push(...jumps.splice(0), ...done.splice(0));
    fire();
  };
  const exit = (onDone) => {
    if (gone) return land(onDone);
    if (onDone) done.push(onDone);
    if (view.still) return calm();
    const t = view.t;
    if (!trip) return leave(t);
    if (trip.out !== null) return;
    const opened = trip.from + tunnelAt;
    leave(t >= opened ? Math.max(t, opened + T.tunnel) : t);
  };
  const windDown = (onDone) => {
    if (gone) return land(onDone);
    if (onDone) done.push(onDone);
    if (view.still) return calm();
    if (!trip) return leave(view.t);
    if (trip.out === null || trip.out > view.t) {
      trip.out = null;
      leave(view.t);
    }
  };
  const hold = () => {
    if (gone || view.still) return;
    if (!trip) begin(view.t, Infinity);
    else if (trip.out === null) trip.stay = Infinity;
  };
  const stage = () => where(view.t).name;
  const phase = () => HOST[stage()] ?? 'cruise';
  const stop = () => {
    calls.push(...jumps.splice(0), ...done.splice(0));
    fire();
    if (gone) return;
    gone = true;
    for (const one of new Set([here, there, extra])) one?.drop();
    if (kit) {
      kit.tunnel.drop();
      kit.copy.drop();
      kit.layer.drop();
      gl.deleteTexture(kit.noise);
    }
    flat.drop();
    chain.drop();
  };
  return { draw, trigger, exit, hold, windDown, phase, stage, at: () => where(view.t), flow: () => shot, stop };
}
