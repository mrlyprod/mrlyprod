import { expect, test } from 'bun:test';
import { letters } from '../../kit/font/font.js';
import { gl } from './fake.js';
import { fit, hud, safe } from './hud.js';

const VIEW = { w: 1280, h: 720, dpr: 2, t: 0, look: () => ({ paper: '#000000', accent: '#008cff' }) };

test('pick takes the nearest mark within 44 css px and misses beyond', () => {
  const face = hud(gl(), VIEW);
  face.marks([{ id: 'a', x: 100, y: 100 }, { id: 'b', x: 400, y: 100 }, { id: 'c', x: 150, y: 100 }]);
  expect([face.pick(30, 100), face.pick(11, 100), face.pick(130, 100), face.pick(330, 100), face.pick(400, 189)]).toEqual(['a', null, 'c', 'b', null]);
});

test('pick scales its radius by the store px per css px, so a fixed frame keeps 44 css px', () => {
  const face = hud(gl(), { ...VIEW, fixed: true });
  face.marks([{ id: 'a', x: 100, y: 100 }]);
  expect([face.pick(130, 100, 0.5), face.pick(115, 100, 0.5), face.pick(180, 100, 1.92), face.pick(190, 100, 1.92)]).toEqual([null, 'a', 'a', null]);
});

test('fit maps a pointer through object-fit contain to store px', () => {
  const rect = { left: 10, top: 20, width: 1000, height: 1000 };
  const k = 1000 / 1920;
  const left = 10 + (1000 - 1080 * k) / 2;
  expect(fit(rect, 1080, 1920, left + 540 * k, 20 + 960 * k).map((v) => +v.toFixed(6))).toEqual([540, 960, 1.92]);
  expect(fit(rect, 1080, 1920, left, 20).map((v) => +v.toFixed(6))).toEqual([0, 0, 1.92]);
});

test('a fixed frame keeps text out of the top 13% and the bottom 18%', () => {
  expect(safe({ w: 1080, h: 1920, dpr: 2, fixed: true })).toEqual({ top: 250, bottom: 346, side: 65 });
  expect(safe({ w: 1280, h: 720, dpr: 2 })).toEqual({ top: 32, bottom: 32, side: 32 });
});

test('a mark with no radius stamps its label and adds no ring or disc quad', () => {
  const fake = gl();
  const face = hud(fake, VIEW);
  face.marks([{ id: 'a', x: 100, y: 100, label: '1', on: true }, { id: 'b', x: 400, y: 100, r: -1000, label: 'NAME', on: true }]);
  face.draw();
  const quads = (word) => fake.draws().filter(({ fs }) => fs.includes(word)).map(({ instances }) => instances);
  const cells = (line) => letters(line).grid.flat().filter(Boolean).length;
  expect([quads('abs(d - vRing.x)'), quads('vRing.x + 0.5 - d'), quads('vColor.rgb * vColor.a')]).toEqual([[1], [1], [cells('1') + cells('NAME')]]);
});
