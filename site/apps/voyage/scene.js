import { letters } from '../../kit/font/font.js';
import { tidy } from '../../lib/knobs.js';
import { rgb, rng } from '../../lib/scene.js';
import { bang, ride, totem } from '../../lib/space/bang.js';
import { ease, look as aim, project } from '../../lib/space/camera.js';
import { blank, blend, context, governor, program, quads, tier } from '../../lib/space/gl2.js';
import { FILM, hyper } from '../../lib/space/hyper.js';
import { hud, safe } from '../../lib/space/hud.js';
import { planet, world } from '../../lib/space/planet.js';
import { add, cross, dot, len, mul, norm, rot, sub } from '../../lib/space/vec.js';
import { cubes, deck, doors, inside, sky, visit } from './cube.js';
import { LANDING, beat, exit, song } from './score.js';
import { kind, system } from './system.js';

export const SPEC = [
  { key: 'worlds', label: 'Worlds', kind: 'slider', def: 5, min: 3, max: 9, step: 1, group: 'System' },
  { key: 'bangs', label: 'Bang worlds', kind: 'slider', def: 2, min: 0, max: 4, step: 1, group: 'System' },
  { key: 'mode', label: 'Mode', kind: 'segment', def: 'play', options: [['play', 'Play'], ['auto', 'Auto']], group: 'Play' },
  { key: 'sound', label: 'Sound', kind: 'toggle', def: 0, group: 'Sound' },
  { key: 'music', label: 'Music', kind: 'toggle', def: 1, group: 'Sound' },
  { key: 'look', label: 'Look', kind: 'segment', def: 'cloud', options: [['cloud', 'Cloud'], ['classic', 'Classic']], group: 'Look' },
  { key: 'hud', label: 'HUD', kind: 'toggle', def: 1, group: 'Look' },
  { key: 'totem', label: 'Totem', kind: 'slider', def: -1, min: -1, max: 255, step: 1, group: 'Look' },
];

const DIGITS = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Jump', does: 'To the picked world', act: 'jump', button: true },
    { key: ['ArrowLeft', 'ArrowUp'], label: 'Previous', act: 'prev' },
    { key: ['ArrowRight', 'ArrowDown'], label: 'Next', act: 'next' },
    { key: DIGITS, label: 'Pick', does: 'A world, or a door inside a bang world', act: 'pick' },
    { key: 'm', label: 'Sound', act: 'mute' },
  ],
  actions: { jump: (scene) => scene.jump?.(), prev: (scene) => scene.step?.(-1), next: (scene) => scene.step?.(1), pick: (scene, e) => scene.pick?.(Number(e.key)), mute: (scene) => scene.mute?.() },
  record: true,
};

export const quiet = ({ sound, music, ...rest }) => rest;

const CHARGE = 1200;
const RUSH = 400;
const JUMP = FILM.wind + FILM.stretch + FILM.pile + FILM.flash;
const OPEN = CHARGE + JUMP;
const WHITE = FILM.bloom + FILM.white;
const LAND = FILM.bloom + FILM.white + FILM.decay + FILM.snap;
const OPENING = 9000;
const ARRIVE = 4000;
const STAY = [14, 22];
const DEEP = [20, 32];
const HOLD = [2.5, 1.2, 7];
const FAR = 6000;
const UNSEEN = 4;
const ORBIT = 3.2;
const CLOSE = 2.6;
const CLOSING = 18;
const RISE = 8;
const SPIN = [0.05, 0.12];
const RAMP = 1;
const LIFT = 0.22;
const SAFE = 0.35;
const SHY = (15 * Math.PI) / 180;
const LEAD = 120;
const DENSITY = 0.8;
const DUST = 0.4;
const SPOT = 2;
const GIANT = 3;
const GAIN = 2.5;
const RING = 16;
const SMALL = 3;
const GAP = 6;
const PITCH = 8;
const BURN = 0.15;
const TAG = -1000;
const MOST = 16;
const WARM = 0.5;
const CUBE = 6;
const TWIRL = 0.4;
const STAR = 2.5;
const BLAZE = 30;
const SHINE = 0.9;
const GLOW = 1.6;
const DOOR = 100;
const LAMP = [1.5, 0.035];
const NOTCHES = { phone: [0.75, 0.6, 0.5], desk: [1, 0.85, 0.7, 0.5], big: [0.75, 0.6, 0.5] };
const BUSY = ['flash', 'tunnel', 'bloom'];
const SALT = 0x2a9f53c1;
const GOLD = 0x9e3779b1;
const TONES = { terran: [0.32, 0.5, 0.78], desert: [0.82, 0.56, 0.34], ice: [0.86, 0.9, 0.96], lava: [0.62, 0.26, 0.14], gas: [0.86, 0.76, 0.6] };

const BALL = `#version 300 es
in vec4 aDisc;
in vec4 aTone;
in vec4 aSun;
uniform vec2 uRes;
out vec2 vAt;
out float vR;
out vec3 vTone;
out vec3 vSun;
void main() {
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1)) * 2.0 - 1.0;
  float reach = aDisc.z + 1.5;
  vec2 p = aDisc.xy + c * reach;
  vAt = c * reach;
  vR = aDisc.z;
  vTone = aTone.rgb;
  vSun = aSun.xyz;
  gl_Position = vec4(p.x / uRes.x * 2.0 - 1.0, 1.0 - p.y / uRes.y * 2.0, 0.0, 1.0);
}
`;

const LIT = `#version 300 es
precision highp float;
in vec2 vAt;
in float vR;
in vec3 vTone;
in vec3 vSun;
uniform float uK;
out vec4 o;
void main() {
  float a = clamp(vR + 0.5 - length(vAt), 0.0, 1.0);
  vec2 q = vAt / vR;
  vec3 n = vec3(q.x, -q.y, -sqrt(max(1.0 - dot(q, q), 0.0)));
  float lit = max(dot(n, vSun), 0.0);
  o = vec4(vTone * (0.03 + lit), 1.0) * a * uK;
}
`;

const clamp1 = (v) => Math.min(Math.max(v, -1), 1);

const axes = (cam) => {
  const m = cam.rot;
  return [[m[0], m[1], m[2]], [m[3], m[4], m[5]], [m[6], m[7], m[8]]];
};

function travel(a, b) {
  if (a.kind === 'bang' || b.kind === 'bang') return FAR;
  const au = len(sub(a.pos, b.pos));
  return Math.min(HOLD[0] + HOLD[1] * Math.log2(1 + au), HOLD[2]) * 1000;
}

function radius(sec) {
  return ORBIT + (CLOSE - ORBIT) * ease.smooth(sec / CLOSING);
}

const START = Math.asin(1 / radius(RISE)) - RISE * SPIN[0];

const turned = (sec) => {
  const late = Math.max(sec - RISE, 0);
  const s = Math.min(late / RAMP, 1);
  return SPIN[0] * sec + (SPIN[1] - SPIN[0]) * (RAMP * (s ** 3 - s ** 4 / 2) + Math.max(late - RAMP, 0));
};

export function orbit(body, u) {
  const s = norm(mul(body.pos, -1));
  const w = norm(sub([0, 1, 0], mul(s, s[1])));
  const sec = Math.max(u, 0) / 1000;
  const r = radius(sec);
  const th = START + turned(sec);
  const pos = add(mul(s, -r * Math.cos(th)), mul(w, r * Math.sin(th)));
  const up = add(mul(s, Math.sin(th)), mul(w, Math.cos(th)));
  return { pos, cam: aim(pos, mul(up, r * Math.tan(LIFT * ease.smooth(sec / RISE))), up) };
}

function shy(d, sun) {
  const c = dot(d, sun);
  if (c <= Math.cos(SHY)) return d;
  let side = sub(d, mul(sun, c));
  if (len(side) < 1e-9) side = cross(sun, Math.abs(sun[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0]);
  return add(mul(sun, Math.cos(SHY)), mul(norm(side), Math.sin(SHY)));
}

function charged(from, target, sun) {
  const [, up0, f0] = axes(from.cam);
  const r = len(from.pos);
  const ph = norm(from.pos);
  if (!target) return { r, ph, q: up0, phi: 0, f0, up0, axis: up0, angle: 0 };
  const d = shy(target, sun);
  const a = dot(ph, d);
  let q = sub(d, mul(ph, a));
  if (len(q) < 1e-6) q = up0;
  q = norm(q);
  const phi = a >= SAFE ? 0 : Math.acos(clamp1(a)) - Math.acos(SAFE);
  let axis = cross(f0, d);
  if (len(axis) < 1e-6) axis = up0;
  return { r, ph, q, phi, f0, up0, axis: norm(axis), angle: Math.acos(clamp1(dot(f0, d))) };
}

function charging(path, k) {
  const pos = mul(add(mul(path.ph, Math.cos(k * path.phi)), mul(path.q, Math.sin(k * path.phi))), path.r);
  const f = rot(path.f0, path.axis, k * path.angle);
  const up = rot(path.up0, path.axis, k * path.angle);
  return { pos, cam: aim(pos, add(pos, f), up) };
}

function pickOf(rand, bodies, here, seen) {
  const weights = bodies.map((one) => (one.id === here ? 0 : seen.has(one.id) ? 1 : UNSEEN));
  let r = rand() * weights.reduce((sum, w) => sum + w, 0);
  for (const one of bodies) {
    if (r < weights[one.id]) return one.id;
    r -= weights[one.id];
  }
  return bodies.find((one) => one.id !== here)?.id ?? here;
}

function plan(seed, bodies) {
  const rand = rng(((seed >>> 0) ^ SALT) >>> 0);
  const seen = new Set();
  const first = Math.floor(rand() * bodies.filter((one) => one.kind !== 'bang').length);
  const leg = (here, at) => {
    seen.add(here);
    const [lo, hi] = bodies[here].kind === 'bang' ? DEEP : STAY;
    const stay = (lo + rand() * (hi - lo)) * 1000;
    return { here, at, stay, pick: pickOf(rand, bodies, here, seen), chosen: false, to: null, go: Infinity, charge: CHARGE, fire: Infinity, open: Infinity, out: Infinity, land: Infinity, held: false, begun: false, fired: false, opened: false, exited: false, path: null };
  };
  return { first, leg };
}

function bright(color) {
  const lin = color.map((c) => c ** 2.2);
  const top = Math.max(...lin);
  return lin.map((c) => (1 - WARM) + (WARM * c) / top);
}

const linear = (hex) => rgb(hex).map((v) => (v / 255) ** 2.2);

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const auto = value.mode === 'auto' || !opts.player;
  const sys = system(seed, { worlds: value.worlds, bangs: value.bangs });
  const { star, bodies } = sys;
  const light = bright(star.color);
  const audio = opts.audio ?? null;
  const score = song(seed);
  const route = plan(seed, bodies);
  const made = new Map();
  const parts = [];
  let trip;
  let face;
  let tags;
  let shade;
  let spots;
  let balls;
  let boxes;
  let screen = null;
  let tint = null;
  let tone = null;
  const deep = (id) => bodies[id]?.kind === 'bang';
  const build = (id, staged) => {
    const one = bodies[id];
    if (one.kind === 'bang') return bang(gl, view, { code: one.code, n: one.n });
    const params = world('exo', one.world);
    params.light = light;
    return planet(gl, view, params, { staged });
  };
  try {
    trip = hyper(gl, view, { seed, sky: { density: DENSITY, dust: DUST }, look: value.look, idle: 0, audio, mode: null });
    parts.push(trip);
    face = hud(gl, view);
    parts.push(face);
    tags = hud(gl, view);
    parts.push(tags);
    shade = hud(gl, view);
    parts.push(shade);
    spots = program(gl, LIT, BALL);
    parts.push(spots);
    balls = quads(gl, { aDisc: 4, aTone: 4, aSun: 4 }, MOST);
    parts.push(balls);
    boxes = cubes(gl);
    parts.push(boxes);
    made.set(route.first, build(route.first));
  } catch (error) {
    for (const one of parts) one.stop ? one.stop() : one.drop();
    console.error(error);
    return blank(canvas);
  }
  let idol = null;
  if (value.totem >= 0) {
    try {
      idol = totem(gl, view, { code: value.totem, n: 3, seed });
    } catch (error) {
      console.error(error);
    }
  }
  const lamp = { color: light.map((c) => c * LAMP[0]), ambient: light.map((c) => c * LAMP[1]) };
  const gov = governor(view, NOTCHES[tier(view)]);
  const ensure = (id, staged = false) => {
    if (made.has(id)) return made.get(id);
    let one = null;
    try {
      one = build(id, staged);
    } catch (error) {
      console.error(error);
    }
    made.set(id, one);
    return one;
  };
  const guard = (id, call) => {
    const one = made.get(id);
    if (!one) return;
    try {
      call(one);
    } catch (error) {
      console.error(error);
      one.drop();
      made.set(id, null);
    }
  };
  const keep = (ids) => {
    for (const [id, one] of made) {
      if (ids.includes(id)) continue;
      one?.drop();
      made.delete(id);
    }
  };
  const stage = () => {
    if (screen) return screen;
    try {
      screen = deck(gl, view);
    } catch (error) {
      console.error(error);
      screen = false;
    }
    return screen;
  };
  const accent = () => {
    const hex = view.look().accent;
    if (hex !== tint) {
      tint = hex;
      tone = linear(hex);
    }
    return tone;
  };
  let count = 0;
  const enter = (id, n) => visit(bodies[id], (seed ^ Math.imul(n + 1, GOLD) ^ bodies[id].code) >>> 0);
  const launch = (leg, t, to, charge = CHARGE) => {
    leg.to = to;
    leg.go = t;
    leg.charge = charge;
    leg.fire = t + charge;
    leg.open = leg.fire + JUMP;
    leg.out = leg.held ? Infinity : exit(leg.open, travel(bodies[leg.here], bodies[to]), FILM.tunnel);
    leg.land = leg.out + LAND;
    leg.path = null;
    leg.visit = null;
  };
  const begin = (here, at, inner = null) => {
    const leg = route.leg(here, at);
    leg.n = count++;
    leg.inner = deep(here) ? (inner ?? enter(here, leg.n)) : null;
    if (auto) launch(leg, beat(at + leg.stay + OPEN) - OPEN, leg.pick);
    return leg;
  };
  const next = (leg) => {
    if (!deep(leg.to)) return null;
    leg.visit ??= enter(leg.to, leg.n + 1);
    return leg.visit;
  };
  let leg = begin(route.first, -OPENING);
  let gone = false;
  let drawn = -Infinity;
  let landed = -Infinity;
  let frame = null;
  let shown = [];
  let said = [];
  let told = '';
  const due = [];
  const widths = new Map();
  const at = (t, fn) => {
    const now = view.t;
    view.t = t;
    fn();
    view.t = now;
  };
  const later = (t, name, args = {}) => {
    if (audio) due.push([t, name, args]);
  };
  const state = (t = view.t) => {
    if (t < leg.go) return t - leg.at < ARRIVE ? 'arrive' : 'stay';
    if (t < leg.fire) return 'charge';
    if (t < leg.open) return 'jump';
    if (t < leg.out) return 'tunnel';
    return 'exit';
  };
  const moving = (t = view.t) => t >= leg.go;
  const mood = (t) => {
    if (t < leg.go) return deep(leg.here) ? 'acid' : t - leg.at < LANDING ? 'land' : 'drift';
    if (t < leg.open) return 'build';
    if (t < leg.out) return 'drive';
    if (t < leg.land) return 'drop';
    return deep(leg.to) ? 'acid' : 'land';
  };
  const tell = () => {
    const now = { here: leg.here, pick: leg.pick, moving: moving() };
    const key = `${now.here} ${now.pick} ${now.moving}`;
    if (key === told) return;
    told = key;
    opts.onState?.(now);
  };
  const arrive = (t) => {
    landed = t;
    leg = begin(leg.to, t, next(leg));
    keep([leg.here]);
  };
  const advance = (t) => {
    for (let tries = 0; tries < 100000; tries++) {
      if (!leg.begun && t >= leg.go) {
        leg.begun = true;
        if (t < leg.land && !deep(leg.to)) ensure(leg.to, true);
        continue;
      }
      if (!leg.fired && t >= leg.fire) {
        if (landed > drawn) at(leg.fire, () => trip.draw());
        at(leg.fire, () => trip.trigger());
        leg.fired = true;
        continue;
      }
      if (leg.fired && !leg.opened && t >= leg.open) {
        leg.opened = true;
        keep([leg.to]);
        if (deep(leg.to)) ensure(leg.to);
        next(leg);
        continue;
      }
      if (leg.opened && !leg.exited && t >= leg.out) {
        at(leg.out, () => trip.exit());
        leg.exited = true;
        later(leg.land, 'arrive');
        continue;
      }
      if (leg.exited && t >= leg.land) {
        arrive(leg.land);
        continue;
      }
      return;
    }
  };
  const finish = (t) => {
    if (leg.opened && t < leg.land) guard(leg.to, (one) => one.ensure?.());
  };
  const within = (id, inner, u) => {
    const shot = inside(inner, u);
    return { body: id, pos: null, cam: sky(inner, shot.cam), inner: shot, visit: inner };
  };
  const shot = (t) => {
    const here = bodies[leg.here];
    if (t < leg.go) return deep(leg.here) ? within(leg.here, leg.inner, t - leg.at) : { body: leg.here, ...orbit(here, t - leg.at) };
    if (!leg.exited || t < leg.out + WHITE) {
      if (deep(leg.here)) return within(leg.here, leg.inner, Math.min(t, leg.open) - leg.at);
      leg.path ??= charged(orbit(here, leg.go - leg.at), leg.to === leg.here ? null : norm(sub(bodies[leg.to].pos, here.pos)), sunOf(leg.here));
      return { body: leg.here, ...charging(leg.path, ease.smooth((t - leg.go) / leg.charge)) };
    }
    if (deep(leg.to)) return within(leg.to, next(leg), t - leg.land);
    return { body: leg.to, ...orbit(bodies[leg.to], 0) };
  };
  const toward = (from, to) => norm(sub(bodies[to].pos, bodies[from].pos));
  const sunOf = (id) => norm(mul(bodies[id].pos, -1));
  const unit = () => view.dpr || 1;
  const paint = (body, cam, k) => {
    const sky = { pos: [0, 0, 0], rot: cam.rot, fov: cam.fov };
    const [right, up, fwd] = axes(cam);
    const local = (s) => [dot(s, right), dot(s, up), dot(s, fwd)];
    const port = gl.getParameter(gl.VIEWPORT);
    const res = [port[2], port[3]];
    const sx = res[0] / view.w;
    const sy = res[1] / view.h;
    const data = balls.data;
    const lit = [];
    let n = 0;
    const disc = (p, r, color, sun) => {
      if (n >= MOST) return;
      data.set([p[0] * sx, p[1] * sy, r * sx, 0, ...color, 1, ...sun, 0], n * 12);
      n += 1;
    };
    if (deep(body)) {
      const p = project(view, sky, sunOf(body));
      if (p) disc(p, STAR * unit(), light.map((c) => c * BLAZE), [0, 0, -1]);
    }
    for (const one of bodies) {
      if (one.id === body) continue;
      const p = project(view, sky, toward(body, one.id));
      if (!p) continue;
      const s = local(sunOf(one.id));
      if (one.kind === 'bang') lit.push([p[0] * sx, p[1] * sy, CUBE * unit() * sx, (view.t / 1000) * TWIRL + one.id, ...light.map((c) => c * SHINE), 1, ...s, 0]);
      else disc(p, (one.type === 'gas' ? GIANT : SPOT) * unit(), TONES[one.type].map((c, i) => c * light[i] * GAIN), s);
    }
    spots.set({ uRes: res, uK: k });
    balls.draw(spots, n, 'alpha');
    blend(gl, null);
    if (lit.length) boxes.draw(lit, res, k, accent().map((c) => c * GLOW));
  };
  const beneath = (k, approach) => {
    if (!frame || k <= 0) return;
    if (frame.inner) {
      paint(frame.body, frame.cam, k);
      const world = ensure(frame.body);
      const out = world && stage();
      if (out) out.draw(world, frame.visit, frame.inner, k, accent(), approach);
      return;
    }
    const cam = { pos: mul(frame.pos, approach), rot: frame.cam.rot, fov: frame.cam.fov };
    paint(frame.body, cam, k);
    ensure(frame.body);
    guard(frame.body, (world) => world.draw(cam, sunOf(frame.body), view.t, k));
  };
  const over = (cam) => idol.draw(cam, ...ride(trip, cam, { ...lamp, dir: sunOf(frame.body) }));
  const bounds = (lines) => {
    const edges = safe(view);
    const ring = RING * unit();
    const size = Math.max(1, Math.round(SMALL * unit()));
    return { left: edges.side + ring, right: view.w - edges.side - ring, top: edges.top + lines * size * PITCH + ring, bottom: view.h - edges.bottom - ring };
  };
  const glare = (cam) => {
    const p = project(view, { pos: [0, 0, 0], rot: cam.rot, fov: cam.fov }, sunOf(leg.here));
    return p && { x: p[0], y: p[1], r: (BURN * Math.min(view.w, view.h)) / (2 * Math.tan(cam.fov / 2)) };
  };
  const edge = (cam, dir, area) => {
    const [right, up, fwd] = axes(cam);
    const x = dot(dir, right);
    const y = dot(dir, up);
    const z = dot(dir, fwd);
    const sky = { pos: [0, 0, 0], rot: cam.rot, fov: cam.fov };
    const p = z > 0 ? project(view, sky, dir) : null;
    if (p && p[0] >= 0 && p[0] <= view.w && p[1] >= 0 && p[1] <= view.h) return [Math.min(Math.max(p[0], area.left), area.right), Math.min(Math.max(p[1], area.top), area.bottom), false];
    const span = Math.hypot(x, y) || 1;
    const dx = x / span;
    const dy = -y / span;
    const cx = (area.left + area.right) / 2;
    const cy = (area.top + area.bottom) / 2;
    const room = Math.min((area.right - cx) / Math.max(Math.abs(dx), 1e-6), (area.bottom - cy) / Math.max(Math.abs(dy), 1e-6));
    return [cx + dx * room, cy + dy * room, true];
  };
  const behind = (pos, dir) => {
    if (!pos) return false;
    const b = dot(pos, dir);
    return b < 0 && dot(pos, pos) - b * b < 1;
  };
  const wide = (name) => {
    if (!widths.has(name)) widths.set(name, letters(name).cols);
    return widths.get(name);
  };
  const spread = (list, area, sun) => {
    const ring = RING * unit();
    const room = 2 * ring + GAP * unit();
    const size = Math.max(1, Math.round(SMALL * unit()));
    const placed = [];
    for (const one of [...list].sort((a, b) => a.y - b.y || a.x - b.x)) {
      const span = wide(one.name) * size + ring + GAP * unit();
      const right = one.x + ring + span <= view.w;
      const box = right ? [one.x - ring, one.x + ring + span] : [one.x - ring - span, one.x + ring];
      const near = (y) => placed.find((p) => Math.abs(p.y - y) < room && p.box[0] < box[1] && box[0] < p.box[1]);
      const hot = (y) => sun && Math.hypot(Math.min(Math.max(sun.x, box[0]), box[1]) - sun.x, Math.min(Math.max(sun.y, y - room / 2), y + room / 2) - sun.y) < sun.r;
      let y = one.y;
      if (hot(y)) y = sun.y + sun.r + room / 2 <= area.bottom ? sun.y + sun.r + room / 2 : Math.max(area.top, sun.y - sun.r - room / 2);
      for (let hit = near(y), tries = 0; hit && tries < list.length; hit = near(y), tries++) y = hit.y + room;
      if (y > area.bottom) y = Math.max(area.top, Math.min(...placed.map((p) => p.y), one.y) - room);
      placed.push({ ...one, y, box, right, half: (wide(one.name) * size) / 2 });
    }
    return placed.sort((a, b) => a.id - b.id);
  };
  const marks = (t, lines) => {
    const now = state(t);
    if (!['stay', 'charge'].includes(now)) return [];
    if (now === 'stay' && frame.inner) return doors(frame.visit, frame.inner, view).map((one) => ({ ...one, id: DOOR + one.id }));
    const list = now === 'charge' ? [leg.to] : bodies.map((one) => one.id).filter((id) => id !== leg.here);
    const area = bounds(lines);
    return spread(
      list.map((id) => {
        const dir = toward(leg.here, id);
        const [x, y, off] = edge(frame.cam, dir, area);
        return { id, x, y, label: String(id + 1), on: id === (now === 'charge' ? leg.to : leg.pick), dim: off || behind(frame.pos, dir), name: bodies[id].name };
      }),
      area,
      glare(frame.cam),
    );
  };
  const names = (list) =>
    list
      .filter((one) => one.name)
      .map((one) => {
        const reach = (RING + GAP) * unit() + one.half;
        return { id: one.id, x: one.x + (one.right ? reach : -reach), y: one.y, r: TAG, label: one.name, dim: one.dim };
      });
  const gap = (a, b) => (a.kind === 'bang' || b.kind === 'bang' ? 'deep space' : `${len(sub(b.pos, a.pos)).toFixed(1)} au`);
  const words = (t) => {
    const now = state(t);
    const top = `system ${star.name}`;
    if (moving(t)) {
      const dest = bodies[leg.to];
      const left = leg.land - t;
      return [top, `${dest.name} ${kind(dest)}`, Number.isFinite(left) ? `hyperspace eta ${(Math.max(left, 0) / 1000).toFixed(1)}` : 'hyperspace'];
    }
    const here = bodies[leg.here];
    const pick = bodies[leg.pick];
    const level = frame?.inner?.level;
    const second = `${here.name} ${kind(here)}${level === undefined ? '' : ` level ${level}`}`;
    const landing = here.kind === 'bang' ? `far beyond ${star.name}` : `orbit ${here.a.toFixed(1)} au from ${star.name}`;
    const third = now === 'arrive' ? landing : !auto && !leg.chosen ? 'pick a destination' : `${gap(here, pick)} to ${pick.name}`;
    return [top, second, third];
  };
  const sounds = (t) => {
    if (!audio) return;
    due.sort((a, b) => a[0] - b[0]);
    while (due.length && due[0][0] - LEAD <= t) {
      const [when, name, args] = due.shift();
      audio.cue(name, when, args);
    }
  };
  const draw = () => {
    if (gone) return;
    const t = view.t;
    advance(t);
    frame = shot(t);
    finish(t);
    audio?.at(t, { name: mood(t + LEAD), flicker: trip.flow().flicker, song: score });
    const scale = BUSY.includes(trip.stage()) || frame.inner ? gov.scale() : gov.tick();
    trip.draw(frame.cam, beneath, idol ? over : null, scale);
    drawn = t;
    sounds(t);
    if (value.hud) {
      said = words(t);
      shown = marks(t, said.length);
      face.lines(said, 'tl');
      face.marks(shown);
      const labels = names(shown);
      const fall = Math.max(1, Math.round(SMALL * unit()));
      tags.marks(labels);
      shade.marks(labels.map((one) => ({ ...one, x: one.x + fall, y: one.y + fall, on: true })));
      face.draw();
      shade.draw();
      tags.draw();
    } else {
      shown = [];
      said = [];
    }
    tell();
  };
  const teleport = (to) => {
    leg = begin(to, deep(to) ? view.t : view.t - OPENING);
    keep([to]);
    ensure(to);
    draw();
  };
  const jump = () => {
    if (gone || moving()) return false;
    if (view.still) {
      teleport(leg.pick);
      return true;
    }
    launch(leg, view.t, leg.pick);
    tell();
    return true;
  };
  const select = (id) => {
    if (gone || moving() || !bodies[id] || id === leg.here) return false;
    leg.pick = id;
    leg.chosen = true;
    if (auto) launch(leg, leg.go, id);
    audio?.cue('tick', view.t, {});
    tell();
    return true;
  };
  const door = (id) => {
    if (gone || moving() || !leg.inner || !leg.inner.path.choose(id)) return false;
    audio?.cue('tick', view.t, {});
    return true;
  };
  const choose = (id) => (id === leg.pick && leg.chosen ? jump() : select(id));
  const step = (by) => {
    const list = bodies.map((one) => one.id).filter((id) => id !== leg.here);
    const at = Math.max(0, list.indexOf(leg.pick));
    return select(list[(at + by + list.length) % list.length]);
  };
  const pick = (number) => (leg.inner && !moving() ? door(Number(number) - 1) : select(Number(number) - 1));
  const tap = (x, y, k) => {
    const id = face.pick(x, y, k) ?? tags.pick(x, y, k);
    if (id === null) return false;
    return id >= DOOR ? door(id - DOOR) : choose(id);
  };
  const holding = () => {
    if (gone || view.still) return;
    leg.held = true;
    if (!moving()) launch(leg, view.t, leg.pick, RUSH);
    else if (!leg.exited) {
      leg.out = Infinity;
      leg.land = Infinity;
    }
  };
  const windDown = (onDone) => {
    if (gone || view.still) {
      onDone?.();
      return;
    }
    const t = view.t;
    if (!leg.exited) {
      if (!leg.opened) {
        if (!deep(leg.here)) leg.path = charged(shot(t), null);
        leg.to = leg.here;
        keep([leg.here]);
        leg.visit = null;
        leg.go = Math.min(leg.go, t);
        leg.fire = Math.min(leg.fire, t);
      }
      leg.fired = true;
      leg.opened = true;
      leg.open = Math.min(leg.open, t);
      leg.out = t;
      leg.land = t + LAND;
      leg.exited = true;
      later(leg.land, 'arrive');
    }
    trip.windDown(onDone);
  };
  const stop = () => {
    if (gone) return;
    gone = true;
    trip.stop();
    idol?.drop();
    for (const one of made.values()) one?.drop();
    made.clear();
    face.drop();
    tags.drop();
    shade.drop();
    spots.drop();
    balls.drop();
    boxes.drop();
    if (screen) screen.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  tell();
  return {
    draw,
    stop,
    hold: holding,
    windDown,
    phase: trip.phase,
    stage: trip.stage,
    state: () => state(),
    mood: () => mood(view.t),
    jump,
    select,
    choose,
    step,
    pick,
    tap,
    marks: () => shown,
    said: () => said,
    here: () => leg.here,
    picked: () => leg.pick,
    inside: () => frame?.inner ?? null,
    system: sys,
  };
}
