import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { rng } from '../../lib/rand.js';
import { BUDGET, FIRST, TICK, bound, bounds, cube, deal, design, giant, graphs, kindOf, lay, pace, picture, quick, study, top, turn, worthy } from './engine.js';

const value = { view: 'flat', base: 3, code: '495', cbase: 2, ccode: '23', number: 3, level: 2, graph: 'core', layout: 'lattice' };

test('the level cap reads the design through the count doors and stops at the deepest level whose network fits the layout budget', () => {
  const caps = [top(math, value), top(math, { ...value, layout: 'force' }), top(math, { ...value, view: 'cube' }), top(math, { ...value, view: 'cube', graph: 'edge' }), top(math, { ...value, view: 'hex' }), top(math, { ...value, code: 'junk' })];
  expect(caps).toEqual([4, 3, 3, 2, 3, FIRST]);
  const at = (level) => bound(math, 'flat', 3, '495', 3, level, 'core');
  expect([at(4), at(5)]).toEqual([4096, 32768]);
  expect(at(4) <= BUDGET.lattice && at(5) > BUDGET.lattice).toBe(true);
});

test('a sparse design climbs past the full-lattice cap until its grid of cells would stall the build', () => {
  expect([bound(math, 'flat', 3, '264', 3, 8, 'edge'), top(math, { ...value, code: '264', graph: 'edge' }), top(math, { ...value, code: '264', graph: 'edge', number: 5 })]).toEqual([1024, 6, 4]);
});

test('the flat space reads code and base, the cube and the hex slice read their own ccode and cbase, and each space offers its own graphs', () => {
  expect([design(value), design({ ...value, view: 'cube' }), design({ ...value, view: 'hex' })]).toEqual([{ dim: 2, base: 3, code: '495' }, { dim: 3, base: 2, code: '23' }, { dim: 3, base: 2, code: '23' }]);
  expect([graphs(value).map(([one]) => one), graphs({ view: 'hex' }).map(([one]) => one)]).toEqual([['core', 'edge', 'tunnel'], ['core', 'dual', 'edge']]);
  expect([kindOf({ view: 'hex', graph: 'tunnel' }), turn({ view: 'hex', graph: 'edge' }, 1), turn(value, -1)]).toEqual(['core', 'core', 'tunnel']);
});

test('the study reads the network of a flat design: its counts and the design\'s own Euler number', () => {
  const plan = study(math, value, rng(1));
  expect(plan.facts).toMatchObject({ code: '495', name: 'carpet', space: 'flat', level: 2, top: 4, side: 9, graph: 'core', layout: 'lattice', nodes: 64, branches: 88, tips: 0, junctions: 44, pieces: 1, length: 88, euler: -8 });
  expect(plan.facts.box).toBeCloseTo(1.57, 2);
  expect([plan.dim, plan.count, plan.positions.length, plan.pairs.length, plan.shade.branches.length]).toEqual([2, 64, 128, 176, 88]);
  plan.net.free();
});

test('a cube reads through the three doors and its hex slice through the six doors, triangles at their true aspect', () => {
  const solid = study(math, { ...value, view: 'cube', graph: 'tunnel' }, rng(1));
  expect(solid.facts).toMatchObject({ dim: 3, side: 9, nodes: 329, branches: 600, tips: 48, pieces: 1, euler: -80 });
  expect(solid.positions.length).toBe(329 * 3);
  const hex = study(math, { ...value, view: 'hex' }, rng(1));
  expect(hex.facts).toMatchObject({ space: 'hex', code: '23', nodes: 306, branches: 378, euler: -6 });
  expect([hex.dim, hex.facts.length]).toEqual([2, expect.any(Number)]);
  expect(hex.facts.length).toBeCloseTo(378 / Math.sqrt(3), 6);
  const dual = study(math, { ...value, view: 'hex', graph: 'dual' }, rng(1));
  expect([dual.facts.graph, dual.facts.nodes]).toEqual(['dual', 486]);
  [solid, hex, dual].forEach((plan) => plan.net.free());
});

test('a deal rolls a worthy code, graph, level and layout from the seed, the same seed the same deal', () => {
  const dealt = Array.from({ length: 40 }, (_, seed) => deal(math, value, rng(seed)));
  expect(deal(math, value, rng(7))).toEqual(dealt[7]);
  expect(new Set(dealt.map((one) => one.graph)).size).toBe(3);
  expect(new Set(dealt.map((one) => one.layout))).toEqual(new Set(['lattice', 'force']));
  const fine = dealt.map((one) => {
    const plan = study(math, { ...value, ...one }, rng(1));
    const good = one.level >= 2 && one.level <= top(math, { ...value, ...one }) && worthy(plan.shade.nodes, giant(math, plan.net));
    plan.net.free();
    return good;
  });
  expect(fine.every(Boolean)).toBe(true);
  const cubed = deal(math, { ...value, view: 'cube' }, rng(3));
  expect(Object.keys(cubed).sort()).toEqual(['ccode', 'graph', 'layout', 'level']);
});

test('a deal that rolls only the layout rests a network in pieces on the lattice, since a force would fling the pieces apart', () => {
  const dust = { ...value, level: 3, code: '25' };
  expect(deal(math, dust, rng(1), { code: false, graph: false, level: false, layout: true })).toEqual({ code: '25', graph: 'core', level: 3, layout: 'lattice' });
});

test('a bigger network ticks slower so a frame steps what a phone can, and a still settles at once only what it can afford', () => {
  expect([pace(64, 2), pace(2000, 2) > pace(1000, 2), pace(2000, 3) > pace(2000, 2)]).toEqual([TICK, true, true]);
  expect([quick(100, 2), quick(2000, 2)]).toEqual([true, false]);
});

test('the fit boxes the positions, lays them centred with y down, and normalises a cube into the unit box', () => {
  const positions = Float64Array.from([1, 2, 3, 6]);
  expect([...bounds(positions, 2, 2)]).toEqual([1, 3, 2, 6]);
  const box = lay(bounds(positions, 2, 2), 400, 300, 20);
  expect(box.k).toBe(65);
  expect([box.ox + 2 * box.k, box.oy + 4 * box.k]).toEqual([200, 150]);
  const { mid, scale } = cube(bounds(Float64Array.from([0, 0, 0, 4, 2, 1]), 3, 2));
  expect([mid, scale]).toEqual([[2, 1, 0.5], 0.5]);
  expect([...bounds(new Float64Array(0), 2, 0)]).toEqual([0, 0, 0, 0]);
});

test('the svg is one path per shade of lines and one group of circles per shade of dots', () => {
  const text = picture(100, 50, [[10, 10, 3], [20, 10, 1]], [[10, 10, 20, 10, 1]], 2.5, 1, ['a', 'b', 'c', 'd']);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50"')).toBe(true);
  expect(text).toContain('<path d="M10 10L20 10" fill="none" stroke="b" stroke-width="1" stroke-linecap="round"/>');
  expect(text).toContain('<g fill="d"><circle cx="10" cy="10" r="2.5"/></g>');
  expect(text).toContain('<g fill="b"><circle cx="20" cy="10" r="2.5"/></g>');
  expect(text).not.toContain('stroke="a"');
});
