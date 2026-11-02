import { expect, test } from "bun:test";
import type { Act, Config } from "./engine.ts";
import { bits, boot, CLEAR, moves, painting, read, ruling, step, TICK, types } from "./engine.ts";

const SIDE = 8;

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

const board = (cells: [number, number][]) => {
  const out = new Uint8Array(SIDE * SIDE);
  for (const [x, y] of cells) out[y * SIDE + x] = 1;
  return bits(out);
};

const GLIDER: [number, number][] = [
  [1, 0],
  [2, 1],
  [0, 2],
  [1, 2],
  [2, 2],
];

const CONWAY: Config = {
  width: SIDE,
  height: SIDE,
  wrap: 1,
  birth: [3],
  survive: [2, 3],
  span: [3, 3],
  mask: "111101111",
  cells: board(GLIDER),
};

const GAME: Act[] = [TICK, TICK, painting(0, 1), TICK, painting(0, 0), TICK, TICK];

const replay = (acts: Act[], config = CONWAY) => acts.reduce(step, boot(config));

test("cells read and write as a string of zeros and ones", () => {
  expect([bits([0, 1, 1, 0]), Array.from(types("0110"))]).toEqual(["0110", [0, 1, 1, 0]]);
});

test("boot counts the population and opens the fate on the first board", () => {
  const state = boot(CONWAY);
  expect([state.gen, state.pop, state.seen.length, state.word, plain(state)]).toEqual([0, 5, 1, "", true]);
  expect(boot({ ...CONWAY, cells: board([]) }).word).toBe("dead");
});

test("a scripted glider on a wrapped board flies one cell down and right every four ticks and comes home after thirty-two", () => {
  let state = boot(CONWAY);
  for (let i = 0; i < 4; i++) state = step(state, TICK);
  expect(state.cells).toBe(board(GLIDER.map(([x, y]) => [x + 1, y + 1])));
  for (let i = 4; i < 32; i++) state = step(state, TICK);
  expect([state.gen, state.pop, state.cells]).toEqual([32, 5, CONWAY.cells]);
});

test("a blinker reads as a loop of two and a block as still", () => {
  const blinker = step(
    step(
      boot({
        ...CONWAY,
        cells: board([
          [3, 2],
          [3, 3],
          [3, 4],
        ]),
      }),
      TICK,
    ),
    TICK,
  );
  expect(blinker.word).toBe("loop 2");
  const block = step(
    boot({
      ...CONWAY,
      cells: board([
        [2, 2],
        [3, 2],
        [2, 3],
        [3, 3],
      ]),
    }),
    TICK,
  );
  expect(block.word).toBe("still");
});

test("a paint flips one cell and forgets the past, a repaint is no move, and a clear empties the board to generation 0", () => {
  const state = step(boot(CONWAY), TICK);
  const painted = step(state, painting(0, 1));
  expect([painted.cells[0], painted.pop, painted.seen.length, painted.gen]).toEqual(["1", 6, 1, 1]);
  expect(step(painted, painting(0, 1))).toBe(painted);
  const cleared = step(painted, CLEAR);
  expect([cleared.cells, cleared.gen, cleared.pop, cleared.word]).toEqual([board([]), 0, 0, "dead"]);
});

test("a rule change keeps the board and its past and runs the new law from the next tick", () => {
  const state = step(boot(CONWAY), TICK);
  const walled = step(state, ruling({ ...CONWAY, wrap: 0, birth: [1] }));
  expect([walled.cells, walled.seen, walled.wrap, walled.birth]).toEqual([state.cells, state.seen, 0, [1]]);
  expect(step(walled, TICK).pop).toBeGreaterThan(state.pop);
});

test("the same config and acts replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(GAME))).toBe(JSON.stringify(replay(GAME)));
  expect(JSON.stringify(replay(GAME))).not.toBe(JSON.stringify(replay(GAME, { ...CONWAY, wrap: 0 })));
});

test("step leaves its input untouched and its output is plain and survives a JSON round trip", () => {
  let state = boot(CONWAY);
  for (const act of [...GAME, CLEAR]) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    expect(plain(next)).toBe(true);
    state = next;
  }
});

test("moves lists the tick, the clear and a flip of every cell", () => {
  const list = moves(boot(CONWAY));
  expect(list.slice(0, 3)).toEqual([TICK, CLEAR, painting(0, 1)]);
  expect([list.length, list[2 + 1 * SIDE + 2]]).toEqual([2 + SIDE * SIDE, painting(SIDE + 2, 0)]);
});

test("read walks a slash path into the state", () => {
  const state = boot(CONWAY);
  expect([read(state), read(state, "pop"), read(state, "span/1"), read(state, "nope/x")]).toEqual([
    state,
    5,
    3,
    undefined,
  ]);
});
