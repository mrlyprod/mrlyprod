import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { rng } from '../../lib/scene.js';
import { BUDGET, HOLD, SETTLE, TICK, bound, bounds, cap, choose, cube, joined, lay, order, pace, picture, reached, share, steady, study, ticks, turn, worthy } from './engine.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { dim: 2, base: 3, code: '495', number: 3, level: 2, graph: 'core', layout: 'grid' };

test('the level cap keeps the lattice sites, or the corners of an edge graph, inside the budget of the layout', () => {
  expect([bound(2, 3, 3, 'core'), bound(2, 3, 3, 'edge'), bound(3, 3, 2, 'tunnel')]).toEqual([729, 784, 729]);
  expect(cap({ dim: 2, number: 3, graph: '', layout: '' })).toBe(3);
  expect(cap({ dim: 2, number: 3, graph: 'core', layout: 'grid' })).toBe(4);
  expect(cap({ dim: 3, number: 3, graph: '', layout: '' })).toBe(2);
  expect(cap({ dim: 3, number: 3, graph: 'core', layout: 'grid' })).toBe(3);
  expect(cap({ dim: 3, number: 3, graph: 'edge', layout: 'grid' })).toBe(2);
  expect(cap({ dim: '3', number: 7, graph: '', layout: 'force' })).toBe(1);
  expect(bound(2, 3, cap({ dim: 2, number: 3, graph: 'edge', layout: 'force' }), 'edge')).toBeLessThanOrEqual(BUDGET.force);
});

test('a blank graph or layout is rolled from the seed, a named one is kept, and the arrows cycle the graphs', () => {
  expect(choose({ graph: 'edge', layout: 'force' }, rng(1))).toEqual({ graph: 'edge', layout: 'force' });
  const rolled = Array.from({ length: 40 }, (_, seed) => choose({ graph: '', layout: '' }, rng(seed)));
  expect(new Set(rolled.map((one) => one.graph)).size).toBe(3);
  expect(new Set(rolled.map((one) => one.layout)).size).toBe(2);
  expect(choose({ graph: '', layout: '' }, rng(5))).toEqual(choose({ graph: '', layout: '' }, rng(5)));
  expect([turn('core', 1), turn('tunnel', 1), turn('core', -1), turn('', 1)]).toEqual(['edge', 'core', 'tunnel', 'edge']);
});

test('the reveal walks the largest piece first from its node nearest the centre, then the rest by size, and a branch shows once both ends do', () => {
  const positions = Float64Array.from([0, 0, 1, 0, 2, 0, 3, 0, 5, 5, 0, 1, 0, 2]);
  const pairs = Uint32Array.from([0, 1, 1, 2, 2, 3, 5, 6]);
  const laid = order(7, pairs, positions, 2);
  expect(laid.pieces).toBe(3);
  expect([...laid.nodes]).toEqual([2, 1, 3, 0, 5, 6, 4]);
  expect([...laid.rank]).toEqual([3, 1, 0, 2, 6, 4, 5]);
  expect([...laid.branches]).toEqual([1, 2, 0, 3]);
  expect([...laid.shown]).toEqual([0, 0, 1, 2, 3, 3, 4, 4]);
});

test('the study reads the network of a design through the unit: its counts, its roles in reveal order and the design\'s own Euler number', () => {
  const plan = study(math, value, rng(1));
  expect(plan.facts).toMatchObject({ code: '495', dim: 2, base: 3, number: 3, level: 2, side: 9, graph: 'core', layout: 'grid', nodes: 64, branches: 88, tips: 0, junctions: 44, pieces: 1, length: 88, euler: -8 });
  expect(plan.facts.box).toBeCloseTo(1.57, 2);
  expect([plan.count, plan.positions.length, plan.pairs.length, plan.order.nodes.length, plan.shade.nodes.length, plan.shade.branches.length]).toEqual([64, 128, 176, 64, 64, 88]);
  expect([...new Set(plan.order.nodes)].length).toBe(64);
  expect(plan.shade.nodes.filter((s) => s === 3).length).toBe(44);
  expect(plan.shade.branches.every((s) => s >= 2)).toBe(true);
  plan.net.free();
});

test('a blank code is rolled from the seed, a cube reads through the three doors, and the same seed gives the same plan', () => {
  const a = study(math, { ...value, code: '', graph: '', layout: '' }, rng(3));
  const b = study(math, { ...value, code: '', graph: '', layout: '' }, rng(3));
  expect([a.facts.code, a.facts.graph, a.facts.layout]).toEqual([b.facts.code, b.facts.graph, b.facts.layout]);
  expect(a.facts.code).not.toBe('');
  const cube3 = study(math, { ...value, dim: 3, base: 2, code: '23', graph: 'tunnel' }, rng(1));
  expect(cube3.facts).toMatchObject({ dim: 3, side: 9, nodes: 329, branches: 600, tips: 48, pieces: 1, euler: -80 });
  expect(cube3.positions.length).toBe(329 * 3);
  a.net.free();
  b.net.free();
  cube3.net.free();
});

test('a rolled code and graph skip networks that are bare, mostly lone dots or in many pieces, and a named pair is kept as asked', () => {
  const flat = { ...value, base: 2, level: 3, code: '', graph: '', layout: 'grid' };
  const rolled = Array.from({ length: 120 }, (_, seed) => {
    const plan = study(math, flat, rng(seed));
    plan.net.free();
    return plan.facts.branches > 0 && worthy(plan.roles, plan.big);
  });
  expect(rolled.every(Boolean)).toBe(true);
  const full = study(math, { ...flat, code: '15', graph: 'tunnel' }, rng(1));
  expect([full.facts.code, full.facts.graph, full.facts.branches]).toEqual(['15', 'tunnel', 0]);
  full.net.free();
});

test('a rolled layout rests on the grid when the network is in pieces, since a force would fling them apart, and a named layout is kept', () => {
  const dust = { ...value, level: 3, code: '25', graph: 'core', layout: '' };
  const rolled = study(math, dust, rng(1));
  const asked = study(math, { ...dust, layout: 'force' }, rng(1));
  expect([rolled.facts.pieces, rolled.big, rolled.facts.layout, asked.facts.layout]).toEqual([9, 3, 'grid', 'force']);
  expect([joined(3, 27), joined(24, 27), joined(25, 27)]).toEqual([false, false, true]);
  rolled.net.free();
  asked.net.free();
});

test('the reveal share runs over the span, holds, and loops; the ticks follow it and settle; a still shows everything settled', () => {
  expect([share(0, 8000, false), share(4000, 8000, false), share(8000, 8000, false), share(8000 + HOLD - 1, 8000, false)]).toEqual([0, 0.5, 1, 1]);
  expect(share(8000 + HOLD + 400, 8000, false)).toBeCloseTo(0.05, 9);
  expect([share(5, 0, false), share(5, 8000, true)]).toEqual([1, 1]);
  expect([ticks(0, 0, false), ticks(TICK * 10, 0, false), ticks(1e7, 0, false), ticks(0, 8000, true)]).toEqual([0, 10, SETTLE, SETTLE]);
  expect(ticks(8000 + HOLD + TICK * 3, 8000, false)).toBe(3);
  expect([reached(0.5, 100, 2), reached(0.5, 100, 3), reached(1, 7, 2)]).toEqual([25, 13, 7]);
});

test('a bigger network ticks slower so a frame steps what a phone can, and a still settles only what it can afford at once', () => {
  expect([pace(100, 2), pace(900, 3), ticks(500, 0, false, pace(900, 3))]).toEqual([TICK, 25, 20]);
  expect([steady('force', 100, 2, true), steady('force', 400, 2, true), steady('force', 400, 2, false), steady('grid', 400, 2, true)]).toEqual(['force', 'grid', 'force', 'grid']);
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
