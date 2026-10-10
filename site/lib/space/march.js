import { rgb, rng } from '../scene.js';
import { cells } from './bang.js';
import { project, uniforms } from './camera.js';
import { blend, fill, hdr, program, target, texture, tier } from './gl2.js';

export const LOOKS = ['graphic', 'studio', 'haze'];

export const GRADES = {
  graphic: { ev: 0, bloom: 0.15, aberration: 0, vignette: 0.25, grain: 0 },
  studio: { ev: 0.1, bloom: 0.3, aberration: 0, vignette: 0.3, grain: 0.006 },
  haze: { ev: 0.2, bloom: 0.7, aberration: 0.0008, vignette: 0.4, grain: 0.012 },
};

const QUALITY = {
  low: { cap: 0.5e6, steps: 96, taps: 2, depth: 8, shadow: 0 },
  desk: { cap: 1.2e6, steps: 200, taps: 8, depth: 12, shadow: 1 },
  high: { cap: 1.8e6, steps: 260, taps: 12, depth: 12, shadow: 1 },
  phone: { cap: 0.35e6, steps: 96, taps: 2, depth: 8, shadow: 0 },
};

export const RAMPS = {
  fire: ['#1c0400', '#9a2005', '#f06a12', '#ffd75e'],
  ice: ['#03111f', '#0d4f8c', '#47c3ee', '#e4fbff'],
  mono: ['#161616', '#5a5a5a', '#a2a2a2', '#ececec'],
};

const TAU = Math.PI * 2;
const ALBEDO = 0.45;
const TONE = 0x1b873593;
const CYCLE = 8;
const KEEP = 0.9;
const STEP = 100;
const HUE = 0.2;
const LOD = 1.5;
const RAW = 3.5;
const BEYOND = 1e6;
const OPEN = [BEYOND, BEYOND, BEYOND];
const NONE = [0, 0, 0, 0];
const BORDER = 3;
const DARK = [0, 0, 0];
const DOWN = [0, -1, 0];

const halton = (i, b) => {
  let f = 1;
  let r = 0;
  for (let k = i; k > 0; k = Math.floor(k / b)) {
    f /= b;
    r += f * (k % b);
  }
  return r;
};

const HALTON = Array.from({ length: CYCLE }, (_, i) => [halton(i + 1, 2) - 0.5, halton(i + 1, 3) - 0.5]);

/* TONE */

const linear = (hex) => rgb(hex).map((v) => (v / 255) ** 2.2);

const blendTo = (a, b, k) => a.map((v, i) => v + (b[i] - v) * k);

export function ramp(name, seed, accent) {
  if (RAMPS[name]) return RAMPS[name].map((hex) => linear(hex).map((v) => v * ALBEDO));
  if (name === 'accent') {
    const a = linear(accent);
    return [blendTo([0.02, 0.02, 0.02], a, 0.25), blendTo([0, 0, 0], a, 0.7), a, blendTo(a, [1, 1, 1], 0.6)].map((c) => c.map((v) => v * ALBEDO));
  }
  const rand = rng((seed ^ TONE) >>> 0);
  const c = [0, 1, 2].map(() => 0.5 + rand() * 0.6);
  const d = [0, 1, 2].map(() => rand());
  return [0.1, 0.4, 0.7, 1].map((x) => c.map((ci, i) => (0.5 + 0.5 * Math.cos(TAU * (ci * x + d[i]))) ** 2.2 * ALBEDO + 0.02));
}

export const tone = (name, seed, accent) => ({ accent: linear(accent), ramp: ramp(name, seed, accent) });

/* BOXES */

export function boxes(all, n) {
  const lo = new Uint8Array(n * n * n * 4);
  const hi = new Uint8Array(n * n * n * 4);
  const clear = (b) => {
    if (b[0] < 0 || b[2] < 0 || b[4] < 0 || b[1] > n || b[3] > n || b[5] > n) return false;
    for (let z = b[4]; z < b[5]; z++) for (let y = b[2]; y < b[3]; y++) for (let x = b[0]; x < b[1]; x++) if (all[(z * n + y) * n + x]) return false;
    return true;
  };
  for (let i = 0; i < n * n * n; i++) {
    if (all[i]) {
      lo[i * 4] = 255;
      continue;
    }
    const b = [i % n, (i % n) + 1, Math.floor(i / n) % n, (Math.floor(i / n) % n) + 1, Math.floor(i / (n * n)), Math.floor(i / (n * n)) + 1];
    for (let grew = true; grew; ) {
      grew = false;
      for (let s = 0; s < 6; s++) {
        const slab = [...b];
        if (s & 1) slab[s - 1] = b[s];
        else slab[s + 1] = b[s];
        slab[s] += s & 1 ? 1 : -1;
        if (!clear(slab)) continue;
        b[s] = slab[s];
        grew = true;
      }
    }
    lo.set([b[0], b[2], b[4]], i * 4 + 1);
    hi.set([b[1], b[3], b[5]], i * 4);
  }
  return { lo, hi };
}

/* MARCH */

const MARCH = `
precision highp float;
precision highp sampler3D;
layout(location = 0) out vec4 o;
#ifdef AUX
layout(location = 1) out vec4 d;
#endif
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform vec2 uJitter;
uniform float uN;
uniform int uDepth;
uniform int uTaps;
uniform int uShadow;
uniform sampler3D uLo;
uniform sampler3D uHi;
uniform float uScale;
uniform float uFog;
uniform float uSun;
uniform float uHull;
uniform float uFloor;
uniform vec3 uShine;
uniform vec3 uBelow;
uniform vec4 uDoor[2];
uniform float uGlow[2];
uniform vec3 uAccent;
uniform vec3 uRamp[4];
uniform float uHue;
uniform ivec4 uNear;
uniform vec3 uLow;
uniform vec3 uHigh;
const float FALL = 0.25;
const float KEY = 0.3;
const float DIM = 12.0;
const float SIGHT = 30.0;
const float MISS = 60000.0;
const float REGION = 2.0;
const float FLAT = 1.6;
const float WIDTH = 1.5;
const float INK = 0.8;
const float FINE = 4.0;
const float STRONG = 1.5;
const float FILL = 0.35;
const float AMBIENT = 0.06;
const float REACH = 2.0;
const float SHADE = 0.3;
const float CONE = 0.08;
const vec3 SUN = vec3(0.505, 0.808, 0.303);
const vec3 LIGHT = vec3(-0.55, 0.7, -0.45);
const vec3 BACK = vec3(0.6, 0.2, -0.5);
float box(vec3 p, vec3 b) {
  vec3 q = abs(p) - b;
  return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}
bool kept(vec3 q) {
  if (max(abs(q.x), max(abs(q.y), abs(q.z))) > REGION) return all(greaterThanEqual(q, -uLow)) && all(lessThanEqual(q, uHigh));
  ivec3 iq = ivec3(q + REGION);
  int i = iq.x * 25 + iq.y * 5 + iq.z;
  return ((uNear[i >> 5] >> (i & 31)) & 1) == 1;
}
vec4 probe(vec3 p, vec3 rd, float grain) {
  float foot = max(grain, uFloor);
  vec3 h = vec3(uN * 0.5);
  vec3 inv = 1.0 / max(abs(rd), vec3(1e-6));
  vec3 ahead = step(0.0, rd);
  vec3 q = floor(p / uN);
  if (q != vec3(0.0)) {
    p -= q * uN;
    if (!kept(q)) {
      vec3 a = p;
      vec3 b = vec3(uN) - p;
      vec3 e = mix(a, b, ahead) * inv;
      return vec4(max(min(min(a.x, b.x), min(min(a.y, b.y), min(a.z, b.z))), 0.0), 0.0, 1.0, max(min(e.x, min(e.y, e.z)), 0.0));
    }
  }
  float s = 1.0;
  int top = int(uN) - 1;
  for (int k = 0; k < 12; k++) {
    if (k >= uDepth) break;
    ivec3 c = clamp(ivec3(floor(p)), ivec3(0), ivec3(top));
    vec4 lo = texelFetch(uLo, c, 0);
    if (lo.x < 0.5) {
      vec3 from = lo.yzw * 255.0;
      vec3 to = texelFetch(uHi, c, 0).xyz * 255.0;
      vec3 a = p - from;
      vec3 b = to - p;
      if (s < foot) {
        vec3 low = step(0.5, from);
        vec3 high = step(to, vec3(uN - 0.5));
        vec3 cut = (to - from) * log(foot / s) / log(uN) / max(low + high, 1.0);
        a -= cut * low;
        b -= cut * high;
      }
      float inner = min(min(min(a.x, b.x), min(a.y, b.y)), min(a.z, b.z));
      if (inner < 0.0) return vec4(-length(max(-min(a, b), 0.0)) * s, 1.0, s, 0.0);
      vec3 e = mix(a, b, ahead) * inv;
      return vec4(inner * s, 0.0, s, max(min(e.x, min(e.y, e.z)), 0.0) * s);
    }
    if (s < foot) break;
    p = (p - vec3(c)) * uN;
    s /= uN;
  }
  return vec4(box(p - h, h) * s, 1.0, s, 0.0);
}
vec3 cube(vec3 p, float foot) {
  return probe(p, vec3(0.0), foot).xyz;
}
vec2 root(vec3 ro, vec3 rd) {
  vec3 h = vec3(uN * 0.5);
  vec3 safe = mix(rd, vec3(1e-6), vec3(lessThan(abs(rd), vec3(1e-6))));
  vec3 a = (-uLow * uN - h - ro) / safe;
  vec3 b = ((uHigh + 1.0) * uN - h - ro) / safe;
  vec3 lo = min(a, b);
  vec3 hi = max(a, b);
  return vec2(max(max(max(lo.x, lo.y), lo.z), 0.0), min(min(hi.x, hi.y), hi.z));
}
vec3 ramp(float x) {
  float y = clamp(x, 0.0, 1.0) * 3.0;
  int i = int(min(floor(y), 2.0));
  return mix(uRamp[i], uRamp[i + 1], y - float(i));
}
vec3 normal(vec3 c, float e, float foot) {
  vec2 k = vec2(1.0, -1.0);
  return normalize(k.xyy * cube(c + k.xyy * e, foot).x + k.yyx * cube(c + k.yyx * e, foot).x + k.yxy * cube(c + k.yxy * e, foot).x + k.xxx * cube(c + k.xxx * e, foot).x);
}
float occlusion(vec3 c, vec3 nr, float lod, float foot) {
  int taps = max(uTaps / 2, 1);
  float near = 0.0;
  float wide = 0.0;
  float sum = 0.0;
  float w = 1.0;
  for (int i = 1; i <= 6; i++) {
    if (i > taps) break;
    float a = lod * 0.7 * float(i);
    float b = uScale * 0.3 * float(i);
    near += (a - cube(c + nr * a, foot).x) / a * w;
    wide += (b - cube(c + nr * b, foot).x) / b * w;
    sum += w;
    w *= 0.65;
  }
  float k = 2.3 / sum;
  return clamp(1.0 - near * k * 0.35, 0.0, 1.0) * clamp(1.0 - wide * k * 0.3, 0.0, 1.0);
}
float shade(vec3 c, vec3 l, float far, float foot) {
  float t = foot * 4.0;
  for (int i = 0; i < 24; i++) {
    if (t > far) break;
    vec3 r = cube(c + l * t, foot * 3.0);
    if (r.y > 0.5) return 0.0;
    t += max(r.x, foot * 2.0);
  }
  return 1.0;
}
float soft(vec3 c, vec3 l, float foot) {
  float lit = 1.0;
  for (int i = 1; i <= 8; i++) {
    float t = foot * 4.0 + uScale * REACH * float(i) / 8.0;
    if (cube(c + l * t, max(foot * LOD, t * CONE)).y > 0.5) lit -= SHADE;
  }
  return clamp(lit, 0.0, 1.0);
}
void main() {
  float span = min(uRes.x, uRes.y);
  vec2 uv = (gl_FragCoord.xy + uJitter - 0.5 * uRes) / span * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  vec3 ro = uPos;
  vec3 h = vec3(uN * 0.5);
  float pix = 2.0 * uTan / span;
#ifdef HAZE
  float far = uScale * 5.5 / max(uFog * 0.25, 1e-3);
#else
  float far = uScale * SIGHT;
#endif
  vec2 shell = root(ro, rd);
  float start = shell.x * uHull;
  float t = shell.x;
  float lod = 1.0;
  float level = 1.0;
  float foot = 0.0;
  bool hit = false;
  bool gone = shell.y < shell.x;
  for (int i = 0; i < STEPS; i++) {
    if (gone || t - start > far) break;
    foot = max(t * pix, 1e-7);
    vec4 r = probe(ro + rd * t + h, rd, foot * LOD);
    lod = r.z;
    if (r.y > 0.5) {
      hit = true;
      break;
    }
    level = r.z;
    t += r.w + foot * 0.5;
    gone = t > shell.y;
  }
  vec3 col = vec3(0.0);
  float end = gone ? MISS : t;
  if (hit) {
    vec3 p = ro + rd * t;
    vec3 c = p + h;
    vec3 nr = normal(c, foot * 0.5, foot * LOD);
    vec3 an = abs(nr);
    float face = an.x > an.y && an.x > an.z ? 0.0 : an.y > an.z ? 2.0 : 1.0;
    vec3 alb = ramp(uHue + face * 0.22);
    float cells = t / uScale;
    float deep = (t - start) / uScale;
#if defined(GRAPHIC)
    vec2 f = face == 0.0 ? c.yz : face == 2.0 ? c.xz : c.xy;
    vec2 g = abs(fract(f / level + 0.5) - 0.5) * level;
    float e = min(g.x, g.y);
    float ink = (1.0 - smoothstep(WIDTH * 0.5 * foot, WIDTH * foot, e)) * smoothstep(FINE * foot, FINE * 4.0 * foot, level);
    float dim = 1.0 - smoothstep(DIM, SIGHT, deep);
    col = alb * FLAT * (1.0 - INK * ink) * dim;
#elif defined(STUDIO)
    float ao = occlusion(c, nr, lod, foot * LOD);
    vec3 key = normalize(uRot * LIGHT);
    vec3 back = normalize(uRot * BACK);
    float lit = max(dot(nr, key), 0.0);
    if (lit > 0.0) lit *= soft(c + nr * foot * 2.0, key, foot);
    float dim = 1.0 - smoothstep(DIM, SIGHT, deep);
    col = alb * ao * (lit * STRONG + max(dot(nr, back), 0.0) * FILL * STRONG + AMBIENT + uShine * max(dot(nr, uBelow), 0.0)) * dim;
#else
    float ao = occlusion(c, nr, lod, foot * LOD);
    float head = max(dot(nr, -rd), 0.0) / (1.0 + cells * cells * FALL);
    vec3 light = vec3(0.95, 0.97, 1.0) * head * 1.7 + vec3(0.035, 0.04, 0.05);
    light += vec3(1.0, 0.95, 0.88) * max(dot(nr, SUN), 0.0) * 1.3 * uSun + uShine * max(dot(nr, uBelow), 0.0);
    for (int i = 0; i < 2; i++) {
      if (uGlow[i] <= 0.0) continue;
      vec3 l = uDoor[i].xyz - p;
      float dl = max(length(l), 1e-6);
      float r = dl / (uDoor[i].w * 0.7);
      float lit = max(dot(nr, l / dl), 0.0) * uGlow[i] / (1.0 + r * r);
      if (uShadow > 0 && lit > 0.01) lit *= shade(c, l / dl, dl - foot * 3.0, foot);
      light += uAccent * lit * 2.6;
    }
    float key = KEY * (0.4 + 0.6 * max(dot(nr, SUN), 0.0)) * exp(-cells * uFog * 0.25) * (1.0 - uSun);
    col = alb * ao * (light * exp(-cells * uFog) + key);
#endif
  }
#ifdef HAZE
  for (int i = 0; i < 2; i++) {
    if (uGlow[i] <= 0.0) continue;
    vec3 q = uDoor[i].xyz - ro;
    float tc = dot(q, rd);
    if (tc <= 0.0 || tc > end) continue;
    float d2 = max(dot(q, q) - tc * tc, 0.0);
    float r = uDoor[i].w * 0.16;
    col += uAccent * uGlow[i] * exp(-d2 / (r * r)) * 0.9 * exp(-tc / uScale * uFog) * smoothstep(0.0, 1.5, length(q) / uDoor[i].w);
  }
#endif
  o = vec4(col, gone ? 0.0 : 1.0);
#ifdef AUX
  d = vec4(end, 0.0, 0.0, 1.0);
#endif
}
`;

const RESOLVE = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform sampler2D uNow;
uniform sampler2D uAux;
uniform sampler2D uPast;
uniform vec3 uPos;
uniform mat3 uRot;
uniform float uTan;
uniform vec2 uRes;
uniform vec2 uJitter;
uniform vec3 uFrom;
uniform mat3 uTurn;
uniform float uWide;
uniform float uKeep;
uniform int uSame;
uniform ivec4 uClip;
uniform vec4 uWas;
const float MISS = 30000.0;
void main() {
  ivec2 at = ivec2(gl_FragCoord.xy);
  vec4 now = texelFetch(uNow, at, 0);
  if (uKeep <= 0.0) {
    o = now;
    return;
  }
  if (uSame == 1) {
    o = mix(now, texelFetch(uPast, at, 0), uKeep);
    return;
  }
  vec4 lo = now;
  vec4 hi = now;
  for (int y = -1; y <= 1; y++) {
    for (int x = -1; x <= 1; x++) {
      vec4 c = texelFetch(uNow, clamp(at + ivec2(x, y), uClip.xy, uClip.zw - 1), 0);
      lo = min(lo, c);
      hi = max(hi, c);
    }
  }
  float span = min(uRes.x, uRes.y);
  vec2 uv = (gl_FragCoord.xy + uJitter - 0.5 * uRes) / span * 2.0 * uTan;
  vec3 rd = normalize(uRot * vec3(uv, 1.0));
  float t = texelFetch(uAux, at, 0).r;
  vec3 q = transpose(uTurn) * (t < MISS ? uPos + rd * t - uFrom : rd);
  if (q.z <= 1e-6) {
    o = now;
    return;
  }
  vec2 back = (q.xy / q.z / (2.0 * uWide) * span + 0.5 * uRes) / uRes;
  if (any(lessThan(back, uWas.xy)) || any(greaterThan(back, uWas.zw))) {
    o = now;
    return;
  }
  o = mix(now, clamp(texture(uPast, back), lo, hi), uKeep);
}
`;

const LAYER = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform sampler2D uSrc;
uniform float uK;
void main() {
  o = texture(uSrc, v) * uK;
}
`;

const source = (look, budget, aux) => `#version 300 es\n#define ${look.toUpperCase()}\n#define STEPS ${budget.steps}\n#define LOD ${(aux ? LOD : RAW).toFixed(1)}\n${aux ? '#define AUX\n' : ''}${MARCH}`;

const budget = (view, quality = '') => QUALITY[quality] ?? (tier(view) === 'phone' ? QUALITY.phone : QUALITY.desk);

export function march(gl, view, design, { look = 'studio', quality = '', toFrame = null } = {}) {
  const { n } = design;
  const made = boxes(cells(design.code, n), n);
  const plan = budget(view, quality);
  const taa = hdr(gl);
  const lo = texture(gl, { w: n, h: n, d: n, data: made.lo, filter: 'nearest' });
  const hi = texture(gl, { w: n, h: n, d: n, data: made.hi, filter: 'nearest' });
  const shot = program(gl, source(LOOKS.includes(look) ? look : 'studio', plan, taa));
  const resolve = taa ? program(gl, RESOLVE) : null;
  const paint = program(gl, LAYER);
  const raw = target(gl, 1, 1, { hdr: true, extra: taa ? 1 : 0 });
  const past = taa ? [target(gl, 1, 1, { hdr: true }), target(gl, 1, 1, { hdr: true })] : [];
  shot.set({ uN: n, uDepth: plan.depth, uTaps: plan.taps, uShadow: plan.shadow, uLo: lo, uHi: hi });
  let clock = view.t;
  let count = 0;
  let side = 0;
  let kept = false;
  let last = null;
  let shown = null;
  let seen = null;
  const scale = () => Math.min(1, Math.sqrt(plan.cap / (view.w * view.h)));
  const fit = () => {
    const k = scale();
    const [w, h] = [Math.max(1, Math.round(view.w * k)), Math.max(1, Math.round(view.h * k))];
    if (raw.w === w && raw.h === h) return true;
    raw.size(w, h);
    for (const one of past) one.size(w, h);
    return false;
  };
  const from = (frame) => {
    if (!last) return null;
    if (last.frame === frame) return last.cam.pos;
    if (!toFrame) return null;
    const p = toFrame({ j: last.frame, p: last.cam.pos }, frame);
    return p.every(Number.isFinite) ? p : null;
  };
  const clip = (cam, frame) => {
    const whole = [0, 0, raw.w, raw.h];
    const low = frame.edge?.[0] ?? OPEN;
    const high = frame.edge?.[1] ?? OPEN;
    if ([...low, ...high].some((v) => v >= BEYOND)) return whole;
    const corners = Array.from({ length: 8 }, (_, i) => [0, 1, 2].map((j) => ((i >> j) & 1 ? (high[j] + 1) * n : -low[j] * n) - n / 2));
    const dots = corners.map((c) => project(raw, cam, c));
    if (dots.some((p) => !p)) return whole;
    const xs = dots.map((p) => p[0]);
    const ys = dots.map((p) => raw.h - p[1]);
    const pin = (v, top) => Math.min(Math.max(v, 0), top);
    return [pin(Math.floor(Math.min(...xs)) - BORDER, raw.w), pin(Math.floor(Math.min(...ys)) - BORDER, raw.h), pin(Math.ceil(Math.max(...xs)) + BORDER, raw.w), pin(Math.ceil(Math.max(...ys)) + BORDER, raw.h)];
  };
  const was = ([x0, y0, x1, y1]) => [x0 > 0 ? (x0 + 1) / raw.w : 0, y0 > 0 ? (y0 + 1) / raw.h : 0, x1 < raw.w ? (x1 - 1) / raw.w : 1, y1 < raw.h ? (y1 - 1) / raw.h : 1];
  const draw = (cam, frame, tint) => {
    const port = gl.getParameter(gl.VIEWPORT);
    const same = fit();
    const dt = view.t - clock;
    clock = view.t;
    const index = frame.frame ?? 0;
    const kind = Boolean(frame.outside);
    const box = clip(cam, frame);
    const prev = kept && same && dt >= 0 && dt <= STEP && last.kind === kind ? from(index) : null;
    if (box[2] <= box[0] || box[3] <= box[1]) {
      seen = null;
      kept = false;
      return;
    }
    const doors = frame.doors ?? [];
    const pad = (k) => doors[k] ?? { pos: [0, 0, 0], scale: 1, glow: 0 };
    const eye = uniforms({ w: raw.w, h: raw.h }, cam);
    blend(gl, null);
    shot.set({
      ...eye,
      uScale: frame.scale,
      uFog: frame.fog ?? 0,
      uSun: frame.sun ?? (kind ? 1 : 0),
      uHull: frame.hull ?? 0,
      uFloor: frame.floor ?? 0,
      uShine: frame.shine?.color ?? DARK,
      uBelow: frame.shine?.dir ?? DOWN,
      uDoor: [0, 1].flatMap((k) => [...pad(k).pos, pad(k).scale]),
      uGlow: [0, 1].map((k) => pad(k).glow),
      uAccent: tint.accent,
      uRamp: tint.ramp.flat(),
      uHue: frame.hue ?? HUE,
      uNear: frame.near ?? NONE,
      uLow: frame.edge?.[0] ?? OPEN,
      uHigh: frame.edge?.[1] ?? OPEN,
    });
    const passes = taa && !prev && (view.fixed || view.still) ? CYCLE : 1;
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(box[0], box[1], box[2] - box[0], box[3] - box[1]);
    for (let i = 0; i < passes; i++) {
      const jitter = taa ? HALTON[count++ % CYCLE] : [0, 0];
      gl.bindFramebuffer(gl.FRAMEBUFFER, raw.fb);
      gl.viewport(0, 0, raw.w, raw.h);
      shot.set({ uJitter: jitter });
      fill(gl);
      if (!taa) continue;
      const into = past[side];
      gl.bindFramebuffer(gl.FRAMEBUFFER, into.fb);
      resolve.set({
        ...eye,
        uJitter: jitter,
        uNow: raw,
        uAux: raw.aux,
        uPast: past[1 - side],
        uFrom: prev ?? cam.pos,
        uTurn: (last ?? { cam }).cam.rot,
        uWide: Math.tan((last ?? { cam }).cam.fov / 2),
        uKeep: prev ? KEEP : i / (i + 1),
        uSame: prev ? 0 : 1,
        uClip: box,
        uWas: was(prev ? last.box : box),
      });
      fill(gl);
      side = 1 - side;
    }
    gl.disable(gl.SCISSOR_TEST);
    shown = taa ? past[1 - side] : raw;
    seen = box;
    kept = taa;
    last = { cam: { pos: [...cam.pos], rot: cam.rot, fov: cam.fov }, frame: index, kind, box };
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(port[0], port[1], port[2], port[3]);
  };
  const layer = (k = 1) => {
    if (!shown || !seen) return;
    const port = gl.getParameter(gl.VIEWPORT);
    const [sx, sy] = [port[2] / raw.w, port[3] / raw.h];
    const [x0, y0, x1, y1] = seen;
    const left = x0 > 0 ? Math.ceil((x0 + 1) * sx) : 0;
    const bottom = y0 > 0 ? Math.ceil((y0 + 1) * sy) : 0;
    const right = x1 < raw.w ? Math.floor((x1 - 1) * sx) : port[2];
    const top = y1 < raw.h ? Math.floor((y1 - 1) * sy) : port[3];
    if (right <= left || top <= bottom) return;
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(port[0] + left, port[1] + bottom, right - left, top - bottom);
    blend(gl, 'alpha');
    paint.set({ uSrc: shown, uK: k });
    fill(gl);
    blend(gl, null);
    gl.disable(gl.SCISSOR_TEST);
  };
  const depth = () => (taa && shown ? raw.aux : null);
  const drop = () => {
    for (const one of [shot, resolve, paint]) one?.drop();
    for (const one of [raw, ...past]) one.drop();
    gl.deleteTexture(lo);
    gl.deleteTexture(hi);
  };
  return { draw, layer, depth, scale, drop };
}
