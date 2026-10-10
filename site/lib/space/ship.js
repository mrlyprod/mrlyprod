import { project, uniforms } from './camera.js';
import { blend, fill, program } from './gl2.js';
import { add, dot, mul, sub } from './vec.js';

const TAIL = 2.5;
const WIDE = 2.2;
const HALO = 1.5;
const FLICKER = 0.1;
const BEAT = 0.04;
const LIVE = 0.001;
const FIRE = [0.4, 0.8, 1.6];

const GLOW = `#version 300 es
precision highp float;
out vec4 o;
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform vec2 uOrigin;
uniform vec3 uAt;
uniform mat3 uTurn;
uniform float uSize;
uniform vec3 uAxis;
uniform vec3 uTail;
uniform vec3 uTip;
uniform float uPlume;
uniform float uLength;
uniform vec3 uFire;
uniform float uHeat;
uniform float uTime;
const float BURN = 1.6;
const float EXHAUST = 0.35;
const float HEAT = 2.5;
const float NOSE = 0.45;
const float SKIN = 0.25;
const vec3 HOT = vec3(1.0, 0.45, 0.15);
const float OPEN = 1e9;
float enter(vec3 ro, vec3 rd) {
  mat3 back = transpose(uTurn);
  vec3 so = back * (ro - uAt);
  vec3 sd = back * rd;
  vec3 inv = 1.0 / mix(sd, vec3(1e-6), vec3(lessThan(abs(sd), vec3(1e-6))));
  vec3 a = (-0.5 * uSize - so) * inv;
  vec3 b = (0.5 * uSize - so) * inv;
  vec3 lo = min(a, b);
  vec3 hi = max(a, b);
  float near = max(max(lo.x, lo.y), lo.z);
  float far = min(min(hi.x, hi.y), hi.z);
  return far >= max(near, 0.0) ? max(near, 0.0) : 1e9;
}
float spot(vec3 ro, vec3 rd, vec3 at, float w, float wall) {
  float ts = dot(at - ro, rd);
  if (ts < 0.0 || ts > wall) return 0.0;
  float dd = length(ro + rd * ts - at);
  return exp(-dd * dd / (w * w));
}
void main() {
  float span = min(uRes.x, uRes.y);
  vec2 uv = (gl_FragCoord.xy - uOrigin - 0.5 * uRes) / span * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  vec3 ro = uPos;
  float wall = enter(ro, rd);
  float skin = wall + uSize * SKIN;
  float flick = 0.8 + 0.2 * sin(uTime * 15.0);
  vec3 col = vec3(0.0);
  if (uPlume > 0.001) {
    col += uFire * uPlume * BURN * flick * spot(ro, rd, uTail, uSize * EXHAUST, skin);
    for (int i = 0; i < 14; i++) {
      float fi = (float(i) + 0.5) / 14.0;
      vec3 s = uTail - uAxis * uLength * fi;
      col += uFire * uPlume * spot(ro, rd, s, uSize * (0.3 - 0.22 * fi), wall) * 0.07 * (1.0 - fi);
    }
  }
  if (uHeat > 0.001) col += HOT * uHeat * HEAT * flick * spot(ro, rd, uTip, uSize * NOSE, OPEN);
  o = vec4(col, 0.0);
}
`;

/* GLOW */

export function ship(gl, view) {
  const prog = program(gl, GLOW);
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
  const draw = (cam, pose) => {
    const { at, axes, size, fwd } = pose;
    const plume = Math.min(Math.max(pose.plume ?? 0, 0), 1) * (1 - FLICKER + FLICKER * Math.sin(view.t * BEAT));
    const heat = Math.max(pose.heat ?? 0, 0);
    if (plume <= LIVE && heat <= LIVE) return;
    const port = gl.getParameter(gl.VIEWPORT);
    const reach = (size / 2) * axes.reduce((sum, v) => sum + Math.abs(dot(v, fwd)), 0);
    const tail = sub(at, mul(fwd, reach));
    const tip = add(at, mul(fwd, reach));
    const length = size * TAIL * (0.4 + plume);
    const corners = Array.from({ length: 8 }, (_, i) => axes.reduce((sum, v, j) => add(sum, mul(v, ((i >> j) & 1 ? HALO : -HALO) * size)), at));
    const trail = plume > LIVE ? [0, 0.5, 1].map((f) => sub(tail, mul(fwd, length * f))) : [];
    const box = frame(port, cam, [...corners, ...trail.flatMap((c) => axes.flatMap((v) => [add(c, mul(v, WIDE * size * 0.3)), sub(c, mul(v, WIDE * size * 0.3))]))]);
    if (!box) return;
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(...box);
    blend(gl, 'add');
    prog.set({
      ...uniforms({ w: port[2], h: port[3] }, cam),
      uOrigin: [port[0], port[1]],
      uAt: at,
      uTurn: axes.flat(),
      uSize: size,
      uAxis: fwd,
      uTail: tail,
      uTip: tip,
      uPlume: plume,
      uLength: length,
      uFire: pose.fire ?? FIRE,
      uHeat: heat,
      uTime: view.t / 1000,
    });
    fill(gl);
    blend(gl, null);
    gl.disable(gl.SCISSOR_TEST);
  };
  const drop = () => prog.drop();
  return { draw, drop };
}
