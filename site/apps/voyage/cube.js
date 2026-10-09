import { rng } from '../../lib/scene.js';
import { route } from '../../lib/space/bang.js';
import { ease, look as aim, project } from '../../lib/space/camera.js';
import { blend, fill, governor, program, quads, target, tier } from '../../lib/space/gl2.js';
import { add, cross, dot, len, mat3, mix, mul, norm, rot, sub } from '../../lib/space/vec.js';

const TAU = Math.PI * 2;
const SUN = norm([0.505, 0.808, 0.303]);
const AWAY = 2.2;
const APPROACH = 2500;
const BLEND = 600;
const TURN = 0.3;
const SWING = 0.6;
const LIFT = 0.35;
const OUT = 1;
const IN = 1.25;
const EARLY = 120;
const PROBE = 0.02;
const ALBEDO = 0.45;
const DESK = 1.2e6;
const PHONE = 0.35e6;
const NOTCHES = [1, 0.8, 0.64, 0.5];
const LAG = 100;
const NEAR = 0.03;
const RIM = 2.2;
const MOST = 8;
const TONE = 0x1b873593;

const SPOT = `#version 300 es
in vec4 aDisc;
in vec4 aTone;
in vec4 aSun;
uniform vec2 uRes;
out vec2 vAt;
out float vR;
out float vSpin;
out vec3 vTone;
out vec3 vSun;
void main() {
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1)) * 2.0 - 1.0;
  float reach = aDisc.z * 3.0 + 1.5;
  vec2 p = aDisc.xy + c * reach;
  vAt = c * reach;
  vR = aDisc.z;
  vSpin = aDisc.w;
  vTone = aTone.rgb;
  vSun = aSun.xyz;
  gl_Position = vec4(p.x / uRes.x * 2.0 - 1.0, 1.0 - p.y / uRes.y * 2.0, 0.0, 1.0);
}
`;

const GLOW = `#version 300 es
precision highp float;
in vec2 vAt;
in float vR;
in float vSpin;
in vec3 vTone;
in vec3 vSun;
uniform float uK;
uniform vec3 uGlow;
out vec4 o;
void main() {
  vec2 q = vAt / vR;
  float c = cos(vSpin);
  float s = sin(vSpin);
  mat3 m = mat3(c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c) * mat3(1.0, 0.0, 0.0, 0.0, 0.83, -0.56, 0.0, 0.56, 0.83);
  vec3 ro = vec3(q.x, -q.y, -4.0) * m;
  vec3 rd = vec3(0.0, 0.0, 1.0) * m;
  vec3 inv = 1.0 / mix(rd, vec3(1e-6), vec3(lessThan(abs(rd), vec3(1e-6))));
  vec3 a = (-0.58 - ro) * inv;
  vec3 b = (0.58 - ro) * inv;
  vec3 lo = min(a, b);
  vec3 hi = max(a, b);
  float enter = max(max(lo.x, lo.y), lo.z);
  float leave = min(min(hi.x, hi.y), hi.z);
  float cover = clamp((leave - enter) * vR, 0.0, 1.0);
  vec3 face = -sign(rd) * step(vec3(enter - 1e-4), lo);
  vec3 n = m * face;
  float lit = max(dot(n, vSun), 0.0);
  vec3 side = abs(ro + rd * enter) / 0.58 * (1.0 - abs(face));
  float edge = smoothstep(0.7, 0.97, max(side.x, max(side.y, side.z)));
  vec3 body = vTone * (0.1 + 0.75 * lit) + uGlow * edge;
  vec3 halo = uGlow * exp(-2.2 * max(length(q) - 0.55, 0.0)) * 0.35;
  o = vec4((body * cover + halo * (1.0 - cover)) * uK, cover * uK);
}
`;

const MIX = `#version 300 es
precision highp float;
in vec2 v;
uniform sampler2D uSrc;
uniform vec2 uTexel;
uniform float uK;
uniform float uOut;
uniform vec3 uRim;
out vec4 o;
float cover(vec2 at) {
  vec3 c = texture(uSrc, at).rgb;
  return smoothstep(0.0, 0.0004, max(c.r, max(c.g, c.b)));
}
void main() {
  vec3 c = texture(uSrc, v).rgb;
  if (uOut < 0.5) {
    o = vec4(c * uK, uK);
    return;
  }
  float a = cover(v);
  float near = 0.0;
  float far = 0.0;
  for (int i = 0; i < 8; i++) {
    float t = float(i) * 0.7853982;
    vec2 d = vec2(cos(t), sin(t)) * uTexel;
    near += cover(v + d * 1.5);
    far += cover(v + d * 4.0);
  }
  vec3 rim = uRim * (a * (1.0 - near / 8.0) * 1.4 + (1.0 - a) * (far / 8.0) * 0.5);
  o = vec4((c * a + rim) * uK, a * uK);
}
`;

/* VISIT */

function palette(seed) {
  const rand = rng((seed ^ TONE) >>> 0);
  const c = [0, 1, 2].map(() => 0.5 + rand() * 0.6);
  const d = [0, 1, 2].map(() => rand());
  return [0.1, 0.4, 0.7, 1].map((x) => c.map((ci, i) => (0.5 + 0.5 * Math.cos(TAU * (ci * x + d[i]))) ** 2.2 * ALBEDO + 0.02));
}

function start(path) {
  let lo = -EARLY;
  let hi = 0;
  for (let i = 0; i < 60; i++) {
    const mid = (lo + hi) / 2;
    if (Number.isFinite(path.at(mid).pos[0])) hi = mid;
    else lo = mid;
  }
  return hi;
}

export function visit(body, seed) {
  const design = { code: body.code, n: body.n };
  const path = route(design, seed);
  const tau = start(path);
  const one = path.at(tau);
  const door = { tau, pos: one.pos, fwd: one.fwd, up: one.up, roll: one.roll, hue: one.hue, speed: mul(sub(path.at(tau + PROBE).pos, one.pos), 1 / PROBE) };
  const out = mul(door.fwd, -1);
  const S = norm(cross(Math.abs(out[1]) < 0.9 ? [0, 1, 0] : [0, 0, 1], out));
  const U = cross(out, S);
  const back = mul(body.dir, -1);
  let axis = cross(back, SUN);
  if (len(axis) < 1e-6) axis = cross(back, Math.abs(back[0]) < 0.9 ? [1, 0, 0] : [0, 1, 0]);
  return { body, design, path, door, deeper: path.next().at, frame: [out, S, U], axis: norm(axis), angle: Math.acos(Math.min(Math.max(dot(back, SUN), -1), 1)), ramp: palette((seed ^ body.code) >>> 0) };
}

const around = (v, a) => {
  const [N, S, U] = v.frame;
  const D = AWAY * v.design.n;
  const th = SWING - TURN * a;
  const flat = D * Math.cos(LIFT);
  return {
    pos: add(add(mul(N, flat * Math.cos(th)), mul(S, flat * Math.sin(th))), mul(U, D * Math.sin(LIFT))),
    vel: mul(add(mul(N, -Math.sin(th)), mul(S, Math.cos(th))), -TURN * flat),
  };
};

const hermite = (p0, v0, p1, v1, s) => {
  const s2 = s * s;
  const s3 = s2 * s;
  return add(add(mul(p0, 2 * s3 - 3 * s2 + 1), mul(v0, s3 - 2 * s2 + s)), add(mul(p1, -2 * s3 + 3 * s2), mul(v1, s3 - s2)));
};

const outside = (frame) => ({ ...frame, outside: true, fog: 0, near: 0, doors: [] });

export function inside(v, u) {
  const a = u / 1000;
  const span = APPROACH / 1000;
  if (a < span) {
    const [, , U] = v.frame;
    const from = around(v, Math.min(a, 0));
    const s = Math.max(a, 0) / span;
    const k = ease.smooth(s);
    const pos = a <= 0 ? from.pos : hermite(from.pos, mul(from.vel, span), v.door.pos, mul(v.door.speed, span), s);
    const at = mix([0, 0, 0], add(v.door.pos, v.door.fwd), k);
    const up = norm(mix(U, v.door.up, k));
    const cam = aim(pos, len(sub(at, pos)) > 1e-6 ? at : add(pos, v.door.fwd), up, v.door.roll * k);
    cam.fov = OUT + (IN - OUT) * k;
    return { cam, frames: [[outside({ scale: 1, hue: v.door.hue }), 1]], dolly: a < 0 };
  }
  const tau = v.door.tau + a - span;
  const one = v.path.at(tau);
  const cam = aim(one.pos, add(one.pos, one.fwd), one.up, one.roll);
  cam.fov = IN;
  const w = one.frame > 0 ? 1 : ease.smooth(1 - (v.deeper - tau) / (BLEND / 1000));
  return { cam, frames: w >= 1 ? [[one, 1]] : w > 0 ? [[outside(one), 1], [one, w]] : [[outside(one), 1]], tau, level: one.level, scale: one.scale };
}

export function sky(v, cam) {
  const back = (row) => rot(row, v.axis, -v.angle);
  const m = cam.rot;
  return { pos: [0, 0, 0], rot: mat3(back([m[0], m[1], m[2]]), back([m[3], m[4], m[5]]), back([m[6], m[7], m[8]])), fov: cam.fov };
}

export function doors(v, shot, view) {
  if (shot.tau === undefined) return [];
  return v.path
    .ahead(shot.tau)
    .map((one) => {
      const at = project(view, shot.cam, one.pos);
      return at && at[2] > shot.scale * NEAR ? { id: one.id, x: at[0], y: at[1], label: String(one.number), on: one.on, dim: !one.seen } : null;
    })
    .filter(Boolean);
}

/* DRAW */

export function cubes(gl) {
  const prog = program(gl, GLOW, SPOT);
  const spots = quads(gl, { aDisc: 4, aTone: 4, aSun: 4 }, MOST);
  const draw = (list, res, k, glow) => {
    list.slice(0, MOST).forEach((one, i) => spots.data.set(one, i * 12));
    prog.set({ uRes: res, uK: k, uGlow: glow });
    spots.draw(prog, Math.min(list.length, MOST), 'alpha');
    blend(gl, null);
  };
  const drop = () => {
    prog.drop();
    spots.drop();
  };
  return { draw, drop };
}

export function deck(gl, view) {
  const mixer = program(gl, MIX);
  let layer = null;
  const gov = governor(view, NOTCHES);
  const cap = tier(view) === 'phone' ? PHONE : DESK;
  let clock = view.t;
  const scale = () => {
    const dt = view.t - clock;
    clock = view.t;
    const notch = dt > 0 && dt <= LAG ? gov.tick() : gov.scale();
    return Math.min(1, Math.sqrt(cap / (view.w * view.h))) * notch;
  };
  const draw = (world, v, shot, k, accent, approach = 1) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const fb = gl.getParameter(gl.FRAMEBUFFER_BINDING);
    const s = scale();
    layer ??= target(gl, view.w * s, view.h * s, { hdr: true });
    layer.size(view.w * s, view.h * s);
    const cam = shot.dolly ? { ...shot.cam, pos: mul(shot.cam.pos, approach) } : shot.cam;
    const tone = { accent, ramp: v.ramp };
    for (const [frame, w] of shot.frames) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, layer.fb);
      gl.viewport(0, 0, layer.w, layer.h);
      world.draw(cam, frame, tone);
      gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
      gl.viewport(port[0], port[1], port[2], port[3]);
      blend(gl, 'alpha');
      mixer.set({ uSrc: layer, uTexel: [1 / layer.w, 1 / layer.h], uK: k * w, uOut: frame.outside ? 1 : 0, uRim: accent.map((c) => c * RIM) });
      fill(gl);
    }
    blend(gl, null);
  };
  const drop = () => {
    mixer.drop();
    layer?.drop();
  };
  return { draw, drop };
}
