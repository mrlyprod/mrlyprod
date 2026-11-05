import { BoxGeometry, DynamicDrawUsage, InstancedBufferAttribute, InstancedMesh, Matrix4, Vector3 } from "three";
import type { Shape } from "./bang.ts";
import type { Body } from "./body.ts";
import { tileMaterial } from "./shade.ts";
import type { Theme } from "./theme.ts";

const INSET = 0.76;
const THIN = 0.22;
const LIFT = 0.9;

export function sim(shape: Shape, extent: number, theme: Theme): Body {
  const { dim, side, cells } = shape;
  const unit = (2 * extent) / side;
  const place = (coord: number[]): [number, number, number] => {
    const at = (c: number) => (c + 0.5) * unit - extent;
    return dim === 2 ? [at(coord[1]), 0, at(coord[0])] : [at(coord[0]), at(coord[2]), at(coord[1])];
  };
  const size = unit * INSET;
  const geometry = new BoxGeometry(size, dim === 2 ? size * THIN : size, size);
  const aLit = new InstancedBufferAttribute(new Float32Array(cells.length), 1);
  aLit.setUsage(DynamicDrawUsage);
  geometry.setAttribute("aLit", aLit);
  geometry.setAttribute("aLand", new InstancedBufferAttribute(new Float32Array(cells.length).fill(1), 1));
  const material = tileMaterial(theme);
  const mesh = new InstancedMesh(geometry, material, cells.length);
  const matrix = new Matrix4();
  for (let i = 0; i < cells.length; i++) mesh.setMatrixAt(i, matrix.makeTranslation(...place(cells[i])));
  mesh.instanceMatrix.needsUpdate = true;
  const centers = cells.map(place);
  const lit = aLit.array as Float32Array;
  const a = new Vector3();
  const b = new Vector3();
  return {
    object: mesh,
    material,
    count: cells.length,
    lit,
    flush() {
      aLit.needsUpdate = true;
    },
    path(from, to, t, out) {
      a.fromArray(centers[from]);
      b.fromArray(centers[to]);
      out.lerpVectors(a, b, t);
      if (dim === 2) out.y += unit * LIFT * Math.sin(Math.PI * t);
      return out;
    },
    span: () => 1 / side,
    dispose() {
      geometry.dispose();
      material.dispose();
      mesh.dispose();
    },
  };
}
