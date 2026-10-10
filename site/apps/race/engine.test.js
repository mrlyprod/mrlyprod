import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BUDGET, GAIN, HOLD, LEAST, LIMIT, NOTCH, REACH, ROLLS, SHARE, STALL, STILL, board, cap, ceiling, fills, goal, grow, heat, home, kin, lay, most, open, pair, plan, reads, roll, room, roots, series, spread, step, turn } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, ...over });

const race = (over, seed = 1) => {
  const { code, walk } = roots(rng(seed));
  return { plan: plan(math, value(over), code), walk };
};

test('the level cap keeps a board under the budget of cells', () => {
  expect([cap(3), cap(5), cap(7)]).toEqual([5, 3, 3]);
  for (const number of [3, 5, 7]) {
    const level = cap(number);
    expect(number ** (2 * level) <= BUDGET && number ** (2 * (level + 1)) > BUDGET).toBe(true);
  }
});

test('the goal is the finish share of the half side, never under one cell', () => {
  expect([goal(81, 50), goal(81, 100), goal(9, 50), goal(3, 10), goal(243, 25)]).toEqual([20, 40, 2, 1, 30]);
});

test('the most a board offers is the top share whose goal stays within the reach in cells, never under the least', () => {
  expect([3, 9, 27, 81, 243, 5, 25, 125, 7, 49, 343].map(most)).toEqual([55, 55, 55, 55, 20, 55, 55, 45, 55, 55, 15]);
  for (const side of [81, 243, 125, 343]) {
    const top = most(side);
    expect(goal(side, top)).toBeLessThanOrEqual(REACH);
    if (top < SHARE) expect(goal(side, top + NOTCH)).toBeGreaterThan(REACH);
  }
  expect(most(2187)).toBe(LEAST);
});

test('a sparse pair ends at the goal at the most its board offers', () => {
  const end = (level, a, b, seed) => {
    const { plan: laid, walk } = race({ level, a, b, finish: most(3 ** level) }, seed);
    const run = series(laid, walk);
    run.finish();
    return run.state.over.how;
  };
  expect([end(5, '486', '303', 673128), end(4, '316', '151', 97 * 104729 + 7)]).toEqual(['goal', 'goal']);
});

test('a typed code is kept modulo the count, a blank or junk one is null, a rolled one is never empty or solid', () => {
  expect([reads('127', 512), reads(' 600 ', 512), reads('-1', 512), reads('7.9', 16)]).toEqual(['127', '88', '511', '7']);
  expect([reads('', 512), reads('carpet', 512), reads(undefined, 512)]).toEqual([null, null, null]);
  const codes = Array.from({ length: 200 }, (_, i) => Number(roll(rng(i), 16)));
  expect(Math.min(...codes)).toBeGreaterThan(0);
  expect(Math.max(...codes)).toBeLessThan(15);
  expect(new Set(Array.from({ length: 4000 }, (_, i) => roll(rng(i), 512))).has('511')).toBe(false);
});

test('the roots of one seed are one code stream and one walk seed, the same twice', () => {
  const one = roots(rng(5));
  const two = roots(rng(5));
  expect([one.code(), one.walk]).toEqual([two.code(), two.walk]);
  expect(roots(rng(6)).walk).not.toBe(one.walk);
});

test('home is the filled cell nearest the centre, the first on a tie, and none on an empty board', () => {
  expect(home(Uint8Array.from([0, 0, 0, 0, 1, 0, 0, 0, 0]), 3)).toBe(4);
  expect(home(Uint8Array.from([0, 1, 0, 1, 0, 1, 0, 1, 0]), 3)).toBe(1);
  expect(home(new Uint8Array(9), 3)).toBe(-1);
  expect(home(Uint8Array.from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 4)).toBe(5);
});

test('the room is the component of home by sides, with the rms distance of an even spread over it', () => {
  const filled = Uint8Array.from([1, 1, 0, 0, 1, 0, 1, 1, 1]);
  expect(room(filled, 3, 4)).toEqual({ cells: 6, rms: Math.sqrt(8 / 6) });
  expect(room(filled, 3, 0)).toEqual({ cells: 6, rms: Math.sqrt(20 / 6) });
  expect(room(Uint8Array.from([1, 0, 1, 0, 1, 0, 1, 0, 1]), 3, 4)).toEqual({ cells: 1, rms: 0 });
  expect(room(filled, 3, -1)).toEqual({ cells: 0, rms: 0 });
});

test('a board grown from 127 at level 4 is 81 a side with 2401 cells all in one room, 7 of 9 filled', () => {
  const grown = grow(math, value(), '127');
  expect([grown.side, grown.lit, grown.cells, grown.home]).toEqual([81, 2401, 2401, 3280]);
  expect(grown.filled[grown.home]).toBe(1);
  const read = board(math, value(), '127');
  expect([read.fills, read.of, read.name, read.title]).toEqual([7, 9, '', 'bang dim 2, base 3, code 127']);
  expect(read.dimension).toBeCloseTo(Math.log(7) / Math.log(3));
  expect(fills(math, value(), '239')).toBe(7);
  expect(board(math, value({ base: 2, level: 2 }), '7').name).toBe('carpet');
});

test('a pair rolled from a seed is the same twice, and a typed side is kept with a mate of its mass', () => {
  const rolled = pair(math, value(), roots(rng(1)).code);
  expect(rolled).toEqual(pair(math, value(), roots(rng(1)).code));
  expect(rolled[0]).not.toBe(rolled[1]);
  expect(pair(math, value(), roots(rng(1)).code, ['127', '239'])).toEqual(['127', '239']);
  const [a, b] = pair(math, value(), roots(rng(1)).code, ['127', null]);
  expect([a, fills(math, value(), b)]).toEqual(['127', 7]);
  const [c, d] = pair(math, value(), roots(rng(1)).code, [null, '239']);
  expect([d, fills(math, value(), c)]).toEqual(['239', 7]);
  expect(pair(math, value({ base: 2 }), roots(rng(2)).code).every((code) => Number(code) > 0 && Number(code) < 15)).toBe(true);
  expect(ROLLS).toBeGreaterThan(8);
});

test('a rolled pair is always of one mass, never solid, and both sides spread past the goal when evenly spread over their room', () => {
  const far = goal(81, 50);
  const seeds = [34 * 7919 + 13, ...Array.from({ length: 150 }, (_, i) => i * 7919 + 13)];
  for (const seed of seeds) {
    const [a, b] = pair(math, value(), roots(rng(seed)).code);
    expect([seed, a === b, a === '511' || b === '511', fills(math, value(), a) === fills(math, value(), b), ceiling(math, value(), a) >= far, ceiling(math, value(), b) >= far]).toEqual([seed, false, false, true, true, true]);
  }
});

test('the ceiling of a code is the rms of its room, which a far cell would overstate', () => {
  const far = goal(81, 50);
  for (const code of ['75', '420', '201', '39', '456', '294', '480', '15']) expect(ceiling(math, value(), code)).toBeLessThan(far);
  expect(ceiling(math, value(), '127')).toBe(grow(math, value(), '127').rms);
  expect(ceiling(math, value(), '127')).toBeGreaterThan(far);
});

test('the kin of a code are the other codes of its mass, never empty or solid', () => {
  const list = kin(math, value(), '127', 512);
  expect(list).toHaveLength(35);
  expect(list.includes('127') || list.includes('0') || list.includes('511')).toBe(false);
  expect(list.every((code) => fills(math, value(), code) === 7)).toBe(true);
  expect(kin(math, value(), '511', 512)).toEqual([]);
});

test('the plan reads the typed codes, the goal and the keep of the fade per tick', () => {
  const { plan: laid } = race({ a: '127', b: '239' });
  expect(laid.sides.map((side) => side.code)).toEqual(['127', '239']);
  expect([laid.side, laid.goal, laid.walkers, laid.speed, laid.still]).toEqual([81, 20, 300, 60, Math.floor(STILL / 600)]);
  expect(laid.keep).toBeCloseTo(0.8 ** (1 / 60));
  expect(race({ heat: 0 }).plan.keep).toBe(1);
});

test('heat decays by the keep a tick since its stamp and holds when nothing fades', () => {
  expect(heat(4, 10, 10, 0.5)).toBe(4);
  expect(heat(4, 10, 12, 0.5)).toBe(1);
  expect(heat(4, 10, 99, 1)).toBe(4);
});

test('the spread is the root mean square distance of the walkers from home', () => {
  expect(spread(Int32Array.from([0, 2, 6, 8]), 4, 3)).toBe(Math.SQRT2);
  expect(spread(Int32Array.from([4, 4]), 4, 3)).toBe(0);
  expect(spread(new Int32Array(0), 4, 3)).toBe(0);
});

test('a step moves every walker one blind step onto a filled cell or leaves it, heats the cell it stands on, and logs the reach', () => {
  const { plan: laid, walk } = race({ a: '127', b: '239', walkers: 50 });
  const state = open(laid);
  expect(state.at.map((at) => at.length)).toEqual([50, 50]);
  expect(state.at[0].every((i) => i === laid.sides[0].home)).toBe(true);
  step(state, laid, rng(walk));
  state.at.forEach((at, s) => {
    const { filled, home: from } = laid.sides[s];
    const side = laid.side;
    for (const i of at) {
      expect(filled[i]).toBe(1);
      expect(Math.abs(Math.floor(i / side) - Math.floor(from / side)) + Math.abs((i % side) - (from % side))).toBeLessThanOrEqual(1);
    }
    expect(state.trail[s].reduce((sum, h) => sum + h, 0)).toBeCloseTo(50);
  });
  expect([state.tick, state.log.length, state.reach[0] > 0, state.over]).toEqual([1, 2, true, null]);
});

test('the race ends at the goal with the side that reached it, and is the same in one jump or by steps', () => {
  const { plan: laid, walk } = race({ a: '127', b: '239' });
  const probe = series(laid, walk);
  probe.finish();
  const end = probe.state.over.tick + 10;
  const stepped = series(laid, walk);
  for (let t = 0; t <= end; t += 1) stepped.to(t);
  const jumped = series(laid, walk);
  jumped.to(end);
  expect([jumped.state.tick, jumped.state.reach, jumped.state.over, [...jumped.state.at[0]]]).toEqual([stepped.state.tick, stepped.state.reach, stepped.state.over, [...stepped.state.at[0]]]);
  expect(jumped.state.over).toMatchObject({ winner: 0, how: 'goal' });
  expect(jumped.state.reach[0]).toBeGreaterThanOrEqual(laid.goal);
  expect(jumped.state.reach[1]).toBeLessThan(laid.goal);
  expect(jumped.state.log.length).toBe(jumped.state.over.tick + 1);
  expect(jumped.tally).toEqual([1, 0, 0]);
});

test('a finish with a limit stops short of the end without a judgement, and a later finish carries on to the same end', () => {
  const { plan: laid, walk } = race({ a: '127', b: '239' });
  const whole = series(laid, walk);
  whole.finish();
  const cut = series(laid, walk);
  cut.finish(100);
  expect([cut.state.tick, cut.state.over, cut.tally]).toEqual([100, null, [0, 0, 0]]);
  cut.finish();
  expect([cut.state.over, cut.tally, [...cut.state.at[0]]]).toEqual([whole.state.over, whole.tally, [...whole.state.at[0]]]);
});

test('a board with no way out stalls into a judgement and an empty board fields no walker', () => {
  const dust = race({ a: '1', b: '1', number: 3, level: 1 });
  const run = series(dust.plan, dust.walk);
  run.finish();
  expect(run.state.over).toMatchObject({ tick: STALL, winner: -1, how: 'stall' });
  expect(run.tally).toEqual([0, 0, 1]);
  const none = race({ a: '0', b: '127', level: 2 });
  expect(open(none.plan).at[0].length).toBe(0);
  expect([GAIN > 0, LIMIT > STALL]).toEqual([true, true]);
});

test('the series holds after a finish, then runs the next race on the next walk seed and keeps the tally', () => {
  const { plan: laid, walk } = race({ a: '127', b: '239' });
  const run = series(laid, walk);
  run.finish();
  const end = run.state.over.tick;
  const hold = Math.ceil((HOLD * laid.speed) / 1000);
  run.to(end + hold - 1);
  expect([run.k, run.state.tick, run.state.over.tick]).toEqual([0, end, end]);
  run.to(end + hold);
  expect([run.k, run.start, run.state.tick, run.state.over]).toEqual([1, end + hold, 0, null]);
  run.to(end + hold + 5);
  expect(run.state.tick).toBe(5);
  const again = series(laid, walk);
  again.to(end + hold + 5);
  expect([...again.state.at[1]]).toEqual([...run.state.at[1]]);
  expect(again.tally).toEqual([1, 0, 0]);
  expect(again.version).toBe(run.version);
});

test('the layout puts the boards side by side or stacked, whichever gives the bigger cell, in whole pixels', () => {
  const wide = lay(800, 400, 81, 16, 16);
  expect([wide.flat, wide.cell, wide.edge]).toEqual([true, 4, 324]);
  expect(wide.boards).toEqual([{ x: 68, y: 38 }, { x: 408, y: 38 }]);
  const tall = lay(400, 800, 81, 16, 16);
  expect([tall.flat, tall.cell]).toEqual([false, 4]);
  expect(tall.boards).toEqual([{ x: 38, y: 68 }, { x: 38, y: 408 }]);
  const tiny = lay(100, 100, 243, 16, 16);
  expect(tiny.cell).toBeLessThan(1);
  expect(tiny.cell).toBeGreaterThan(0);
});

test('a turn of half a circle takes blue to orange and twice is the colour again, grey staying grey', () => {
  expect(turn('#008cff', 180)).toBe('#ff7300');
  expect(turn(turn('#008cff', 180), 180)).toBe('#008cff');
  expect(turn('rgb(0, 140, 255)', 180)).toBe('#ff7300');
  expect(turn('#808080', 90)).toBe('#808080');
});
