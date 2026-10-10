import { tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { codes, route } from '../../lib/space/bang.js';
import { look as aim, drift, ease, project } from '../../lib/space/camera.js';
import { blank, blend, context, fill, governor, program, tier } from '../../lib/space/gl2.js';
import { FILM, hyper } from '../../lib/space/hyper.js';
import { GRADES, march, tone } from '../../lib/space/march.js';
import { planet, world } from '../../lib/space/planet.js';
import { ship } from '../../lib/space/ship.js';
import { add, basis, cross, dot, len, mat3, mix, mul, norm, rot, sub } from '../../lib/space/vec.js';
import { arrange, clock, cues, song } from './score.js';

const PLACES = [['', 'Random'], ['earth', 'Earth'], ['mars', 'Mars'], ['jupiter', 'Jupiter'], ['saturn', 'Saturn'], ['moon', 'Moon'], ['venus', 'Venus'], ['neptune', 'Neptune'], ['terran', 'Terran'], ['desert', 'Desert'], ['ice', 'Ice'], ['lava', 'Lava'], ['gas', 'Gas']];

export const SPEC = [
  { key: 'from', label: 'From', kind: 'pick', def: '', options: PLACES, group: 'Trip' },
  { key: 'to', label: 'To', kind: 'pick', def: '', options: PLACES, group: 'Trip' },
  { key: 'ship', label: 'Ship', kind: 'slider', def: -1, min: -1, max: 255, step: 1, group: 'Trip' },
  { key: 'length', label: 'Length', kind: 'slider', def: 60, min: 30, max: 180, step: 15, group: 'Film' },
  { key: 'look', label: 'Look', kind: 'segment', def: 'studio', options: [['graphic', 'Graphic'], ['studio', 'Studio'], ['haze', 'Haze']], group: 'Look' },
  { key: 'sound', label: 'Sound', kind: 'toggle', def: 0, group: 'Sound' },
  { key: 'music', label: 'Music', kind: 'toggle', def: 1, group: 'Sound' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Again', does: 'Restart the film from the departure', act: 'again', button: true },
    { key: 'm', label: 'Sound', act: 'mute' },
  ],
  actions: { again: (scene) => scene.again?.(), mute: (scene) => scene.mute?.() },
  record: true,
};

export const quiet = ({ sound, music, ...rest }) => rest;

const NAMED = ['earth', 'mars', 'jupiter', 'saturn', 'moon', 'venus', 'neptune'];
const TYPES = ['terran', 'desert', 'ice', 'lava', 'gas'];
const WEIGHTS = [0.42, 0.16, 0.14, 0.1, 0.18];
const SALT = 0x2a9f53c1;
const GOLD = 0x9e3779b1;
const OTHER = 0x68e31da4;
const N = 3;
const SKY = { density: 0.8, dust: 0.4 };
const NOTCHES = { phone: [0.75, 0.6, 0.5], desk: [1, 0.85, 0.7, 0.5], big: [0.75, 0.6, 0.5] };
const BUSY = ['flash', 'tunnel', 'bloom'];
const LEAVE = FILM.bloom + FILM.white + FILM.decay + FILM.snap;
const VEIL = FILM.bloom + FILM.white;
const AFTER = -clock(0).out;
const SPAN = clock(0).end - clock(0).arrive;
const OPEN = clock(0).open;
const ENTER = clock(0).enter;
const INTO = ENTER - OPEN;
const LEAD = 120;
const STALE = 250;
const STILL = 6000;
const FADE = 1200;
const HELD = 9500;
const RUSH = 400;
const LOOPS = 100000;
const DEG = Math.PI / 180;
const FIRES = [[0.4, 0.8, 1.6], [1.5, 0.7, 0.3], [0.8, 0.5, 1.5], [0.5, 1.4, 0.9]];
const YAW = 0.3;
const PROTO = 0.085;
const DEEP = 1.25;
const HIGH = 1.1;
const BELOW = 9 * DEG;
const DAY = 65 * DEG;
const SUNSET = 95 * DEG;
const WIDE = [0.87, 0.98];
const ZOOM = [7000, 9000];
const SHOW = 1100;
const EMERGE = 1400;
const PASS = 8000;
const GONE = 12500;
const LOW = 1.01;
const UNDER = 66 * DEG;
const SLIP = 0.1;
const BESIDE = [0.13, 0.04];
const CLIMB = 20 * DEG;
const AWAY = 2.6;
const LEAN = 0.6;
const SPEEDS = [0.035, 0.05, 1.2];
const TURN = [3600, 5600];
const CHASE = [8000, 10000];
const OFFSET = [0.08, 0.05, -0.36];
const NOSE = 0.03;
const SMALL = 0.03;
const HEAT = [0.9, 4400];
const ENGINE = [9900, 10600];
const FAR = 2.8;
const SLANT = 0.8;
const LIMB = 0.45;
const PHASE = 60 * DEG;
const RAISE = 25 * DEG;
const SPOT = [0.53, 0.5];
const DIVE = [0.4, 0.62];
const TOP = 1.04;
const SHARE = 0.08;
const FINAL = 0.02;
const COAST = 0.3;
const GLIDE = 2500;
const SWAY = 0.02;
const DIE = 1300;
const LIGHT = 1500;
const BURN = 700;
const BEND = 1200;
const PULL = 2.2;
const HOT = 5500;
const COOL = 700;
const DOT = 3;
const BEACON = 6;
const QUENCH = 800;
const LEVELS = 3;
const SHINE = 0.25;
const REVEAL = 10;
const SETTLE = 400;
const RAW = { ev: 0.5, bloom: 0.8, aberration: 0.0035, vignette: 0.45, grain: 0.015 };
const EARLY = 120;
const PROBE = 0.02;
const HALVES = 60;
const ORIGIN = [0, 0, 0];
const FORWARD = [0, 0, 1];
const UP = [0, 1, 0];
const SQUARE = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

const SHADE = `#version 300 es
precision highp float;
out vec4 o;
uniform float uK;
void main() {
  o = vec4(0.0, 0.0, 0.0, uK);
}
`;

const POINT = `#version 300 es
precision highp float;
out vec4 o;
uniform vec2 uAt;
uniform float uR;
uniform vec3 uColor;
uniform float uK;
void main() {
  float k = clamp(uR + 0.5 - length(gl_FragCoord.xy - uAt), 0.0, 1.0) * uK;
  o = vec4(uColor * k, k);
}
`;

/* ROLL */

const clamp = (x) => Math.min(Math.max(x, 0), 1);

const unit = (rand) => {
  const z = rand() * 2 - 1;
  const a = rand() * Math.PI * 2;
  const r = Math.sqrt(1 - z * z);
  return [r * Math.cos(a), z, r * Math.sin(a)];
};

const across = (rand, n) => norm(cross(n, unit(rand)));

function weigh(rand, skip) {
  const pool = TYPES.map((type, i) => [type, type === skip ? 0 : WEIGHTS[i]]);
  let r = rand() * pool.reduce((sum, [, w]) => sum + w, 0);
  for (const [type, w] of pool) {
    if (r < w) return type;
    r -= w;
  }
  return pool.findLast(([, w]) => w > 0)[0];
}

function place(pick, seed, rand, skip) {
  if (NAMED.includes(pick)) return world(pick, seed);
  if (TYPES.includes(pick)) return world('exo', seed, { type: pick });
  return world('exo', Math.floor(rand() * 4294967296), { type: weigh(rand, skip) });
}

export function roll(value, seed, loop) {
  const rand = rng(((seed >>> 0) ^ SALT ^ Math.imul(loop + 1, GOLD)) >>> 0);
  const a = place(value.from, seed >>> 0, rand, '');
  const b = place(value.to, ((seed >>> 0) ^ OTHER) >>> 0, rand, a.kind);
  const pool = codes(N);
  const code = value.ship > 0 ? value.ship : pool[Math.floor(rand() * pool.length)];
  const up = unit(rand);
  const head = across(rand, up);
  const toward = unit(rand);
  return { loop, a, b, code, up, head, toward, tilt: across(rand, toward), fire: FIRES[Math.floor(rand() * FIRES.length)], turn: rand() < 0.5 ? -YAW : YAW, paint: Math.floor(rand() * 4294967296), depart: setting(up, head) };
}

/* DEPARTURE */

function setting(U, H) {
  const R = cross(U, H);
  const C = mul(U, HIGH);
  const pitch = Math.acos(1 / HIGH) + BELOW;
  const view = add(mul(H, Math.cos(pitch)), mul(U, -Math.sin(pitch)));
  const side = (Math.cos(SUNSET) + Math.sin(pitch) * Math.cos(DAY)) / (Math.cos(pitch) * Math.sin(DAY));
  const lateral = Math.sqrt(Math.max(1 - side * side, 0));
  const sun = add(mul(U, Math.cos(DAY)), mul(add(mul(H, side), mul(R, lateral)), Math.sin(DAY)));
  const ray = norm(add(add(mul(H, Math.cos(UNDER)), mul(U, -Math.sin(UNDER))), mul(R, SLIP)));
  const b = dot(ray, C);
  const reach = -b - Math.sqrt(b * b - (HIGH * HIGH - LOW * LOW));
  const way = add(mul(H, Math.cos(CLIMB)), mul(U, Math.sin(CLIMB)));
  const pass = mul(norm(add(add(C, mul(H, BESIDE[0])), mul(R, BESIDE[1]))), HIGH);
  const keys = [
    { t: EMERGE, p: add(C, mul(ray, reach)), v: mul(norm(add(U, mul(H, LEAN))), SPEEDS[0] / 1000) },
    { t: PASS, p: pass, v: mul(way, SPEEDS[1] / 1000) },
    { t: GONE, p: add(pass, mul(way, AWAY)), v: mul(way, SPEEDS[2] / 1000) },
  ];
  return { U, C, view, sun: norm(sun), keys, goal: way };
}

function hermite(keys, u) {
  const first = keys[0];
  const last = keys.at(-1);
  if (u <= first.t) return { p: add(first.p, mul(first.v, u - first.t)), v: first.v };
  if (u >= last.t) return { p: add(last.p, mul(last.v, u - last.t)), v: last.v };
  const i = keys.findIndex((one) => one.t > u) - 1;
  const [a, b] = [keys[i], keys[i + 1]];
  const span = b.t - a.t;
  const s = (u - a.t) / span;
  const [s2, s3] = [s * s, s * s * s];
  const sum = (w) => add(add(mul(a.p, w[0]), mul(a.v, span * w[1])), add(mul(b.p, w[2]), mul(b.v, span * w[3])));
  return { p: sum([2 * s3 - 3 * s2 + 1, s3 - 2 * s2 + s, -2 * s3 + 3 * s2, s3 - s2]), v: mul(sum([6 * s2 - 6 * s, 3 * s2 - 4 * s + 1, -6 * s2 + 6 * s, 3 * s2 - 2 * s]), 1 / span) };
}

const within = (u, [a, b]) => ease.smooth((u - a) / (b - a));

const rest = (u) => {
  if (u >= ENTER) return 0;
  if (u < OPEN) return OPEN - u + INTO / 2;
  const x = (u - OPEN) / INTO;
  return INTO * (0.5 - x + x ** 3 - x ** 4 / 2);
};

const twist = (rate, u) => (-rate * rest(u)) / 1000;

export function leave(rolled, u) {
  const { U, C, view, sun, keys, goal } = rolled.depart;
  const { p: at, v } = hermite(keys, u);
  const fwd = norm(v);
  const [sr, su, sf] = basis(fwd, U);
  const k = SMALL / PROTO;
  const off = add(add(mul(sr, OFFSET[0] * k), mul(su, OFFSET[1] * k)), mul(sf, OFFSET[2] * k));
  const chase = within(u, CHASE);
  const pos = mix(C, add(at, off), chase);
  const follow = norm(mix(view, norm(sub(add(at, mul(sf, NOSE * k)), pos)), within(u, TURN)));
  const ahead = norm(mix(follow, fwd, chase));
  const cam = aim(pos, add(pos, ahead), U);
  cam.fov = WIDE[0] + (WIDE[1] - WIDE[0]) * within(u, ZOOM);
  const pose = { at, fwd, up: U, turn: twist(rolled.turn, u), size: SMALL, heat: HEAT[0] * (1 - within(u, [EMERGE, HEAT[1]])), plume: within(u, ENGINE), fire: rolled.fire };
  return { cam, sun, pose, goal, shown: u >= SHOW };
}

/* ARRIVAL */

function sphere(from, dir, r) {
  const b = dot(from, dir);
  const h = b * b - (dot(from, from) - r * r);
  return add(from, mul(dir, h >= 0 ? -b - Math.sqrt(h) : -b));
}

const coast = (a) => {
  const total = (COAST * GLIDE) / 2000;
  if (a < 0) return (COAST * a) / 1000 - total;
  const s = Math.min(a, GLIDE);
  return (COAST * (s - (s * s) / (2 * GLIDE))) / 1000 - total;
};

export function come(rolled, a, t, view) {
  const n = rolled.toward;
  const V = rolled.tilt;
  const c = mul(n, -1);
  const side = norm(cross(V, c));
  const short = Math.min(view.w, view.h);
  const tan = Math.tan(SLANT / 2);
  const span = [(2 * tan * view.w) / short, (2 * tan * view.h) / short];
  const turn = Math.asin(1 / FAR) - Math.atan((LIMB - 0.5) * span[0]);
  const F = norm(add(mul(c, Math.cos(turn)), mul(side, Math.sin(turn))));
  const E = mul(n, FAR);
  const [right, up] = basis(F, V);
  const ray = ([x, y]) => norm(add(F, add(mul(right, (x - 0.5) * span[0]), mul(up, (0.5 - y) * span[1]))));
  const Q = sphere(E, ray(DIVE), TOP);
  const near = (len(sub(Q, E)) * FINAL) / SHARE;
  const size = near * SHARE * span[0];
  const along = ray(SPOT);
  const P = add(E, mul(along, near));
  const shift = mul(F, coast(a));
  const pos = add(add(E, shift), mul(drift(rolled.paint, t, SWAY), ease.smooth(a / GLIDE)));
  const cam = aim(pos, add(pos, F), V);
  cam.fov = SLANT;
  const dive = norm(sub(Q, P));
  const pull = ease.in(clamp((a - LIGHT) / (SPAN - FADE - LIGHT)), PULL);
  const at = add(add(P, shift), mul(sub(Q, P), pull));
  const fwd = norm(mix(along, dive, ease.smooth((a - LIGHT) / BEND)));
  const plume = a < 0 ? 1 : Math.max(1 - ease.smooth(a / DIE), ease.smooth((a - LIGHT) / BURN));
  const sun = norm(add(mul(n, Math.cos(PHASE)), mul(norm(add(mul(side, -Math.cos(RAISE)), mul(V, Math.sin(RAISE)))), Math.sin(PHASE))));
  const pose = { at, fwd, up: V, turn: (rolled.turn * a) / 1000, size, plume, heat: ease.smooth((a - HOT) / (SPAN - COOL - HOT)), fire: rolled.fire };
  return { cam, sun, pose, shown: true };
}

/* SHIP */

function seat(pose, [cr, cu, cf]) {
  const [sr, su, sf] = basis(pose.fwd, pose.up);
  const out = (v) => {
    const w = rot(v, cu, pose.turn);
    return add(add(mul(sr, dot(cr, w)), mul(su, dot(cu, w))), mul(sf, dot(cf, w)));
  };
  return SQUARE.map(out);
}

export function inside(shot, door) {
  const { cam, pose } = shot;
  const axes = seat(pose, door.axes);
  const local = (w) => axes.map((a) => dot(a, w));
  const m = cam.rot;
  const rot3 = mat3(local([m[0], m[1], m[2]]), local([m[3], m[4], m[5]]), local([m[6], m[7], m[8]]));
  return { eye: { pos: mul(local(sub(cam.pos, pose.at)), N / pose.size), rot: rot3, fov: cam.fov }, heading: local(pose.fwd), below: local(norm(mul(pose.at, -1))) };
}

export function shine(globe, at, sun) {
  const lit = (1 + dot(sun, norm(at))) / 2;
  return globe.light.map((c) => (c * SHINE * lit) / Math.max(1, dot(at, at)));
}

/* APPROACH */

export function entry(way) {
  const finite = (tau) => Number.isFinite(way.at(tau).pos[0]);
  let lo = -EARLY;
  let hi = 0;
  if (finite(lo) || !finite(hi)) lo = hi;
  for (let i = 0; i < HALVES && hi > lo; i++) {
    const mid = (lo + hi) / 2;
    if (finite(mid)) hi = mid;
    else lo = mid;
  }
  const state = { ...way.at(hi) };
  const probe = way.at(hi + PROBE);
  const rate = (a, b) => mul(sub(a, b), 1 / PROBE);
  return { tau: hi, state, speed: rate(probe.pos, state.pos), turn: rate(probe.fwd, state.fwd), tilt: rate(probe.up, state.up), bank: (probe.roll - state.roll) / PROBE, axes: basis(state.fwd, state.up) };
}

export function approach(rolled, door, a) {
  const { eye } = inside(leave(rolled, OPEN), door);
  const { state } = door;
  const cu = door.axes[1];
  const s = clamp(a / INTO);
  const k = ease.smooth(s);
  const bend = ((s ** 3 - s * s) * INTO) / 1000;
  const start = mul(cross(cu, eye.pos), -rolled.turn / 1000);
  const { p: pos } = hermite([{ t: 0, p: eye.pos, v: start }, { t: INTO, p: state.pos, v: mul(door.speed, 1 / 1000) }], a);
  const heading = rot(door.axes[2], cu, -twist(rolled.turn, OPEN + a));
  const roll = k * state.roll + bend * door.bank;
  const cam = aim(pos, add(pos, add(heading, mul(door.turn, bend))), add(cu, mul(door.tilt, bend)), roll);
  cam.fov = WIDE[1] + (DEEP - WIDE[1]) * k;
  const doors = (state.doors ?? []).map((one) => ({ ...one, glow: one.glow * k }));
  return { cam, roll, heading, frame: { ...state, doors, outside: true, hull: 1, sun: 1 - k, floor: N ** (1 - LEVELS - REVEAL * k), fog: (state.fog ?? 0) * k } };
}

const flying = (way, tau) => {
  const state = way.at(tau);
  const cam = aim(state.pos, add(state.pos, state.fwd), state.up, state.roll);
  cam.fov = DEEP;
  return { cam, roll: state.roll, frame: state };
};

/* SCENE */

const dolly = (cam, k) => (k === 1 ? cam : { pos: mul(cam.pos, k), rot: cam.rot, fov: cam.fov });

const outer = (door) => ({ ...door.state, doors: [], outside: true, hull: 1, sun: 1, floor: N ** (1 - LEVELS), fog: 0 });

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const length = value.length * 1000;
  const audio = opts.audio ?? null;
  const grade = GRADES[value.look];
  let trip = null;
  let veil = null;
  let point = null;
  let glow = null;
  try {
    trip = hyper(gl, view, { seed, sky: SKY, look: 'cloud', idle: 0, far: true, audio: null, mode: null, floor: 'cloud' });
    veil = program(gl, SHADE);
    point = program(gl, POINT);
    glow = ship(gl, view);
  } catch (error) {
    trip?.stop();
    veil?.drop();
    point?.drop();
    console.error(error);
    return blank(canvas);
  }
  const gov = governor(view, NOTCHES[tier(view)]);
  const timed = (r, start, span) => {
    const m = clock(span);
    Object.assign(r, { start, span, fire: start + m.fire, open: start + m.open, enter: start + m.enter, out: start + m.out, arrive: start + m.arrive, end: start + m.end, next: start + m.next, score: arrange(span) });
  };
  const begin = (loop, start, held = false) => {
    const r = { loop, rolled: roll(value, seed, loop), sent: new Set(), fired: false, exited: false, held, pending: false, blend: null, made: {}, tone: null, accent: null, song: null };
    timed(r, start, held ? Infinity : length);
    return r;
  };
  const drop = (r) => {
    for (const one of Object.values(r?.made ?? {})) one?.drop?.();
    if (r) r.made = {};
  };
  let run = begin(0, 0);
  let prev = null;
  let gone = false;
  let drawn = -Infinity;
  let busy = -Infinity;
  let down = -Infinity;
  let last = null;
  const at = (t, fn) => {
    const now = view.t;
    view.t = t;
    fn();
    view.t = now;
  };
  const build = (r, key, make) => {
    if (!(key in r.made)) {
      try {
        r.made[key] = make();
      } catch (error) {
        console.error(error);
        r.made[key] = null;
      }
    }
    return r.made[key];
  };
  const guard = (r, key, call) => {
    const one = r.made[key];
    if (!one) return;
    try {
      call(one);
    } catch (error) {
      console.error(error);
      one.drop();
      r.made[key] = null;
    }
  };
  const home = (r) => build(r, 'a', () => planet(gl, view, r.rolled.a));
  const there = (r, staged) => build(r, 'b', () => planet(gl, view, r.rolled.b, { staged }));
  const path = (r) =>
    build(r, 'route', () => {
      const way = route({ code: r.rolled.code, n: N }, seed);
      return { way, door: entry(way) };
    });
  const inner = (r) => build(r, 'march', () => march(gl, view, { code: r.rolled.code, n: N }, { look: value.look, toFrame: path(r)?.way.toFrame }));
  const next = (loop, start, held = false) => {
    drop(run);
    prev = run;
    run = begin(loop, start, held);
  };
  const advance = (t) => {
    for (let guard = 0; guard < LOOPS; guard++) {
      const r = run;
      if (!r.fired && t >= r.fire) {
        if (drawn < busy) at(r.fire, () => trip.draw());
        at(r.fire, () => trip.trigger());
        r.fired = true;
        busy = Infinity;
        continue;
      }
      if (r.fired && !r.exited && t >= r.out) {
        at(r.out, () => trip.exit());
        r.exited = true;
        busy = r.out + LEAVE;
        continue;
      }
      if (r.pending && t >= r.arrive) {
        next(r.loop + 1, r.arrive - HELD, true);
        continue;
      }
      if (t >= r.next) {
        next(r.loop + 1, r.next);
        continue;
      }
      return;
    }
  };
  const act = (t, r = run) => {
    if (t < r.fire) return 'depart';
    if (t < r.open) return 'jump';
    if (t < r.enter) return 'approach';
    if (t < r.out) return 'tunnel';
    if (t < r.arrive) return 'exit';
    if (t < r.end) return 'arrive';
    return 'black';
  };
  const arranged = (ms) => {
    const r = ms >= run.start || !prev ? run : prev;
    return r.score(ms - r.start);
  };
  const sounds = (t) => {
    if (!audio) return;
    for (const r of [prev, run]) {
      if (!r) continue;
      cues(r).forEach(([when, name, args], i) => {
        if (r.sent.has(i) || when - LEAD > t) return;
        r.sent.add(i);
        if (when >= t - STALE) audio.cue(name, when, args);
      });
    }
  };
  const tuned = (r) => {
    r.song ??= song(seed, r.loop);
    return r.song;
  };
  const paints = (r) => {
    const accent = view.look().accent;
    if (accent !== r.accent) {
      r.accent = accent;
      r.tone = tone('', r.rolled.paint, accent);
    }
    return r.tone;
  };
  const scale = () => (BUSY.includes(trip.stage()) ? gov.scale() : gov.tick());
  const ramp = (a) => {
    const k = ease.smooth(a / INTO);
    const raw = { ...RAW, ev: RAW.ev * (1 - ease.smooth(a / SETTLE)) };
    return Object.fromEntries(Object.entries(grade).map(([key, v]) => [key, mix(raw[key], v, k)]));
  };
  const body = (r, flight, eye, frame, lit) => {
    flight.draw(eye, frame, paints(r));
    const pose = { at: ORIGIN, axes: SQUARE, size: N, fwd: norm(lit.heading), plume: lit.plume, heat: lit.heat, fire: r.rolled.fire };
    return () => {
      glow.draw(eye, { ...pose, plume: 0 });
      flight.layer(1);
      glow.draw(eye, { ...pose, heat: 0 });
    };
  };
  const departure = (r, t) => {
    const shot = leave(r.rolled, t - r.start);
    const b = r.blend;
    if (b && t < b.at + RUSH) {
      const k = ease.smooth((t - b.at) / RUSH);
      const pos = mix(b.cam.pos, shot.cam.pos, k);
      const fwd = norm(mix([b.cam.rot[6], b.cam.rot[7], b.cam.rot[8]], [shot.cam.rot[6], shot.cam.rot[7], shot.cam.rot[8]], k));
      const fov = shot.cam.fov;
      shot.cam = aim(pos, add(pos, fwd), r.rolled.up);
      shot.cam.fov = mix(b.cam.fov, fov, k);
      shot.pose = { ...shot.pose, at: mix(b.ship, shot.pose.at, k), fwd: norm(mix(b.fwd, shot.pose.fwd, k)), turn: mix(b.turn, shot.pose.turn, k) };
    }
    last = { run: r, cam: shot.cam, ship: shot.pose.at, fwd: shot.pose.fwd, turn: shot.pose.turn };
    home(r);
    there(r, true)?.ensure();
    const way = path(r);
    const flight = inner(r);
    const px = flight && t >= r.open - FILM.flash ? flight.scale() : scale();
    let over = null;
    if (way && flight && shot.shown) {
      const { eye, heading, below } = inside(shot, way.door);
      over = body(r, flight, eye, { ...outer(way.door), shine: { color: shine(r.rolled.a, shot.pose.at, shot.sun), dir: below } }, { heading, plume: shot.pose.plume, heat: shot.pose.heat });
    }
    const beneath = (k, approach) => {
      guard(r, 'a', (globe) => globe.draw(dolly(shot.cam, approach), shot.sun, t, k));
      beacon(shot.cam, shot.goal, mul(r.rolled.b.light, BEACON), k);
    };
    trip.draw(shot.cam, beneath, over, px);
  };
  const tunnel = (r, t) => {
    const way = path(r);
    const flight = inner(r);
    there(r, true)?.ensure();
    if (!way || !flight) return trip.draw(aim(ORIGIN, FORWARD, UP), null, null, gov.scale(), grade);
    const a = t - r.open;
    const near = t < r.enter;
    const shot = near ? approach(r.rolled, way.door, a) : flying(way.way, way.door.tau + (t - r.enter) / 1000);
    const lit = { heading: shot.heading ?? FORWARD, plume: near ? 1 - ease.smooth(a / QUENCH) : 0, heat: 0 };
    const over = body(r, flight, shot.cam, shot.frame, lit);
    const ahead = aim(ORIGIN, r.rolled.depart.goal, r.rolled.up, shot.roll);
    ahead.fov = shot.cam.fov;
    trip.draw(ahead, null, over, flight.scale(), near ? ramp(a) : grade);
  };
  const arrival = (r, t) => {
    const shot = come(r.rolled, t - r.arrive, t, view);
    there(r, false);
    const way = path(r);
    const flight = inner(r);
    const px = scale();
    let over = null;
    if (way && flight) {
      const { eye, heading, below } = inside(shot, way.door);
      over = body(r, flight, eye, { ...outer(way.door), shine: { color: shine(r.rolled.b, shot.pose.at, shot.sun), dir: below } }, { heading, plume: shot.pose.plume, heat: shot.pose.heat });
    }
    const beneath = (k, approach) => guard(r, 'b', (globe) => globe.draw(dolly(shot.cam, approach), shot.sun, t, k));
    trip.draw(shot.cam, beneath, over, px);
  };
  const beacon = (cam, dir, color, k) => {
    const p = project(view, { pos: ORIGIN, rot: cam.rot, fov: cam.fov }, dir);
    if (!p || k <= 0) return;
    const port = gl.getParameter(gl.VIEWPORT);
    const sx = port[2] / view.w;
    const sy = port[3] / view.h;
    const at = [port[0] + p[0] * sx, port[1] + (view.h - p[1]) * sy];
    const pad = Math.ceil(DOT * (view.dpr || 1) * sx) + 2;
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(Math.floor(at[0]) - pad, Math.floor(at[1]) - pad, 2 * pad, 2 * pad);
    blend(gl, 'alpha');
    point.set({ uAt: at, uR: (DOT * (view.dpr || 1) * sx) / 2, uColor: color, uK: k });
    fill(gl);
    blend(gl, null);
    gl.disable(gl.SCISSOR_TEST);
  };
  const dark = () => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  };
  const shade = (k) => {
    if (k <= 0) return;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    blend(gl, 'alpha');
    veil.set({ uK: Math.min(k, 1) });
    fill(gl);
    blend(gl, null);
  };
  const draw = () => {
    if (gone) return;
    const t = view.still ? run.start + STILL : view.t;
    advance(t);
    const r = run;
    sounds(t);
    audio?.at(t, { name: r.score(t - r.start).name, song: tuned(r), arrange: arranged });
    const lifted = t < down + LEAVE;
    if (t >= r.end && !lifted) return dark();
    if (t >= r.open && t < r.out + VEIL) tunnel(r, t);
    else if (t >= r.out) arrival(r, t);
    else departure(r, t);
    drawn = view.t;
    shade(lifted ? 0 : 1 - Math.min(clamp((t - r.start) / FADE), clamp((r.end - t) / FADE)));
  };
  const hold = () => {
    if (gone || view.still) return;
    const t = view.t;
    const r = run;
    if (t < r.fire) {
      if (r.held) return;
      r.held = true;
      if (t - r.start < HELD) {
        r.blend = last?.run === r ? { at: t, ...last } : null;
        timed(r, t - HELD, Infinity);
      } else timed(r, r.start, Infinity);
      return;
    }
    if (t < r.out) {
      r.held = true;
      if (r.span !== Infinity) timed(r, r.start, Infinity);
      return;
    }
    if (t < r.arrive) {
      r.pending = true;
      return;
    }
    next(r.loop + 1, t - HELD, true);
  };
  const windDown = (onDone) => {
    if (gone || view.still) {
      onDone?.();
      return;
    }
    const t = view.t;
    const r = run;
    if (t < r.open) {
      Object.assign(r, { fired: true, exited: true, open: Infinity, enter: Infinity, out: Infinity, arrive: Infinity, end: Infinity, next: t + LEAVE });
    } else if (t < r.out) {
      timed(r, r.start, t - r.start + AFTER);
      r.exited = true;
    }
    busy = t + LEAVE;
    down = t;
    trip.windDown(onDone);
  };
  const again = () => {
    if (gone) return;
    const t = view.t;
    if (trip.stage() !== 'cruise') {
      trip.windDown();
      busy = t + LEAVE;
    }
    Object.assign(run, { sent: new Set(), fired: false, exited: false, held: false, pending: false, blend: null });
    timed(run, t, length);
    if (view.still) draw();
  };
  const stop = () => {
    if (gone) return;
    gone = true;
    drop(prev);
    drop(run);
    trip.stop();
    veil.drop();
    point.drop();
    glow.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, stop, hold, windDown, again, phase: trip.phase, stage: trip.stage, state: () => act(view.still ? run.start + STILL : view.t), loop: () => run.loop, rolled: () => run.rolled, section: () => arranged(view.t).name };
}
