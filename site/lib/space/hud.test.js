import { expect, test } from 'bun:test';
import { letters } from '../../kit/font/font.js';
import { gl } from './fake.js';
import { hud, safe } from './hud.js';

const VIEW = { w: 1280, h: 720, dpr: 2, t: 0, look: () => ({ paper: '#000000', accent: '#008cff' }) };

const PHONE = { w: 390, h: 844, dpr: 1 };

const DESKTOP = { w: 1440, h: 900, dpr: 1 };

const ZOOM = ['net 232', 'n 3  level 2', 'bits 11101000  fill 7/27', 'dim 1.77  class 23', 'pillars'];

const INK = 1;

const ink = (view) => {
  const fake = gl();
  const face = hud(fake, view);
  face.lines(ZOOM);
  face.draw();
  const [, , , data, , size] = fake.log.findLast((one) => one[0] === 'bufferSubData');
  return Array.from({ length: size / 8 }, (_, i) => Array.from(data.slice(i * 8, i * 8 + 8))).filter((quad) => quad[7] === INK);
};

test('a fixed frame keeps text out of the top 13% and the bottom 18%', () => {
  expect(safe({ w: 1080, h: 1920, dpr: 2, fixed: true })).toEqual({ top: 250, bottom: 346, side: 65 });
  expect(safe({ w: 1280, h: 720, dpr: 2 })).toEqual({ top: 32, bottom: 32, side: 32 });
});

test('lines drop empty rows and stamp every lit glyph cell of the rest in capitals, a shadow under each, in one draw', () => {
  const fake = gl();
  const face = hud(fake, VIEW);
  face.lines(['code 23', null, '', 'skim']);
  face.draw();
  const cells = (line) => letters(line).grid.flat().filter(Boolean).length;
  expect(fake.draws().map(({ instances }) => instances)).toEqual([2 * (cells('CODE 23') + cells('SKIM'))]);
});

test('the cell shrinks until the longest line fits between the side margins, and stays 3 px times dpr on a wide frame', () => {
  const phone = ink(PHONE);
  const side = safe(PHONE).side;
  expect([Math.min(...phone.map(([x]) => x)) >= side, Math.max(...phone.map(([x, , w]) => x + w)) <= PHONE.w - side, phone[0][2] < 3]).toEqual([true, true, true]);
  expect(ink(DESKTOP)[0][2]).toBe(3);
});
