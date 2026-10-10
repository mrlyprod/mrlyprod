import { tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { drift, ease, look } from '../../lib/space/camera.js';
import { blank, context, governor, tier } from '../../lib/space/gl2.js';
import { NAMES, planet, pole, world } from '../../lib/space/planet.js';
import { post } from '../../lib/space/post.js';
import { sky } from '../../lib/space/sky.js';
import { add, cross, dot, len, mul, norm, rot, sub } from '../../lib/space/vec.js';

const named = (word) => word[0].toUpperCase() + word.slice(1);

export const CAMS = ['orbit', 'drift', 'approach', 'flyby'];

export const SPEC = [
  { key: 'world', label: 'World', kind: 'pick', def: 'earth', options: NAMES.map((name) => [name, named(name)]), group: 'World' },
  { key: 'moons', label: 'Moons', kind: 'pick', def: -1, options: [[-1, 'Own'], [0, 'None'], [1, '1'], [2, '2'], [3, '3']], group: 'World' },
  { key: 'rings', label: 'Rings', kind: 'segment', def: '', options: [['', 'Auto'], ['on', 'On'], ['off', 'Off']], group: 'World' },
  { key: 'cam', label: 'Camera', kind: 'pick', def: 'orbit', options: CAMS.map((cam) => [cam, named(cam)]), group: 'View' },
  { key: 'sun', label: 'Sun', kind: 'slider', def: 60, min: 0, max: 180, step: 5, unit: 'deg', group: 'View' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0, max: 4, step: 0.25, unit: 'x', group: 'View' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Next world', act: 'next', button: true },
    { key: ['ArrowLeft', 'ArrowUp'], label: 'Previous', act: 'prev' },
    { key: ['ArrowRight', 'ArrowDown'], label: 'Next', act: 'next' },
  ],
  actions: { next: (scene) => scene.step?.(1), prev: (scene) => scene.step?.(-1) },
  record: true,
};

const TAU = Math.PI * 2;
const FOV = 1;
const LAT = 0.32;
const ORBIT = 3.2;
const LOOP = 90000;
const AFAR = 3.6;
const FAR = 12;
const NEAR = 2.4;
const CLOSE = 40000;
const FLY = 1.3;
const PASS = 30000;
const EPOCH = 0;
const RISE = 0.6;
const REACH = 2;
const HIGH = 0.6;
const DIP = 0.87;
const LIFT = 0.2;
const TERMINATOR = -0.35;
const NOTCHES = { phone: [0.75, 0.6, 0.5], desk: [1, 0.85, 0.7, 0.5], big: [0.75, 0.6, 0.5] };
const LOOK = { ev: 0, bloom: 0.55, vignette: 0.3, grain: 0.012, aberration: 0.0012, flare: 0.04 };
const SALT = 0x2f6b9d31;

export const next = (name, by) => NAMES[(NAMES.indexOf(name) + by + NAMES.length) % NAMES.length];

const tilt = (params, [x, y, z]) => {
  const c = Math.cos(params.tilt);
  const s = Math.sin(params.tilt);
  return [c * x - s * y, s * x + c * y, z];
};

const around = (params, dist, lat, az) => tilt(params, [dist * Math.cos(lat) * Math.cos(az), dist * Math.sin(lat), dist * Math.cos(lat) * Math.sin(az)]);

function fit(params, aspect) {
  if (!params.ring) return 0;
  const t = Math.tan(FOV / 2);
  const wide = (aspect >= 1 ? Math.atan(t * aspect) : FOV / 2) * 0.9;
  const tall = (aspect >= 1 ? FOV / 2 : Math.atan(t / aspect)) * 0.9;
  const out = params.ring.outer;
  for (let d = ORBIT; d < 40; d += 0.05) {
    const v = [out - d * Math.cos(LAT), -d * Math.sin(LAT)];
    if (Math.atan(out / d) < wide && Math.acos((d - out * Math.cos(LAT)) / Math.hypot(...v)) < tall) return d;
  }
  return 40;
}

function phase(cam, at, deg) {
  const to = norm(sub(cam.pos, at));
  const up = [cam.rot[3], cam.rot[4], cam.rot[5]];
  const axis = rot(norm(sub(up, mul(to, dot(up, to)))), to, TERMINATOR);
  return norm(rot(to, axis, (deg * Math.PI) / 180));
}

function flyby(params, az, t) {
  const c = around(params, 1, HIGH, az);
  const east = norm(cross(pole(params), c));
  const sun = norm(add(mul(east, Math.cos(DIP)), mul(c, -Math.sin(DIP))));
  const rise = (1 - FLY * Math.cos(DIP)) / Math.sin(DIP);
  const u = (((t % PASS) + PASS) % PASS) / PASS;
  const pos = add(mul(c, FLY), mul(east, rise + (u - RISE) * REACH));
  const up = norm(pos);
  const ahead = norm(sub(east, mul(up, dot(east, up))));
  const dip = Math.acos(1 / len(pos)) - LIFT;
  return { cam: look(pos, add(pos, add(mul(ahead, Math.cos(dip)), mul(up, -Math.sin(dip)))), up), sun };
}

function eye(params, cam, seed, start, room, t) {
  if (cam === 'drift') {
    return { pos: add(around(params, Math.max(AFAR, room), LAT, start + t / 100000), drift(seed, t, 0.25)), at: drift(seed + 1, t, 0.05) };
  }
  if (cam === 'approach') {
    const u = (((t % CLOSE) + CLOSE) % CLOSE) / CLOSE;
    return { pos: around(params, FAR * (NEAR / FAR) ** ease.out(u, 2), LAT * 0.6, start + t / 200000), at: [0, 0, 0] };
  }
  return { pos: around(params, Math.max(ORBIT, room), LAT, start + (TAU * t) / LOOP), at: [0, 0, 0] };
}

export function shot(params, { cam = 'orbit', sun = 60, seed = 0, t = 0, aspect = 16 / 9 } = {}) {
  const start = rng(((seed >>> 0) ^ SALT) >>> 0)() * TAU;
  if (cam === 'flyby') return flyby(params, start, t);
  const room = fit(params, aspect);
  const view = (when) => {
    const { pos, at } = eye(params, cam, seed, start, room, when);
    return { cam: look(pos, at, pole(params)), at };
  };
  const first = view(EPOCH);
  return { cam: view(t).cam, sun: phase(first.cam, first.at, sun) };
}

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const params = world(value.world, seed, { moons: value.moons, rings: value.rings });
  const level = tier(view);
  let film;
  let field;
  let body;
  try {
    film = post(gl, view);
    field = sky(gl, view, { seed, density: 0.7, dust: 0.6 });
    body = planet(gl, view, params);
  } catch (error) {
    console.error(error);
    return blank(canvas);
  }
  const gov = governor(view, NOTCHES[level]);
  const look = level === 'phone' ? { ...LOOK, flare: 0 } : LOOK;
  let seen = opts;
  let now = value;
  let last = 0;
  let spun = 0;
  const knobs = () => {
    const live = opts.live?.current ?? opts;
    if (live !== seen) {
      seen = live;
      now = tidy(SPEC, live);
    }
    return now;
  };
  const draw = () => {
    const { cam: kind, sun: deg, speed } = knobs();
    spun += (view.t - last) * speed;
    last = view.t;
    film.begin(gov.tick());
    const { cam, sun } = shot(params, { cam: kind, sun: deg, seed, t: view.t, aspect: view.w / view.h });
    field.draw(cam, { k: 0.5 });
    body.draw(cam, sun, spun);
    film.end(look);
  };
  const stop = () => {
    body.drop();
    field.drop();
    film.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, stop };
}
