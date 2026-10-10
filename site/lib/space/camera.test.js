import { expect, test } from 'bun:test';
import { look, project, uniforms } from './camera.js';
import { add, mul } from './vec.js';

test('a point projected to store px casts back onto itself through the shader ray', () => {
  const view = { w: 1280, h: 720 };
  const cam = look([1, 2, -3], [0.2, 0.5, 4], [0, 1, 0], 0.3);
  const point = [2.5, 1.2, 6];
  const [x, y, z] = project(view, cam, point);
  const { uPos, uRot, uTan, uRes } = uniforms(view, cam);
  const frag = [x, uRes[1] - y];
  const uv = frag.map((v, i) => ((v - uRes[i] / 2) / Math.min(...uRes)) * 2 * uTan);
  const ray = [0, 1, 2].map((i) => uRot[i] * uv[0] + uRot[3 + i] * uv[1] + uRot[6 + i]);
  add(uPos, mul(ray, z)).forEach((v, i) => expect(v).toBeCloseTo(point[i], 4));
  expect(project(view, cam, [1, 2, -10])).toBeNull();
});
