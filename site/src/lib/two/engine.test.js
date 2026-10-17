import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { rng } from '../rand.js';
import { cap, count, design, joins, lay, layers, sheet, tag } from './engine.js';

const carpet = math.two.create('495', 3, 2, 0, 3);

const sum = (list) => list.reduce((all, [, , len]) => all + len, 0);

const read = (x, y, tile = carpet) => count(math, { code: '495', base: 3, number: 3, level: 2, x, y }, tile, sheet(math, tile, x, y));

test('a sheet multiplies the fills by the copies exactly, while its exposed edges fall short of the copies alone', () => {
  const one = read(1, 1);
  const many = read(5, 5);
  expect([many.cols, many.rows, many.copies]).toEqual([45, 45, 25]);
  expect(many.fills).toBe(25 * one.fills);
  expect(many.one).toBe(one.exposed);
  expect(many.exposed).toBeLessThan(25 * one.exposed);
});

test('the seams are the lines where the copies meet, across the whole sheet', () => {
  expect(joins([18, 27], 9)).toEqual([[0, 9, 0, 18], [0, 18, 0, 18], [1, 9, 0, 27]]);
  expect(joins([9, 9], 9)).toEqual([]);
});

test('the counts read the sheet census: fills, voids, corners, edges and euler', () => {
  const facts = read(3, 2);
  const census = math.two.census(sheet(math, carpet, 3, 2));
  expect([facts.fills, facts.voids, facts.corners, facts.edges, facts.euler]).toEqual([census.fills, census.voids, census.vertices, census.edges, census.euler]);
  expect([facts.name, facts.side]).toEqual(['carpet', 9]);
  expect(facts.dimension).toBeCloseTo(math.counts.dimension('495', 3, 2, 3));
});

test('the layers light the fills over faint voids, and invert swaps the two', () => {
  const block = sheet(math, carpet, 2, 2);
  const plain = layers(math, block, 0);
  const inverse = layers(math, block, 1);
  const fills = math.two.fills(block);
  expect([sum(plain.lit), sum(plain.faint)]).toEqual([fills, 18 * 18 - fills]);
  expect(inverse.lit).toEqual(plain.faint);
  expect(inverse.faint).toEqual(plain.lit);
});

test('a blank or junk code rolls one from the seed, the same one twice', () => {
  const rolled = design(math, 3, '', rng(7));
  expect(rolled).toBe(design(math, 3, 'junk', rng(7)));
  expect(Number(rolled)).toBeGreaterThan(0);
  expect(design(math, 3, '495', rng(7))).toBe('495');
});

test('the level cap shrinks as the copies grow, so the sheet stays in budget', () => {
  expect([cap(math, 3), cap(math, 3, 25), cap(math, 5, 25), cap(math, 7, 144)]).toEqual([5, 4, 2, 1]);
});

test('the status tag is the name or the bare code, short enough to leave the seed room on a phone bar', () => {
  const tags = [{ name: '', code: '486' }, { name: 'carpet', code: '495' }].map(tag);
  expect(tags).toEqual(['code 486', 'carpet 495']);
  expect(tags.every((one) => one.length <= 12)).toBe(true);
});

test('the layout fits the sheet in the short side and centres it', () => {
  expect(lay(45, 45, 800, 600, 0.9)).toEqual({ px: 12, x: 130, y: 30 });
  expect(lay(90, 45, 800, 600, 0.9)).toEqual({ px: 8, x: 40, y: 120 });
});
