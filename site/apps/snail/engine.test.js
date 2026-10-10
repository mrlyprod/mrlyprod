import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { FADE, FAINT, HOLD, LEAST, MOST, PAD, SIZE, TILE, camera, cameras, fade, head, index, mean, phase, picture, schedule, study, tag } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), base: 2, code: '7', ...over });

const near = (got, want) => got.forEach((v, i) => expect(v).toBeCloseTo(want[i], 9));

const ARCHIVE = { tiles: 300, primes: 62, levels: [1, 2, 4, 8, 16, 32, 64, 128, 45], area: 5345865, width: 4608, height: 4352 };

test('the study of the carpet of 2 to 300 reads the archive facts and grows every number', () => {
  const plan = study(math, num, value({ number: 2, top: 300 }), rng(1));
  const { facts } = plan;
  expect([facts.code, facts.name, facts.tiles, facts.primes, facts.grown, facts.area, facts.width, facts.height, facts.side]).toEqual(['7', 'carpet', 300, 62, 299, ARCHIVE.area, ARCHIVE.width, ARCHIVE.height, 256]);
  expect(facts.levels.map((row) => row.count)).toEqual(ARCHIVE.levels);
  expect(facts.levels.map((row) => row.side)).toEqual([1, 2, 4, 8, 16, 32, 64, 128, 256]);
  expect(plan.art.map((cell) => (cell ? cell.shape[0] : 0))).toEqual([0, 2, 4, 8, 16, 32, 64, 128, 256]);
  expect(plan.lines[8].reduce((sum, [, , len]) => sum + len, 0)).toBe(3 ** 8);
});

test('the status tag is the name and code, or the code alone, short enough to leave the seed whole on a phone', () => {
  expect(tag(study(math, num, value({ top: 20 }), rng(1)).facts)).toBe('carpet 7');
  expect(tag({ name: '', code: '4' })).toBe('code 4');
});

test('under prime growth only the primes grow and the rest stay unit cells', () => {
  const { facts, shell } = study(math, num, value({ number: 2, top: 300, growth: 'Prime' }), rng(1));
  expect([facts.grown, facts.primes, facts.levels[0].count]).toEqual([62, 62, 238]);
  expect(shell.tiles.every((tile) => tile.prime === tile.level > 0)).toBe(true);
});

test('a blank or junk code rolls one from the seed, the same for one seed', () => {
  const rolled = study(math, num, value({ code: '', top: 20 }), rng(7)).facts.code;
  expect(rolled).toBe(study(math, num, value({ code: 'junk', top: 20 }), rng(7)).facts.code);
  expect(Number(rolled)).toBeGreaterThan(0);
  expect(study(math, num, value({ code: '', top: 20 }), rng(8)).facts.code).not.toBe(rolled);
});

test('each digit level takes a span clamped between the least and the most, a tile at most sixty ms', () => {
  const times = schedule(3, 300);
  expect(times.levels.map((one) => [one.lo, one.count])).toEqual([[1, 2], [3, 6], [9, 18], [27, 54], [81, 162], [243, 58]]);
  expect(times.levels.map((one) => one.span)).toEqual([LEAST, LEAST, 18 * TILE, MOST, MOST, MOST]);
  expect(times.length).toBe(2 * LEAST + 18 * TILE + 3 * MOST);
  expect(times.levels.map((one) => one.start)).toEqual([0, LEAST, 2 * LEAST, 2 * LEAST + 18 * TILE, 2 * LEAST + 18 * TILE + MOST, 2 * LEAST + 18 * TILE + 2 * MOST]);
  expect(schedule(2, 1).levels).toEqual([{ lo: 1, count: 1, start: 0, span: LEAST }]);
});

test('the index runs from nothing to every tile, each tile done at the end of its own slice', () => {
  const times = schedule(3, 300);
  expect([index(times, -5), index(times, 0), index(times, times.length), index(times, times.length + 99)]).toEqual([0, 0, 300, 300]);
  expect(index(times, LEAST)).toBe(2);
  expect(index(times, LEAST + LEAST / 3)).toBe(4);
  const last = times.levels[5];
  expect(index(times, last.start + last.span / 2)).toBe(242 + 29);
});

test('the phase runs the drawing, holds, fades out over a second, then starts over, and a still is the end at full strength', () => {
  const length = schedule(3, 300).length;
  const loop = length + HOLD + FADE;
  expect(phase(0, length, false)).toBe(0);
  expect(phase(length, length, false)).toBe(length);
  expect(phase(loop - 1, length, false)).toBe(loop - 1);
  expect(phase(loop + 10, length, false)).toBe(10);
  expect(phase(-10, length, false)).toBe(loop - 10);
  expect(phase(10, length, true)).toBe(length);
  expect([0, length, length + HOLD].map((now) => fade(now, length))).toEqual([1, 1, 1]);
  expect(fade(length + HOLD + FADE / 2, length)).toBeCloseTo(0.5, 9);
  expect(fade(loop, length)).toBe(0);
  const shades = Array.from({ length: 21 }, (_, i) => fade(length + HOLD + (i * FADE) / 20, length));
  expect(shades.slice(1).every((shade, i) => shade < shades[i])).toBe(true);
  expect(fade(phase(10, length, true), length)).toBe(1);
});

test('the fit camera of a tile frames the box of every tile so far inside the padding', () => {
  const { shell } = study(math, num, value({ number: 2, top: 300 }), rng(1));
  const lens = cameras(shell, 800, 600, 0, 2);
  const [cx, cy, ls] = camera(schedule(2, 300), lens, 1e9);
  expect([cx, cy]).toEqual([(shell.low[0] + shell.high[0]) / 2, (shell.low[1] + shell.high[1]) / 2]);
  expect(Math.exp(ls)).toBeCloseTo(Math.min((800 - 4 * PAD) / 4608, (600 - 4 * PAD) / 4352));
  const first = lens.cams.slice(3, 6);
  expect([first[0], first[1]]).toEqual([0.5, 0.5]);
  expect(Math.exp(first[2])).toBeCloseTo(600 - 4 * PAD);
  const third = lens.cams.slice(9, 12);
  expect([third[0], third[1]]).toEqual([1.5, 2]);
  expect(Math.exp(third[2])).toBeCloseTo((600 - 4 * PAD) / 4);
});

test('a cell size pins the scale and rides the head from one tile centre to the next', () => {
  const { shell } = study(math, num, value({ number: 2, top: 20 }), rng(1));
  const lens = cameras(shell, 800, 600, 2, 1.5);
  const times = schedule(2, 20);
  const tile = shell.tiles[19];
  expect(camera(times, lens, times.length)).toEqual([tile.x + tile.side / 2, tile.y + tile.side / 2, Math.log(3)]);
  expect(camera(times, lens, 0)).toEqual([0.5, 0.5, Math.log(3)]);
  expect(camera(times, lens, LEAST + LEAST / 4)).toEqual([...head(shell.tiles, 1.5), Math.log(3)]);
});

test('the mean over a window is the time mean of the fits seen in it and settles on the last past the end', () => {
  const { shell } = study(math, num, value({ number: 2, top: 20 }), rng(1));
  const times = schedule(2, 20);
  const lens = cameras(shell, 800, 600, 0, 1);
  const cam = (i) => Array.from(lens.cams.slice(3 * i, 3 * i + 3));
  near(mean(times, lens, 0, LEAST), cam(1));
  near(mean(times, lens, LEAST, 2 * LEAST), cam(2).map((v, j) => (v + cam(3)[j]) / 2));
  near(mean(times, lens, times.length, times.length + HOLD), cam(20));
});

const DPR = 2;

const STAGES = [[2, 780, 1600], [3, 1600, 900], [5, 1000, 1000], [7, 1600, 900]];

const walk = (number, w, h, step = 25) => {
  const { shell, times } = study(math, num, value({ number, top: 300 }), rng(9));
  const lens = cameras(shell, w, h, 0, DPR);
  const frames = [];
  for (let t = 0; t <= times.length + HOLD; t += step) {
    const b = index(times, t);
    frames.push({ b, cam: camera(times, lens, t) });
  }
  return { tiles: shell.tiles, frames };
};

test('the fit camera keeps the box of the tiles so far, the one being laid and the head with it, on the stage at every moment', () => {
  for (const [number, w, h] of STAGES) {
    const { tiles, frames } = walk(number, w, h);
    let worst = -Infinity;
    for (const { b, cam: [cx, cy, ls] } of frames) {
      const k = Math.exp(ls);
      const shown = tiles.slice(0, Math.min(tiles.length, Math.max(1, Math.ceil(b))));
      const edges = [
        Math.min(...shown.map((tile) => w / 2 + (tile.x - cx) * k)),
        Math.min(...shown.map((tile) => h / 2 - (tile.y + tile.side - cy) * k)),
        Math.min(...shown.map((tile) => w - (w / 2 + (tile.x + tile.side - cx) * k))),
        Math.min(...shown.map((tile) => h - (h / 2 - (tile.y - cy) * k))),
      ];
      worst = Math.max(worst, (PAD * DPR) / 2 - Math.min(...edges));
    }
    expect(worst).toBeLessThanOrEqual(1e-6);
  }
});

test('the fit camera never jumps the zoom more than a third in one frame', () => {
  for (const [number, w, h] of STAGES) {
    const { frames } = walk(number, w, h, 1000 / 60);
    const steps = frames.slice(1).map(({ cam }, i) => Math.abs(cam[2] - frames[i].cam[2]));
    expect(Math.max(...steps)).toBeLessThan(Math.log(4 / 3));
  }
});

test('the head rides from one centre to the next with the fraction of the tile being laid', () => {
  const { shell } = study(math, num, value({ number: 2, top: 5 }), rng(1));
  const tiles = shell.tiles;
  expect(head(tiles, 0)).toEqual([0.5, 0.5]);
  expect(head(tiles, 0.7)).toEqual([0.5, 0.5]);
  expect(head(tiles, 1)).toEqual([0.5, 0.5]);
  expect(head(tiles, 1.5)).toEqual([1.25, 0.75]);
  expect(head(tiles, 5)).toEqual([-3, 4]);
  expect(head(tiles, 9)).toEqual([-3, 4]);
});

test('the svg defines each level once, stamps a use per grown tile and a rect per unit, and draws the path', () => {
  const plan = study(math, num, value({ number: 2, top: 20, growth: 'Prime' }), rng(1));
  const text = picture(plan, '#008cff');
  const { width, height } = plan.facts;
  expect(text).toContain(`viewBox="0 0 ${width} ${height}"`);
  expect(text).toMatch(new RegExp(`(width|height)="${SIZE}"`));
  expect(text.match(/<g id="l\d+">/g)).toHaveLength(4);
  expect(text.match(/<use /g)).toHaveLength(8);
  expect(text.match(/<rect x="\d+" y="\d+" width="1" height="1" opacity=/g)).toHaveLength(12);
  expect(text.match(new RegExp(` opacity="${FAINT}"`, 'g'))).toHaveLength(12);
  expect(text.match(/<path d="M/g)).toHaveLength(1);
  expect(picture(plan, '#008cff', false)).not.toContain('<path ');
  expect(text).toContain('fill="#008cff"');
});
