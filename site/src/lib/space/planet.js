import { bake, blend, CUBE, fill, link, mipmap, texture, tier } from "../gl/gl2.js";
import { rng } from "../rand.js";
import { BAKE, BANDS, DRAW, RING } from "./planet.glsl.js";

const TAU = Math.PI * 2;
const FACES = { phone: 512, desk: 1024, big: 1024 };
const STEPS = { phone: 6, desk: 10, big: 10 };
const LIGHT = 5;
const GLARE = 400;
const DISC = 0.012;
const KINDS = { terran: 0, desert: 1, ice: 2, lava: 3, gas: 4, moon: 5, deck: 6 };

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

/* PLANET */

const glsl = (defs, body) =>
  `#version 300 es\n${Object.entries(defs)
    .map(([k, v]) => `#define ${k} ${v}`)
    .join("\n")}${body}`;

const flat = (rows, n, width) => {
  const out = new Float32Array(n * width);
  for (const [i, row] of rows.slice(0, n).entries()) out.set(row.slice(0, width), i * width);
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
  const rocky = !["gas", "deck"].includes(params.kind);
  const cloudy = (params.cover ?? 0) > 0 && params.kind !== "gas";
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
  const maps = [0, rocky ? 1 : -1, cloudy ? 2 : -1]
    .filter((map) => map >= 0)
    .map((map) => ({ map, link: baker(gl, map, kind), tex: null }));
  const ring = params.ring;
  const a = params.air;
  const count = Math.min(params.moons.length, 3);
  const drawing = link(
    gl,
    glsl(
      {
        KIND: kind,
        BUMPY: rocky ? 1 : 0,
        CLOUDY: cloudy ? 1 : 0,
        MOONS: count,
        AIR: a ? 1 : 0,
        RINGS: ring ? 1 : 0,
        STEPS: STEPS[level],
      },
      DRAW,
    ),
  );
  const light = (params.light ?? [1, 1, 1]).map((c) => c * LIGHT);
  const tones = flat(
    params.moons.map((m) => [...m.tone, m.rough]),
    3,
    4,
  );
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
    rings = texture(gl, {
      w: ring ? RING : 1,
      h: 1,
      data: ring ? profile(ring) : new Uint8Array(4),
      filter: ring ? "mip" : "linear",
    });
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
  const live = {
    uPos: null,
    uRot: null,
    uTan: 1,
    uRes: res,
    uSun: toward,
    uBody: new Float32Array(9),
    uSky: new Float32Array(9),
    uJet: 0,
    uMoon: new Float32Array(12),
    uFade: 1,
  };
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
    blend(gl, "alpha");
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
