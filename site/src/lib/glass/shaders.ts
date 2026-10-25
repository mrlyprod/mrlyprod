/* VERTEX */

export const QUAD = `#version 300 es
in vec2 a;
uniform vec4 uBox;
out vec2 vUv;
void main() {
  vUv = a;
  gl_Position = vec4(mix(uBox.xy, uBox.zw, a), 0.0, 1.0);
}`;

/* COPY */

export const COPY = `#version 300 es
precision highp float;
uniform sampler2D uTex;
uniform vec4 uUv;
in vec2 vUv;
out vec4 o;
void main() {
  o = texture(uTex, uUv.xy + vUv * uUv.zw);
}`;

/* DOWN */

export const DOWN = `#version 300 es
precision highp float;
uniform sampler2D uTex;
uniform vec2 uStep;
in vec2 vUv;
out vec4 o;
void main() {
  vec2 s = uStep;
  o = (texture(uTex, vUv) * 4.0 + texture(uTex, vUv - s) + texture(uTex, vUv + s) + texture(uTex, vUv + vec2(s.x, -s.y)) + texture(uTex, vUv - vec2(s.x, -s.y))) / 8.0;
}`;

/* LENS */

export const LENS = `#version 300 es
precision highp float;
uniform sampler2D uScene;
uniform sampler2D uLo;
uniform sampler2D uHi;
uniform vec2 uView;
uniform vec4 uRect;
uniform float uDpr;
uniform vec4 uRadii;
uniform float uBezel;
uniform float uDepth;
uniform float uThick;
uniform float uRefract;
uniform float uMix;
uniform float uLerp;
uniform float uSat;
uniform float uSpec;
uniform float uChroma;
uniform float uRim;
uniform vec4 uTint;
uniform vec2 uLight;
out vec4 o;

float box(vec2 p, vec2 b, vec4 r) {
  r.xy = p.x > 0.0 ? r.xy : r.zw;
  r.x = p.y > 0.0 ? r.x : r.y;
  vec2 q = abs(p) - b + r.x;
  return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r.x;
}

float rise(float s) {
  float u = 1.0 - s;
  return pow(1.0 - u * u * u * u, 0.25);
}

float slope(float s) {
  float u = 1.0 - s;
  return u * u * u * pow(max(1.0 - u * u * u * u, 1e-4), -0.75);
}

vec4 cubic(sampler2D t, vec2 uv) {
  vec2 sz = vec2(textureSize(t, 0));
  vec2 p = uv * sz - 0.5;
  vec2 i = floor(p);
  vec2 f = p - i;
  vec2 f2 = f * f;
  vec2 f3 = f2 * f;
  vec2 w0 = (1.0 - 3.0 * f + 3.0 * f2 - f3) / 6.0;
  vec2 w1 = (4.0 - 6.0 * f2 + 3.0 * f3) / 6.0;
  vec2 w2 = (1.0 + 3.0 * f + 3.0 * f2 - 3.0 * f3) / 6.0;
  vec2 w3 = f3 / 6.0;
  vec2 g0 = w0 + w1;
  vec2 g1 = w2 + w3;
  vec2 h0 = (i - 0.5 + w1 / g0) / sz;
  vec2 h1 = (i + 1.5 + w3 / g1) / sz;
  return g0.y * (g0.x * texture(t, h0) + g1.x * texture(t, vec2(h1.x, h0.y))) + g1.y * (g0.x * texture(t, vec2(h0.x, h1.y)) + g1.x * texture(t, h1));
}

void main() {
  vec2 px = vec2(gl_FragCoord.x, uRect.w * uDpr - gl_FragCoord.y) / uDpr;
  vec2 hb = uRect.zw * 0.5;
  vec2 p = px - hb;
  vec4 r = min(uRadii, vec4(min(hb.x, hb.y)));
  float sd = box(p, hb, r);
  float a = clamp(0.5 - sd * uDpr, 0.0, 1.0);
  if (a <= 0.0) {
    o = vec4(0.0);
    return;
  }
  float d = -sd;
  float e = 0.5;
  vec2 g = vec2(box(p + vec2(e, 0.0), hb, r) - box(p - vec2(e, 0.0), hb, r), box(p + vec2(0.0, e), hb, r) - box(p - vec2(0.0, e), hb, r));
  vec2 side = length(g) > 1e-5 ? normalize(g) : vec2(0.0);
  vec3 n = vec3(0.0, 0.0, 1.0);
  vec2 shift = vec2(0.0);
  float bezel = min(uBezel, min(hb.x, hb.y));
  if (d < bezel && bezel > 0.0) {
    float s = clamp(d / bezel, 1e-3, 1.0);
    n = normalize(vec3(side * slope(s) * uDepth / bezel, 1.0));
    vec3 t = refract(vec3(0.0, 0.0, -1.0), n, 1.0 / 1.5);
    shift = t.xy * ((uDepth * rise(s) + uThick) / max(-t.z, 1e-3)) * uRefract;
  }
  vec2 uv = (uRect.xy + px + shift) / uView;
  vec2 c = shift * uChroma / uView;
  vec3 sharp = vec3(texture(uScene, uv - c).r, texture(uScene, uv).g, texture(uScene, uv + c).b);
  vec3 soft = mix(cubic(uLo, uv).rgb, cubic(uHi, uv).rgb, uLerp);
  vec3 col = mix(sharp, soft, uMix);
  float luma = dot(col, vec3(0.2126, 0.7152, 0.0722));
  col = mix(vec3(luma), col, uSat);
  col = mix(col, uTint.rgb, uTint.a);
  float rim = 1.0 - smoothstep(0.0, uRim, d);
  float l = dot(side, uLight);
  col += rim * (max(l, 0.0) + 0.4 * max(-l, 0.0)) * uSpec;
  col = mix(col, vec3(1.0), pow(1.0 - n.z, 4.0) * 0.15);
  o = vec4(clamp(col, 0.0, 1.0) * a, a);
}`;
