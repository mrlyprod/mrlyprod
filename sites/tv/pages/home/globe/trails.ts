import {
  BufferAttribute,
  BufferGeometry,
  Color,
  DoubleSide,
  DynamicDrawUsage,
  Group,
  Mesh,
  Points,
  ShaderMaterial,
  Vector2,
  Vector3,
} from "three";
import type { Body } from "./body.ts";
import type { Trip } from "./pretend.ts";
import type { Rgb, Theme } from "./theme.ts";

export type Trails = {
  object: Group;
  update: (trips: Trip[], time: number, body: Body, fade: number) => void;
  resize: (width: number, height: number, dpr: number) => void;
  paint: (theme: Theme) => void;
  style: (width: number, xray: boolean) => void;
  dispose: () => void;
};

const MAX = 128;
const SAMPLES = 16;
const HEAD = 9;

const RIBBON_VERTEX = `
attribute vec3 aNext;
attribute float aSide;
attribute float aAlpha;
uniform vec2 uResolution;
uniform float uWidth;
varying float vAlpha;
varying float vSide;
void main() {
  vec4 cur = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  vec4 nxt = projectionMatrix * modelViewMatrix * vec4(aNext, 1.0);
  vec2 dir = nxt.xy / nxt.w * uResolution - cur.xy / cur.w * uResolution;
  float len = length(dir);
  dir = len > 0.0001 ? dir / len : vec2(1.0, 0.0);
  vec2 nrm = vec2(-dir.y, dir.x);
  float width = uWidth * (0.3 + 0.7 * aAlpha);
  cur.xy += nrm * aSide * width / uResolution * cur.w;
  vAlpha = aAlpha;
  vSide = aSide;
  gl_Position = cur;
}`;

const RIBBON_FRAGMENT = `
uniform vec3 uColor;
varying float vAlpha;
varying float vSide;
void main() {
  float edge = 1.0 - smoothstep(0.4, 1.0, abs(vSide));
  gl_FragColor = vec4(uColor, vAlpha * edge);
}`;

const HEAD_VERTEX = `
attribute float aAlpha;
uniform float uSize;
varying float vAlpha;
void main() {
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  gl_PointSize = uSize * (0.6 + 0.4 * aAlpha);
  vAlpha = aAlpha;
}`;

const HEAD_FRAGMENT = `
uniform vec3 uColor;
uniform vec3 uCore;
varying float vAlpha;
void main() {
  float r = length(gl_PointCoord - 0.5) * 2.0;
  float core = 1.0 - smoothstep(0.0, 0.45, r);
  float halo = (1.0 - smoothstep(0.25, 1.0, r)) * 0.5;
  gl_FragColor = vec4(mix(uColor, uCore, core), (core + halo) * vAlpha);
}`;

const rgb = (c: Rgb) => new Color(c[0], c[1], c[2]);

const dynamic = (size: number, parts: number) => {
  const one = new BufferAttribute(new Float32Array(size * parts), parts);
  one.setUsage(DynamicDrawUsage);
  return one;
};

export function trails(): Trails {
  const verts = MAX * SAMPLES * 2;
  const ribbon = new BufferGeometry();
  const position = dynamic(verts, 3);
  const next = dynamic(verts, 3);
  const alpha = dynamic(verts, 1);
  const side = new Float32Array(verts);
  for (let i = 0; i < verts; i++) side[i] = i % 2 ? 1 : -1;
  ribbon.setAttribute("position", position);
  ribbon.setAttribute("aNext", next);
  ribbon.setAttribute("aAlpha", alpha);
  ribbon.setAttribute("aSide", new BufferAttribute(side, 1));
  const index: number[] = [];
  for (let i = 0; i < MAX; i++) {
    for (let s = 0; s < SAMPLES - 1; s++) {
      const b = (i * SAMPLES + s) * 2;
      index.push(b, b + 1, b + 2, b + 1, b + 3, b + 2);
    }
  }
  ribbon.setIndex(index);
  const ribbonMaterial = new ShaderMaterial({
    vertexShader: RIBBON_VERTEX,
    fragmentShader: RIBBON_FRAGMENT,
    uniforms: { uResolution: { value: new Vector2(1, 1) }, uWidth: { value: 2 }, uColor: { value: new Color() } },
    transparent: true,
    depthWrite: false,
    side: DoubleSide,
  });
  const heads = new BufferGeometry();
  const headPosition = dynamic(MAX, 3);
  const headAlpha = dynamic(MAX, 1);
  heads.setAttribute("position", headPosition);
  heads.setAttribute("aAlpha", headAlpha);
  const headMaterial = new ShaderMaterial({
    vertexShader: HEAD_VERTEX,
    fragmentShader: HEAD_FRAGMENT,
    uniforms: { uSize: { value: HEAD }, uColor: { value: new Color() }, uCore: { value: new Color() } },
    transparent: true,
    depthWrite: false,
  });
  const object = new Group();
  const ribbonMesh = new Mesh(ribbon, ribbonMaterial);
  const headMesh = new Points(heads, headMaterial);
  ribbonMesh.frustumCulled = headMesh.frustumCulled = false;
  ribbonMesh.renderOrder = 1;
  headMesh.renderOrder = 2;
  object.add(ribbonMesh, headMesh);
  const points = Array.from({ length: SAMPLES }, () => new Vector3());
  const tail = new Vector3();
  let dpr = 1;
  let width = 2;
  return {
    object,
    update(trips, time, body, fade) {
      const count = Math.min(trips.length, MAX);
      for (let i = 0; i < count; i++) {
        const { from, to, start, dur } = trips[i];
        const head = Math.min(1, (time - start) / dur);
        const rear = Math.max(0, (time - start - fade) / dur);
        const life = Math.max(0, 1 - Math.max(0, time - start - dur) / fade);
        for (let j = 0; j < SAMPLES; j++) body.path(from, to, rear + ((head - rear) * j) / (SAMPLES - 1), points[j]);
        for (let j = 0; j < SAMPLES; j++) {
          const p = points[j];
          const n =
            j + 1 < SAMPLES
              ? points[j + 1]
              : tail
                  .copy(p)
                  .sub(points[j - 1])
                  .add(p);
          const v = (i * SAMPLES + j) * 2;
          const a = life * (j / (SAMPLES - 1)) ** 1.6;
          for (const k of [v, v + 1]) {
            position.setXYZ(k, p.x, p.y, p.z);
            next.setXYZ(k, n.x, n.y, n.z);
            alpha.setX(k, a);
          }
        }
        const last = points[SAMPLES - 1];
        headPosition.setXYZ(i, last.x, last.y, last.z);
        headAlpha.setX(i, life);
      }
      position.needsUpdate = next.needsUpdate = alpha.needsUpdate = true;
      headPosition.needsUpdate = headAlpha.needsUpdate = true;
      ribbon.setDrawRange(0, count * (SAMPLES - 1) * 6);
      heads.setDrawRange(0, count);
    },
    resize(w, h, ratio) {
      dpr = ratio;
      ribbonMaterial.uniforms.uResolution.value.set(w * ratio, h * ratio);
      ribbonMaterial.uniforms.uWidth.value = width * ratio;
      headMaterial.uniforms.uSize.value = HEAD * ratio;
    },
    paint(theme) {
      ribbonMaterial.uniforms.uColor.value = rgb(theme.accent);
      headMaterial.uniforms.uColor.value = rgb(theme.accent);
      headMaterial.uniforms.uCore.value = rgb(theme.fg);
    },
    style(px, xray) {
      width = px;
      ribbonMaterial.uniforms.uWidth.value = px * dpr;
      ribbonMaterial.depthTest = headMaterial.depthTest = !xray;
      ribbonMaterial.needsUpdate = headMaterial.needsUpdate = true;
    },
    dispose() {
      ribbon.dispose();
      heads.dispose();
      ribbonMaterial.dispose();
      headMaterial.dispose();
    },
  };
}
