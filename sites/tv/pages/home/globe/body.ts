import type { Object3D, ShaderMaterial, Vector3 } from "three";

export type Body = {
  object: Object3D;
  material: ShaderMaterial;
  count: number;
  lit: Float32Array;
  flush: () => void;
  path: (from: number, to: number, t: number, out: Vector3) => Vector3;
  span: (from: number, to: number) => number;
  dispose: () => void;
};
