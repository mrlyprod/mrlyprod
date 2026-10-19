import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { defaults } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { BASES, BUDGET, KOCH, MOST, PRESETS, bands, boxOf, boxed, boxes, cap, coded, corners, deepest, dial, every, flat, glance, lattice, lay, picture, preset, reach, rebase, rolled, spell, study, toggle, turn } from './engine.js';
import { SPEC } from './scene.js';

const value = (over) => ({ ...defaults(SPEC), seed: 1, ...over });

const named = (name, over) => value({ ...preset(num, math, name), ...over });

test('the defaults are the crate Koch curve at level 5 drawn as a line', () => {
  expect(preset(num, math, 'koch')).toEqual(KOCH);
});

test('every preset loads a whole design on a base its ring offers, at a level under its cap', () => {
  for (const [name] of PRESETS) {
    const patch = preset(num, math, name);
    expect([name, BASES[patch.ring].includes(patch.base), patch.level <= cap(patch)]).toEqual([name, true, true]);
  }
});

test('the deepest level keeps the fill inside the budget and a lone digit at the ceiling', () => {
  expect([deepest(2), deepest(3), deepest(4), deepest(7), deepest(8), deepest(9), deepest(1), deepest(0)]).toEqual([17, 10, 8, 6, 5, 5, MOST, MOST]);
  expect(2 ** 17 <= BUDGET && 2 ** 18 > BUDGET).toBe(true);
});

test('a base is spelled with its letter', () => {
  expect([spell('i', 1, 1), spell('w', 2, 1), spell('w', 3, 0), spell('i', 0, -1), spell('w', -1, -2), spell('i', 0, 0)]).toEqual(['1 + i', '2 + w', '3', '-i', '-1 - 2w', '0']);
});

test('the dial reads each held digit with its class and twist, the canonical residues and the units of the ring', () => {
  const dialed = dial(num, math, value());
  expect(dialed.held.map(({ cls, u }) => [cls, u])).toEqual([[0, 0], [1, 1], [7, 5], [4, 0]]);
  expect([dialed.residues.length, dialed.units.length, dialed.letter, dialed.error]).toEqual([9, 6, 'w', '']);
});

test('a congruent digit is dropped and an empty list stands in every residue, each with a note', () => {
  const twice = dial(num, math, value({ d: '0:0_3:0', t: '0_0' }));
  expect([twice.held.length, twice.error.includes('congruent')]).toEqual([1, true]);
  const none = dial(num, math, value({ d: 'junk', t: '' }));
  expect([none.held.length, none.error.includes('every residue')]).toEqual([9, true]);
});

test('a chip brings its class in untwisted at the end, takes a held one out, and never the last', () => {
  const dialed = dial(num, math, value());
  expect(toggle(dialed, 2)).toEqual({ d: '0:0_1:0_2:1_2:0_1:1', t: '0_1_5_0_0' });
  expect(toggle(dialed, 7)).toEqual({ d: '0:0_1:0_2:0', t: '0_1_0' });
  expect(toggle(dial(num, math, value({ d: '1:0', t: '3' })), 1)).toBe(null);
});

test('a twist turns one digit, Drop the twists turns none and Every residue holds each class untwisted', () => {
  const dialed = dial(num, math, value());
  expect(turn(dialed, 4, 3).t).toBe('0_1_5_3');
  expect(flat(dialed)).toEqual({ d: KOCH.d, t: '0_0_0_0' });
  expect(every(dialed)).toEqual({ d: '0:0_1:0_1:1_0:1_-1:0_-1:-1_0:-1_2:1_1:2', t: '0_0_0_0_0_0_0_0_0' });
});

test('a new ring or base starts from every residue, keeping the base when the new ring offers it', () => {
  expect(rebase(num, math, 'gaussian', '3:0')).toMatchObject({ ring: 'gaussian', base: '3:0', t: '0_0_0_0_0_0_0_0_0' });
  expect(rebase(num, math, 'gaussian', '3:1')).toEqual({ ring: 'gaussian', base: '2:0', d: '0:0_1:0_0:1_1:1', t: '0_0_0_0' });
});

test('a code builds the canonical digits of its classes, and a code of no class or past the classes builds nothing', () => {
  const dialed = dial(num, math, value());
  expect(coded(dialed, math, '147')).toEqual({ d: '0:0_1:0_-1:0_2:1', t: '0_0_0_0' });
  expect([coded(dialed, math, '0'), coded(dialed, math, '512'), coded(dialed, math, 'koch')]).toEqual([null, null, null]);
});

test('a reroll holds two classes or more with twists of the ring, the same for one seed', () => {
  const dialed = dial(num, math, value());
  const rolls = Array.from({ length: 200 }, (_, i) => rolled(dialed, rng(i)));
  expect(rolls.every(({ d, t }) => d.split('_').length >= 2 && t.split('_').every((u) => Number(u) < 6))).toBe(true);
  expect(new Set(rolls.map(({ d }) => d)).size).toBeGreaterThan(50);
  expect(rolled(dialed, rng(7))).toEqual(rolled(dialed, rng(7)));
});

test('a box code loads its box digits on base 3 of the square lattice and reads back only while they stay untwisted in the box', () => {
  const carpet = value(boxed(num, math, '495'));
  expect(boxed(num, math, '495')).toEqual({ ring: 'gaussian', base: '3:0', d: preset(num, math, 'carpet').d, t: '0_0_0_0_0_0_0_0' });
  expect(boxOf(dial(num, math, carpet))).toBe('495');
  expect(boxOf(dial(num, math, { ...carpet, t: '1_0_0_0_0_0_0_0' }))).toBe('');
  expect(boxOf(dial(num, math, value({ ring: 'gaussian', base: '3:0', d: '0:0_-1:0', t: '0_0' })))).toBe('');
});

test('a glance goes as deep as the preset asks while the words stay few, and a single digit stops at the ceiling', () => {
  expect([glance(4, 5), glance(2, 14), glance(2), glance(9), glance(1), glance(4, 99)]).toEqual([5, 11, 11, 3, MOST, 5]);
  expect(study(num, math, preset(num, math, 'twindragon'), 3).cap).toBe(3);
});

test('the boxes are the named designs of the base 3 square, then each class that no name covers, with no code twice', () => {
  const { named, rest } = boxes(math);
  expect(named).toHaveLength(10);
  expect(named[0]).toEqual(['495', 'carpet']);
  expect(rest.every(([, word]) => word === '')).toBe(true);
  const codes = [...named, ...rest].map(([code]) => code);
  expect(new Set(codes).size).toBe(codes.length);
  expect(rest).toHaveLength(18);
});

test('the Koch study reads base 3 of norm 9, four digits off the canonical system and no glue', () => {
  const facts = study(num, math, value()).facts(5);
  expect(facts).toMatchObject({ ring: 'Z[w]', base: '3', norm: 9, digits: 4, units: 6, code: '147', canonical: false, cap: 8, level: 5, fill: 1024, distinct: 1024, glued: false, error: '' });
  expect(facts.dimension).toBeCloseTo(Math.log(4) / Math.log(3));
});

test('the terdragon glues words onto one point, so its distinct count falls under its fill', () => {
  expect(study(num, math, named('terdragon')).facts(4)).toMatchObject({ ring: 'Z[w]', base: '2 + w', norm: 3, fill: 81, distinct: 43, glued: true });
});

test('a twist moves where a copy lands and never the word count', () => {
  const twisted = study(num, math, value());
  const plain = study(num, math, value({ t: '0_0_0_0' }));
  expect(plain.levels.map((pts) => pts.length)).toEqual(twisted.levels.map((pts) => pts.length));
  expect([...plain.levels[2]]).not.toEqual([...twisted.levels[2]]);
});

test('the stage holds every level with its cells, so moving the level never moves the scale', () => {
  for (const name of ['koch', 'twindragon', 'carpet']) {
    const plan = study(num, math, named(name));
    const [x0, y0, x1, y1] = plan.stage;
    for (const pts of plan.levels) for (let i = 0; i < pts.length; i += 2) expect(pts[i] >= x0 && pts[i] <= x1 && pts[i + 1] >= y0 && pts[i + 1] <= y1).toBe(true);
    expect(plan.levels).toHaveLength(plan.cap);
  }
});

test('a whole base on the square lattice draws y down like a tile, and every other base y up', () => {
  expect(['carpet', 'twindragon', 'koch'].map((name) => study(num, math, named(name)).sign)).toEqual([1, -1, -1]);
  const one = study(num, math, named('carpet')).levels[0];
  expect([...one].map((v) => Math.round(v * 3))).toEqual([0, 0, 1, 0, 2, 0, 0, 1, 2, 1, 0, 2, 1, 2, 2, 2]);
});

test('the lattice of a level is the base scale to the level, the cell a square or a hexagon through its neighbours', () => {
  expect(lattice(2, 4, 2)).toEqual({ spacing: 0.5, radius: 0.5 / Math.SQRT2 });
  expect(lattice(4, 6, 1)).toEqual({ spacing: 0.5, radius: 0.5 / Math.sqrt(3) });
  const square = corners(4, Math.PI / 4, 1, 1);
  expect([...square].map((v) => Math.round(v * 1e6) / 1e6)).toEqual([1, 0, 0, 1, -1, 0, -0, -1]);
  const hex = corners(6, 0, 3, 2);
  expect([hex.length, hex[0], hex[1]]).toEqual([12, 2 * Math.cos(Math.PI / 6), 2 * Math.sin(Math.PI / 6)]);
});

test('a small cell is as wide as the bounding box of the cell, so a turned square lattice leaves no holes', () => {
  for (const level of [1, 2, 3, 4, 5]) {
    const { spacing, radius } = lattice(2, 4, level);
    const ring = corners(4, Math.PI / 4, level, radius);
    const xs = [ring[0], ring[2], ring[4], ring[6]];
    expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(spacing * reach(4, Math.PI / 4, level));
  }
  expect([reach(4, Math.PI / 4, 16), reach(4, Math.PI / 4, 17)]).toEqual([1, Math.SQRT2].map((v) => expect.closeTo(v)));
  expect([reach(6, Math.PI / 6, 5), reach(4, 0, 9)]).toEqual([1, 1]);
});

test('the fit centres the frame inside the padding, turns a wide picture onto a tall stage, and a pinned scale wins', () => {
  const flat = lay([0, 0, 4, 1], 800, 600, 10);
  expect(flat).toEqual({ k: 195, cx: 2, cy: 0.5, ox: 400, oy: 300, quarter: false });
  const tall = lay([0, 0, 4, 1], 600, 800, 10);
  expect([tall.k, tall.quarter]).toEqual([195, true]);
  expect(lay([0, 0, 1, 1], 600, 800, 10).quarter).toBe(false);
  expect(lay([0, 0, 4, 1], 800, 600, 10, 50).k).toBe(50);
});

test('the svg of a box tile is one rect per run, of a dragon one use of the cell per distinct point, of a line one path through every word', () => {
  const sheet = picture(study(num, math, named('carpet')), '#008cff', 2, 0);
  expect(sheet).toContain('width="90" height="90" viewBox="0 0 9 9"');
  expect(sheet.match(/<rect /g).length).toBeLessThan(64);
  expect([...sheet.matchAll(/width="(\d+)" height="1"/g)].reduce((sum, [, w]) => sum + Number(w), 0)).toBe(64);
  expect(sheet).toContain('fill="#008cff"');
  const dragon = picture(study(num, math, named('terdragon')), '#008cff', 4, 0);
  expect(dragon.match(/<use /g)).toHaveLength(43);
  expect(dragon).toContain('<path id="c" d="M');
  const [, w, h] = dragon.match(/^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg" width="([\d.]+)" height="([\d.]+)"/);
  expect(Math.max(Number(w), Number(h))).toBe(2048);
  const curve = picture(study(num, math, value()), '#008cff', 3, 1);
  expect([curve.match(/<path /g).length, curve.match(/L/g).length]).toEqual([1, 63]);
});

test('the runs of a set of words are read row by row, with no raster of the plane', () => {
  expect(bands([[1n, 0n], [0n, 2n], [3n, 0n], [0n, 0n], [1n, 2n]])).toEqual([[0, 0, 2], [0, 3, 1], [2, 0, 2]]);
  expect(bands([])).toEqual([]);
});

test('the svg of a box tile at the level cap is a handful of rects drawn in a moment, whatever the side of the plane', () => {
  const start = performance.now();
  for (const code of ['448', '1']) {
    const plan = study(num, math, value(boxed(num, math, code)));
    const sheet = picture(plan, '#008cff', plan.cap, 0);
    expect(sheet.match(/<rect /g)).toHaveLength(1);
  }
  expect(performance.now() - start).toBeLessThan(2000);
});
