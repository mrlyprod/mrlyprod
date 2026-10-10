import { rgb } from '../scene.js';
import { blend, program, quads } from './gl2.js';
import { dot, len, mix, sub } from './vec.js';

const RATE = 6;
const PERIOD = 1.5;
const RAIL = 0.35;
const HEAD = 2.5;
const TAIL = 1.2;
const FRONT = 0.05;
const END = 2;
const GAIN = 1.2;
const WIDTH = 0.003;
const NEAR = 1e-4;
const MOST = 512;

const LINE = `#version 300 es
in vec4 aA;
in vec4 aB;
uniform vec2 uRes;
uniform vec2 uOrigin;
uniform float uWidth;
out vec2 vAt;
flat out vec4 vA;
flat out vec4 vB;
const float REACH = 4.0;
void main() {
  vec2 d = aB.xy - aA.xy;
  float l = length(d);
  vec2 u = l > 1e-4 ? d / l : vec2(1.0, 0.0);
  vec2 n = vec2(-u.y, u.x);
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1));
  float r = uWidth * REACH;
  vec2 p = mix(aA.xy - u * r, aB.xy + u * r, c.x) + n * r * (c.y * 2.0 - 1.0);
  vAt = p;
  vA = aA;
  vB = aB;
  gl_Position = vec4((p - uOrigin) / uRes * 2.0 - 1.0, 0.0, 1.0);
}
`;

const GLOW = `#version 300 es
precision highp float;
in vec2 vAt;
flat in vec4 vA;
flat in vec4 vB;
uniform vec2 uRes;
uniform vec2 uOrigin;
uniform float uWidth;
uniform vec3 uColor;
uniform float uK;
uniform int uHide;
uniform sampler2D uDepth;
out vec4 o;
const float HALO = 0.25;
const float SPREAD = 0.25;
const float SLACK = 0.02;
void main() {
  vec2 d = vB.xy - vA.xy;
  float h = clamp(dot(vAt - vA.xy, d) / max(dot(d, d), 1e-6), 0.0, 1.0);
  float r = length(vAt - vA.xy - d * h) / uWidth;
  float k = mix(vA.w, vB.w, h) * (exp(-r * r) + HALO * exp(-r * r * SPREAD));
  if (uHide == 1) {
    float wall = texture(uDepth, (gl_FragCoord.xy - uOrigin) / uRes).r;
    float z = mix(vA.z, vB.z, h);
    k *= 1.0 - smoothstep(wall, wall * (1.0 + SLACK), z);
  }
  o = vec4(uColor * k * uK, 0.0);
}
`;

const linear = (hex) => rgb(hex).map((v) => (v / 255) ** 2.2);

const finite = (p) => p.every(Number.isFinite);

function rail(one, age) {
  const points = one.points ?? [];
  const step = one.step ?? 1;
  const last = points.length - 1;
  const head = age >= 0 ? ((age % PERIOD) * RATE) / step : -1;
  const at = (x) => (Number.isInteger(x) ? points[x] : mix(points[Math.floor(x)], points[Math.floor(x) + 1], x - Math.floor(x)));
  const glow = (x) => {
    const fade = Math.min(1, ((last - x) * step) / END);
    const pulse = x <= head ? HEAD * Math.exp(((x - head) * step) / TAIL) : 0;
    return (RAIL + pulse) * fade;
  };
  const marks = points.map((_, i) => i);
  if (head >= 0 && head < last) marks.push(head, Math.min(head + FRONT / step, last));
  return marks.sort((a, b) => a - b).map((x) => [at(x), glow(x)]);
}

export function trails(gl, view) {
  const prog = program(gl, GLOW, LINE);
  const batch = quads(gl, { aA: 4, aB: 4 }, MOST);
  let paint = null;
  let colour = null;
  const eye = (cam) => {
    const m = cam.rot;
    return [[m[0], m[1], m[2]], [m[3], m[4], m[5]], [m[6], m[7], m[8]]];
  };
  const place = (cam, port, p) => {
    const [r, u, f] = eye(cam);
    const d = sub(p, cam.pos);
    const focal = Math.min(port[2], port[3]) / (2 * Math.tan(cam.fov / 2));
    const z = dot(d, f);
    return [port[0] + port[2] / 2 + (dot(d, r) / z) * focal, port[1] + port[3] / 2 + (dot(d, u) / z) * focal, len(d)];
  };
  const spatial = (cam, port, a, b) => {
    const f = eye(cam)[2];
    const za = dot(sub(a[0], cam.pos), f);
    const zb = dot(sub(b[0], cam.pos), f);
    if (za <= NEAR && zb <= NEAR) return null;
    const cut = (p, q, zp, zq) => (zp > NEAR ? p : [mix(p[0], q[0], (NEAR - zp) / (zq - zp)), p[1] + (q[1] - p[1]) * ((NEAR - zp) / (zq - zp))]);
    const [p, q] = [cut(a, b, za, zb), cut(b, a, zb, za)];
    return [[...place(cam, port, p[0]), p[1]], [...place(cam, port, q[0]), q[1]]];
  };
  const pieces = (one, cam, port) => {
    if (!cam || (one.points ?? []).length < 2) return [];
    const run = rail(one, (view.t - (one.born ?? 0)) / 1000);
    const out = [];
    for (let i = 1; i < run.length; i++) {
      if ((run[i - 1][1] <= 0 && run[i][1] <= 0) || !finite(run[i - 1][0]) || !finite(run[i][0])) continue;
      const seg = spatial(cam, port, run[i - 1], run[i]);
      if (seg) out.push(seg);
    }
    return out;
  };
  const draw = (list, cam, k = 1, depth = null) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const accent = view.look().accent;
    if (accent !== paint) {
      paint = accent;
      colour = linear(accent).map((v) => v * GAIN);
    }
    let count = 0;
    for (const one of list ?? []) {
      for (const [a, b] of pieces(one, cam, port)) {
        if (count >= MOST) break;
        batch.data.set([...a, ...b], count++ * 8);
      }
    }
    if (!count) return 0;
    prog.set({ uRes: [port[2], port[3]], uOrigin: [port[0], port[1]], uWidth: WIDTH * Math.min(port[2], port[3]), uColor: colour, uK: k, uHide: depth ? 1 : 0, uDepth: depth });
    batch.draw(prog, count, 'add');
    blend(gl, null);
    return count;
  };
  const drop = () => {
    prog.drop();
    batch.drop();
  };
  return { draw, drop };
}
