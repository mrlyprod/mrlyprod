import { rng } from '../scene.js';
import { uniforms } from './camera.js';
import { bake, blend, fill, program, quads, tier } from './gl2.js';
import { norm } from './vec.js';

const TAU = Math.PI * 2;
const DESK = 6000;
const PHONE = 2500;
const INNER = 0.08;
const NEAR = 0.01;
const EDGE = 0.2;
const THIN = 1.5;
const TAPER = 0.6;
const PAD = 16;
const FAINT = 0.003;
const TWINKLE = 0.1;
const FACE = 256;
const SALT = 0x68e31da4;
const UNSET = [-1, -1, -1];
const WHITE = [1, 1, 1];

const clamp = (x) => Math.min(Math.max(x, 0), 1);

const smooth = (x) => {
  const p = clamp(x);
  return p * p * (3 - 2 * p);
};

export function stars(seed, count) {
  const rand = rng(seed >>> 0);
  const out = new Float32Array(count * 8);
  for (let i = 0; i < count; i++) {
    const z = rand() * 2 - 1;
    const a = rand() * TAU;
    const r = Math.sqrt(1 - z * z);
    out.set([r * Math.cos(a), r * Math.sin(a), z, Math.cbrt(rand()), rand() ** 3, Math.sqrt(rand()), rand() * TAU, 0], i * 8);
  }
  return out;
}

const STREAK = `#version 300 es
in vec4 aEnds;
in vec4 aLook;
uniform vec2 uRes;
uniform vec3 uHalo;
out vec2 vAt;
out float vLen;
out vec2 vWide;
out float vGain;
out vec3 vHalo;
const vec3 WARM = vec3(1.0, 0.58, 0.28);
const vec3 PALE = vec3(0.46, 0.76, 0.9);
const vec3 BLUE = vec3(0.1, 0.26, 1.0);
void main() {
  vec2 head = aEnds.xy;
  vec2 tail = aEnds.zw;
  float len = length(head - tail);
  vec2 ax = len > 1e-3 ? (head - tail) / len : vec2(1.0, 0.0);
  vec2 nx = vec2(-ax.y, ax.x);
  float r = 2.0 * max(aLook.x, aLook.y) + 1.0;
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1));
  float along = mix(-r, len + r, c.x);
  float across = mix(-r, r, c.y);
  vec2 p = tail + ax * along + nx * across;
  vAt = vec2(along, across);
  vLen = len;
  vWide = aLook.yx;
  vGain = aLook.z;
  vHalo = uHalo.x >= 0.0 ? uHalo : aLook.w < 0.5 ? mix(WARM, PALE, aLook.w * 2.0) : mix(PALE, BLUE, aLook.w * 2.0 - 1.0);
  gl_Position = vec4(p.x / uRes.x * 2.0 - 1.0, 1.0 - p.y / uRes.y * 2.0, 0.0, 1.0);
}
`;

const GLOW = `#version 300 es
precision highp float;
in vec2 vAt;
in float vLen;
in vec2 vWide;
in float vGain;
in vec3 vHalo;
uniform vec3 uTint;
out vec4 o;
void main() {
  float h = vLen > 1e-3 ? clamp(vAt.x / vLen, 0.0, 1.0) : 1.0;
  vec2 q = vec2(vAt.x - h * vLen, vAt.y);
  float s = mix(vWide.x, vWide.y, h) * 0.4;
  float d2 = dot(q, q) / (s * s);
  vec3 c = vec3(exp(-0.5 * d2)) + vHalo * exp(-0.125 * d2) * 0.3;
  o = vec4(c * uTint * vGain * (0.25 + 0.75 * h), 1.0);
}
`;

const DUST = `#version 300 es
precision highp float;
in vec3 d;
out vec4 o;
uniform vec3 uGalaxy;
uniform float uSeed;
float hash3(vec3 p) {
  p = fract(p * 0.3183099 + vec3(0.1, 0.2, 0.3));
  p *= 17.0;
  return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}
float noise(vec3 x) {
  vec3 i = floor(x);
  vec3 f = fract(x);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(mix(hash3(i), hash3(i + vec3(1, 0, 0)), f.x), mix(hash3(i + vec3(0, 1, 0)), hash3(i + vec3(1, 1, 0)), f.x), f.y),
             mix(mix(hash3(i + vec3(0, 0, 1)), hash3(i + vec3(1, 0, 1)), f.x), mix(hash3(i + vec3(0, 1, 1)), hash3(i + vec3(1, 1, 1)), f.x), f.y), f.z);
}
float fbm4(vec3 p) {
  float a = 0.5, s = 0.0;
  for (int i = 0; i < 4; i++) { s += a * noise(p); p = p * 2.02 + vec3(1.7, 9.2, 3.1); a *= 0.5; }
  return s;
}
void main() {
  vec3 n = normalize(d);
  float g = pow(max(1.0 - abs(dot(n, uGalaxy)) * 2.2, 0.0), 3.0);
  float cloud = fbm4(n * 4.0 + uSeed * 0.1) * (1.0 - 0.7 * fbm4(n * 9.0 + 4.0));
  o = vec4(sqrt(vec3(0.5, 0.55, 0.7) * g * cloud * 0.12), 1.0);
}
`;

const HAZE = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform samplerCube uDust;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform float uK;
void main() {
  vec2 uv = (gl_FragCoord.xy - 0.5 * uRes) / min(uRes.x, uRes.y) * 2.0 * uTan;
  vec3 c = texture(uDust, normalize(uRot * vec3(uv, 1.0))).rgb;
  o = vec4(c * c * uK, 1.0);
}
`;

export function sky(gl, view, { seed = 0, density = 1, dust = 1 } = {}) {
  const count = Math.max(0, Math.round((tier(view) === 'phone' ? PHONE : DESK) * density));
  const field = stars(seed, count);
  const glow = program(gl, GLOW, STREAK);
  const batch = quads(gl, { aEnds: 4, aLook: 4 }, count);
  const tilt = rng((seed ^ SALT) >>> 0);
  const haze = dust > 0 ? program(gl, HAZE) : null;
  const cube = dust > 0 ? bake(gl, FACE, DUST, { uGalaxy: norm([tilt() - 0.5, tilt() - 0.5, tilt() - 0.5]), uSeed: tilt() * 100 }) : null;
  const draw = (cam, flow = {}) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const w = port[2];
    const h = port[3];
    const gain = flow.k ?? 1;
    if (haze) {
      blend(gl, 'add');
      haze.set({ ...uniforms({ w, h }, cam), uDust: cube, uK: dust * gain });
      fill(gl);
    }
    const away = clamp(flow.far ?? 0);
    const s = (flow.s ?? 0) * (1 - away);
    const back = Math.max(0, flow.shutter ?? 0);
    const [ax, ay, az] = norm(flow.axis ?? [0, 0, 1]);
    const m = cam.rot;
    const focal = Math.min(w, h) / (2 * Math.tan(cam.fov / 2));
    const unit = ((view.dpr || 1) * w) / view.w;
    const now = view.t / 1000;
    const out = batch.data;
    let n = 0;
    for (let i = 0; i < count; i++) {
      const o = i * 8;
      const deep = INNER + (1 - INNER) * field[o + 3];
      const reach = deep + (1 - deep) * away;
      const px = field[o] * reach;
      const py = field[o + 1] * reach;
      const pz = field[o + 2] * reach;
      const a0 = px * ax + py * ay + pz * az;
      const a = ((((a0 - s + 1) % 2) + 2) % 2) - 1;
      const wrap = smooth((1 - a) / EDGE) * smooth((a + 1) / EDGE);
      const edge = wrap + (1 - wrap) * away;
      if (edge <= 0) continue;
      const hx = px + ax * (a - a0);
      const hy = py + ay * (a - a0);
      const hz = pz + az * (a - a0);
      const vz = hx * m[6] + hy * m[7] + hz * m[8];
      if (vz <= NEAR) continue;
      const vx = hx * m[0] + hy * m[1] + hz * m[2];
      const vy = hx * m[3] + hy * m[4] + hz * m[5];
      let tx = vx + back * (ax * m[0] + ay * m[1] + az * m[2]);
      let ty = vy + back * (ax * m[3] + ay * m[4] + az * m[5]);
      let tz = vz + back * (ax * m[6] + ay * m[7] + az * m[8]);
      if (tz < NEAR) {
        const c = (vz - NEAR) / (vz - tz);
        tx = vx + (tx - vx) * c;
        ty = vy + (ty - vy) * c;
        tz = NEAR;
      }
      const sx = w / 2 + (vx / vz) * focal;
      const sy = h / 2 - (vy / vz) * focal;
      const ex = w / 2 + (tx / tz) * focal;
      const ey = h / 2 - (ty / tz) * focal;
      if (Math.max(sx, ex) < -PAD || Math.min(sx, ex) > w + PAD || Math.max(sy, ey) < -PAD || Math.min(sy, ey) > h + PAD) continue;
      const mag = field[o + 4];
      const phase = field[o + 6];
      const near = (1 - Math.min(Math.hypot(vx, vy, vz), 1)) * (1 - away);
      const rear = (1 - Math.min(Math.hypot(tx, ty, tz), 1)) * (1 - away);
      const size = (0.7 + 1.6 * mag) * unit;
      const head = size * (0.6 + 0.9 * near);
      const tail = size * (0.6 + 0.9 * rear) * TAPER;
      const twinkle = 1 + TWINKLE * Math.sin(now * TAU * (0.4 + (phase % 1) * 0.6) + phase);
      let bright = (0.15 + 2.2 * mag) * (0.35 + 0.65 * near) * edge * gain * twinkle * smooth((vz - NEAR) / NEAR);
      if (head < THIN) bright *= head / THIN;
      if (bright < FAINT) continue;
      const at = n++ * 8;
      out[at] = sx;
      out[at + 1] = sy;
      out[at + 2] = ex;
      out[at + 3] = ey;
      out[at + 4] = Math.max(THIN, head);
      out[at + 5] = Math.max(THIN, tail);
      out[at + 6] = bright;
      out[at + 7] = field[o + 5];
    }
    glow.set({ uRes: [w, h], uHalo: flow.halo ?? UNSET, uTint: flow.tint ?? WHITE });
    batch.draw(glow, n, 'add');
    blend(gl, null);
  };
  const drop = () => {
    glow.drop();
    haze?.drop();
    batch.drop();
    if (cube) gl.deleteTexture(cube);
  };
  return { draw, drop, count };
}
