import { blend, fill, hdr, program, target, tier } from './gl2.js';

const LOOK = { ev: 0, bloom: 1, aberration: 0, vignette: 0.3, grain: 0.015, flare: 0, fade: 1 };
const KNEE = [1, 0.5];
const DIM = [0.6, 0.3];
const TINT = [0.033, 0.181, 1];
const FRAME = 16.67;
const LOOP = 997;

const HEAD = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform sampler2D uSrc;
uniform vec2 uTexel;
vec3 at(vec2 p) { return texture(uSrc, v + p * uTexel).rgb; }
`;

const TAPS = `
  vec3 a = tap(vec2(-2.0, 2.0)), b = tap(vec2(0.0, 2.0)), c = tap(vec2(2.0, 2.0));
  vec3 d = tap(vec2(-2.0, 0.0)), e = tap(vec2(0.0)), f = tap(vec2(2.0, 0.0));
  vec3 g = tap(vec2(-2.0, -2.0)), h = tap(vec2(0.0, -2.0)), i = tap(vec2(2.0, -2.0));
  vec3 j = tap(vec2(-1.0, 1.0)), k = tap(vec2(1.0, 1.0)), l = tap(vec2(-1.0, -1.0)), m = tap(vec2(1.0, -1.0));
`;

const BRIGHT = `${HEAD}
uniform float uGain;
uniform vec2 uKnee;
vec3 tap(vec2 p) { return min(max(at(p), vec3(0.0)) * uGain, vec3(6e4)); }
vec4 group(vec3 p, vec3 q, vec3 r, vec3 s, float w) {
  vec3 c = (p + q + r + s) * 0.25;
  float k = w / (1.0 + dot(c, vec3(0.2126, 0.7152, 0.0722)));
  return vec4(c * k, k);
}
void main() {
${TAPS}
  vec4 sum = group(j, k, l, m, 0.5) + group(a, b, d, e, 0.125) + group(b, c, e, f, 0.125) + group(d, e, g, h, 0.125) + group(e, f, h, i, 0.125);
  vec3 col = sum.rgb / max(sum.a, 1e-5);
  float br = max(col.r, max(col.g, col.b));
  float soft = clamp(br - uKnee.x + uKnee.y, 0.0, 2.0 * uKnee.y);
  soft = soft * soft / (4.0 * uKnee.y + 1e-5);
  o = vec4(col * max(soft, br - uKnee.x) / max(br, 1e-5), 1.0);
}
`;

const DOWN = `${HEAD}
vec3 tap(vec2 p) { return at(p); }
void main() {
${TAPS}
  o = vec4(e * 0.125 + (a + c + g + i) * 0.03125 + (b + d + f + h) * 0.0625 + (j + k + l + m) * 0.125, 1.0);
}
`;

const UP = `${HEAD}
void main() {
  vec3 s = at(vec2(-1.0, 1.0)) + at(vec2(1.0, 1.0)) + at(vec2(-1.0, -1.0)) + at(vec2(1.0, -1.0));
  s += (at(vec2(0.0, 1.0)) + at(vec2(-1.0, 0.0)) + at(vec2(1.0, 0.0)) + at(vec2(0.0, -1.0))) * 2.0;
  o = vec4((s + at(vec2(0.0)) * 4.0) / 16.0, 1.0);
}
`;

const SMEAR = `${HEAD}
void main() {
  vec3 s = vec3(0.0);
  float w = 0.0;
  for (int i = -24; i <= 24; i++) {
    float k = exp(-3.0 * abs(float(i)) / 24.0);
    s += at(vec2(float(i) * 2.0, 0.0)) * k;
    w += k;
  }
  o = vec4(s / w, 1.0);
}
`;

const FINISH = `#version 300 es
precision highp float;
in vec2 v;
out vec4 o;
uniform sampler2D uScene;
uniform sampler2D uBloom;
uniform sampler2D uWide;
uniform sampler2D uStreak;
uniform vec2 uRes;
uniform float uGain;
uniform float uGlow;
uniform float uAberration;
uniform float uVignette;
uniform float uGrain;
uniform float uFlare;
uniform float uFade;
uniform float uFrame;
uniform vec3 uTint;
vec3 aces(vec3 x) { return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0); }
float hash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * 0.1031);
  q += dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}
vec3 flare() {
  vec3 f = texture(uStreak, v).rgb * uTint;
  for (int i = 0; i < 3; i++) {
    vec2 g = 0.5 + (v - 0.5) * (-0.6 - 0.5 * float(i));
    f += texture(uWide, g).rgb * (1.0 - smoothstep(0.0, 0.75, length(g - 0.5))) * mix(uTint, vec3(1.0), 0.4) * 0.15;
  }
  return f;
}
void main() {
  vec2 px = (v - 0.5) * uRes;
  float r = length(px) / (0.5 * length(uRes));
  vec2 shift = px / max(length(px), 1e-4) * uAberration * uRes.x * r * r / uRes;
  vec3 c = vec3(texture(uScene, v + shift).r, texture(uScene, v).g, texture(uScene, v - shift).b) * uGain;
  c += texture(uBloom, v).rgb * uGlow;
  if (uFlare > 0.0) c += flare() * uFlare;
  c = pow(aces(max(c, vec3(0.0))), vec3(1.0 / 2.2));
  c *= mix(1.0 - uVignette, 1.0, 1.0 - smoothstep(0.35, 1.2, length((v - 0.5) * vec2(uRes.x / uRes.y, 1.0))));
  c += (hash(gl_FragCoord.xy + vec2(7.13, 3.71) * uFrame) - 0.5) * uGrain;
  c += (hash(gl_FragCoord.xy * 1.37 + 0.5) - 0.5) / 255.0;
  o = vec4(clamp(c * uFade, 0.0, 1.0), 1.0);
}
`;

export function post(gl, view) {
  const high = hdr(gl);
  const phone = tier(view) === 'phone';
  const levels = phone ? 4 : 5;
  const first = phone ? 4 : 2;
  const wide = phone ? 1 : 2;
  const [threshold, knee] = high ? KNEE : DIM;
  const streak = target(gl, view.w / (first << wide), view.h / (first << wide), { hdr: high });
  const scene = target(gl, view.w, view.h, { hdr: high });
  const mips = Array.from({ length: levels }, (_, i) => target(gl, view.w / (first << i), view.h / (first << i), { hdr: high }));
  const bright = program(gl, BRIGHT);
  const down = program(gl, DOWN);
  const up = program(gl, UP);
  const smear = program(gl, SMEAR);
  const finish = program(gl, FINISH);
  const texel = (t) => [1 / t.w, 1 / t.h];
  const pass = (to, prog, values) => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, to.fb);
    gl.viewport(0, 0, to.w, to.h);
    prog.set(values);
    fill(gl);
  };
  const begin = (scale = 1) => {
    const k = Math.min(Math.max(scale, 0.1), 1);
    scene.size(view.w * k, view.h * k);
    mips.forEach((mip, i) => mip.size(scene.w / (first << i), scene.h / (first << i)));
    streak.size(mips[wide].w, mips[wide].h);
    gl.bindFramebuffer(gl.FRAMEBUFFER, scene.fb);
    gl.viewport(0, 0, scene.w, scene.h);
    blend(gl, null);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    return scene;
  };
  const end = (look = {}) => {
    const L = { ...LOOK, ...look };
    const gain = 2 ** L.ev;
    const glow = L.bloom > 0 || L.flare > 0;
    blend(gl, null);
    if (glow) {
      pass(mips[0], bright, { uSrc: scene, uTexel: texel(scene), uGain: gain, uKnee: [threshold, knee] });
      for (let i = 1; i < levels; i++) pass(mips[i], down, { uSrc: mips[i - 1], uTexel: texel(mips[i - 1]) });
      if (L.flare > 0) pass(streak, smear, { uSrc: mips[wide], uTexel: texel(mips[wide]) });
      blend(gl, 'add');
      for (let i = levels - 2; i >= 0; i--) pass(mips[i], up, { uSrc: mips[i + 1], uTexel: texel(mips[i + 1]) });
      blend(gl, null);
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    finish.set({
      uScene: scene,
      uBloom: mips[0],
      uWide: mips[wide],
      uStreak: streak,
      uRes: [view.w, view.h],
      uGain: gain,
      uGlow: glow ? L.bloom / levels : 0,
      uAberration: L.aberration,
      uVignette: L.vignette,
      uGrain: view.fixed ? 0 : L.grain,
      uFlare: glow ? L.flare : 0,
      uFade: L.fade,
      uFrame: Math.floor(view.t / FRAME) % LOOP,
      uTint: TINT,
    });
    fill(gl);
  };
  const drop = () => {
    for (const prog of [bright, down, up, smear, finish]) prog.drop();
    for (const t of [streak, scene, ...mips]) t.drop();
  };
  return { begin, end, drop };
}
