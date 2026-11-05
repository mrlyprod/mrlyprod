import { Color, ShaderMaterial, Vector2 } from "three";
import { mixRgb, type Rgb, type Theme } from "./theme.ts";

const SPAN = 2;

const VERTEX = `
attribute float aLit;
attribute float aLand;
varying float vLit;
varying float vLand;
varying float vDepth;
varying vec3 vNormal;
void main() {
  vec3 p = position;
  vec3 n = normal;
  #ifdef USE_INSTANCING
  p = (instanceMatrix * vec4(p, 1.0)).xyz;
  n = mat3(instanceMatrix) * n;
  #endif
  vLit = aLit;
  vLand = aLand;
  vNormal = normalize(normalMatrix * n);
  vec4 mv = modelViewMatrix * vec4(p, 1.0);
  vDepth = -mv.z;
  gl_Position = projectionMatrix * mv;
}`;

const FRAGMENT = `
uniform vec3 uSea;
uniform vec3 uLand;
uniform vec3 uGlow;
uniform vec2 uFog;
varying float vLit;
varying float vLand;
varying float vDepth;
varying vec3 vNormal;
void main() {
  vec3 n = normalize(vNormal);
  float key = 0.58 + 0.42 * max(dot(n, normalize(vec3(-0.4, 0.55, 1.0))), 0.0);
  float fog = 1.0 - 0.55 * clamp((vDepth - uFog.x) / uFog.y, 0.0, 1.0);
  vec3 base = mix(uSea, uLand, vLand);
  vec3 color = mix(base, uGlow, clamp(vLit, 0.0, 1.0));
  gl_FragColor = vec4(color * key * fog, 1.0);
}`;

const rgb = (c: Rgb) => new Color(c[0], c[1], c[2]);

export function tileMaterial(theme: Theme): ShaderMaterial {
  const material = new ShaderMaterial({
    vertexShader: VERTEX,
    fragmentShader: FRAGMENT,
    uniforms: {
      uSea: { value: new Color() },
      uLand: { value: new Color() },
      uGlow: { value: new Color() },
      uFog: { value: new Vector2(2, SPAN) },
    },
  });
  paint(material, theme);
  return material;
}

export function fog(material: ShaderMaterial, distance: number): void {
  material.uniforms.uFog.value.set(distance - SPAN / 2, SPAN);
}

export function paint(material: ShaderMaterial, theme: Theme): void {
  const { bg, fg, accent, line } = theme;
  material.uniforms.uSea.value = rgb(mixRgb(mixRgb(bg, line, 0.85), accent, theme.dark ? 0.22 : 0.18));
  material.uniforms.uLand.value = rgb(mixRgb(mixRgb(bg, fg, theme.dark ? 0.8 : 0.72), accent, 0.1));
  material.uniforms.uGlow.value = rgb(accent);
}
