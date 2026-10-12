import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { AUTO, BEAT, KEEP, LEAST, MOST, PACE, THICK, THIN, UNIT, arc, bend, corner, curves, fit, looping, picture, resolve, schedule, study, thick, trail } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 3, code: '495', ...over });

const SHADES = { loop: '#ff8f2c', strand: '#008cff', dim: 'rgb(115, 64, 20)', fill: '#1f1f20' };

test('the level runs to eight at base 2, five at base 3, four at base 4 and three at base 5', () => {
  const level = (over) => tidy(SPEC, value({ level: 9, ...over }), { math }).level;
  expect([level({ base: 2 }), level({ base: 3 }), level({ base: 4 }), level({ base: 5 }), level({ base: 3, level: 2 })]).toEqual([8, 5, 4, 3, 2]);
});

test('a blank code is the carpet of its base, a design word is its code and any other text is kept for the crate to read', () => {
  expect([2, 3, 4, 5].map((base) => resolve(math, base, ''))).toEqual([2, 3, 4, 5].map((base) => math.name.words(2, base).find(([, word]) => word === 'carpet')[0]));
  expect([resolve(math, 2, ''), resolve(math, 3, ' ')]).toEqual(['7', '495']);
  expect([resolve(math, 2, 'Net'), resolve(math, 3, '600'), resolve(math, 2, 'abc')]).toEqual(['14', '600', 'abc']);
});

test('a rolled design loops at the level it is asked for at every base, the same for one seed', () => {
  for (const [base, level] of [[2, 1], [2, 3], [3, 2], [4, 2], [5, 1]]) {
    const rolled = Array.from({ length: 20 }, (_, seed) => looping(math, base, level, rng(seed)));
    expect([base, level, rolled.every((code) => math.arcs.draw(code, base, level).loops > 0)]).toEqual([base, level, true]);
    expect(looping(math, base, level, rng(3))).toBe(rolled[3]);
  }
  expect(new Set(Array.from({ length: 20 }, (_, seed) => looping(math, 3, 3, rng(seed)))).size).toBeGreaterThan(10);
});

test('a filled cell wraps its lower-left and upper-right corners, an empty one the other two, row 0 at the bottom', () => {
  expect([corner(1, 0, 2, 3), corner(1, 1, 2, 3), corner(0, 0, 2, 3), corner(0, 1, 2, 3)]).toEqual([[2, 3], [3, 4], [3, 3], [2, 4]]);
});

test('the arcs of a curve chain through shared midpoints, a loop closes and a strand ends on the border', () => {
  const plan = curves(math, 495, 3, 2);
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

test('the one loop of the diagonal is the circle round the centre, its arcs sweeping a quarter each', () => {
  const plan = curves(math, 9, 2, 1);
  expect([plan.loops, plan.strands]).toEqual([1, 4]);
  const k = plan.closed.indexOf(1);
  const arcs = Array.from(plan.order.subarray(plan.bounds[k], plan.bounds[k + 1]), (a) => arc(plan, a));
  expect(arcs.map((g) => [g.cx, g.cy])).toEqual([[1, 1], [1, 1], [1, 1], [1, 1]]);
  expect(arcs.map((g) => Math.abs(g.sweep))).toEqual([Math.PI / 2, Math.PI / 2, Math.PI / 2, Math.PI / 2]);
  expect(new Set(arcs.map((g) => `${g.x},${g.y}`)).size).toBe(4);
});

test('the study reads the counts and the law, lights every loop in the crate order and clamps the level to the cap', () => {
  const plan = study(math, value({ level: 2 }));
  expect(plan.facts).toEqual({ code: '495', name: 'carpet', base: 3, level: 2, side: 9, cells: 81, filled: 64, loops: 3, strands: 18, law: { formula: '(8^n - 1)/7 - 3^n + n + 1', loops: 3 } });
  expect(Array.from(plan.lit)).toEqual(Array.from(plan.closed.keys()).filter((k) => plan.closed[k]));
  expect(study(math, value({ base: 2, code: '', level: 5 })).facts).toMatchObject({ code: '7', name: 'carpet', side: 32, loops: 50, strands: 64, law: { loops: 50 } });
  expect(study(math, value({ base: 2, code: '5', level: 2 })).facts).toMatchObject({ name: 'vtree', side: 4, loops: 0, strands: 8, law: { formula: '0', loops: 0 } });
  expect(study(math, value({ code: '500', level: 2 })).facts.law).toBe(null);
  expect(study(math, value({ base: 5, code: '', level: 9 })).facts.level).toBe(3);
});

test('a code the crate cannot read is a fault in its own words with nothing to draw', () => {
  const bad = study(math, value({ base: 2, code: '99' }));
  expect(bad.side).toBe(undefined);
  expect(bad.facts).toEqual({ code: '99', base: 2, level: 5, fault: 'code 99 out of range for dimension 2 base 2 (0..15).' });
  expect(study(math, value({ code: 'abc' })).facts.fault).toContain('decimal');
});

test('the study keeps the last few plans, so a level dragged back is the same plan, and lets the oldest go', () => {
  const one = study(math, value({ level: 2 }));
  study(math, value({ level: 3 }));
  expect(study(math, value({ level: 2 }))).toBe(one);
  for (let code = 1; code <= KEEP; code++) study(math, value({ code: String(code), level: 1 }));
  expect(study(math, value({ level: 2 }))).not.toBe(one);
});

test('the schedule traces loops one after another at the pace, spreads a few over the least span and caps many at the most', () => {
  const beat = (len) => BEAT + (len / PACE) * 1000;
  const few = schedule([4, 8]);
  expect([few.span, few.start[0], few.end[0], few.end[1]]).toEqual([LEAST, 0, beat(4), LEAST]);
  expect(few.start[1]).toBe(LEAST - beat(8));
  const lengths = Array.from({ length: 12 }, (_, i) => 8 + i * 2);
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

test('the fit centres the square on the stage inside the padding', () => {
  expect(fit(9, 800, 600, 30)).toEqual({ px: 60, x: 130, y: 30 });
  expect(fit(4, 400, 1000, 0)).toEqual({ px: 100, x: 0, y: 300 });
});

test('an auto stroke is a share of a cell held between a hair and three pixels, a set one a share never under a device pixel', () => {
  expect([thick(0, 10, 2), thick(0, 1, 2), thick(0, 200, 2)]).toEqual([AUTO * 10, THIN * 2, THICK * 2]);
  expect([thick(0.3, 100, 2), thick(0.02, 10, 2)]).toEqual([30, 2]);
});

test('a trail is one move and an arc per step, closed on a loop, and the picture is a flipped sheet of rects and two paths in the shades', () => {
  expect(bend(5, 10, 15, true)).toBe('A5 5 0 0 1 10 15');
  const plan = study(math, value({ code: '9', base: 2, level: 1 }));
  const loop = trail(plan, plan.closed.indexOf(1), UNIT);
  expect(loop.match(/A/g)).toHaveLength(4);
  expect(loop.startsWith('M')).toBe(true);
  expect(loop.endsWith('Z')).toBe(true);
  expect(loop).toContain('A5 5 0 0 ');
  const strand = trail(plan, plan.closed.indexOf(0), UNIT);
  expect(strand.match(/A/g)).toHaveLength(1);
  expect(strand.endsWith('Z')).toBe(false);
  const text = picture(plan, { look: 'all', width: 0, cells: 1 }, SHADES);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20"><g transform="matrix(1 0 0 -1 0 20)">')).toBe(true);
  expect(text.match(/<rect /g)).toHaveLength(2);
  expect(text).toContain(`<g fill="${SHADES.fill}">`);
  expect(text.match(/<path /g)).toHaveLength(2);
  expect(text).toContain(`stroke="${SHADES.strand}" stroke-width="1.6"`);
  expect(text).toContain(`stroke="${SHADES.loop}"`);
  const bare = picture(plan, { look: 'loops', width: 0.3, cells: 0 }, SHADES);
  expect(bare.match(/<path /g)).toHaveLength(1);
  expect(bare).not.toContain('<rect');
  expect(bare).toContain('stroke-width="3"');
  expect(picture(plan, { look: 'strands', width: 0.3, cells: 0 }, SHADES)).not.toContain(SHADES.loop);
});
