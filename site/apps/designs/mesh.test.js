import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { TILT, eye, frustum, mesh } from './mesh.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

test('the mesh keeps one quad per exposed face as two triangles wound with the normal, inside the unit box', () => {
  const cell = math.three.create(23, 3, 1, 2);
  const built = mesh(math.three.quads(cell));
  expect(built.faces).toBe(Number(math.three.census(cell).surface));
  expect([built.position.length, built.normal.length, built.index.length]).toEqual([built.faces * 12, built.faces * 12, built.faces * 6]);
  const at = (i) => [0, 1, 2].map((k) => built.position[i * 3 + k]);
  let agreed = 0;
  for (let tri = 0; tri < built.faces * 2; tri++) {
    const [a, b, c] = [0, 1, 2].map((k) => at(built.index[tri * 3 + k]));
    const u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const cross = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    const n = built.index[tri * 3] * 3;
    if (cross[0] * built.normal[n] + cross[1] * built.normal[n + 1] + cross[2] * built.normal[n + 2] > 0) agreed++;
  }
  expect(agreed).toBe(built.faces * 2);
  expect([Math.min(...built.position), Math.max(...built.position)]).toEqual([-1, 1]);
});

test('the eye orbits the origin at its reach and the frustum fits the cube at any aspect', () => {
  expect(eye(0, 0, 4)).toEqual([4, 0, 0]);
  const [x, y, z] = eye(Math.PI / 2, TILT, 3);
  expect([Math.abs(x) < 1e-9, y, z]).toEqual([true, 3 * Math.cos(TILT), 3 * Math.sin(TILT)]);
  const wide = frustum(2, 1);
  expect([wide.top, wide.right, wide.left, wide.bottom]).toEqual([Math.sqrt(3), 2 * Math.sqrt(3), -2 * Math.sqrt(3), -Math.sqrt(3)]);
  const tall = frustum(0.5, 1);
  expect([tall.top, tall.right]).toEqual([2 * Math.SQRT2, Math.SQRT2]);
  expect(frustum(1, 0.5).top).toBeCloseTo(2 * Math.sqrt(3), 9);
});
