import { blend, bright, rgb } from "../color.ts";
import type { Uniforms } from "../gl/gl2.ts";
import { rng } from "../rand.ts";
import { ESCAPE, MIX, SHARP } from "./consts.ts";

/* TYPES */

export type Tone = (accent: string, paper: string) => Uniforms;

const DIM = 0.14;
const LACE = 0.3;
const LIN = 1 / 128;
const PHASE = 3;
const STRIDE = 0.15;
const SHIFT = 2;
const WASH = 0.18;
const DEEP = 0.55;

/* PROGRAMS */

export const COPY = `#version 300 es
precision highp float;
precision highp sampler2D;
uniform sampler2D uTex;
uniform vec2 uRes;
out vec4 o;
void main() { o = texture(uTex, gl_FragCoord.xy / uRes); }
`;

export const LINE = `#version 300 es
in vec2 aA;
in vec2 aB;
in vec4 aShape;
in vec4 aInk;
uniform vec2 uRes;
out vec2 vP;
flat out vec2 vA;
flat out vec2 vB;
flat out vec4 vShape;
flat out vec4 vInk;
void main() {
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1));
  float pad = aShape.x + aShape.y + 1.0;
  vP = mix(min(aA, aB) - pad, max(aA, aB) + pad, c);
  vA = aA;
  vB = aB;
  vShape = aShape;
  vInk = aInk;
  gl_Position = vec4(vP / uRes * 2.0 - 1.0, 0.0, 1.0);
}
`;

export const INK = `#version 300 es
precision highp float;
in vec2 vP;
flat in vec2 vA;
flat in vec2 vB;
flat in vec4 vShape;
flat in vec4 vInk;
out vec4 o;
void main() {
  vec2 ab = vB - vA;
  float k = clamp(dot(vP - vA, ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
  float d = length(vP - vA - ab * k);
  float stroke = clamp(vShape.y + 0.5 - abs(d - vShape.x), 0.0, 1.0);
  float inside = vShape.z * clamp(vShape.x + 0.5 - d, 0.0, 1.0);
  o = vInk * max(stroke, inside);
}
`;

/* SHADER */

export const shader = (probe: boolean, classic: boolean) => `#version 300 es
precision highp float;
precision highp int;
precision highp sampler2D;
uniform sampler2D uOrbit;
uniform int uLen;
uniform int uCrit;
uniform int uCritLen;
uniform int uMax;
uniform bool uJulia;
uniform vec2 uRes;
uniform vec2 uOff;
uniform vec2 uU;
uniform vec2 uV;
${probe ? "" : "uniform float uAlpha;\n"}${
  probe || classic
    ? ""
    : `uniform bool uRamp;
uniform vec3 uA;
uniform vec3 uB;
uniform vec3 uC;
uniform vec3 uD;
uniform vec3 uS[4];
uniform float uFreq;
uniform float uDrift;
uniform float uPx;
uniform float uFall;
`
}${
  classic
    ? `uniform vec3 uGround;
uniform vec3 uInside;
uniform vec3 uAccent;
uniform float uTime;
`
    : ""
}out vec4 o;
vec2 at(int k) { return texelFetch(uOrbit, ivec2(k & 4095, k >> 12), 0).xy; }
void main() {
  vec2 p = gl_FragCoord.xy - 0.5 * uRes;
  vec2 dc = uOff + p.x * uU + p.y * uV;
  vec2 d = uJulia ? dc : vec2(0.0);
  vec2 e = uJulia ? vec2(0.0) : dc;
  int base = 0;
  int len = uLen;
  int m = 0;
  vec2 Z = at(0);
  vec2 w = Z + d;
${
  probe || classic
    ? ""
    : `  vec2 dz = uJulia ? vec2(uPx, 0.0) : vec2(0.0);
  vec2 kick = uJulia ? vec2(0.0) : vec2(uPx, 0.0);
`
}  float r = 0.0;
  int n = 0;
  bool gone = false;
  float low = 3.0e38;
  int per = 0;
  for (int i = 0; i < uMax; i++) {
${
  probe || classic
    ? ""
    : `    dz = 2.0 * vec2(w.x * dz.x - w.y * dz.y, w.x * dz.y + w.y * dz.x) + kick;
`
}    vec2 t = 2.0 * Z + d;
    d = vec2(t.x * d.x - t.y * d.y, t.x * d.y + t.y * d.x) + e;
    m++;
    Z = at(base + m);
    vec2 z = Z + d;
    w = z;
    r = dot(z, z);
    n = i + 1;
    if (r > ${ESCAPE.toFixed(1)}) {
      gone = true;
      break;
    }
${
  probe
    ? `    if (r < low) {
      if (per == 0 || n % per != 0 || r < ${SHARP} * low) per = n;
      low = r;
    }
`
    : ""
}    if (max(abs(z.x), abs(z.y)) < max(abs(d.x), abs(d.y)) || m >= len - 1) {
      d = z;
      m = 0;
      base = uCrit;
      len = uCritLen;
      Z = at(base);
    }
  }
  float sl = float(n) + 1.0 - log2(max(0.5 * log2(max(r, 1.0)), 1e-6));
${
  probe
    ? `  int q = gone ? int(clamp(sl * 8.0, 0.0, 65535.0)) : 0;
  per = min(per, 32767);
  o = vec4(float(q >> 8), float(q & 255), float((gone ? 0 : 128) + (per >> 8)), float(per & 255)) / 255.0;
`
    : classic
      ? `  if (!gone) {
    o = vec4(uInside * uAlpha, uAlpha);
    return;
  }
  float k = 0.5 + 0.5 * cos(${PHASE.toFixed(1)} + ${STRIDE.toFixed(2)} * (sl + ${SHIFT.toFixed(1)}) + uTime);
  o = vec4(mix(uGround, uAccent, k) * uAlpha, uAlpha);
`
      : `  if (!gone) {
    o = vec4(0.0, 0.0, 0.0, uAlpha);
    return;
  }
  float x = log(max(sl, 1.0)) + sl * ${LIN.toFixed(6)};
  vec3 col;
  if (uRamp) {
    float k = fract(x * uFreq + uDrift);
    k = (1.0 - abs(2.0 * k - 1.0)) * 3.0;
    col = k < 1.0 ? mix(uS[0], uS[1], k) : k < 2.0 ? mix(uS[1], uS[2], k - 1.0) : mix(uS[2], uS[3], k - 2.0);
  } else {
    col = uA + uB * cos(6.2831853 * (uC * x + uD + uDrift));
  }
  float de = 0.5 * sqrt(r) * log(r) / max(length(dz), 1e-30);
  col *= mix(${DIM.toFixed(2)}, 1.0, exp(-de * uFall));
  col = mix(col, mix(col, vec3(1.0), ${LACE.toFixed(2)}), 1.0 - smoothstep(0.2, 0.9, de));
  col = clamp(col, 0.0, 1.0);
  o = vec4(col * uAlpha, uAlpha);
`
}}
`;

/* COLOUR */

export const glow = (sl: number) => 0.5 + 0.5 * Math.cos(PHASE + STRIDE * (sl + SHIFT));

const RAMPS: Record<string, string[] | undefined> = {
  fire: ["#000000", "#9a2005", "#f06a12", "#ffe79a"],
  ice: ["#000000", "#0d4f8c", "#47c3ee", "#f2fdff"],
  mono: ["#000000", "#4a4a4a", "#a2a2a2", "#ffffff"],
};

export const unit = (hex: string) => rgb(hex).map((v) => v / 255);

export function tones(name: string, seed: number): Tone {
  if (name === "classic")
    return (accent, paper) =>
      bright(paper)
        ? {
            uGround: unit(blend(paper, accent, WASH)),
            uInside: unit(blend(accent, "#000000", DEEP)),
            uAccent: unit(accent),
          }
        : { uGround: unit(paper), uInside: unit(paper), uAccent: unit(accent) };
  if (RAMPS[name]) return () => ({ uRamp: true, uS: RAMPS[name]!.flatMap(unit) });
  if (name === "accent") {
    return (accent) => {
      const a = unit(accent);
      return { uRamp: true, uS: [[0, 0, 0], a.map((v) => v * 0.55), a, a.map((v) => v + (1 - v) * 0.75)].flat() };
    };
  }
  const rand = rng((seed ^ MIX) >>> 0);
  const f = 0.6 + 0.6 * rand();
  const d0 = rand();
  const spread = 0.05 + 0.15 * rand();
  return () => ({
    uRamp: false,
    uA: [0.5, 0.5, 0.5],
    uB: [0.45, 0.45, 0.45],
    uC: [f, f, f],
    uD: [d0, d0 + spread, d0 + 2 * spread],
  });
}
