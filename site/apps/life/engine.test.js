import { expect, test } from 'bun:test';
import * as life from '../../../pkgs/mrlyjs/life.js';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { rng } from '../../lib/scene.js';
import { BOARD, LINE, codeOf, drawn, fate, fitLevel, hash, levelOf, maskName, maskOf, parse, population, ruleOf, sources, sow, spell } from './engine.js';

life.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/life/mrlyjs_life_bg.wasm', import.meta.url)).arrayBuffer() });
math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

test('counts read as digits, as separated numbers, as ranges, and nothing from a dash', () => {
  expect([parse('23'), parse('1 3 5'), parse('34-45'), parse('5-3, 9'), parse('-'), parse('3 3 1'), parse('2-9', 4)]).toEqual([
    [2, 3],
    [1, 3, 5],
    [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
    [3, 4, 5, 9],
    [],
    [1, 3],
    [2, 3, 4],
  ]);
});

test('counts spell as digits under ten and as runs above', () => {
  expect([spell([2, 3]), spell([]), spell([34, 35, 36, 40]), spell([2, 12])]).toEqual(['23', '-', '34-36,40', '2,12']);
});

test('a rule cuts its counts to the mask budget and names itself', () => {
  expect(ruleOf(life, { born: '3', stay: '23' }, 8)).toEqual({ birth: [3], survive: [2, 3], name: 'B3/S23' });
  expect(ruleOf(life, { born: '3 9', stay: '-' }, 8)).toEqual({ birth: [3], survive: [], name: 'B3/S-' });
});

test('a side named after a sequence draws its counts inside the budget without zero and one, and spells the name', () => {
  expect(ruleOf(life, { born: 'primes', stay: '23' }, 8)).toEqual({ birth: [2, 3, 5, 7], survive: [2, 3], name: 'Bprimes/S23' });
  expect(ruleOf(life, { born: ' Carpet_fills', stay: 'binary' }, 8)).toEqual({ birth: [8], survive: [2, 4, 8], name: 'Bcarpet_fills/Sbinary' });
  expect([drawn(life, '23', 8), drawn(life, '', 8)]).toEqual([null, null]);
  const names = sources(life);
  expect([names.includes('primes'), names.includes('carpet_fills'), names.includes('random_0'), names.length]).toEqual([true, true, false, 22]);
});

test('a code reads as a whole number inside the plane universe, 7 when it is junk', () => {
  expect([codeOf('7'), codeOf('15'), codeOf('99'), codeOf('-4'), codeOf('carpet'), codeOf('')]).toEqual([7, 15, 15, 0, 7, 7]);
});

test('a mask code names its design when the plane universe names it', () => {
  expect([maskName(math, 7), maskName(math, 14), maskName(math, 3), maskName(math, 11), maskName(math, 15)]).toEqual(['carpet', 'net', 'htree', '', '']);
});

test('a level fits under a cap from above and from below', () => {
  expect([levelOf(3, 3, 27), levelOf(5, 3, 27), levelOf(9, 2, 27), levelOf(3, 0, 27)]).toEqual([3, 2, 1, 1]);
  expect([fitLevel(3, BOARD), fitLevel(5, BOARD), fitLevel(9, BOARD), fitLevel(3, LINE)]).toEqual([4, 3, 2, 5]);
});

test('the mask of code 7 at number 3 is the Moore neighbourhood, eight cells round a hole', () => {
  const { mask, cells, level } = maskOf(life, { code: '7', number: 3, level: 1 });
  expect([mask.shape, Array.from(mask.data), cells, level]).toEqual([[3, 3], [1, 1, 1, 1, 0, 1, 1, 1, 1], 8, 1]);
  expect(maskOf(life, { code: '15', number: 9, level: 3 }).level).toBe(1);
});

test('a soup, one cell and a design seed the board; the design is the code grown to fit and centred', () => {
  const soup = sow({ mode: 'life', from: 'noise', density: 0.3 }, rng(7), math);
  const pop = population(soup);
  expect([soup.length, pop > 0.25 * soup.length, pop < 0.35 * soup.length]).toEqual([BOARD * BOARD, true, true]);
  expect(sow({ mode: 'life', from: 'noise', density: 0.3 }, rng(7), math)).toEqual(soup);
  const one = sow({ mode: 'life', from: 'one', density: 0.3 }, rng(7), math);
  expect([population(one), one[64 * BOARD + 64]]).toEqual([1, 1]);
  const design = sow({ mode: 'life', from: 'design', code: '7', number: 3 }, rng(7), math);
  const carpet = math.two.create(7, 3, 4, 0, 2).types;
  const left = (BOARD - 81) >> 1;
  const cut = [];
  for (let r = 0; r < 81; r++) cut.push(...design.subarray((left + r) * BOARD + left, (left + r) * BOARD + left + 81));
  expect([population(design), Uint8Array.from(cut)]).toEqual([population(carpet), carpet]);
});

test('a named pattern seeds the board centred, and its middle row seeds a line', () => {
  const left = (BOARD >> 1) - 1;
  const top = (BOARD >> 1) - 1;
  const glider = sow({ mode: 'life', from: 'glider' }, rng(1), math);
  expect([population(glider), glider[top * BOARD + left + 1], glider[(top + 1) * BOARD + left + 2], glider[(top + 2) * BOARD + left]]).toEqual([5, 1, 1, 1]);
  expect(population(sow({ mode: 'life', from: 'pentomino' }, rng(1), math))).toBe(5);
  const blinker = sow({ mode: 'wolfram', from: 'blinker' }, rng(1), math);
  expect([blinker.length, population(blinker), blinker[126], blinker[127], blinker[128]]).toEqual([LINE, 3, 1, 1, 1]);
  const row = sow({ mode: 'wolfram', from: 'glider' }, rng(1), math);
  expect([population(row), row[128]]).toEqual([1, 1]);
});

test('a wolfram seed is one line: one cell at its centre, or the middle row of the design', () => {
  const one = sow({ mode: 'wolfram', from: 'one' }, rng(1), math);
  expect([one.length, population(one), one[127]]).toEqual([LINE, 1, 1]);
  const row = sow({ mode: 'wolfram', from: 'design', code: '7', number: 3 }, rng(1), math);
  const carpet = math.two.create(7, 3, 5, 0, 2).types;
  const left = (LINE - 243) >> 1;
  expect(Array.from(row.subarray(left, left + 243))).toEqual(Array.from(carpet.subarray(121 * 243, 122 * 243)));
});

test('a fate reads dead, still, or the loop period from the recent hashes', () => {
  const seen = [];
  const words = [fate(seen, 1, 5), fate(seen, 2, 5), fate(seen, 2, 5), fate(seen, 3, 5), fate(seen, 4, 5), fate(seen, 3, 5), fate(seen, 9, 0)];
  expect(words).toEqual(['', '', 'still', '', '', 'loop 2', 'dead']);
  expect(hash(Uint8Array.from([1, 0, 1]))).not.toBe(hash(Uint8Array.from([1, 1, 0])));
});
