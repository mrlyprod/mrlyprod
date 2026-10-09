import { expect, test } from 'bun:test';
import { look } from './camera.js';
import { gl } from './fake.js';
import { sky, stars } from './sky.js';

test('one seed makes one star field and another seed another', () => {
  expect(stars(7, 500)).toEqual(stars(7, 500));
  expect(stars(7, 500)).not.toEqual(stars(8, 500));
});

const quads = (flow) => {
  const fake = gl();
  const view = { w: 1280, h: 720, dpr: 1, t: 0 };
  const field = sky(fake, view, { seed: 7, dust: 0 });
  fake.viewport(0, 0, 1280, 720);
  fake.log.length = 0;
  field.draw(look([0, 0, 0], [0, 0, 1]), flow);
  const [, , , data, , length] = fake.log.find(([key]) => key === 'bufferSubData');
  return Array.from({ length: length / 8 }, (_, i) => data.slice(i * 8, i * 8 + 8));
};

test('every star and streak is drawn at least 1.5 px wide', () => {
  for (const flow of [{ s: 0 }, { s: 3.7, shutter: 0.9, k: 3 }]) {
    const list = quads(flow);
    expect(list.length).toBeGreaterThan(300);
    expect(Math.min(...list.map((q) => Math.min(q[4], q[5])))).toBeGreaterThanOrEqual(1.5);
  }
});

test('the field wraps along the axis with a period of 2, so a flight of 2 repeats the sky', () => {
  expect(quads({ s: 2.3, shutter: 0.4 })).toEqual(quads({ s: 0.3, shutter: 0.4 }));
});

test('a flow may tint the streaks and replace the halo ramp, and leaves them be when unset', () => {
  const fake = gl();
  const field = sky(fake, { w: 1280, h: 720, dpr: 1, t: 0 }, { seed: 7, dust: 0 });
  fake.viewport(0, 0, 1280, 720);
  const sent = (flow) => {
    fake.log.length = 0;
    field.draw(look([0, 0, 0], [0, 0, 1]), flow);
    return fake.log.filter(([key, at]) => key === 'uniform3fv' && ['uHalo', 'uTint'].includes(at.name)).map(([, at, v]) => [at.name, v]);
  };
  expect(sent({ s: 0 })).toEqual([['uHalo', [-1, -1, -1]], ['uTint', [1, 1, 1]]]);
  expect(sent({ s: 0, halo: [1, 1, 1], tint: [0.3, 0.5, 1] })).toEqual([['uHalo', [1, 1, 1]], ['uTint', [0.3, 0.5, 1]]]);
});

test('with no shutter every streak has its tail on its head', () => {
  const list = quads({ s: 1.3, shutter: 0 });
  expect(list.length).toBeGreaterThan(300);
  expect(list.every((q) => q[0] === q[2] && q[1] === q[3])).toBe(true);
});
