import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { defaults } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Act, Config } from "./engine.ts";
import {
  aiming,
  BUDGET,
  bake,
  bits,
  board,
  boot,
  cap,
  ceiling,
  deal,
  fills,
  GAIN,
  grow,
  HOLD,
  heat,
  home,
  kin,
  LIMIT,
  lay,
  moves,
  pair,
  plan,
  REACH,
  ROLLS,
  reach,
  read,
  reads,
  roll,
  room,
  roots,
  STALL,
  STILL,
  series,
  spread,
  step,
  TICK,
  top,
} from "./engine.ts";
import type { Knobs } from "./scene.ts";
import { SPEC } from "./scene.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const value = (over?: Partial<Knobs>) => ({ ...(defaults(SPEC) as Knobs), seed: 1, ...over });

const race = (over?: Partial<Knobs>, seed = 1) => {
  const { code, walk } = roots(rng(seed));
  return { plan: plan(math, value(over), code), walk };
};

test("the level cap keeps a board under the budget of cells", () => {
  expect([cap(3), cap(5), cap(7)]).toEqual([5, 3, 3]);
  for (const number of [3, 5, 7]) {
    const level = cap(number);
    expect(number ** (2 * level) <= BUDGET && number ** (2 * (level + 1)) > BUDGET).toBe(true);
  }
});

test("the most a finish asks is 55 hundredths of the half side, never past the reach of a swarm or under one cell", () => {
  expect([3, 9, 27, 81, 243, 5, 25, 125, 7, 49, 343].map(top)).toEqual([1, 2, 7, 22, 28, 1, 7, 28, 2, 13, 28]);
  expect(Math.max(...[3, 9, 27, 81, 243, 2187].map(top))).toBe(REACH);
});

test("a sparse pair ends at the goal at the most its board offers", () => {
  const end = (level: number, a: string, b: string, seed: number) => {
    const { plan: laid, walk } = race({ level, a, b, finish: top(3 ** level) }, seed);
    const run = series(laid, walk);
    run.finish();
    return run.state.over!.how;
  };
  expect([end(5, "486", "303", 673128), end(4, "316", "151", 97 * 104729 + 7)]).toEqual(["goal", "goal"]);
});

test("a typed code is kept modulo the count, a blank or junk one is null, a rolled one is never empty or solid", () => {
  expect([reads("127", 512), reads(" 600 ", 512), reads("-1", 512), reads("7.9", 16)]).toEqual([
    "127",
    "88",
    "511",
    "7",
  ]);
  expect([reads("", 512), reads("carpet", 512), reads(undefined, 512)]).toEqual([null, null, null]);
  const codes = Array.from({ length: 200 }, (_, i) => Number(roll(rng(i), 16)));
  expect(Math.min(...codes)).toBeGreaterThan(0);
  expect(Math.max(...codes)).toBeLessThan(15);
  expect(new Set(Array.from({ length: 4000 }, (_, i) => roll(rng(i), 512))).has("511")).toBe(false);
});

test("the roots of one seed are one code stream and one walk seed, the same twice", () => {
  const one = roots(rng(5));
  const two = roots(rng(5));
  expect([one.code(), one.walk]).toEqual([two.code(), two.walk]);
  expect(roots(rng(6)).walk).not.toBe(one.walk);
});

test("home is the filled cell nearest the centre, the first on a tie, and none on an empty board", () => {
  expect(home(Uint8Array.from([0, 0, 0, 0, 1, 0, 0, 0, 0]), 3)).toBe(4);
  expect(home(Uint8Array.from([0, 1, 0, 1, 0, 1, 0, 1, 0]), 3)).toBe(1);
  expect(home(new Uint8Array(9), 3)).toBe(-1);
  expect(home(Uint8Array.from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 4)).toBe(5);
});

test("the room is the component of home by sides, with the rms distance of an even spread over it", () => {
  const filled = Uint8Array.from([1, 1, 0, 0, 1, 0, 1, 1, 1]);
  expect(room(filled, 3, 4)).toEqual({ cells: 6, rms: Math.sqrt(8 / 6) });
  expect(room(filled, 3, 0)).toEqual({ cells: 6, rms: Math.sqrt(20 / 6) });
  expect(room(Uint8Array.from([1, 0, 1, 0, 1, 0, 1, 0, 1]), 3, 4)).toEqual({ cells: 1, rms: 0 });
  expect(room(filled, 3, -1)).toEqual({ cells: 0, rms: 0 });
});

test("a board grown from 127 at level 4 is 81 a side with 2401 cells all in one room, 7 of 9 filled", () => {
  const grown = grow(math, value(), "127");
  expect([grown.side, grown.lit, grown.cells, grown.home]).toEqual([81, 2401, 2401, 3280]);
  expect(grown.filled[grown.home]).toBe(1);
  const read = board(math, value(), "127");
  expect([read.fills, read.of, read.name, read.title]).toEqual([7, 9, "", "bang dim 2, base 3, code 127"]);
  expect(read.dimension).toBeCloseTo(Math.log(7) / Math.log(3));
  expect(fills(math, value(), "239")).toBe(7);
  expect(board(math, value({ base: 2, level: 2 }), "7").name).toBe("carpet");
});

test("a pair rolled from a seed is the same twice, and a typed side is kept with a mate of its mass", () => {
  const rolled = pair(math, value(), roots(rng(1)).code);
  expect(rolled).toEqual(pair(math, value(), roots(rng(1)).code));
  expect(rolled[0]).not.toBe(rolled[1]);
  expect(pair(math, value(), roots(rng(1)).code, ["127", "239"])).toEqual(["127", "239"]);
  const [a, b] = pair(math, value(), roots(rng(1)).code, ["127", null]);
  expect([a, fills(math, value(), b)]).toEqual(["127", 7]);
  const [c, d] = pair(math, value(), roots(rng(1)).code, [null, "239"]);
  expect([d, fills(math, value(), c)]).toEqual(["239", 7]);
  expect(
    pair(math, value({ base: 2 }), roots(rng(2)).code).every((code) => Number(code) > 0 && Number(code) < 15),
  ).toBe(true);
  expect(ROLLS).toBeGreaterThan(8);
});

test("a rolled pair is always of one mass, never solid, and both sides spread past the goal when evenly spread over their room", () => {
  const far = value().finish;
  const seeds = [34 * 7919 + 13, ...Array.from({ length: 150 }, (_, i) => i * 7919 + 13)];
  for (const seed of seeds) {
    const [a, b] = pair(math, value(), roots(rng(seed)).code);
    expect([
      seed,
      a === b,
      a === "511" || b === "511",
      fills(math, value(), a) === fills(math, value(), b),
      ceiling(math, value(), a) >= far,
      ceiling(math, value(), b) >= far,
    ]).toEqual([seed, false, false, true, true, true]);
  }
});

test("the ceiling of a code is the rms of its room, which a far cell would overstate", () => {
  const far = value().finish;
  for (const code of ["75", "420", "201", "39", "456", "294", "480", "15"])
    expect(ceiling(math, value(), code)).toBeLessThan(far);
  expect(ceiling(math, value(), "127")).toBe(grow(math, value(), "127").rms);
  expect(ceiling(math, value(), "127")).toBeGreaterThan(far);
});

test("the kin of a code are the other codes of its mass, never empty or solid", () => {
  const list = kin(math, value(), "127", 512);
  expect(list).toHaveLength(35);
  expect(list.includes("127") || list.includes("0") || list.includes("511")).toBe(false);
  expect(list.every((code) => fills(math, value(), code) === 7)).toBe(true);
  expect(kin(math, value(), "511", 512)).toEqual([]);
});

test("the plan reads the typed codes, the finish in cells and the keep of the fade per tick, none by default", () => {
  const { plan: laid } = race({ a: "127", b: "239" });
  expect(laid.sides.map((side) => side.code)).toEqual(["127", "239"]);
  expect([laid.side, laid.goal, laid.walkers, laid.speed, laid.still, laid.keep]).toEqual([
    81,
    20,
    300,
    720,
    Math.floor(STILL / 600),
    1,
  ]);
  expect(race({ heat: 20, speed: 60 }).plan.keep).toBeCloseTo(0.8 ** (1 / 60));
});

test("heat decays by the keep a tick since its stamp and holds when nothing fades", () => {
  expect(heat(4, 10, 10, 0.5)).toBe(4);
  expect(heat(4, 10, 12, 0.5)).toBe(1);
  expect(heat(4, 10, 99, 1)).toBe(4);
});

const DUST: Config = { side: 3, boards: ["111111111", "010111010"], homes: [4, 4], walkers: 6, goal: 2, seed: 42 };
const plain = (x: unknown): boolean =>
  x === null ||
  typeof x === "string" ||
  Number.isInteger(x) ||
  (typeof x === "object" && Object.values(x).every(plain));
const replay = (acts: Act[], config = DUST) => acts.reduce(step, boot(config));
const GAME: Act[] = [TICK, TICK, aiming(1), TICK, aiming(3), TICK, TICK];

test("the spread sums the squared distances of the walkers from home, and the reach is its root mean", () => {
  expect([spread([0, 2, 6, 8], 4, 3), spread([4, 4], 4, 3), spread([], 4, 3)]).toEqual([8, 0, 0]);
  const state = { ...boot(DUST), at: [[0, 2, 6, 8], []], sums: [8, 0] };
  expect(reach(state)).toEqual([Math.SQRT2, 0]);
});

test("boot fields every walker at home on its board as strings and integers, and none on an empty board", () => {
  const state = boot(DUST);
  expect([state.at, state.tick, state.over, plain(state)]).toEqual([
    [Array(6).fill(4), Array(6).fill(4)],
    0,
    null,
    true,
  ]);
  const { plan: laid, walk } = race({ a: "0", b: "127", level: 2 });
  const none = boot(deal(laid, walk));
  expect([none.at[0].length, none.boards[1].length, none.boards[1]]).toEqual([0, 81, bits(laid.sides[1].filled)]);
});

test("a tick moves every walker one blind step onto a filled cell or leaves it", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239", walkers: 50 });
  const state = step(boot(deal(laid, walk)), TICK);
  state.at.forEach((at, s) => {
    const { filled, home: from } = laid.sides[s];
    const side = laid.side;
    for (const i of at) {
      expect(filled[i]).toBe(1);
      expect(
        Math.abs(Math.floor(i / side) - Math.floor(from / side)) + Math.abs((i % side) - (from % side)),
      ).toBeLessThanOrEqual(1);
    }
    expect(at.some((i) => i !== from)).toBe(true);
  });
  expect([state.tick, reach(state)[0] > 0, state.over]).toEqual([1, true, null]);
});

test("the series heats only the cells walkers arrive on and logs the reach every tick", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239", walkers: 50 });
  const run = series(laid, walk);
  run.to(1);
  run.state.at.forEach((at, s) => {
    const { home: from } = laid.sides[s];
    expect([...run.trace.trail[s]].reduce((sum, h) => sum + h, 0)).toBe(at.filter((i) => i !== from).length);
    expect(run.trace.trail[s][from]).toBe(0);
  });
  expect(run.trace.log).toEqual([[0, 0], reach(run.state)]);
});

test("the race ends at the goal with the side that reached it, and is the same in one jump or by steps", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239" });
  const probe = series(laid, walk);
  probe.finish();
  const end = probe.state.over!.tick + 10;
  const stepped = series(laid, walk);
  for (let t = 0; t <= end; t += 1) stepped.to(t);
  const jumped = series(laid, walk);
  jumped.to(end);
  expect(JSON.stringify(jumped.state)).toBe(JSON.stringify(stepped.state));
  expect(jumped.state.over).toMatchObject({ winner: 0, how: "goal" });
  expect(reach(jumped.state)[0]).toBeGreaterThanOrEqual(laid.goal);
  expect(reach(jumped.state)[1]).toBeLessThan(laid.goal);
  expect(jumped.trace.log.length).toBe(jumped.state.over!.tick + 1);
  expect(plain(jumped.state)).toBe(true);
});

test("a finish with a limit stops short of the end without a judgement, and a later finish carries on to the same end", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239" });
  const whole = series(laid, walk);
  whole.finish();
  const cut = series(laid, walk);
  cut.finish(100);
  expect([cut.state.tick, cut.state.over]).toEqual([100, null]);
  cut.finish();
  expect(JSON.stringify(cut.state)).toBe(JSON.stringify(whole.state));
});

test("a board with no way out stalls into a judgement", () => {
  const dust = race({ a: "1", b: "1", number: 3, level: 1 });
  const run = series(dust.plan, dust.walk);
  run.finish();
  expect(run.state.over).toMatchObject({ tick: STALL, winner: -1, how: "stall" });
  expect([GAIN > 0, LIMIT > STALL]).toEqual([true, true]);
});

test("a new finish ends a race past it, a goal end below a raised finish runs on, and a stall stays", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239" });
  const run = series(laid, walk, Infinity);
  run.to(400);
  expect(run.state.over).toBe(null);
  const far = Math.max(...reach(run.state));
  const past = step(run.state, aiming(Math.floor(far)));
  expect([past.goal, past.over]).toMatchObject([Math.floor(far), { tick: 400, how: "goal" }]);
  expect(step(past, aiming(Math.ceil(far) + 1)).over).toBe(null);
  const stalled = { ...run.state, over: { tick: 400, winner: 0, how: "stall" as const } };
  expect(step(stalled, aiming(laid.goal)).over!.how).toBe("stall");
});

test("a bake folds the fade so far into every trail and stamps it now", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239", heat: 50, speed: 6 });
  const run = series(laid, walk);
  run.to(30);
  const { trace } = run;
  const seen = trace.trail[0].map((h, i) => heat(h, trace.stamp[0][i], 30, laid.keep));
  bake(trace, 30, laid.keep);
  expect([...trace.trail[0]].map((h) => Math.round(h * 1e4))).toEqual([...seen].map((h) => Math.round(h * 1e4)));
  expect(trace.stamp[0].every((t) => t === 30)).toBe(true);
});

test("the same seed and acts replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(GAME))).toBe(JSON.stringify(replay(GAME)));
  expect(JSON.stringify(replay(GAME))).not.toBe(JSON.stringify(replay(GAME, { ...DUST, seed: 43 })));
});

test("step leaves its input untouched and its output survives a JSON round trip", () => {
  let state = boot(DUST);
  for (const act of GAME) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    state = next;
  }
});

test("moves lists the tick, and nothing once over", () => {
  const state = boot(DUST);
  expect(moves(state)).toEqual([TICK]);
  expect(moves({ ...state, over: { tick: 0, winner: -1, how: "limit" } })).toEqual([]);
});

test("read walks a slash path into the state", () => {
  const state = boot(DUST);
  expect([read(state), read(state, "goal"), read(state, "at/1/0"), read(state, "nope/x")]).toEqual([
    state,
    2,
    4,
    undefined,
  ]);
});

test("with no hold the series stays on the finished race", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239" });
  const run = series(laid, walk, Infinity);
  run.finish();
  const end = run.state.over;
  run.to(end!.tick * 100);
  expect([run.k, run.state.over]).toEqual([0, end]);
});

test("the series holds after a finish, then runs the next race on the next walk seed", () => {
  const { plan: laid, walk } = race({ a: "127", b: "239" });
  const run = series(laid, walk);
  run.finish();
  const end = run.state.over!.tick;
  const hold = Math.ceil((HOLD * laid.speed) / 1000);
  run.to(end + hold - 1);
  expect([run.k, run.state.tick, run.state.over!.tick]).toEqual([0, end, end]);
  run.to(end + hold);
  expect([run.k, run.start, run.state.tick, run.state.over]).toEqual([1, end + hold, 0, null]);
  run.to(end + hold + 5);
  expect(run.state.tick).toBe(5);
  const again = series(laid, walk);
  again.to(end + hold + 5);
  expect([...again.state.at[1]]).toEqual([...run.state.at[1]]);
  expect(again.version).toBe(run.version);
});

test("the layout puts the boards side by side or stacked, whichever gives the bigger cell, in whole pixels", () => {
  const wide = lay(800, 400, 81, 16, 16);
  expect([wide.flat, wide.cell, wide.edge]).toEqual([true, 4, 324]);
  expect(wide.boards).toEqual([
    { x: 68, y: 38 },
    { x: 408, y: 38 },
  ]);
  const tall = lay(400, 800, 81, 16, 16);
  expect([tall.flat, tall.cell]).toEqual([false, 4]);
  expect(tall.boards).toEqual([
    { x: 38, y: 68 },
    { x: 38, y: 408 },
  ]);
  const tiny = lay(100, 100, 243, 16, 16);
  expect(tiny.cell).toBeLessThan(1);
  expect(tiny.cell).toBeGreaterThan(0);
});
