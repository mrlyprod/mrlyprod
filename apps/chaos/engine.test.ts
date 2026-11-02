import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import * as num from "@mrlyprod/num";
import { rng } from "../../net/sdk/rand.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import type { Act, Source, State } from "./engine.ts";
import {
  binary,
  boot,
  cap,
  cell,
  configOf,
  csv,
  cut,
  DRAW,
  digits,
  distance,
  feed,
  fit,
  fromCloud,
  games,
  hit,
  law,
  lay,
  MOSAICS,
  meter,
  modeOf,
  modes,
  moves,
  odds,
  origin,
  pair,
  read,
  replay,
  restricted,
  ruleCode,
  schedule,
  sideOf,
  sides,
  spelt,
  spot,
  stationary,
  step,
  study,
  taking,
  toCloud,
} from "./engine.ts";
import { SPEC, tidied } from "./scene.ts";

const units = { math, num };

const value = (over: Raw) => tidied({ ...defaults(SPEC), seed: 1, base: 2, code: "7", ...over }, units);

const source = (over: Raw) => study(units, value(over));

const game = (
  one: Source,
  k: number,
  start: [number, number],
  seed = 1,
  sched: Uint8Array | null = null,
  heavy = false,
) => boot(configOf(one, k, heavy, sched, start, seed));

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const plain = (x: unknown): boolean =>
  x === null ||
  typeof x === "string" ||
  Number.isInteger(x) ||
  (typeof x === "object" && Object.values(x as object).every(plain));

const fact = (one: Source, label: string) => one.facts.find(([name]) => name === label)?.[1];

test("the side pick starts at the base, a word reads base 2, and the rule view the square", () => {
  expect([sides(value({})), sides(value({ base: 3 })), sides(value({ view: "word", base: 3 }))]).toEqual([
    [
      [0, "2"],
      [3, "3"],
      [5, "5"],
    ],
    [
      [0, "3"],
      [5, "5"],
    ],
    [
      [0, "2"],
      [3, "3"],
      [5, "5"],
    ],
  ]);
  expect([
    sideOf(value({})),
    sideOf(value({ side: 3 })),
    sideOf(value({ base: 3 })),
    sideOf(value({ view: "word", base: 3 })),
    sideOf(value({ view: "rule", base: 3 })),
  ]).toEqual([2, 3, 3, 2, 2]);
});

test("the restricted draws need the binary square, so the modes follow the view and the rule view is always restricted", () => {
  const names = (over: Raw) => modes(value(over)).map(([mode]) => mode);
  expect([
    names({}),
    names({ side: 3 }),
    names({ base: 3 }),
    names({ view: "hex" }),
    names({ view: "word" }),
    names({ view: "mosaic" }),
    names({ view: "mosaic", mosaic: "relay" }),
    names({ view: "slice" }),
    names({ view: "rule" }),
  ]).toEqual([
    ["random", "restricted", "bruijn"],
    ["random", "bruijn"],
    ["random", "bruijn"],
    ["random", "bruijn"],
    ["random", "restricted"],
    ["random", "restricted"],
    ["random"],
    ["random", "restricted"],
    ["random", "restricted", "bruijn"],
  ]);
  expect([binary(value({})), binary(value({ view: "hex" })), restricted(value({ view: "rule" }))]).toEqual([
    true,
    false,
    true,
  ]);
  expect([
    modeOf(value({ view: "rule" })),
    modeOf(value({ mode: "restricted" })),
    modeOf(value({ mode: "restricted", base: 3 })),
    modeOf(value({ mode: "bruijn", view: "mosaic" })),
  ]).toEqual(["restricted", "restricted", "random", "random"]);
});

test("a cut reads p over q inside the square and falls back to two thirds, a typed word reads its codes, a rule its code and width", () => {
  expect([cut("2/3"), cut("1"), cut(" 3 / 4 "), cut("5/2"), cut("0/3"), cut("x"), cut("1/0")]).toEqual([
    [2, 3],
    [1, 1],
    [3, 4],
    [2, 3],
    [2, 3],
    [2, 3],
    [2, 3],
  ]);
  expect([spelt("7 9 9"), spelt("3,16,0,7"), spelt(""), spelt("x")]).toEqual([
    [7, 9, 9],
    [3, 7],
    [7, 9],
    [7, 9],
  ]);
  expect([
    ruleCode(math, value({ game: "repeat" })),
    ruleCode(math, value({ game: "pair" })),
    ruleCode(math, value({ game: "typed", rule: "50115" })),
    ruleCode(math, value({ game: "typed", rule: games(math)[7].code })),
    ruleCode(math, value({ game: "typed", rule: "nope" })),
    ruleCode(math, value({ game: "typed", rule: "99999999999999999999" })),
  ]).toEqual([
    { width: 2, code: 31710n },
    { width: 3, code: 11529205150454382585n },
    { width: 2, code: 50115n },
    { width: 3, code: 11529205150454382585n },
    { width: 2, code: 31710n },
    { width: 2, code: 31710n },
  ]);
});

test("the digits of a design are its filled cells at the side, corner c = x + side y, read off the door", () => {
  expect(digits(math, 7, 2, 2)).toEqual([
    { x: 0, y: 0, c: 0 },
    { x: 1, y: 0, c: 1 },
    { x: 0, y: 1, c: 2 },
  ]);
  expect(digits(math, 7, 3, 2).map((one) => one.c)).toEqual([0, 1, 2, 3, 5, 6, 7, 8]);
  expect(digits(math, 2, 3, 2)).toEqual([
    { x: 1, y: 0, c: 1 },
    { x: 1, y: 2, c: 7 },
  ]);
  expect(digits(math, 0, 2, 2)).toEqual([]);
});

test("the gasket opens as the design: three digits at base 2, dimension log2 3, one state, a cap of six levels", () => {
  const one = source({});
  expect([one.kind, one.base, one.letters.length, one.auto.states, one.auto.tags, one.cap, one.rho]).toEqual([
    "design",
    2,
    3,
    1,
    1,
    6,
    3,
  ]);
  expect(one.dimension).toBeCloseTo(Math.log2(3), 12);
  expect([one.title, fact(one, "design"), fact(one, "digits")]).toEqual([
    "carpet 7, side 2",
    "carpet 7",
    "3 of 4 at side 2",
  ]);
  expect(source({ side: 3 })).toMatchObject({ base: 3, rho: 8, cap: 6 });
  expect(source({ side: 3 }).dimension).toBeCloseTo(Math.log(8) / Math.log(3), 12);
  expect(source({ side: 5 })).toMatchObject({ base: 5, cap: 4 });
  expect(source({ base: 3, code: "495" })).toMatchObject({ base: 3, rho: 8, title: "carpet 495, side 3" });
  expect(source({ code: "0" }).error).toContain("no digit");
  expect(study(units, value({}))).toBe(one);
});

test("a design under a window rule is the product automaton, and the rule view is the square under it", () => {
  const one = source({ mode: "restricted", game: "repeat" });
  expect([one.auto.states, one.auto.tags, one.auto.moves[0].length, one.rho]).toEqual([4, 1, 3, 2]);
  expect(one.dimension).toBeCloseTo(1, 12);
  expect(one.title).toBe("carpet 7, side 2 under no repeat");
  const square = source({ view: "rule", game: "repeat" });
  expect([square.kind, square.letters.length, square.auto.states, square.rho, square.base]).toEqual([
    "rule",
    4,
    5,
    3,
    2,
  ]);
  expect(square.dimension).toBeCloseTo(Math.log2(3), 12);
  expect([fact(square, "windows"), fact(square, "kappa"), fact(square, "dial cells by level")]).toEqual([
    "12 of 16",
    "0.207519, the bits a digit spends on memory",
    "4, 12, 36, 108, 324, 972",
  ]);
  expect(source({ view: "rule", game: "typed", rule: "0" }).error).toContain("no window");
});

test("the named games read the census through the door: their window counts, Perron roots and dimensions", () => {
  const read = games(math)
    .filter((one) => one.key !== "typed")
    .map((one) => {
      const made = source({ view: "rule", game: one.key });
      return [one.name, fact(made, "windows"), Number(made.rho.toFixed(9)), Number(made.dimension.toFixed(6))];
    });
  expect(read).toEqual([
    ["No repeat", "12 of 16", 3, 1.584963],
    ["Not one place anticlockwise", "12 of 16", 3, 1.584963],
    ["Not the opposite corner", "12 of 16", 3, 1.584963],
    ["Only a neighbour", "8 of 16", 2, 1],
    ["Not a neighbour", "8 of 16", 2, 1],
    ["The shift rule", "8 of 16", 2, 1],
    ["Only anticlockwise", "4 of 16", 1, 0],
    ["No neighbour after a repeat", "56 of 64", 3.414213562, 1.771553],
  ]);
  expect(fact(source({ view: "rule", game: "pair" }), "kappa")).toBe("0.164232, the bits a digit spends on memory");
});

test("the Thue-Morse word over the gasket and the diagonal has period 16, sizes 3 and 2, and the scale dimension of the door", () => {
  const one = source({ view: "word" });
  expect([one.kind, one.base, one.auto.states, one.auto.tags, one.letters.map((l) => l.c), one.cap]).toEqual([
    "word",
    2,
    16,
    16,
    [0, 1, 2, 3],
    6,
  ]);
  expect(fact(one, "word")).toBe("7 9 9 7 9 7 7 9 9 7 7 9 7 9 9 7");
  expect(fact(one, "letter sizes")).toBe("3, 2, 2, 3, 2, 3, 3, 2, 2, 3, 3, 2, 3, 2, 2, 3");
  expect(one.dimension).toBeCloseTo(Math.log2(6) / 2, 12);
  expect(one.auto.moves[0].map((move) => [move.letter, move.next])).toEqual([
    [0, 1],
    [1, 1],
    [2, 1],
  ]);
  expect(one.auto.moves[1].map((move) => [move.letter, move.next])).toEqual([
    [0, 2],
    [3, 2],
  ]);
  expect(source({ view: "word", word: "periodic" })).toMatchObject({ auto: { states: 2 } });
  expect(source({ view: "word", word: "typed", letters: "7 9 9" })).toMatchObject({
    auto: { states: 3 },
    title: "Typed 7 9 9, side 2",
  });
  expect(source({ view: "word", side: 3 })).toMatchObject({ base: 3, cap: 5 });
});

test("the note's mosaic is a game on two slots with the golden matrix, and the fills count by the door", () => {
  const one = source({ view: "mosaic" });
  expect([one.kind, one.base, one.auto.states, one.auto.tags, one.letters.map((l) => l.c), one.cap]).toEqual([
    "mosaic",
    2,
    2,
    2,
    [0, 1, 2, 3],
    6,
  ]);
  expect(one.rho).toBeCloseTo((1 + Math.sqrt(5)) ** 2 / 4, 12);
  expect(one.dimension).toBeCloseTo(1.388484, 6);
  expect([fact(one, "matrix"), fact(one, "cells by level")]).toEqual([
    "[2, 1] [1, 1]",
    "A: 3, 8, 21, 55, 144, 377; B: 2, 5, 13, 34, 89, 233",
  ]);
  expect(one.auto.moves.map((moves) => moves.map((move) => [one.letters[move.letter].c, move.next]))).toEqual([
    [
      [0, 0],
      [2, 0],
      [3, 1],
    ],
    [
      [1, 0],
      [0, 1],
    ],
  ]);
  expect(MOSAICS.map((preset) => source({ view: "mosaic", mosaic: preset.key }).cap)).toEqual([6, 6, 6, 5]);
});

test("the gasket cut at two thirds runs on the heights 1/3 and 2/3 with root sqrt 2, and a dead start height moves to a live one", () => {
  const one = source({ view: "slice" });
  expect([one.kind, one.base, one.auto.states, one.auto.start, one.line, one.letters.map((l) => l.c)]).toEqual([
    "slice",
    2,
    2,
    1,
    2 / 3,
    [0, 1, 2],
  ]);
  expect(one.rho).toBeCloseTo(Math.SQRT2, 12);
  expect(one.dimension).toBeCloseTo(0.5, 12);
  expect([fact(one, "heights"), fact(one, "matrix"), fact(one, "cells on the line by level"), one.error]).toEqual([
    "1/3, 2/3",
    "[0, 1] [2, 0]",
    "2, 2, 4, 4, 8, 8",
    "",
  ]);
  const golden = source({ view: "slice", code: "11" });
  expect(golden.rho).toBeCloseTo((1 + Math.sqrt(5)) / 2, 12);
  expect(fact(golden, "cells on the line by level")).toBe("2, 3, 5, 8, 13, 21");
  expect(source({ view: "slice", cut: "5/2" }).title).toBe("carpet 7 cut at 2/3");
  const half = source({ view: "slice", cut: "1/2" });
  expect([half.auto.start, half.line, half.error, fact(half, "dead ends")]).toEqual([
    2,
    1,
    "the height 1/2 is a dead end, so the game starts at 1",
    "1/2",
  ]);
  expect(half.auto.moves.map((moves) => moves.map((move) => [half.letters[move.letter].c, move.next]))).toEqual([
    [[0, 0]],
    [],
    [
      [1, 2],
      [2, 2],
    ],
  ]);
});

test("the hexagonal gasket is three digits at base 2 in Z[w], untwisted, its box over the triangle", () => {
  const one = source({ view: "hex" });
  expect([one.kind, one.base, one.letters.length, one.auto.states, one.cap, one.hex!.ring, one.hex!.norm]).toEqual([
    "hex",
    2,
    3,
    1,
    6,
    "Eisenstein",
    4,
  ]);
  expect(one.dimension).toBeCloseTo(Math.log2(3), 12);
  expect(one.hex!.beta).toEqual([2, 0]);
  expect(one.hex!.digits[2][0]).toBeCloseTo(0.5, 12);
  expect(one.hex!.digits[2][1]).toBeCloseTo(Math.sqrt(3) / 2, 12);
  expect(one.hex!.box[0]).toBeLessThan(0);
  expect(one.hex!.box[2]).toBeGreaterThan(1);
  expect(one.hex!.box[2]).toBeLessThan(1.05);
  expect(one.hex!.box[3]).toBeGreaterThan(Math.sqrt(3) / 2);
  expect([fact(one, "radix design"), fact(one, "digits")]).toEqual([
    "Gasket, Z[w], base 2, code 7",
    "3, norm 4, no twist",
  ]);
  expect(source({ view: "hex", hex: "koch" })).toMatchObject({
    letters: [{ c: 0 }, { c: 1 }, { c: 2 }, { c: 3 }],
    cap: 6,
  });
  expect(fact(source({ view: "hex", hex: "koch" }), "digits")).toBe("4, norm 9, 2 turned");
  expect(source({ view: "hex", hex: "flowsnake" }).cap).toBe(6);
});

test("uniform draws give the uniform law on the design's cells, weighted draws the product law, both summing to one", () => {
  const gasket = source({});
  const flat = law(gasket, 2, false);
  expect([flat.cells, flat.tags, flat.total, flat.lost]).toEqual([16, 1, 9, false]);
  for (const at of flat.support) expect(flat.probs[at]).toBeCloseTo(1 / 9, 12);
  const heavy = law(gasket, 2, true);
  const p = odds(3, true);
  expect(Array.from(p)).toEqual([1 / 6, 2 / 6, 3 / 6]);
  expect(heavy.probs[Number(math.game.word_cell([2, 1], 2, 2))]).toBeCloseTo(p[2] * p[1], 12);
  expect(heavy.probs.reduce((sum, v) => sum + v, 0)).toBeCloseTo(1, 12);
  expect(Array.from(flat.support)).toEqual(Array.from(heavy.support));
});

test("a restricted or graph game reads the stationary law on cells, and a reducible rule keeps both classes", () => {
  const one = law(source({ view: "rule", game: "repeat" }), 2, false);
  expect(one.total).toBe(12);
  for (const at of one.support) expect(one.probs[at]).toBeCloseTo(1 / 12, 10);
  const mosaic = source({ view: "mosaic" });
  const two = law(mosaic, 2, false);
  expect([two.cells, two.tags, two.total]).toEqual([16, 2, 8 + 5]);
  expect(two.probs.reduce((sum, v) => sum + v, 0)).toBeCloseTo(1, 12);
  const pi = stationary(
    mosaic.auto,
    mosaic.auto.moves.map((moves) => odds(moves.length, false)),
  );
  expect(pi[0]).toBeCloseTo(0.6, 12);
  expect(pi[1]).toBeCloseTo(0.4, 12);
  const far = law(source({ view: "rule", game: "far" }), 2, false);
  expect(far.total).toBe(8);
  const word = law(source({ view: "word", word: "periodic" }), 2, false);
  expect([word.tags, word.total]).toEqual([2, 12]);
  for (const at of word.support) expect(word.probs[at]).toBeCloseTo(1 / 12, 10);
  const cut = law(source({ view: "slice", cut: "1/2" }), 1, false);
  expect(cut.lost).toBe(false);
});

test("the game is the shift register: its cell is the door's cell of its point, from the start on, at every base", () => {
  for (const [over, k] of [
    [{}, 3],
    [{ base: 3, code: "495" }, 2],
    [{ side: 3 }, 2],
    [{ view: "slice", code: "11" }, 3],
  ] as [Raw, number][]) {
    const one = source(over);
    let state = game(one, k, [0.3, 0.6], 3);
    expect(cell(state)).toBe(Number(math.game.cell(spot(state), one.base, k)));
    for (let i = 0; i < 2000; i++) {
      state = step(state, DRAW);
      expect(cell(state)).toBe(Number(math.game.cell(spot(state), one.base, k)));
    }
    expect(state.n).toBe(2000);
  }
  const gasket = game(source({}), 2, [0.3, 0.6]);
  expect(cell(gasket)).toBe(9);
  const moved = step(gasket, taking(1));
  expect(spot(moved)[0]).toBeCloseTo(0.65, 12);
  expect(spot(moved)[1]).toBeCloseTo(0.3, 12);
  expect(cell(moved)).toBe(6);
});

test("a de Bruijn lap paints every level-k cell exactly once from the kth draw on, and the histogram sits at distance zero at the lap's end", () => {
  const gasket = source({});
  const sched = schedule(math, gasket, 2)!;
  expect(Array.from(sched)).toEqual([0, 0, 1, 0, 2, 1, 1, 2, 2]);
  const lawed = law(gasket, 2, false);
  const met = meter(lawed, sched.length);
  let state = game(gasket, 2, [0.9, 0.1], 1, sched);
  for (let n = 1; n <= 2 * sched.length + 2; n++) {
    state = step(state, DRAW);
    if (n >= 2) hit(met, pair(lawed, cell(state), 0));
    if (n === sched.length + 1) expect([met.painted, met.laps, met.perfect, distance(met)]).toEqual([9, 1, true, 0]);
    if (n === 2 * sched.length + 1) expect([met.laps, distance(met)]).toEqual([2, 0]);
  }
  expect([met.painted, met.laps, met.perfect, met.outside, met.counted]).toEqual([9, 2, true, 0, 19]);
  expect(distance(met)).toBeCloseTo(8 / 171, 12);
  expect(schedule(math, source({ view: "mosaic" }), 2)).toBeNull();
  expect(schedule(math, source({ view: "hex" }), 3)!.length).toBe(27);
});

test("a random game paints inside the law, its distance falls, and a replay of the logged moves lands on the same point and histogram", () => {
  const one = source({ view: "mosaic" });
  const lawed = law(one, 3, false);
  const met = meter(lawed, 0);
  const tape = feed(5);
  const start = origin(one, tape.rand);
  const first = game(one, 3, start, tape.seed);
  let state = first;
  const log = new Uint8Array(4000);
  for (let n = 1; n <= 4000; n++) {
    state = step(state, DRAW);
    log[n - 1] = state.moved;
    if (n >= 3) hit(met, pair(lawed, cell(state), one.auto.tag[state.node]));
  }
  expect([met.outside, met.painted, met.counted]).toEqual([0, lawed.total, 3998]);
  expect(distance(met)).toBeLessThan(0.1);
  const again = meter(lawed, 0);
  const back = replay(first, again, one.auto.tag, log, 4000);
  expect([back.n, spot(back), cell(back), back.node]).toEqual([state.n, spot(state), cell(state), state.node]);
  expect(Array.from(again.hist)).toEqual(Array.from(met.hist));
  expect(csv(met).split("\n")[0]).toBe("cell,tag,count,law");
  expect(csv(met).split("\n").length).toBe(lawed.total + 2);
});

const GAME: Act[] = [DRAW, DRAW, taking(2), DRAW, taking(0), DRAW, DRAW, taking(1)];

const replayed = (acts: Act[], seed = 42, over: Raw = { view: "hex" }) =>
  acts.reduce(step, game(source(over), 3, [0.25, 0.5], seed, null, true));

test("the same seed and acts replay to the same state, byte for byte", () => {
  for (const over of [{ view: "hex" }, { view: "mosaic" }, {}] as Raw[]) {
    expect(JSON.stringify(replayed(GAME, 42, over))).toBe(JSON.stringify(replayed(GAME, 42, over)));
    expect(JSON.stringify(replayed(GAME, 42, over))).not.toBe(JSON.stringify(replayed(GAME, 43, over)));
  }
});

test("step leaves its input untouched and its output is plain and survives a JSON round trip", () => {
  for (const over of [{ view: "hex" }, { view: "slice" }, {}] as Raw[]) {
    let state = game(source(over), 3, [0.25, 0.5], 42, null, true);
    for (const act of GAME) {
      const before = JSON.stringify(state);
      const next = step(freeze(state), act);
      expect(JSON.stringify(state)).toBe(before);
      expect(JSON.parse(JSON.stringify(next))).toEqual(next);
      expect(plain(next)).toBe(true);
      state = next;
    }
  }
});

test("a scripted de Bruijn game takes every move the schedule names and paints each level-2 cell once a lap", () => {
  const gasket = source({});
  const sched = schedule(math, gasket, 2)!;
  let drawn: State = game(gasket, 2, [0.5, 0.5], 1, sched);
  let taken: State = game(gasket, 2, [0.5, 0.5], 1, sched);
  const cells: number[] = [];
  for (const j of [...sched, ...sched]) {
    drawn = step(drawn, DRAW);
    taken = step(taken, taking(j));
    cells.push(cell(taken));
  }
  expect(JSON.stringify(drawn)).toBe(JSON.stringify(taken));
  expect(new Set(cells.slice(sched.length)).size).toBe(9);
  expect([taken.n, taken.pos, taken.seed]).toEqual([18, 0, 1]);
});

test("moves lists the draw and every move of the node, and read walks a slash path into the state", () => {
  const state = game(source({}), 2, [0.5, 0.5]);
  expect(moves(state)).toEqual([DRAW, taking(0), taking(1), taking(2)]);
  expect([read(state), read(state, "k"), read(state, "letters/1/0"), read(state, "nope/x")]).toEqual([
    state,
    2,
    1,
    undefined,
  ]);
});

test("the meter counts painted cells, points outside the law and a cell painted twice in a lap", () => {
  const lawed = law(source({}), 1, false);
  const met = meter(lawed, 3);
  hit(met, 0);
  hit(met, 3);
  expect([met.painted, met.outside, met.counted]).toEqual([1, 1, 2]);
  expect(distance(met)).toBeCloseTo(2 / 3, 12);
  hit(met, 0);
  expect([met.laps, met.perfect]).toEqual([1, false]);
  expect(distance(meter(lawed, 0))).toBe(1);
});

test("a stuck game has no move: an empty design or a rule with no window", () => {
  const empty = game(source({ code: "0" }), 1, [0.5, 0.5]);
  const stuck = step(empty, DRAW);
  expect([stuck.stuck, stuck.n, moves(empty), step(stuck, DRAW)]).toEqual([1, 0, [], stuck]);
  expect(step(game(source({}), 1, [0.5, 0.5]), taking(5)).stuck).toBe(1);
  expect(law(source({ code: "0" }), 1, false)).toMatchObject({ total: 0, lost: true });
});

test("the cloud fit letterboxes a box into the square with y up, and a tap maps back", () => {
  const unit = fit([0, 0, 1, 1], 100);
  expect(toCloud(unit, 0, 0)).toEqual([0, 100]);
  expect(toCloud(unit, 1, 1)).toEqual([100, 0]);
  expect(fromCloud(unit, 25, 75)).toEqual([0.25, 0.25]);
  const wide = fit([-1, 0, 3, 2], 100);
  expect(toCloud(wide, -1, 0)).toEqual([0, 75]);
  expect(toCloud(wide, 3, 2)).toEqual([100, 25]);
  expect(lay(800, 600, 10)).toEqual({ x: 110, y: 10, size: 580 });
  expect(lay(390, 844, 12)).toEqual({ x: 12, y: 239, size: 366 });
});

test("a start lies in the square, on the cut's line for a slice, and in the box for a hex base", () => {
  const rand = rng(2);
  const [x, y] = origin(source({}), rand);
  expect(x >= 0 && x < 1 && y >= 0 && y < 1).toBe(true);
  const [sx, sy] = origin(source({ view: "slice" }), rand);
  expect(sx + sy).toBeCloseTo(2 / 3, 12);
  const hex = source({ view: "hex" });
  const [hx, hy] = origin(hex, rand);
  const [x0, y0, x1, y1] = hex.hex!.box;
  expect(hx >= x0 && hx <= x1 && hy >= y0 && hy <= y1).toBe(true);
});

test("the level cap keeps states times cells within the budget", () => {
  expect([
    cap(source({})),
    cap(source({ base: 3, code: "495" })),
    cap(source({ side: 5 })),
    cap(source({ view: "word" })),
  ]).toEqual([6, 6, 4, 6]);
});
