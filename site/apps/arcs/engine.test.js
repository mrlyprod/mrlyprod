import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BEAT, HOLD, LEAST, MOST, ORBIT, PACE, SWAY, UNIT, arc, bend, cap, choose, corner, curves, drift, fit, looping, phase, picture, schedule, study, trail } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 3, code: '', ...over });

const SHADES = { accent: '#008cff', dim: 'rgb(0, 63, 115)', fill: 'rgba(0, 140, 255, 0.08)' };

const walk = (code, base, level) => {
  const drawn = math.arcs.draw(code, base, level);
  return { drawn, plan: curves(drawn) };
};

test('the level cap keeps the cells under the budget: eight at base 2, five at base 3, and five for a base it does not know', () => {
  expect([cap(2), cap(3), cap(undefined), cap('5')]).toEqual([8, 5, 5, 5]);
  const level = (over) => tidy(SPEC, { ...value({ level: 9 }), ...over }).level;
  expect([level({ base: 2 }), level({ base: 3 }), level({ base: 3, level: 2 })]).toEqual([8, 5, 2]);
});

test('a typed code is kept modulo the count, a blank or unreadable one rolls a design that loops at the level, the same for one seed', () => {
  expect(choose(math, value({ code: '495' }), rng(1))).toBe('495');
  expect(choose(math, value({ code: '600' }), rng(1))).toBe('88');
  expect(choose(math, value({ code: '-1' }), rng(1))).toBe('511');
  const rolled = choose(math, value({}), rng(1));
  expect(rolled).toBe(choose(math, value({ code: 'carpet' }), rng(1)));
  expect(math.arcs.draw(rolled, 3, 3).loops).toBeGreaterThan(0);
  const looping = Array.from({ length: 40 }, (_, seed) => math.arcs.draw(choose(math, value({}), rng(seed)), 3, 3).loops > 0);
  expect(looping.every(Boolean)).toBe(true);
  expect(new Set(Array.from({ length: 40 }, (_, seed) => choose(math, value({}), rng(seed)))).size).toBeGreaterThan(20);
});

test('a rolled design loops at the level it is asked for, at base 2 where most codes never loop', () => {
  for (const level of [2, 3, 5]) {
    const rolled = Array.from({ length: 60 }, (_, seed) => looping(math, 2, level, rng(seed)));
    expect(rolled.every((code) => math.arcs.draw(code, 2, level).loops > 0)).toBe(true);
    expect(new Set(rolled).size).toBeGreaterThan(2);
  }
  expect(looping(math, 3, 3, rng(4))).toBe(choose(math, value({ level: 3 }), rng(4)));
});

test('a filled cell wraps its upper-left and lower-right corners, an empty one the other two', () => {
  expect([corner(1, 0, 2, 3), corner(1, 1, 2, 3), corner(0, 0, 2, 3), corner(0, 1, 2, 3)]).toEqual([[2, 3], [3, 4], [3, 3], [2, 4]]);
});

test('the walk finds the loops and the strands the crate counts, each arc once, on every base-2 code and the named base-3 designs', () => {
  const cases = [];
  for (let code = 0; code < 16; code++) for (let level = 1; level <= 4; level++) cases.push([code, 2, level]);
  for (const code of [495, 186, 455, 341, 170, 16]) for (let level = 1; level <= 3; level++) cases.push([code, 3, level]);
  for (const [code, base, level] of cases) {
    const { drawn, plan } = walk(code, base, level);
    const loops = plan.closed.reduce((sum, one) => sum + one, 0);
    expect([code, base, level, plan.loops, plan.strands, loops, plan.closed.length - loops, plan.bounds.at(-1)]).toEqual([code, base, level, drawn.loops, drawn.strands, drawn.loops, drawn.strands, 2 * drawn.side * drawn.side]);
    expect(new Set(plan.order).size).toBe(plan.order.length);
  }
});

test('the arcs of a curve chain through shared midpoints, a loop closes and a strand ends on the border', () => {
  const { plan } = walk(495, 3, 2);
  const ends = (j) => {
    const g = arc(plan, plan.order[j]);
    const v = `${g.cx},${g.y + 0.5}`;
    const h = `${g.x + 0.5},${g.cy}`;
    return plan.dir[j] > 0 ? [v, h] : [h, v];
  };
  const edge = (point) => point.split(',').some((c) => Number(c) === 0 || Number(c) === plan.side);
  for (let k = 0; k + 1 < plan.bounds.length; k++) {
    const from = plan.bounds[k];
    const to = plan.bounds[k + 1];
    for (let j = from; j + 1 < to; j++) expect(ends(j)[1]).toBe(ends(j + 1)[0]);
    if (plan.closed[k]) expect(ends(to - 1)[1]).toBe(ends(from)[0]);
    else expect([edge(ends(from)[0]), edge(ends(to - 1)[1])]).toEqual([true, true]);
  }
});

test("a loop bit of the crate that the walk does not find is refused", () => {
  const { drawn } = walk(495, 3, 2);
  expect(() => curves({ ...drawn, cells: drawn.cells.map((byte) => byte ^ 2) })).toThrow('disagree');
  expect(() => curves({ ...drawn, loops: drawn.loops + 1 })).toThrow('crate counts');
});

test('the one loop of the diagonal is the circle round the centre, its arcs sweeping a quarter each', () => {
  const { plan } = walk(9, 2, 1);
  expect([plan.loops, plan.strands]).toEqual([1, 4]);
  const k = plan.closed.indexOf(1);
  const arcs = Array.from(plan.order.subarray(plan.bounds[k], plan.bounds[k + 1]), (a) => arc(plan, a));
  expect(arcs.map((g) => [g.cx, g.cy])).toEqual([[1, 1], [1, 1], [1, 1], [1, 1]]);
  expect(arcs.map((g) => Math.abs(g.sweep))).toEqual([Math.PI / 2, Math.PI / 2, Math.PI / 2, Math.PI / 2]);
  expect(new Set(arcs.map((g) => `${g.x},${g.y}`)).size).toBe(4);
});

test('the study memoises the walk, checks it against the crate and reads the law', () => {
  const plan = study(math, value({ code: '495', level: 2 }), rng(1));
  expect(plan.facts).toEqual({ code: '495', name: 'carpet', base: 3, level: 2, side: 9, cells: 81, filled: 64, loops: 3, strands: 18, law: { formula: '(8^n - 1)/7 - 3^n + n + 1', loops: 3 } });
  expect(plan.lit.length).toBe(3);
  expect(plan.clock.span).toBe(LEAST);
  const again = study(math, value({ code: '495', level: 2 }), rng(1));
  expect(again.order).toBe(plan.order);
  expect(Array.from(again.lit)).toEqual(Array.from(plan.lit));
  expect(Array.from(study(math, value({ code: '495', level: 2 }), rng(2)).lit)).not.toEqual(Array.from(plan.lit));
  expect(study(math, value({ code: '5', base: 2, level: 2 }), rng(1)).facts).toMatchObject({ name: 'vtree', side: 4, loops: 0, strands: 8, law: { formula: '0', loops: 0 } });
  expect(study(math, value({ code: '500', level: 2 }), rng(1)).facts.law).toBe(null);
});

test('the schedule traces loops one after another at the pace, spreads a few over the least span and caps many at the most', () => {
  const beat = (len) => BEAT + (len / PACE) * 1000;
  const few = schedule([4, 8]);
  expect([few.span, few.start[0], few.end[0], few.end[1]]).toEqual([LEAST, 0, beat(4), LEAST]);
  expect(few.start[1]).toBe(LEAST - beat(8));
  const lengths = Array.from({ length: 20 }, (_, i) => 8 + i * 2);
  const some = schedule(lengths);
  const sum = lengths.reduce((a, len) => a + beat(len), 0);
  expect(some.span).toBe(sum);
  expect(Array.from(some.start).map((s) => Math.round(s))).toEqual(lengths.map((_, i) => Math.round(lengths.slice(0, i).reduce((a, len) => a + beat(len), 0))));
  const many = schedule(Array.from({ length: 400 }, () => 20));
  expect([many.span, many.start[0], many.end[399]]).toEqual([MOST, 0, MOST]);
  expect(Array.from(many.start).every((s, i) => !i || s >= many.start[i - 1])).toBe(true);
  expect(schedule([]).span).toBe(0);
  expect(Array.from(many.byStart)).toEqual(Array.from(many.start.keys()));
  const long = schedule([4000]);
  expect([long.span, long.start[0], long.end[0]]).toEqual([MOST, 0, MOST]);
});

test('a compressed schedule starts its loops out of order, so the lists by start and by end are sorted on their own', () => {
  const mixed = schedule([40, 400, 40, 400, 40, 400, 40, 400]);
  expect(mixed.span).toBe(MOST);
  expect(Array.from(mixed.start).some((s, i) => i && s < mixed.start[i - 1])).toBe(true);
  for (const [list, times] of [[mixed.byStart, mixed.start], [mixed.byEnd, mixed.end]]) {
    expect(Array.from(list).sort((a, b) => a - b)).toEqual(Array.from(times.keys()));
    expect(Array.from(list).every((k, i) => !i || times[k] >= times[list[i - 1]])).toBe(true);
  }
});

test('the phase runs over the span and the hold then starts over, and a still is past everything', () => {
  expect([phase(0, 6000, false), phase(3000, 6000, false), phase(6000 + HOLD - 1, 6000, false), phase(6000 + HOLD, 6000, false)]).toEqual([0, 3000, 6000 + HOLD - 1, 0]);
  expect(phase(-100, 6000, false)).toBe(6000 + HOLD - 100);
  expect(phase(5, 6000, true)).toBe(Infinity);
});

test('the drift is nothing at t 0, sways inside half the margin of each axis and is a function of t alone', () => {
  const box = { px: 60, x: 130, y: 30 };
  expect(drift(0, box)).toEqual([0, 0]);
  const points = Array.from({ length: 400 }, (_, i) => drift(i * 250, box));
  expect(points.every(([x, y]) => Math.abs(x) <= SWAY * box.x && Math.abs(y) <= SWAY * box.y)).toBe(true);
  expect(Math.max(...points.map(([x]) => x))).toBeGreaterThan(0.9 * SWAY * box.x);
  expect(Math.min(...points.map(([, y]) => y))).toBeLessThan(-0.9 * SWAY * box.y);
  expect(drift(ORBIT[0] * Math.PI / 2, box)[0]).toBeCloseTo(SWAY * box.x, 6);
  expect(drift(1234, box)).toEqual(drift(1234, { ...box }));
  expect(drift(1234, { px: 1, x: -5, y: -5 })).toEqual([0, 0]);
});

test('the fit centres the square on the stage inside the padding', () => {
  expect(fit(9, 800, 600, 30)).toEqual({ px: 60, x: 130, y: 30 });
  expect(fit(4, 400, 1000, 0)).toEqual({ px: 100, x: 0, y: 300 });
});

test('a trail is one move and an arc per step, closed on a loop, and the picture is a sheet of rects and two paths in the shades', () => {
  expect(bend(5, 10, 15, true)).toBe('A5 5 0 0 1 10 15');
  const plan = study(math, value({ code: '9', base: 2, level: 1 }), rng(1));
  const loop = trail(plan, plan.closed.indexOf(1), UNIT);
  expect(loop.match(/A/g)).toHaveLength(4);
  expect(loop.startsWith('M')).toBe(true);
  expect(loop.endsWith('Z')).toBe(true);
  expect(loop).toContain('A5 5 0 0 ');
  const strand = trail(plan, plan.closed.indexOf(0), UNIT);
  expect(strand.match(/A/g)).toHaveLength(1);
  expect(strand.endsWith('Z')).toBe(false);
  const text = picture(plan, { look: 'all', width: 0.16, cells: 1 }, SHADES);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20">')).toBe(true);
  expect(text.match(/<rect /g)).toHaveLength(2);
  expect(text).toContain(`<g fill="${SHADES.fill}">`);
  expect(text.match(/<path /g)).toHaveLength(2);
  expect(text).toContain(`stroke="${SHADES.dim}" stroke-width="1.6"`);
  expect(text).toContain(`stroke="${SHADES.accent}"`);
  const bare = picture(plan, { look: 'loops', width: 0.3, cells: 0 }, SHADES);
  expect(bare.match(/<path /g)).toHaveLength(1);
  expect(bare).not.toContain('<rect');
  expect(bare).toContain('stroke-width="3"');
  expect(picture(plan, { look: 'strands', width: 0.3, cells: 0 }, SHADES)).not.toContain(SHADES.accent);
});
