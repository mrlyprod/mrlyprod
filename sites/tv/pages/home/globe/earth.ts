import {
  BufferAttribute,
  BufferGeometry,
  DynamicDrawUsage,
  Mesh,
  MeshBasicMaterial,
  SphereGeometry,
  Vector3,
} from "three";
import type { Body } from "./body.ts";
import { grid, type Vec } from "./grid.ts";
import { LEVEL, land } from "./land.ts";
import { tileMaterial } from "./shade.ts";
import type { Theme } from "./theme.ts";

const INSET = 0.84;
const LIFT = 0.3;
const CORE = 0.985;

export function earth(theme: Theme): Body {
  const { centers, corners } = grid(LEVEL);
  const mask = land();
  const verts = corners.reduce((a, c) => a + c.length + 1, 0);
  const position = new Float32Array(verts * 3);
  const normal = new Float32Array(verts * 3);
  const aLand = new Float32Array(verts);
  const aLit = new Float32Array(verts);
  const index: number[] = [];
  const offset = new Int32Array(centers.length);
  let v = 0;
  const put = (p: Vec, n: Vec, on: number) => {
    position.set(p, v * 3);
    normal.set(n, v * 3);
    aLand[v] = on;
    v++;
  };
  centers.forEach((c, i) => {
    offset[i] = v;
    const ring = corners[i];
    put(c, c, mask[i]);
    for (const k of ring) {
      const along = k[0] * c[0] + k[1] * c[1] + k[2] * c[2];
      put(
        [
          c[0] + (k[0] - c[0] * along) * INSET,
          c[1] + (k[1] - c[1] * along) * INSET,
          c[2] + (k[2] - c[2] * along) * INSET,
        ],
        c,
        mask[i],
      );
    }
    for (let j = 0; j < ring.length; j++)
      index.push(offset[i], offset[i] + 1 + j, offset[i] + 1 + ((j + 1) % ring.length));
  });
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(position, 3));
  geometry.setAttribute("normal", new BufferAttribute(normal, 3));
  geometry.setAttribute("aLand", new BufferAttribute(aLand, 1));
  const lit = new BufferAttribute(aLit, 1);
  lit.setUsage(DynamicDrawUsage);
  geometry.setAttribute("aLit", lit);
  geometry.setIndex(index);
  const material = tileMaterial(theme);
  const mesh = new Mesh(geometry, material);
  const core = new Mesh(new SphereGeometry(CORE, 48, 32), new MeshBasicMaterial({ colorWrite: false }));
  core.renderOrder = -1;
  mesh.add(core);
  const cell = new Float32Array(centers.length);
  const a = new Vector3();
  const b = new Vector3();
  return {
    object: mesh,
    material,
    count: centers.length,
    lit: cell,
    flush() {
      centers.forEach((_, i) => {
        const n = corners[i].length + 1;
        aLit.fill(cell[i], offset[i], offset[i] + n);
      });
      lit.needsUpdate = true;
    },
    path(from, to, t, out) {
      a.fromArray(centers[from]);
      b.fromArray(centers[to]);
      const omega = Math.acos(Math.max(-1, Math.min(1, a.dot(b))));
      const sin = Math.sin(omega) || 1;
      out
        .copy(a)
        .multiplyScalar(Math.sin((1 - t) * omega) / sin)
        .addScaledVector(b, Math.sin(t * omega) / sin);
      return out.multiplyScalar(1 + LIFT * Math.sin(Math.PI * t) * (0.2 + 0.8 * (omega / Math.PI)));
    },
    span(from, to) {
      a.fromArray(centers[from]);
      b.fromArray(centers[to]);
      return Math.acos(Math.max(-1, Math.min(1, a.dot(b)))) / Math.PI;
    },
    dispose() {
      geometry.dispose();
      material.dispose();
      core.geometry.dispose();
      core.material.dispose();
    },
  };
}
