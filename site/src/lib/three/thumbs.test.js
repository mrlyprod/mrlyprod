import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { SHAPES } from './engine.js';
import { BLOCK, POSES, block, glance } from './thumbs.js';

const faces = (shape, pose) => {
  const log = [];
  const pen = new Proxy({}, { get: (_, key) => () => log.push(key), set: () => true });
  glance(pen, shape, POSES[pose], 56, { paper: '#ffffff', accent: '#008cff' });
  return log.filter((key) => key === 'fill').length;
};

test('a look draws one cube from its corner, its edge and its face: three faces, three and two', () => {
  expect(['corner', 'edge', 'face'].map((pose) => faces(BLOCK, pose))).toEqual([3, 3, 2]);
});

test('every crop is a thumb of its own, a smaller solid than the whole block', () => {
  const sizes = SHAPES.map((one) => block(math, one).cells.length);
  expect(sizes.every((size) => size > 0 && size < block(math, '').cells.length)).toBe(true);
  expect(block(math, 'ball')).toBe(block(math, 'ball'));
});
