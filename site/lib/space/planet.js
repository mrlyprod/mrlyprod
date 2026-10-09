import { rng } from '../scene.js';
import { CUBE, bake, blend, fill, link, mipmap, texture, tier } from './gl2.js';

const TAU = Math.PI * 2;
const FACES = { phone: 512, desk: 1024, big: 1024 };
const STEPS = { phone: 6, desk: 10, big: 10 };
const RING = 1024;
const LIGHT = 5;
const GLARE = 400;
const DISC = 0.012;
const BANDS = 14;
const SALT = 0x7f4a7c15;
const EXTRA = 0x3c6ef372;
const KINDS = { terran: 0, desert: 1, ice: 2, lava: 3, gas: 4, moon: 5, deck: 6 };
const TYPES = ['terran', 'desert', 'ice', 'lava', 'gas'];
const WEIGHTS = [0.42, 0.16, 0.14, 0.1, 0.18];
const ALBEDO = 0.8;
const PALETTES = [
  { dark: '#8c6a3e', mid: '#b8935c', light: '#e0d2b0', accent: '#96634a', storm: '#b2603c' },
  { dark: '#4d6270', mid: '#6a9296', light: '#c4d2cf', accent: '#5b7088', storm: '#8a6a6a' },
  { dark: '#8c6a72', mid: '#c19b9c', light: '#e2d8c8', accent: '#6c5c6c', storm: '#c0707a' },
];

/* COLOUR */

const lin = (hex, k = ALBEDO) => [0, 2, 4].map((i) => (Number.parseInt(hex.slice(1 + i, 3 + i), 16) / 255) ** 2.2 * k);

function hsl(h, s, l, k = ALBEDO) {
  const f = (n) => {
    const a = (n + h * 12) % 12;
    return l - s * Math.min(l, 1 - l) * Math.max(-1, Math.min(a - 3, 9 - a, 1));
  };
  return [f(0), f(8), f(4)].map((c) => Math.max(0, c) ** 2.2 * k);
}

const air = ({ h, hm, g, ray, mie, dust, top }) => ({ h, hm, g, top, ray: ray.map((v) => v / h), mie: mie.map((v) => v / hm), dust: dust.map((v) => v / hm) });

const EARTH_AIR = air({ h: 0.004, hm: 0.0012, g: 0.76, ray: [0.046, 0.108, 0.265], mie: [0.015, 0.015, 0.015], dust: [0.017, 0.017, 0.017], top: 1.03 });

/* WORLDS */

const SATURN_RINGS = [
  [1.11, 1.236, 0.004, 0.9, 0.3],
  [1.239, 1.45, 0.07, 0.12, 0.2],
  [1.45, 1.527, 0.12, 0.1, 0.3],
  [1.527, 1.64, 0.9, 0.02, 0.62],
  [1.64, 1.951, 2.6, 0, 0.9],
  [1.951, 2.025, 0.09, 0.2, 0.35],
  [2.025, 2.214, 0.65, 0.04, 0.72],
  [2.214, 2.219, 0.01, 0.6, 0.5],
  [2.219, 2.262, 0.45, 0.06, 0.66],
  [2.318, 2.334, 0.25, 1, 0.75],
];

const WORLDS = {
  earth: {
    kind: 'terran',
    noise: 11,
    tilt: 0.41,
    spin: 0.045,
    phase: 0.6,
    freq: 0.95,
    sea: 0.165,
    bump: 0.035,
    cover: 0.5,
    lights: 1,
    colors: ['#16488a', '#134656', '#2a3a1a', '#c4a283', '#6f604f'].map((c) => lin(c)),
    cloud: lin('#f6f8fb', 0.88),
    drift: 0.004,
    city: [1, 0.62, 0.3].map((c) => c * 1.4),
    glow: [0, 0, 0],
    air: EARTH_AIR,
    spec: 1,
    rough: 0.42,
    limb: 0,
    flat: 0,
    ring: null,
    moons: [{ r: 0.273, a: 7.5, incl: 0.09, rate: 0.03, tone: lin('#a6a29c', 0.25), rough: 0.55 }],
  },
  mars: {
    kind: 'desert',
    noise: 23,
    tilt: 0.44,
    spin: 0.044,
    phase: 2.3,
    freq: 1.1,
    sea: 0,
    bump: 0.05,
    cover: 0,
    lights: 0,
    colors: ['#b4703e', '#584232', '#f0ebe6', '#c89064', '#6e4a36'].map((c) => lin(c)),
    feature: [-0.14, 0.3, 1.25, 0.3],
    cloud: lin('#e4e2e6', 0.55),
    drift: 0.003,
    city: [0, 0, 0],
    glow: [0, 0, 0],
    air: air({ h: 0.0018, hm: 0.0038, g: 0.62, ray: [0.002, 0.004, 0.01], mie: [0.12, 0.08, 0.055], dust: [0.13, 0.12, 0.11], top: 1.015 }),
    spec: 0,
    rough: 0,
    limb: 0,
    flat: 0.45,
    ring: null,
    moons: [
      { r: 0.032, a: 2.76, incl: 0.02, rate: 0.3, tone: lin('#55493f', 0.5), rough: 0.9 },
      { r: 0.022, a: 6.9, incl: 0.03, rate: 0.08, tone: lin('#6a5d52', 0.5), rough: 0.8 },
    ],
  },
  jupiter: {
    kind: 'gas',
    noise: 31,
    tilt: 0.055,
    spin: 0.12,
    phase: 1.1,
    freq: 1,
    jets: 0.0025,
    bump: 1,
    bands: [
      [-1, '#4a443e'],
      [-0.85, '#6a6052'],
      [-0.66, '#a89474'],
      [-0.54, '#9a8866'],
      [-0.42, '#dccdb0'],
      [-0.3, '#a08a64'],
      [-0.14, '#b49c74'],
      [0.02, '#ebe2d0'],
      [0.12, '#d8c49a'],
      [0.22, '#a08664'],
      [0.36, '#ddd0b4'],
      [0.46, '#a08c6a'],
      [0.6, '#c2ae8a'],
      [1, '#4e473f'],
    ].map(([y, c]) => [y, ...lin(c)]),
    feature: [-0.39, 1.4, 0.11, 0.85],
    tone: lin('#c06a44'),
    cover: 0.12,
    limb: 0.45,
    flat: 0,
    air: null,
    ring: null,
    moons: [
      { r: 0.026, a: 4.2, incl: 0.01, rate: 0.12, tone: lin('#d8c070', 0.7), rough: 0.6 },
      { r: 0.022, a: 5.6, incl: 0.01, rate: 0.08, tone: lin('#d8d0c0', 0.75), rough: 0.35 },
      { r: 0.037, a: 7.5, incl: 0.01, rate: 0.05, tone: lin('#9a8e80', 0.5), rough: 0.6 },
    ],
  },
  saturn: {
    kind: 'gas',
    noise: 37,
    tilt: 0.466,
    spin: 0.11,
    phase: 0.4,
    freq: 0.8,
    jets: 0.0018,
    bump: 0.35,
    bands: [
      [-1, '#7c6c54'],
      [-0.75, '#a89068'],
      [-0.5, '#c8ad7c'],
      [-0.3, '#d9c290'],
      [-0.12, '#c9aa76'],
      [0, '#e2cc98'],
      [0.12, '#d6bc88'],
      [0.28, '#bfa070'],
      [0.45, '#d4bc8a'],
      [0.62, '#b89c70'],
      [0.8, '#a09478'],
      [1, '#8a8676'],
    ].map(([y, c]) => [y, ...lin(c)]),
    feature: [0, 0, 0.01, 0],
    tone: lin('#e8dcb8'),
    cover: 0.05,
    limb: 0.35,
    flat: 0,
    air: null,
    ring: { inner: 1.11, outer: 2.34, zones: SATURN_RINGS, seed: 61, dark: lin('#6e6458'), bright: lin('#dccbab'), halo: 0.004 },
    moons: [
      { r: 0.012, a: 3.95, incl: 0.0, rate: 0.14, tone: lin('#f4f4f4', 0.95), rough: 0.15 },
      { r: 0.018, a: 6.1, incl: 0.01, rate: 0.07, tone: lin('#d0ccc6', 0.75), rough: 0.4 },
      { r: 0.043, a: 9, incl: 0.006, rate: 0.03, tone: lin('#c4904a', 0.6), rough: 0.12 },
    ],
  },
  moon: {
    kind: 'moon',
    noise: 41,
    tilt: 0.03,
    spin: 0.012,
    phase: 0.2,
    freq: 1.3,
    sea: 0,
    bump: 0.11,
    cover: 0,
    lights: 0,
    colors: ['#b0a99f', '#60646a', '#ebe6dc', '#8e8880', '#706e6a'].map((c) => lin(c, 0.4)),
    city: [0, 0, 0],
    glow: [0, 0, 0],
    air: null,
    spec: 0,
    rough: 0,
    limb: 0,
    flat: 1,
    ring: null,
    moons: [],
  },
  venus: {
    kind: 'deck',
    noise: 43,
    tilt: 0.05,
    spin: 0.018,
    phase: 0.9,
    freq: 1,
    sea: 0,
    bump: 0,
    cover: 0,
    lights: 0,
    colors: ['#f0e8da', '#d8c6a4', '#e8dcc4', '#c8b48c', '#f4eee4'].map((c) => lin(c, 0.9)),
    city: [0, 0, 0],
    glow: [0, 0, 0],
    air: air({ h: 0.012, hm: 0.008, g: 0.7, ray: [0.004, 0.008, 0.016], mie: [0.06, 0.055, 0.045], dust: [0.065, 0.065, 0.065], top: 1.07 }),
    spec: 0,
    rough: 0,
    limb: 0.15,
    flat: 0,
    ring: null,
    moons: [],
  },
  neptune: {
    kind: 'gas',
    noise: 47,
    tilt: 0.49,
    spin: 0.09,
    phase: 2.6,
    freq: 0.9,
    jets: 0.003,
    bump: 0.5,
    bands: [
      [-1, '#2a44b0'],
      [-0.7, '#3456c8'],
      [-0.4, '#3e64dc'],
      [-0.25, '#2e4cbc'],
      [0, '#4068e0'],
      [0.3, '#3a5fd6'],
      [0.6, '#4a70e2'],
      [1, '#3050c0'],
    ].map(([y, c]) => [y, ...lin(c)]),
    feature: [-0.36, 2.2, 0.08, 0.8],
    tone: lin('#1c2c86'),
    cover: 0.22,
    limb: 0.3,
    flat: 0,
    air: null,
    ring: null,
    moons: [{ r: 0.055, a: 6, incl: 2.73, rate: 0.04, tone: lin('#d8c8c0', 0.7), rough: 0.35 }],
  },
};

export const NAMES = [...Object.keys(WORLDS), 'exo'];

/* EXO */

function roll(rand) {
  let r = rand();
  let i = 0;
  while (i < TYPES.length - 1 && r > WEIGHTS[i]) {
    r -= WEIGHTS[i];
    i++;
  }
  return TYPES[i];
}

function moonOf(rand) {
  const R = (a, b) => a + rand() * (b - a);
  const a = R(3.6, 9);
  const icy = rand() < 0.4;
  return { r: R(0.025, 0.11), a, incl: R(-0.3, 0.3), rate: (rand() < 0.15 ? -1 : 1) * 0.5 * a ** -1.5, tone: icy ? hsl(R(0.5, 0.62), R(0, 0.2), R(0.65, 0.85), 0.7) : hsl(R(0.03, 0.12), R(0.05, 0.35), R(0.3, 0.55), 0.5), rough: R(0.2, 0.9) };
}

function ringOf(rand, seed, body) {
  const inner = 1.3 + rand() * 0.3;
  const outer = inner + 0.45 + rand() * 0.75;
  const zones = [];
  let r = inner;
  while (r < outer) {
    const w = 0.03 + rand() * 0.22;
    const gap = rand() < 0.22;
    zones.push([r, Math.min(r + w, outer), gap ? rand() * 0.04 : 0.08 + rand() ** 2 * 2.2, rand() * 0.4, rand()]);
    r += w;
  }
  const hue = rand();
  return { inner, outer, zones, seed, dark: body ? body.map((c) => c * 0.45) : hsl(hue, 0.15, 0.35), bright: body ? body.map((c) => Math.min(c * 1.15, 0.8)) : hsl(hue, 0.2, 0.7) };
}

function exo(seed) {
  const rand = rng((seed ^ SALT) >>> 0);
  const R = (a, b) => a + rand() * (b - a);
  const kind = roll(rand);
  const p = { kind, noise: Math.floor(R(0, 997)), tilt: R(-0.5, 0.5), spin: R(0.03, 0.07), phase: R(0, TAU), freq: R(0.95, 1.6), sea: 0, bump: 0.04, cover: 0, lights: 0, drift: R(0.002, 0.006), city: [0, 0, 0], glow: [0, 0, 0], spec: 0, rough: 0.42, limb: 0, flat: 0, air: null, ring: null };
  if (kind === 'terran') {
    const sea = R(0.5, 0.66);
    Object.assign(p, {
      sea: R(-0.04, 0.12),
      cover: R(0.35, 0.75),
      lights: rand() < 0.5 ? 1 : 0,
      colors: [hsl(sea, R(0.55, 0.85), R(0.08, 0.13)), hsl(sea + R(-0.04, 0.02), R(0.4, 0.7), R(0.17, 0.27)), hsl(R(0.17, 0.38), R(0.25, 0.55), R(0.12, 0.22)), hsl(R(0.04, 0.11), R(0.3, 0.6), R(0.45, 0.65)), hsl(R(0.04, 0.1), R(0.08, 0.22), R(0.3, 0.45))],
      cloud: hsl(0.6, 0.05, 0.95, 0.85),
      city: [1, R(0.5, 0.75), R(0.2, 0.4)].map((c) => c * 0.7),
      air: air({ h: 0.004, hm: 0.0012, g: 0.76, ray: [0.046, 0.108, 0.265].map((v) => v * R(0.6, 1.6)), mie: [0.015, 0.015, 0.015], dust: [0.017, 0.017, 0.017], top: 1.03 }),
      spec: 1,
    });
  } else if (kind === 'desert') {
    const hue = R(0.02, 0.12);
    const bright = hsl(hue, R(0.4, 0.7), R(0.45, 0.6));
    const tau = R(0.03, 0.3);
    Object.assign(p, {
      cover: R(0, 0.15),
      flat: R(0.2, 0.6),
      bump: 0.05,
      colors: [bright, hsl(hue + R(-0.02, 0.02), R(0.15, 0.35), R(0.17, 0.28)), hsl(0.1, 0.1, 0.9), hsl(hue + 0.02, R(0.4, 0.6), R(0.55, 0.7)), hsl(hue, R(0.2, 0.4), R(0.3, 0.4))],
      feature: [0, 0, 0, 0],
      cloud: hsl(hue, 0.15, 0.88, 0.8),
      air: air({ h: 0.004, hm: 0.004, g: 0.65, ray: [0.004, 0.008, 0.02], mie: bright.map((c) => tau * (0.4 + c)), dust: [tau * 1.3, tau * 1.3, tau * 1.3], top: 1.035 }),
    });
  } else if (kind === 'ice') {
    const thin = rand() < 0.5;
    Object.assign(p, {
      cover: thin ? R(0.1, 0.3) : 0,
      flat: 0.3,
      bump: 0.03,
      colors: [hsl(R(0.5, 0.62), R(0.05, 0.25), R(0.75, 0.88)), hsl(R(0.03, 0.1), R(0.3, 0.6), R(0.35, 0.5)), hsl(R(0.55, 0.62), R(0.4, 0.7), R(0.15, 0.3)), hsl(R(0.5, 0.6), R(0.1, 0.3), R(0.6, 0.75)), hsl(0.58, 0.1, 0.5)],
      cloud: hsl(0.6, 0.05, 0.95, 0.85),
      air: thin ? air({ h: 0.005, hm: 0.002, g: 0.7, ray: [0.02, 0.05, 0.12], mie: [0.01, 0.01, 0.01], dust: [0.012, 0.012, 0.012], top: 1.04 }) : null,
    });
  } else if (kind === 'lava') {
    Object.assign(p, {
      cover: R(0.1, 0.35),
      bump: 0.05,
      colors: [hsl(R(0, 0.08), R(0.05, 0.2), R(0.06, 0.1)), hsl(R(0, 0.08), R(0.1, 0.25), R(0.14, 0.22)), hsl(0.05, 0.5, 0.3), hsl(0.05, 0.3, 0.25), hsl(0.05, 0.2, 0.2)],
      cloud: hsl(0.05, 0.1, 0.3, 0.6),
      glow: ((k) => [1, R(0.9, 1), R(0.7, 1)].map((c) => c * k))(R(3, 4.5)),
      air: air({ h: 0.005, hm: 0.003, g: 0.7, ray: [0.01, 0.015, 0.025], mie: [0.08, 0.05, 0.03], dust: [0.1, 0.1, 0.1], top: 1.035 }),
    });
  } else {
    const palette = PALETTES[Math.floor(rand() * PALETTES.length)];
    const n = 9 + Math.floor(rand() * 5);
    const bands = Array.from({ length: n }, (_, i) => {
      const y = i === 0 ? -1 : i === n - 1 ? 1 : -1 + (2 * (i + R(-0.3, 0.3))) / (n - 1);
      const k = ALBEDO * R(0.9, 1.1);
      if (i > 0 && i < n - 1 && rand() < 0.2) return [y, ...lin(palette.accent, k)];
      const t = rand() < 0.3 ? R(0.25, 0.75) : i % 2 === 0 && i > 0 && i < n - 1 ? R(0.55, 1) : R(0.05, 0.5);
      const [lo, hi] = (t < 0.5 ? [palette.dark, palette.mid] : [palette.mid, palette.light]).map((hex) => lin(hex, k));
      const at = (t * 2) % 1;
      return [y, ...lo.map((c, j) => c + (hi[j] - c) * at)];
    });
    const storm = rand() < 0.6;
    Object.assign(p, {
      spin: R(0.08, 0.13),
      freq: R(0.8, 1.2),
      jets: R(0.0015, 0.003),
      bump: R(0.7, 1),
      bands,
      feature: storm ? [R(-0.5, 0.5), R(0, TAU), R(0.05, 0.12), R(0.5, 0.9)] : [0, 0, 0.01, 0],
      tone: lin(palette.storm),
      cover: R(0.12, 0.3),
      limb: R(0.35, 0.5),
    });
  }
  const count = rand() < 0.55 ? 1 : rand() < 0.5 ? 2 : 0;
  p.moons = Array.from({ length: count }, () => moonOf(rand));
  if (rand() < (kind === 'gas' ? 0.7 : 0.2)) p.ring = ringOf(rand, Math.floor(rand() * 1e6), kind === 'gas' ? p.bands[Math.floor(p.bands.length / 2)].slice(1) : null);
  p.light = hsl(R(0.05, 0.6), R(0, 0.25), 0.9, 1).map((c, i, all) => c / Math.max(...all));
  return p;
}

/* WORLD */

export function world(name, seed = 0, { moons: count = -1, rings = '' } = {}) {
  const own = name === 'exo' ? exo(seed >>> 0) : { light: [1, 1, 1], ...structuredClone(WORLDS[name] ?? WORLDS.earth) };
  const rand = rng(((seed >>> 0) ^ EXTRA) >>> 0);
  own.name = WORLDS[name] ? name : 'exo';
  own.moons = own.moons.map((m) => ({ ...m, phase: rand() * TAU }));
  if (count >= 0) {
    while (own.moons.length < count) own.moons.push({ ...moonOf(rand), phase: rand() * TAU });
    own.moons = own.moons.slice(0, Math.min(count, 3));
  }
  own.moons = own.moons.slice(0, 3);
  if (rings === 'off') own.ring = null;
  if (rings === 'on' && !own.ring) own.ring = ringOf(rand, (seed ^ EXTRA) >>> 0, own.bands ? own.bands[Math.floor(own.bands.length / 2)].slice(1) : null);
  return own;
}

/* MOTION */

const turn = (tilt, [x, y, z]) => {
  const c = Math.cos(tilt);
  const s = Math.sin(tilt);
  return [c * x - s * y, s * x + c * y, z];
};

export const pole = (params) => turn(params.tilt, [0, 1, 0]);

function place(tilt, m, t, out, at) {
  const a = (m.phase ?? 0) + (m.rate * t) / 1000;
  const x = m.a * Math.cos(a);
  const y = m.a * Math.sin(a) * Math.sin(m.incl);
  const c = Math.cos(tilt);
  const s = Math.sin(tilt);
  out[at] = c * x - s * y;
  out[at + 1] = s * x + c * y;
  out[at + 2] = m.a * Math.sin(a) * Math.cos(m.incl);
  out[at + 3] = m.r;
  return out;
}

export function moons(params, t) {
  return params.moons.slice(0, 3).map((m) => {
    const [x, y, z, r] = place(params.tilt, m, t, [0, 0, 0, 0], 0);
    return { pos: [x, y, z], r };
  });
}

function spin(tilt, angle, out) {
  const ct = Math.cos(tilt);
  const st = Math.sin(tilt);
  const ca = Math.cos(angle);
  const sa = Math.sin(angle);
  out[0] = ct * ca;
  out[1] = -st;
  out[2] = ct * sa;
  out[3] = st * ca;
  out[4] = ct;
  out[5] = st * sa;
  out[6] = -sa;
  out[7] = 0;
  out[8] = ca;
  return out;
}

function profile(ring) {
  const out = new Uint8Array(RING * 4);
  const rand = rng(ring.seed >>> 0);
  const waves = Array.from({ length: 12 }, () => [40 + rand() * 900, rand() * TAU, 0.04 + rand() * 0.12]);
  const byte = (v) => Math.round(Math.min(Math.max(v, 0), 1) * 255);
  for (let i = 0; i < RING; i++) {
    const r = ring.inner + ((ring.outer - ring.inner) * (i + 0.5)) / RING;
    const zone = ring.zones.find(([a, b]) => r >= a && r < b);
    const [tau, dust, tone] = zone ? zone.slice(2) : [0, 0, 0];
    const ripple = waves.reduce((sum, [f, ph, a]) => sum + a * Math.sin(f * r + ph), 0);
    out.set([byte(1 - Math.exp(-tau * Math.max(0, 1 + ripple))), byte(dust), byte(tone + ripple * 0.5), 255], i * 4);
  }
  return out;
}

/* SHADERS */

const NOISE = `
uvec3 pcg(uvec3 v) {
  v = v * 1664525u + 1013904223u;
  v.x += v.y * v.z;
  v.y += v.z * v.x;
  v.z += v.x * v.y;
  v ^= v >> 16u;
  v.x += v.y * v.z;
  v.y += v.z * v.x;
  v.z += v.x * v.y;
  return v;
}
vec3 hash(vec3 i) {
  return vec3(pcg(uvec3(ivec3(i) + 65536))) * (1.0 / 4294967295.0);
}
vec3 grad(vec3 i) {
  return normalize(hash(i) * 2.0 - 1.0 + 1e-4);
}
float gnoise(vec3 p) {
  vec3 i = floor(p);
  vec3 f = p - i;
  vec3 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  float a = dot(grad(i), f);
  float b = dot(grad(i + vec3(1.0, 0.0, 0.0)), f - vec3(1.0, 0.0, 0.0));
  float c = dot(grad(i + vec3(0.0, 1.0, 0.0)), f - vec3(0.0, 1.0, 0.0));
  float d = dot(grad(i + vec3(1.0, 1.0, 0.0)), f - vec3(1.0, 1.0, 0.0));
  float e = dot(grad(i + vec3(0.0, 0.0, 1.0)), f - vec3(0.0, 0.0, 1.0));
  float g = dot(grad(i + vec3(1.0, 0.0, 1.0)), f - vec3(1.0, 0.0, 1.0));
  float h = dot(grad(i + vec3(0.0, 1.0, 1.0)), f - vec3(0.0, 1.0, 1.0));
  float k = dot(grad(i + vec3(1.0)), f - vec3(1.0));
  return mix(mix(mix(a, b, u.x), mix(c, d, u.x), u.y), mix(mix(e, g, u.x), mix(h, k, u.x), u.y), u.z) * 1.6;
}
const mat3 TURN = mat3(0.0, 0.8, 0.6, -0.8, 0.36, -0.48, -0.6, -0.48, 0.64);
float fbm(vec3 p, int n) {
  float s = 0.0;
  float a = 0.5;
  for (int i = 0; i < 8; i++) {
    if (i >= n) break;
    s += a * gnoise(p);
    p = TURN * p * 2.02;
    a *= 0.5;
  }
  return s;
}
`;

const BAKE = `
precision highp float;
precision highp int;
in vec3 d;
out vec4 o;
uniform vec3 uOff;
uniform float uSea;
uniform float uFreq;
uniform float uCover;
uniform float uLights;
uniform float uBump;
uniform vec3 uA;
uniform vec3 uB;
uniform vec3 uC;
uniform vec3 uD;
uniform vec3 uE;
uniform vec4 uBands[${BANDS}];
uniform int uCount;
uniform vec4 uFeature;
uniform vec3 uTone;
${NOISE}
float ridged(vec3 p, int n) {
  float s = 0.0;
  float a = 0.5;
  float w = 1.0;
  for (int i = 0; i < 8; i++) {
    if (i >= n) break;
    float r = 1.0 - abs(gnoise(p));
    r *= r;
    s += a * r * w;
    w = clamp(r * 1.5, 0.0, 1.0);
    p = TURN * p * 2.02;
    a *= 0.5;
  }
  return s;
}
vec3 warp(vec3 p, float k) {
  return p + k * vec3(fbm(p + vec3(1.7, 9.2, 3.1), 4), fbm(p + vec3(8.3, 2.8, 5.5), 4), fbm(p + vec3(4.1, 6.6, 7.9), 4));
}
vec3 spin(vec3 v, vec3 k, float a) {
  float c = cos(a);
  float s = sin(a);
  return v * c + cross(k, v) * s + k * dot(k, v) * (1.0 - c);
}
float wrap(float x) {
  return x - 6.2831853 * floor((x + 3.1415927) / 6.2831853);
}
float crater(vec3 p) {
  vec3 i = floor(p);
  vec3 f = p - i;
  float h = 0.0;
  for (int z = -1; z <= 1; z++) {
    for (int y = -1; y <= 1; y++) {
      for (int x = -1; x <= 1; x++) {
        vec3 at = vec3(float(x), float(y), float(z));
        vec3 r = hash(i + at + 101.0);
        if (r.x > 0.6) continue;
        float s = 0.12 + 0.3 * r.y * r.y;
        vec3 c = at + 0.2 + 0.6 * hash(i + at);
        float e = length(f - c) / s;
        if (e > 2.5) continue;
        float k = e < 1.0 ? (e * e - 1.0) * 0.8 + 0.25 * smoothstep(0.6, 1.0, e) : 0.25 * exp(-(e - 1.0) * 4.0);
        h += k * s;
      }
    }
  }
  return h;
}
float craters(vec3 q) {
  return crater(q * 4.0 + uOff) / 4.0 + crater(q * 9.0 + uOff * 1.3) / 9.0 + crater(q * 20.0 + uOff * 1.7) / 20.0 + crater(q * 45.0 + uOff * 2.1) / 45.0;
}
float terran(vec3 q, out float m) {
  vec3 p = warp(q * uFreq + uOff, 0.55);
  float c = fbm(p, 7) + 0.2 * gnoise(q * 0.8 + uOff.yzx);
  m = ridged(q * uFreq * 2.6 + uOff.zxy, 6);
  return c + smoothstep(uSea, uSea + 0.12, c) * m * 0.22;
}
float wetness(vec3 q) {
  float lat = abs(q.y);
  float dry = smoothstep(0.18, 0.3, lat) * smoothstep(0.52, 0.38, lat);
  return clamp(0.6 + fbm(q * 2.2 + uOff.yxz + 13.0, 5) * 2.2 - dry * 0.5 + 0.25 * smoothstep(0.14, 0.0, lat), 0.0, 1.0);
}
float cities(vec3 q, float h, float wet) {
  float alt = h - uSea;
  float land = smoothstep(0.0, 0.008, alt);
  float lat = abs(q.y);
  float people = smoothstep(-0.05, 0.25, fbm(q * 2.6 + uOff.zyx + 21.0, 4));
  float coast = exp(-max(alt, 0.0) * 16.0);
  float home = land * (1.0 - smoothstep(0.6, 0.72, lat)) * smoothstep(0.2, 0.4, wet) * (1.0 - smoothstep(0.12, 0.25, alt));
  float town = smoothstep(0.05, 0.45, fbm(q * 34.0 + uOff + 5.0, 4) + 0.3 * people + 0.25 * coast - 0.2);
  float spark = smoothstep(0.1, 0.5, gnoise(q * 140.0 + uOff.yzx));
  return clamp(home * people * town * (0.45 + 0.8 * spark), 0.0, 1.0) * uLights;
}
vec4 terranSurface(vec3 q) {
  float m;
  float h = terran(q, m);
  float lat = abs(q.y);
  float wet = wetness(q);
  float alt = h - uSea;
  vec3 ice = vec3(0.78, 0.82, 0.88);
  vec3 grass = mix(uC, uD, 0.5);
  vec3 ground = mix(uD, grass, smoothstep(0.25, 0.5, wet));
  ground = mix(ground, uC, smoothstep(0.55, 0.85, wet));
  ground = mix(ground, uE, smoothstep(0.12, 0.3, alt) * 0.8);
  ground = mix(ground, mix(uE, uC, 0.4), smoothstep(0.6, 0.78, lat + 0.06 * gnoise(q * 9.0)) * 0.7);
  float snow = max(smoothstep(0.82, 0.9, lat + 0.05 * gnoise(q * 7.0 + 3.0)), smoothstep(0.42, 0.55, alt + lat * 0.22));
  ground = mix(ground, ice, snow);
  ground *= 0.82 + 0.36 * fbm(q * 40.0 + uOff, 3);
  float shore = smoothstep(uSea - 0.09, uSea, h);
  vec3 sea = mix(uA, uB, shore * shore);
  float floe = smoothstep(0.86, 0.93, lat + 0.04 * gnoise(q * 6.0 + 9.0));
  sea = mix(sea, ice, floe);
  float isle = smoothstep(uSea - 0.003, uSea + 0.003, h);
  return vec4(sqrt(mix(sea, ground, isle)), (1.0 - isle) * (1.0 - floe));
}
float canyon(vec3 q) {
  if (uFeature.w <= 0.0) return 0.0;
  float la = asin(clamp(q.y, -1.0, 1.0));
  float along = wrap(atan(q.z, q.x) - uFeature.y) / uFeature.z;
  if (along < -0.1 || along > 1.1) return 0.0;
  float bend = la - uFeature.x - 0.03 * sin(along * 9.0) - 0.02 * gnoise(q * 12.0);
  float ends = smoothstep(-0.05, 0.1, along) * smoothstep(1.05, 0.8, along);
  return uFeature.w * exp(-pow(bend / 0.022, 2.0)) * ends * (0.7 + 0.5 * gnoise(q * 25.0));
}
float volcanoes(vec3 q) {
  if (uFeature.w <= 0.0) return 0.0;
  float h = 0.0;
  for (int i = 0; i < 4; i++) {
    float la = uFeature.x + 0.32 + 0.12 * float(i) - (i == 3 ? 0.55 : 0.0);
    float lo = uFeature.y - 0.55 - 0.1 * float(i) - (i == 3 ? 0.45 : 0.0);
    vec3 c = vec3(cos(la) * cos(lo), sin(la), cos(la) * sin(lo));
    float r = i == 3 ? 0.1 : 0.055;
    float e = distance(q, c) / r;
    h += (i == 3 ? 0.35 : 0.2) * exp(-e * e * 1.6) - 0.08 * exp(-e * e * 30.0);
  }
  return h;
}
float desert(vec3 q, out float cr) {
  vec3 p = warp(q * uFreq + uOff, 0.35);
  float hemi = smoothstep(-0.25, 0.3, -q.y + 0.3 * gnoise(q * 1.3 + uOff.zxy));
  cr = craters(q) * (0.35 + 0.9 * hemi);
  return fbm(p, 7) * 0.45 + hemi * 0.12 + cr - canyon(q) + volcanoes(q);
}
vec4 desertSurface(vec3 q) {
  float cr;
  float h = desert(q, cr);
  float dark = smoothstep(0.0, 0.07, fbm(warp(q * 1.4 + uOff.yzx + 4.0, 0.8), 6) + 0.12 * smoothstep(0.3, -0.5, q.y));
  vec3 alb = mix(uA, uB, dark * (0.72 + 0.36 * fbm(q * 9.0 + uOff.zxy, 4)));
  alb = mix(alb, uD, smoothstep(0.05, 0.35, fbm(q * 6.0 + uOff, 4)) * 0.35 * (1.0 - dark));
  alb *= 0.92 + 0.16 * fbm(q * 40.0 + uOff, 4);
  alb = mix(alb, uE, smoothstep(0.02, 0.12, canyon(q)) * 0.6);
  alb *= 0.9 + 0.6 * clamp(cr * 3.0, -0.15, 0.25);
  float cap = smoothstep(0.9, 0.95, q.y + 0.03 * gnoise(q * 8.0)) + 0.8 * smoothstep(0.95, 0.975, -q.y + 0.02 * gnoise(q * 8.0 + 2.0));
  return vec4(sqrt(mix(alb, uC, clamp(cap, 0.0, 1.0))), 0.0);
}
float iceHeight(vec3 q) {
  return fbm(q * uFreq + uOff, 6) * 0.25 + ridged(q * uFreq * 3.0 + uOff.yzx, 5) * 0.12;
}
vec4 iceSurface(vec3 q) {
  vec3 alb = mix(uA, uD, smoothstep(-0.2, 0.3, fbm(warp(q * 2.0 + uOff, 0.5), 6)));
  float lines = smoothstep(0.8, 0.95, ridged(q * uFreq * 4.0 + uOff.zxy, 4));
  alb = mix(alb, uB, lines * 0.7);
  alb = mix(alb, uC, smoothstep(0.35, 0.6, fbm(q * 1.2 + uOff.yzx + 6.0, 5)) * 0.4);
  return vec4(sqrt(alb), 0.0);
}
float lavaHeight(vec3 q) {
  return fbm(q * uFreq + uOff, 7) * 0.4 + ridged(q * uFreq * 2.5 + uOff.yzx, 5) * 0.15;
}
vec2 fissures(vec3 q) {
  vec3 p = warp(q * uFreq * 2.0 + uOff.yzx, 0.4);
  float a = abs(gnoise(p * 1.5));
  float b = abs(gnoise(p * 3.7 + 5.1)) * 1.6;
  float c = abs(gnoise(p * 9.0 + 9.3)) * 2.6;
  float core = max(exp(-a * a / 0.0006), max(0.8 * exp(-b * b / 0.0006), 0.5 * exp(-c * c / 0.0006)));
  float halo = exp(-a * a / 0.012) + 0.5 * exp(-b * b / 0.012);
  float heat = smoothstep(-0.2, 0.3, fbm(q * 1.6 + uOff.zxy, 4));
  return vec2(clamp((core + 0.22 * halo) * heat, 0.0, 1.0), clamp(halo * heat, 0.0, 1.0));
}
float lavaGlow(vec3 q) {
  return fissures(q).x;
}
vec4 lavaSurface(vec3 q) {
  float plate = smoothstep(-0.2, 0.3, fbm(q * 3.0 + uOff, 6));
  vec3 alb = mix(uA, uB, plate);
  alb = mix(alb, uB * 1.5, smoothstep(0.1, 0.5, fbm(q * 7.0 + uOff.yzx, 5)) * 0.35);
  alb *= 0.85 + 0.3 * fbm(q * 30.0 + uOff.zxy, 3);
  alb *= 1.0 - 0.55 * fissures(q).y;
  return vec4(sqrt(alb), 0.0);
}
float basins(vec3 q) {
  float s = 0.0;
  for (int i = 0; i < 4; i++) {
    vec3 r = hash(vec3(float(i), 23.0, uOff.y));
    float la = (r.x * 2.0 - 1.0) * 0.8;
    float ca = cos(la);
    vec3 c = vec3(cos(r.z * 6.2831853) * ca, sin(la), sin(r.z * 6.2831853) * ca);
    s += smoothstep(1.2, 0.5, distance(q, c) / (0.22 + 0.3 * r.y));
  }
  return s;
}
float mare(vec3 q) {
  return smoothstep(-0.1, 0.24, 0.8 * fbm(warp(q * uFreq + uOff, 0.5), 5) + 0.1 * q.y + 0.06 * fbm(q * 14.0 + uOff.zxy, 3) + 0.3 * basins(q));
}
float moonHeight(vec3 q, out float cr) {
  cr = craters(q);
  float m = mare(q);
  return mix(cr + 0.03 * ridged(q * 8.0 + uOff.yzx, 4), cr * 0.25 - 0.04, m) + fbm(q * 3.0 + uOff.zxy, 5) * 0.06;
}
float rays(vec3 q) {
  float s = 0.0;
  for (int i = 0; i < 4; i++) {
    vec3 r = hash(vec3(float(i), 17.0, uOff.x));
    float la = (r.x * 2.0 - 1.0) * 0.9;
    float ca = cos(la);
    vec3 c = vec3(cos(r.z * 6.2831853) * ca, sin(la), sin(r.z * 6.2831853) * ca);
    vec3 up = abs(c.y) < 0.9 ? vec3(0.0, 1.0, 0.0) : vec3(1.0, 0.0, 0.0);
    vec3 t1 = normalize(cross(up, c));
    vec3 t2 = cross(c, t1);
    vec3 v = q - c * dot(q, c);
    float th = acos(clamp(dot(q, c), -1.0, 1.0));
    float az = atan(dot(v, t2), dot(v, t1));
    vec3 ring = vec3(cos(az), sin(az), r.y * 40.0);
    float spoke = smoothstep(-0.1, 0.5, gnoise(ring * (1.6 + 2.4 * r.y)) + 0.25 * gnoise(ring * 7.0 + 9.0));
    float patchy = 0.25 + 0.75 * smoothstep(-0.15, 0.3, fbm(q * 10.0 + r * 30.0, 3));
    float reach = 0.4 + 0.7 * r.y;
    s += spoke * patchy * smoothstep(reach, 0.04, th) * (0.5 + 0.5 * r.x) + 0.8 * smoothstep(0.035, 0.008, th);
  }
  return clamp(s, 0.0, 1.0);
}
vec4 moonSurface(vec3 q) {
  float cr;
  moonHeight(q, cr);
  float m = mare(q);
  vec3 high = mix(uA, uD, smoothstep(-0.1, 0.3, fbm(q * 5.0 + uOff.yzx, 4)));
  high *= 0.9 + 0.2 * smoothstep(0.3, 0.9, ridged(q * 24.0 + uOff.zxy, 5));
  vec3 sea = uB * (0.92 + 0.16 * fbm(q * 30.0 + uOff.zxy, 4));
  vec3 alb = mix(high, sea, m);
  alb *= 0.84 + 0.9 * clamp(cr * 4.0, -0.12, 0.3) + 0.1 * fbm(q * 18.0 + uOff, 4);
  alb = mix(alb, uC, rays(q) * 0.4);
  return vec4(sqrt(alb), 0.0);
}
vec3 bands(float y) {
  y = clamp(y, -1.0, 1.0);
  vec3 c = uBands[0].yzw;
  for (int i = 1; i < ${BANDS}; i++) {
    if (i >= uCount) break;
    vec4 a = uBands[i - 1];
    vec4 b = uBands[i];
    if (y >= a.x) c = mix(a.yzw, b.yzw, smoothstep(0.3, 0.7, (y - a.x) / max(b.x - a.x, 1e-4)));
  }
  return c;
}
vec3 eddies(vec3 q) {
  vec3 p = q;
  for (int i = 0; i < 28; i++) {
    vec3 r = hash(vec3(float(i), 3.0, uOff.y));
    float la = (r.x * 2.0 - 1.0) * 0.85;
    float ca = sqrt(1.0 - la * la);
    vec3 c = vec3(cos(r.z * 6.2831853) * ca, la, sin(r.z * 6.2831853) * ca);
    float size = 0.025 + 0.05 * r.y * r.y;
    float e = distance(p, c) / size;
    p = spin(p, c, (fract(r.x * 13.0) < 0.5 ? -1.0 : 1.0) * 2.5 * exp(-e * e));
  }
  for (int i = 0; i < 24; i++) {
    if (uCount < 2) break;
    vec3 r = hash(vec3(float(i), 5.0, uOff.y));
    int k = 1 + int(r.x * float(uCount - 1));
    float la = clamp(0.5 * (uBands[k - 1].x + uBands[k].x), -0.85, 0.85);
    float ca = sqrt(1.0 - la * la);
    vec3 c = vec3(cos(r.z * 6.2831853) * ca, la, sin(r.z * 6.2831853) * ca);
    float e = distance(p, c) / (0.02 + 0.03 * r.y);
    p = spin(p, c, (fract(r.x * 29.0) < 0.5 ? -1.0 : 1.0) * 2.5 * exp(-e * e));
  }
  return p;
}
vec4 gasSurface(vec3 q) {
  float la = asin(clamp(q.y, -1.0, 1.0));
  vec2 rel = vec2(wrap(atan(q.z, q.x) - uFeature.y) * cos(uFeature.x), la - uFeature.x);
  float e = length(rel / vec2(uFeature.z * 1.8, uFeature.z));
  vec3 sc = vec3(cos(uFeature.x) * cos(uFeature.y), sin(uFeature.x), cos(uFeature.x) * sin(uFeature.y));
  vec3 p = spin(q, sc, 3.0 * exp(-e * e * 1.2) * uFeature.w);
  p = mix(p, eddies(p), uBump);
  vec3 s = vec3(p.x * 2.0, p.y * 11.0, p.z * 2.0) * uFreq + uOff;
  float t = fbm(warp(s, 0.8), 7);
  float edge = length(bands(p.y + 0.02) - bands(p.y - 0.02));
  float y = p.y + 0.04 * t * uBump * (0.3 + 4.0 * edge) + 0.014 * fbm(warp(s * 3.0, 0.5), 5);
  vec3 c = bands(y);
  c *= 1.0 + uBump * 0.1 * fbm(warp(vec3(p.x * 7.0, p.y * 60.0, p.z * 7.0) * uFreq + uOff.yzx, 0.6), 6);
  c *= 1.0 + uBump * 0.12 * fbm(warp(vec3(p.x * 5.0, p.y * 16.0, p.z * 5.0) * uFreq + uOff.zyx, 0.8), 5);
  c = mix(c, uTone, smoothstep(1.0, 0.55, e) * uFeature.w);
  float wisp = smoothstep(0.25, 0.55, fbm(vec3(p.x * 3.0, p.y * 26.0, p.z * 3.0) * uFreq + uOff.zxy, 5));
  c = mix(c, vec3(0.75, 0.74, 0.7) * max(max(c.r, c.g), c.b) * 1.25 + 0.1, wisp * uCover);
  for (int i = 0; i < 6; i++) {
    vec3 r = hash(vec3(float(i), 11.0, uOff.z));
    float ol = -0.55 + 0.25 * r.x;
    vec3 oc = vec3(cos(ol) * cos(r.z * 6.2831853), sin(ol), cos(ol) * sin(r.z * 6.2831853));
    vec3 d = q - oc;
    float oe = length(vec2(length(d.xz) * 0.55, d.y)) / (0.012 + 0.012 * r.y);
    c = mix(c, vec3(0.8, 0.78, 0.74) * max(max(c.r, c.g), c.b) * 1.2, smoothstep(1.0, 0.6, oe) * uBump * 0.8);
  }
  return vec4(sqrt(c), 0.0);
}
vec4 deckSurface(vec3 q) {
  vec3 s = vec3(q.x * 1.5, q.y * 4.0, q.z * 1.5) * uFreq + uOff;
  float n = fbm(warp(s, 0.9), 6);
  float y = abs(q.y);
  vec3 c = mix(uA, uB, clamp(smoothstep(-0.25, 0.35, n) * 0.45 + 0.25 * smoothstep(0.6, 0.0, y) * smoothstep(-0.1, 0.3, n), 0.0, 1.0));
  return vec4(sqrt(c), 0.0);
}
float height(vec3 q, out float wet, out float hm) {
  float cr;
  wet = 0.0;
  hm = 0.0;
  if (KIND == 0) {
    float m;
    hm = terran(q, m);
    wet = wetness(q);
    return max(hm, uSea);
  }
  if (KIND == 1) return desert(q, cr);
  if (KIND == 2) return iceHeight(q);
  if (KIND == 3) return lavaHeight(q);
  if (KIND == 5) return moonHeight(q, cr);
  return 0.0;
}
vec4 reliefMap(vec3 q) {
  float wet;
  float hm;
  float h = height(q, wet, hm);
  vec3 dx = dFdx(q);
  vec3 dy = dFdy(q);
  float hx = dFdx(h);
  float hy = dFdy(h);
  float a = dot(dx, dx);
  float b = dot(dx, dy);
  float c = dot(dy, dy);
  float det = a * c - b * b;
  vec2 k = det > 0.0 ? vec2(c * hx - b * hy, a * hy - b * hx) / det : vec2(0.0);
  vec3 n = normalize(q - uBump * (k.x * dx + k.y * dy));
  float glow = KIND == 0 ? cities(q, hm, wet) : KIND == 3 ? lavaGlow(q) : 0.0;
  return vec4(n - q + 0.5, glow);
}
vec4 cloudMap(vec3 q) {
  vec3 p = q;
  for (int i = 0; i < 9; i++) {
    vec3 r = hash(vec3(float(i), 7.0, uOff.x));
    float la = (0.35 + 0.45 * r.x) * (r.y < 0.5 ? -1.0 : 1.0);
    float ca = sqrt(1.0 - la * la);
    vec3 c = vec3(cos(r.z * 6.2831853) * ca, la, sin(r.z * 6.2831853) * ca);
    float size = 0.05 + 0.07 * fract(r.x * 7.0);
    float e = distance(p, c) / size;
    p = spin(p, c, sign(la) * 3.2 * exp(-e * e));
  }
  float lat = q.y;
  vec3 s = vec3(p.x, p.y * 2.6, p.z);
  vec3 w = warp(s * 1.8 + uOff + 40.0, 0.7);
  float big = fbm(w, 6);
  float mid = fbm(warp(s * 6.0 + uOff.yzx, 0.4), 5);
  float fine = fbm(vec3(s.x, s.y * 1.6, s.z) * 30.0 + uOff.zxy, 4);
  float belt = KIND == 0 ? 0.16 * exp(-pow(lat / 0.09, 2.0)) - 0.2 * exp(-pow((abs(lat) - 0.38) / 0.12, 2.0)) + 0.05 * smoothstep(0.5, 0.75, abs(lat)) : 0.25 * smoothstep(0.75, 0.95, abs(lat)) - 0.1;
  float v = big + 0.35 * mid + belt + (uCover - 0.5) * 0.7;
  float body = smoothstep(-0.12, 0.42, v + 0.22 * fine);
  float wisp = KIND == 0 ? smoothstep(-0.16, 0.0, v) * smoothstep(0.0, 0.35, fine + 0.1) * 0.55 : 0.0;
  float pop = KIND == 0 ? smoothstep(0.2, 0.45, fbm(s * 90.0 + uOff.yxz, 3)) * smoothstep(-0.25, -0.05, v) * smoothstep(0.42, 0.25, abs(lat)) * 0.3 : 0.0;
  return vec4(max(body, max(wisp, pop)), smoothstep(0.02, 0.4, v), 0.0, 1.0);
}
void main() {
  vec3 q = normalize(d);
#if MAP == 1
  o = reliefMap(q);
#elif MAP == 2
  o = cloudMap(q);
#elif KIND == 0
  o = terranSurface(q);
#elif KIND == 1
  o = desertSurface(q);
#elif KIND == 2
  o = iceSurface(q);
#elif KIND == 3
  o = lavaSurface(q);
#elif KIND == 4
  o = gasSurface(q);
#elif KIND == 5
  o = moonSurface(q);
#else
  o = deckSurface(q);
#endif
}
`;

const DRAW = `
precision highp float;
precision highp int;
in vec2 v;
out vec4 o;
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform vec3 uSun;
uniform vec3 uLight;
uniform float uSize;
uniform float uGlare;
uniform mat3 uBody;
uniform mat3 uSky;
uniform float uJet;
uniform samplerCube uSurface;
uniform samplerCube uRelief;
uniform samplerCube uClouds;
uniform sampler2D uRings;
uniform vec3 uCloudTone;
uniform vec3 uCity;
uniform vec3 uGlow;
uniform float uSpec;
uniform float uRough;
uniform float uLimb;
uniform float uFlat;
uniform float uTop;
uniform vec3 uRay;
uniform vec3 uMie;
uniform vec3 uDust;
uniform vec3 uScale;
uniform vec4 uRing;
uniform vec3 uRingDark;
uniform vec3 uRingBright;
uniform vec3 uPole;
uniform vec4 uMoon[3];
uniform vec4 uMoonTone[3];
uniform float uFade;
const float PI = 3.14159265;
${NOISE}
vec2 sphere(vec3 ro, vec3 rd, float r) {
  float b = dot(ro, rd);
  float c = dot(ro, ro) - r * r;
  float h = b * b - c;
  if (h < 0.0) return vec2(-1.0);
  h = sqrt(h);
  return vec2(-b - h, -b + h);
}
float chapman(float X, float h, float mu) {
  float c = sqrt(1.5707963 * (X + h));
  if (mu >= 0.0) return c / (c * mu + 1.0) * exp(-h);
  float x0 = sqrt(1.0 - mu * mu) * (X + h);
  float c0 = sqrt(1.5707963 * x0);
  return 2.0 * c0 * exp(min(X - x0, 60.0)) - c / (1.0 - c * mu) * exp(-h);
}
vec3 thru(float h, float mu) {
  float cr = chapman(1.0 / uScale.x, h / uScale.x, mu);
  float cm = chapman(1.0 / uScale.y, h / uScale.y, mu);
  return exp(-(uRay * uScale.x * cr + uDust * uScale.y * cm));
}
vec3 air(vec3 ro, vec3 rd, float t0, float t1, float tc, out vec3 T) {
  T = vec3(1.0);
  vec3 S = vec3(0.0);
  if (AIR == 0 || t1 <= t0) return S;
  float mu = dot(rd, uSun);
  float g = uScale.z;
  float pr = 0.0596831 * (1.0 + mu * mu);
  float pm = 0.1193662 * (1.0 - g * g) * (1.0 + mu * mu) / ((2.0 + g * g) * pow(max(1.0 + g * g - 2.0 * g * mu, 1e-4), 1.5));
  tc = clamp(tc, t0, t1);
  int na = tc >= t1 ? STEPS : STEPS / 2;
  for (int i = 0; i < 16; i++) {
    if (i >= STEPS) break;
    bool front = i < na;
    float n = float(front ? na : STEPS - na);
    float x0 = float(front ? i : i - na) / n;
    float x1 = x0 + 1.0 / n;
    float xm = x0 + 0.5 / n;
    float a = front ? t0 + (tc - t0) * (1.0 - (1.0 - x0) * (1.0 - x0)) : tc + (t1 - tc) * x0 * x0;
    float b = front ? t0 + (tc - t0) * (1.0 - (1.0 - x1) * (1.0 - x1)) : tc + (t1 - tc) * x1 * x1;
    float m = front ? t0 + (tc - t0) * (1.0 - (1.0 - xm) * (1.0 - xm)) : tc + (t1 - tc) * xm * xm;
    float ds = b - a;
    vec3 p = ro + rd * m;
    float r = length(p);
    float h = max(r - 1.0, 0.0);
    float dr = exp(-h / uScale.x);
    float dm = exp(-h / uScale.y);
    vec3 ext = uRay * dr + uDust * dm;
    vec3 scat = uRay * dr * pr + uMie * dm * pm;
    vec3 keep = exp(-ext * ds);
    S += T * thru(h, dot(p, uSun) / r) * scat * (1.0 - keep) / max(ext, vec3(1e-6));
    T *= keep;
  }
  return S * uLight;
}
vec3 jets(vec3 q) {
  float w = uJet * (0.6 * sin(q.y * 23.0) + 0.4 * sin(q.y * 9.0 + 1.0));
  float c = cos(w);
  float s = sin(w);
  return vec3(c * q.x - s * q.z, q.y, s * q.x + c * q.z);
}
float ringShade(vec3 p) {
  float ds = dot(uSun, uPole);
  if (RINGS == 0 || abs(ds) < 1e-4) return 1.0;
  float t = -dot(p, uPole) / ds;
  float u = (length(p + uSun * t) - uRing.x) / (uRing.y - uRing.x);
  float lod = log2(max(fwidth(u) * ${RING}.0, 1.0));
  if (t <= 0.0 || u <= 0.0 || u >= 1.0) return 1.0;
  return exp(-(-log(max(1.0 - textureLod(uRings, vec2(u, 0.5), lod).r, 0.02))) / abs(ds));
}
float moonShade(vec3 p) {
  float s = 1.0;
  for (int i = 0; i < 3; i++) {
    if (i >= MOONS) break;
    vec3 m = uMoon[i].xyz - p;
    float along = dot(m, uSun);
    if (along <= 0.0) continue;
    float d = length(m - uSun * along);
    float pen = along * uSize + 1e-3;
    s *= smoothstep(uMoon[i].w - pen, uMoon[i].w + pen, d);
  }
  return s;
}
float planetShade(vec3 p) {
  float along = dot(p, uSun);
  if (along >= 0.0) return 1.0;
  float d = length(p - uSun * along);
  float pen = -along * uSize + 0.004;
  return smoothstep(1.0 - pen, 1.0 + pen, d);
}
vec3 ember(float g) {
  return g * g * mix(vec3(1.0, 0.101, 0.0097), vec3(1.0, 0.456, 0.061), smoothstep(0.45, 1.0, g));
}
vec3 body(vec3 p, vec3 rd, vec4 s, vec4 rel, vec4 cl, float cs, float rs) {
  vec3 n = normalize(p);
  vec3 alb = s.rgb * s.rgb;
  vec3 nb = n;
  if (BUMPY == 1) nb = normalize(transpose(uBody) * normalize(uBody * n + rel.rgb - 0.5));
  float mu0 = dot(n, uSun);
  float muv = max(dot(n, -rd), 1e-3);
  float shade = rs * moonShade(p);
  vec3 sunAt = uLight * shade * (AIR == 1 ? thru(0.0, mu0) : vec3(smoothstep(-0.004, 0.004, mu0)));
  float m0 = max(dot(nb, uSun), 0.0) * smoothstep(-0.06, 0.02, mu0);
  float brdf = m0 * pow(muv, uLimb);
  brdf = mix(brdf, 2.0 * m0 / (m0 + muv), uFlat);
  float under = 1.0 - 0.6 * cs;
  vec3 col = alb / PI * brdf * sunAt * under;
  if (uSpec > 0.0) {
    vec3 hv = normalize(uSun - rd);
    float nh = max(dot(n, hv), 0.0);
    float a = uRough * uRough;
    float a2 = a * a;
    float dd = nh * nh * (a2 - 1.0) + 1.0;
    float D = a2 / (PI * dd * dd);
    float F = 0.02 + 0.98 * pow(1.0 - max(dot(-rd, hv), 0.0), 5.0);
    float k = a * 0.5;
    float g0 = max(mu0, 0.0);
    float G = g0 / (g0 * (1.0 - k) + k) * muv / (muv * (1.0 - k) + k);
    col += sunAt * D * F * G / (4.0 * muv) * s.a * uSpec * under;
  }
  if (CLOUDY == 1) {
    vec3 sunC = uLight * shade * (AIR == 1 ? thru(1.25 * uScale.x, mu0) : vec3(smoothstep(-0.01, 0.01, mu0)));
    float lit = clamp((mu0 + 0.06) / 1.06, 0.0, 1.0);
    vec3 cloud = uCloudTone / PI * sunC * lit * (0.8 + 0.2 * cl.g);
    col = mix(col, cloud, cl.r);
  }
  float night = 1.0 - smoothstep(-0.1, 0.0, mu0);
  col += uCity * rel.a * night * (1.0 - 0.85 * cl.r);
  col += uGlow * ember(rel.a) * mix(1.0, 0.2, smoothstep(-0.05, 0.4, mu0)) * (1.0 - 0.6 * cl.r);
  return col + alb * uLight * 0.0012;
}
vec4 ring(vec3 ro, vec3 rd, out float t) {
  t = 1e9;
  float dn = dot(rd, uPole);
  if (abs(dn) < 1e-6) return vec4(0.0);
  float tp = -dot(ro, uPole) / dn;
  if (tp <= 0.0) return vec4(0.0);
  vec3 p = ro + rd * tp;
  float r = length(p);
  float u = (r - uRing.x) / (uRing.y - uRing.x);
  float muv = abs(dn);
  float hg = 0.0795775 * 0.51 / pow(max(1.49 - 1.4 * dot(rd, uSun), 1e-3), 1.5);
  if (u >= 1.0 && r < 8.0 && uRing.w > 0.0) {
    float a = uRing.w * exp(-pow((r - 3.9) / 1.4, 2.0)) / max(muv, 0.02);
    t = tp;
    return vec4(uLight * vec3(0.8, 0.85, 1.0) * hg * 0.6 * a * planetShade(p), a);
  }
  if (u <= 0.0 || u >= 1.0) return vec4(0.0);
  float px = tp * 2.0 * uTan / min(uRes.x, uRes.y) / max(muv, 0.05) / (uRing.y - uRing.x) * ${RING}.0;
  vec4 m = textureLod(uRings, vec2(u, 0.5), log2(max(px, 1.0)));
  float tau = -log(max(1.0 - m.r, 0.02));
  float ds = dot(uSun, uPole);
  float mus = max(abs(ds), 0.03);
  vec3 tone = mix(uRingDark, uRingBright, m.b);
  vec3 L;
  if ((ds > 0.0) == (dot(ro, uPole) > 0.0)) L = tone / PI * 2.0 * mus / (mus + muv) * (1.0 - exp(-tau * (1.0 / mus + 1.0 / muv)));
  else L = tone / PI * 2.0 * mus / (mus + muv) * (1.0 - exp(-tau / mus)) * exp(-tau / muv) * 0.9;
  L += vec3(1.0, 0.96, 0.9) * m.g * hg * 0.12 * min(tau + 0.05, 1.0) / max(muv, 0.05);
  t = tp;
  return vec4(L * uLight * planetShade(p), 1.0 - exp(-tau / muv));
}
vec4 moon(int i, vec3 ro, vec3 rd, out float t) {
  vec3 c = uMoon[i].xyz;
  float r = uMoon[i].w;
  vec2 h = sphere(ro - c, rd, r);
  t = 1e9;
  if (h.x <= 0.0) return vec4(0.0);
  t = h.x;
  vec3 p = ro + rd * h.x;
  vec3 n = (p - c) / r;
  vec4 tone = uMoonTone[i];
  vec3 alb = tone.rgb * clamp(1.0 + tone.a * fbm(n * 3.0 + c * 0.37, 5) * 1.6, 0.2, 2.0);
  float m0 = max(dot(n, uSun), 0.0);
  float muv = max(dot(n, -rd), 1e-3);
  return vec4(uLight * alb / PI * 2.0 * m0 / (m0 + muv) * planetShade(p) + alb * uLight * 0.0012, 1.0);
}
vec3 sun(vec3 rd) {
  if (dot(rd, uSun) <= 0.0) return vec3(0.0);
  float x = length(cross(rd, uSun)) / uSize;
  float disc = 1.0 - smoothstep(0.85, 1.15, x);
  float limb = sqrt(max(1.0 - min(x * x, 1.0), 0.0));
  return uLight * (uGlare * disc * (0.45 + 0.55 * limb) + 2.0 * exp(-x * 0.6));
}
void main() {
  vec2 uv = (gl_FragCoord.xy - 0.5 * uRes) / min(uRes.x, uRes.y) * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  vec3 ro = uPos;
  vec2 hp = sphere(ro, rd, 1.0);
  bool hit = hp.x > 0.0;
  vec3 pc = hit ? ro + rd * hp.x : normalize(ro + rd * max(-dot(ro, rd), 0.0));
  vec3 n = normalize(pc);
  vec3 q = uBody * n;
  if (KIND == 4) q = jets(q);
  vec4 s = texture(uSurface, q);
  vec4 rel = BUMPY == 1 ? texture(uRelief, q) : vec4(0.5, 0.5, 0.5, 0.0);
  vec4 cl = vec4(0.0);
  float cs = 0.0;
  if (CLOUDY == 1) {
    vec3 qc = uSky * n;
    cl = texture(uClouds, qc);
    cs = texture(uClouds, normalize(qc + 0.02 * (uSky * uSun))).r;
  }
  float rs = ringShade(pc);
  float tt[5];
  vec4 cc[5];
  for (int i = 0; i < 5; i++) {
    tt[i] = 1e9;
    cc[i] = vec4(0.0);
  }
  int k = 0;
  vec3 T = vec3(1.0);
  vec2 ha = AIR == 1 ? sphere(ro, rd, uTop) : vec2(-1.0);
  if (hit || ha.y > 0.0) {
    float t0 = max(ha.x, 0.0);
    float t1 = hit ? hp.x : ha.y;
    vec3 S = air(ro, rd, t0, t1, hit ? t1 : -dot(ro, rd), T);
    vec4 c = vec4(S, 1.0 - (T.r + T.g + T.b) / 3.0);
    if (hit) c = vec4(S + T * body(pc, rd, s, rel, cl, cs, rs), 1.0);
    tt[k] = AIR == 1 ? t0 : hp.x;
    cc[k] = c;
    k++;
  }
  if (RINGS == 1) {
    float t;
    vec4 c = ring(ro, rd, t);
    if (t < 1e8) {
      tt[k] = t;
      cc[k] = c;
      k++;
    }
  }
  for (int i = 0; i < 3; i++) {
    if (i >= MOONS) break;
    float t;
    vec4 c = moon(i, ro, rd, t);
    if (t < 1e8) {
      tt[k] = t;
      cc[k] = c;
      k++;
    }
  }
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      if (j + 1 < k && tt[j + 1] < tt[j]) {
        float t = tt[j];
        tt[j] = tt[j + 1];
        tt[j + 1] = t;
        vec4 c = cc[j];
        cc[j] = cc[j + 1];
        cc[j + 1] = c;
      }
    }
  }
  vec3 col = vec3(0.0);
  float left = 1.0;
  for (int i = 0; i < 5; i++) {
    if (i >= k) break;
    col += left * cc[i].rgb;
    left *= 1.0 - cc[i].a;
  }
  col += left * sun(rd) * T;
  o = vec4(col, 1.0 - left) * uFade;
}
`;

/* PLANET */

const glsl = (defs, body) => `#version 300 es\n${Object.entries(defs).map(([k, v]) => `#define ${k} ${v}`).join('\n')}${body}`;

const flat = (rows, n, width) => {
  const out = new Float32Array(n * width);
  rows.slice(0, n).forEach((row, i) => out.set(row.slice(0, width), i * width));
  return out;
};

const zero = [0, 0, 0];

const kept = new WeakMap();

const baker = (gl, map, kind) => {
  if (!kept.has(gl)) kept.set(gl, new Map());
  const cache = kept.get(gl);
  const key = `${map} ${kind}`;
  if (!cache.has(key)) cache.set(key, link(gl, glsl({ MAP: map, KIND: kind }, BAKE), CUBE));
  return cache.get(key);
};

export function planet(gl, view, params, { staged = false } = {}) {
  const level = tier(view);
  const size = FACES[level];
  const kind = KINDS[params.kind] ?? 0;
  const rocky = !['gas', 'deck'].includes(params.kind);
  const cloudy = (params.cover ?? 0) > 0 && params.kind !== 'gas';
  const offs = rng((params.noise ?? 0) >>> 0);
  const colors = params.colors ?? [];
  const bands = params.bands ?? [];
  const recipe = {
    uOff: [offs() * 200, offs() * 200, offs() * 200],
    uSea: params.sea ?? 0,
    uFreq: params.freq ?? 1,
    uCover: params.cover ?? 0,
    uLights: params.lights ?? 0,
    uBump: params.bump ?? 0,
    uA: colors[0] ?? zero,
    uB: colors[1] ?? zero,
    uC: colors[2] ?? zero,
    uD: colors[3] ?? zero,
    uE: colors[4] ?? zero,
    uBands: flat(bands, BANDS, 4),
    uCount: Math.min(bands.length, BANDS),
    uFeature: params.feature ?? [0, 0, 0.01, 0],
    uTone: params.tone ?? zero,
  };
  const maps = [0, rocky ? 1 : -1, cloudy ? 2 : -1].filter((map) => map >= 0).map((map) => ({ map, link: baker(gl, map, kind), tex: null }));
  const ring = params.ring;
  const a = params.air;
  const count = Math.min(params.moons.length, 3);
  const drawing = link(gl, glsl({ KIND: kind, BUMPY: rocky ? 1 : 0, CLOUDY: cloudy ? 1 : 0, MOONS: count, AIR: a ? 1 : 0, RINGS: ring ? 1 : 0, STEPS: STEPS[level] }, DRAW));
  const light = (params.light ?? [1, 1, 1]).map((c) => c * LIGHT);
  const tones = flat(params.moons.map((m) => [...m.tone, m.rough]), 3, 4);
  const fixed = {
    uLight: light,
    uSize: DISC,
    uGlare: GLARE / Math.PI,
    uCloudTone: params.cloud ?? zero,
    uCity: params.city ?? zero,
    uGlow: params.glow ?? zero,
    uSpec: params.spec ?? 0,
    uRough: params.rough || 0.4,
    uLimb: params.limb ?? 0,
    uFlat: params.flat ?? 0,
    uTop: a ? a.top : 0,
    uRay: a ? a.ray : zero,
    uMie: a ? a.mie : zero,
    uDust: a ? a.dust : zero,
    uScale: a ? [a.h, a.hm, a.g] : [1, 1, 0],
    uRing: ring ? [ring.inner, ring.outer, 1, ring.halo ?? 0] : [1, 2, 0, 0],
    uRingDark: ring ? ring.dark : zero,
    uRingBright: ring ? ring.bright : zero,
    uPole: pole(params),
    uMoonTone: tones,
  };
  let prog = null;
  let rings = null;
  let next = 0;
  let gone = false;
  const finish = () => {
    for (const job of maps) mipmap(gl, job.tex);
    rings = texture(gl, { w: ring ? RING : 1, h: 1, data: ring ? profile(ring) : new Uint8Array(4), filter: ring ? 'mip' : 'linear' });
    const map = (n) => (maps.find((job) => job.map === n) ?? maps[0]).tex;
    prog = drawing.done();
    prog.set({ ...fixed, uSurface: map(0), uRelief: map(1), uClouds: map(2), uRings: rings });
  };
  const step = (force) => {
    if (prog || gone) return true;
    if (next < maps.length) {
      const job = maps[next];
      if (!force && !job.link.ready()) return false;
      job.tex = bake(gl, size, job.link.done(), recipe, false);
      next += 1;
      if (next < maps.length) return false;
    }
    if (!force && !drawing.ready()) return false;
    finish();
    return true;
  };
  const build = () => {
    while (!step(true));
  };
  if (!staged) build();
  const res = new Float32Array(2);
  const toward = new Float32Array(3);
  const live = { uPos: null, uRot: null, uTan: 1, uRes: res, uSun: toward, uBody: new Float32Array(9), uSky: new Float32Array(9), uJet: 0, uMoon: new Float32Array(12), uFade: 1 };
  const draw = (cam, sun, t, k = 1) => {
    if (gone) return;
    if (!prog) build();
    const port = gl.getParameter(gl.VIEWPORT);
    const s = t / 1000;
    const turn = (params.phase ?? 0) + (params.spin ?? 0) * s;
    const far = Math.hypot(sun[0], sun[1], sun[2]) || 1;
    for (let i = 0; i < count; i++) place(params.tilt, params.moons[i], t, live.uMoon, i * 4);
    for (let i = 0; i < 3; i++) toward[i] = sun[i] / far;
    res[0] = port[2];
    res[1] = port[3];
    live.uPos = cam.pos;
    live.uRot = cam.rot;
    live.uTan = Math.tan(cam.fov / 2);
    spin(params.tilt, turn, live.uBody);
    spin(params.tilt, turn + (params.drift ?? 0) * s, live.uSky);
    live.uJet = (params.jets ?? 0) * 20 * Math.sin(s / 20);
    live.uFade = k;
    prog.set(live);
    blend(gl, 'alpha');
    fill(gl);
    blend(gl, null);
  };
  const drop = () => {
    if (gone) return;
    gone = true;
    drawing.drop();
    for (const tex of [...maps.map((job) => job.tex), rings]) if (tex) gl.deleteTexture(tex);
  };
  return { draw, ensure: () => step(false), drop };
}
