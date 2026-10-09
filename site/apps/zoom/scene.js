import { tidy } from '../../lib/knobs.js';
import { rgb, rng } from '../../lib/scene.js';
import { bang, codes, gate, route } from '../../lib/space/bang.js';
import { look as aim, orbit, project } from '../../lib/space/camera.js';
import { deep } from '../../lib/space/deep.js';
import { blank, context, governor, tier } from '../../lib/space/gl2.js';
import { hud } from '../../lib/space/hud.js';
import { post } from '../../lib/space/post.js';
import { add } from '../../lib/space/vec.js';

export const SPEC = [
  { key: 'kind', label: 'Kind', kind: 'segment', def: 'bang', options: [['bang', 'Bang'], ['set', 'Set'], ['julia', 'Julia']], group: 'Shape' },
  { key: 'code', label: 'Code', kind: 'slider', def: 23, min: -1, max: 255, step: 1, group: 'Shape' },
  { key: 'n', label: 'N', kind: 'segment', def: 3, options: [[3, '3'], [5, '5']], group: 'Shape' },
  { key: 'auto', label: 'Auto', kind: 'toggle', def: 1, group: 'Flight' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.25, max: 2, step: 0.25, group: 'Flight' },
  { key: 'palette', label: 'Palette', kind: 'pick', def: '', options: [['', 'Seeded'], ['accent', 'Accent'], ['fire', 'Fire'], ['ice', 'Ice'], ['mono', 'Mono']], group: 'Look' },
];

const DIGITS = ['1', '2', '3', '4'];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Go', does: 'To the lit marker', act: 'go', button: true },
    { key: DIGITS, label: 'Pick a marker', act: 'pick' },
  ],
  actions: { go: (scene) => scene.go?.(), pick: (scene, e) => scene.pick?.(Number(e.key)) },
  record: true,
};

const NAMES = { 23: 'carpet', 232: 'net', 22: 'star', 129: 'void', 17: 'xtree', 5: 'ytree', 3: 'ztree' };

const RAMPS = {
  fire: ['#1c0400', '#9a2005', '#f06a12', '#ffd75e'],
  ice: ['#03111f', '#0d4f8c', '#47c3ee', '#e4fbff'],
  mono: ['#161616', '#5a5a5a', '#a2a2a2', '#ececec'],
};

const TAU = Math.PI * 2;
const FOV = 1.25;
const DESK = 1.2e6;
const PHONE = 0.35e6;
const NOTCHES = [1, 0.8, 0.64, 0.5];
const LAG = 100;
const HOLD = 0.45;
const EASE = 0.6;
const ALBEDO = 0.45;
const AWAY = 2.2;
const TILT = 0.45;
const SPIN = 0.15;
const NEAR = 0.03;
const LOOK = { ev: 0.2, bloom: 0.7, aberration: 0.0008, vignette: 0.4, grain: 0.012 };
const FLAT = { ev: 0, bloom: 0.3, aberration: 0, vignette: 0.3, grain: 0.008 };
const PLANE = { phone: 0.5, desk: 0.75 };
const SALT = 0x7a3c19e5;
const TONE = 0x1b873593;
const TURN = 0x51ed27;

const linear = (hex) => rgb(hex).map((v) => (v / 255) ** 2.2);

const blend = (a, b, k) => a.map((v, i) => v + (b[i] - v) * k);

function ramp(name, seed, accent) {
  if (RAMPS[name]) return RAMPS[name].map((hex) => linear(hex).map((v) => v * ALBEDO));
  if (name === 'accent') {
    const a = linear(accent);
    return [blend([0.02, 0.02, 0.02], a, 0.25), blend([0, 0, 0], a, 0.7), a, blend(a, [1, 1, 1], 0.6)].map((c) => c.map((v) => v * ALBEDO));
  }
  const rand = rng((seed ^ TONE) >>> 0);
  const c = [0, 1, 2].map(() => 0.5 + rand() * 0.6);
  const d = [0, 1, 2].map(() => rand());
  return [0.1, 0.4, 0.7, 1].map((x) => c.map((ci, i) => (0.5 + 0.5 * Math.cos(TAU * (ci * x + d[i]))) ** 2.2 * ALBEDO + 0.02));
}

const named = (code) => (NAMES[code] ? `${NAMES[code]} ${code}` : `bang ${code}`);

export function pickCode(value, seed) {
  if (value.code >= 0) return value.code;
  const pool = codes(value.n);
  return pool.length ? pool[Math.floor(rng((seed ^ SALT) >>> 0)() * pool.length)] : 23;
}

const complex = ([re, im]) => `${re.toFixed(3)}${im < 0 ? '-' : '+'}${Math.abs(im).toFixed(3)}i`;

function plane(canvas, gl, view, value, seed) {
  const made = [];
  let film;
  let space;
  let face;
  try {
    film = post(gl, view);
    made.push(film);
    space = deep(gl, view, { kind: value.kind, seed, palette: value.palette, speed: value.speed, auto: value.auto });
    made.push(space);
    face = hud(gl, view);
    made.push(face);
  } catch (error) {
    for (const one of made) one.drop();
    console.error(error);
    return blank(canvas);
  }
  const gov = governor(view, NOTCHES);
  const base = PLANE[tier(view) === 'phone' ? 'phone' : 'desk'];
  let clock = view.t;
  let shown = [];
  let said = [];
  const words = () => {
    const info = space.info();
    const lit = shown.find((one) => one.on);
    const head = value.kind === 'julia' ? `julia ${complex(info.c)}` : 'set';
    const depth = `depth 10^${Math.round(Math.log10(space.depth()))}`;
    if (info.phase === 'arrive') return [head, depth, `minibrot p ${info.goal}`];
    if (info.phase === 'fade') return [head, depth, 'again'];
    if (info.goal) return [head, depth, `minibrot p ${info.goal} ahead`];
    if (info.chosen && lit) return [head, depth, `mark ${lit.number}`];
    if (!value.auto) return [head, depth, 'pick a mark'];
    return [head, depth, lit ? `auto ${lit.number}` : 'auto'];
  };
  const draw = () => {
    const dt = view.t - clock;
    clock = view.t;
    const notch = dt > 0 && dt <= LAG ? gov.tick() : gov.scale();
    space.draw(film.begin(base * notch));
    film.end(FLAT);
    shown = space.marks();
    said = words();
    face.lines(said, 'tl');
    face.marks(shown);
    face.draw();
  };
  const choose = (id) => (id === null || id === undefined ? false : space.go(id));
  const go = () => choose(shown.find((one) => one.on)?.id);
  const pick = (number) => choose(shown.find((one) => one.number === Number(number))?.id);
  const tap = (x, y, k) => choose(face.pick(x, y, k));
  const stop = () => {
    for (const one of made) one.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, stop, go, pick, tap, marks: () => shown, said: () => said, depth: () => space.depth(), info: () => space.info() };
}

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  if (value.kind !== 'bang') return plane(canvas, gl, view, value, seed);
  const n = value.n;
  const code = pickCode(value, seed);
  const design = { code, n };
  const open = gate(code, n);
  const made = [];
  let film;
  let world;
  let face;
  try {
    film = post(gl, view);
    made.push(film);
    world = bang(gl, view, design);
    made.push(world);
    face = hud(gl, view);
    made.push(face);
  } catch (error) {
    for (const one of made) one.drop();
    console.error(error);
    return blank(canvas);
  }
  const path = open ? route(design, seed) : null;
  const gov = governor(view, NOTCHES);
  const cap = tier(view) === 'phone' ? PHONE : DESK;
  const phase = rng((seed ^ TURN) >>> 0)() * TAU;
  let flown = 0;
  let last = 0;
  let rate = 1;
  let clock = view.t;
  let tau = 0;
  let tint = null;
  let tone = null;
  let shown = [];
  let list = [];
  let said = [];
  const flight = () => {
    if (value.auto) return (view.t / 1000) * value.speed;
    const step = (Math.max(0, view.t - last) / 1000) * value.speed;
    last = view.t;
    const next = path.next();
    const want = next.chosen ? 1 : Math.min(1, Math.max(next.at - HOLD - flown, 0) / EASE);
    rate = want < rate ? want : Math.min(want, rate + step / EASE);
    flown += step * rate;
    for (let door = path.next(); flown >= door.at - HOLD; door = path.next()) {
      if (!door.chosen) {
        flown = door.at - HOLD;
        break;
      }
      path.at(door.at);
    }
    return flown;
  };
  const words = (state) => {
    const top = [named(code), `n ${n}${state ? `  level ${state.level}` : ''}`];
    if (!state) return [...top, 'no way in'];
    const lit = list.find((one) => one.on);
    if (!value.auto && !state.chosen) return [...top, 'pick a door'];
    return [...top, lit ? `${state.chosen ? 'door' : 'auto'} ${lit.number}` : 'auto'];
  };
  const draw = () => {
    const dt = view.t - clock;
    clock = view.t;
    const notch = dt > 0 && dt <= LAG ? gov.tick() : gov.scale();
    const scale = Math.min(1, Math.sqrt(cap / (view.w * view.h))) * notch;
    const accent = view.look().accent;
    if (accent !== tint) {
      tint = accent;
      tone = { accent: linear(accent), ramp: ramp(value.palette, seed, accent) };
    }
    let cam;
    let frame;
    let state = null;
    if (path) {
      tau = flight();
      state = path.at(tau);
      cam = aim(state.pos, add(state.pos, state.fwd), state.up, state.roll);
      cam.fov = FOV;
      frame = state;
      list = path.ahead(tau);
      shown = list.map((one) => {
        const at = project(view, cam, one.pos);
        return at && at[2] > state.scale * NEAR ? { id: one.id, x: at[0], y: at[1], label: String(one.number), on: one.on, dim: !one.seen, number: one.number } : null;
      }).filter(Boolean);
    } else {
      cam = orbit([0, 0, 0], AWAY * n, TILT, SPIN, phase, view.t);
      cam.fov = FOV * 0.8;
      frame = { outside: true, scale: AWAY * n, fog: 0, doors: [], hue: 0.25 };
      shown = [];
    }
    film.begin(scale);
    world.draw(cam, frame, tone);
    film.end(LOOK);
    said = words(state);
    face.lines(said, 'tl');
    face.marks(shown);
    face.draw();
  };
  const choose = (id) => {
    if (!path || id === null || id === undefined) return false;
    return path.choose(id);
  };
  const go = () => choose(list.find((one) => one.on)?.id);
  const pick = (number) => choose(Number(number) - 1);
  const tap = (x, y, k) => choose(face.pick(x, y, k));
  const stop = () => {
    for (const one of made) one.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, stop, go, pick, tap, code, open, marks: () => shown, said: () => said };
}
