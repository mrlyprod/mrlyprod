import { expect, test } from "bun:test";
import type { Act, Config } from "./engine.ts";
import { boot, FIT, moves, read, setting, staging, step, TICK, ticking } from "./engine.ts";

const CONFIG: Config = {
  design: "",
  number: "",
  level: "",
  count: 6,
  speed: 180,
  size: 0.42,
  w: 1200,
  h: 800,
  dpr: 1,
  lock: 0,
  seed: 7,
};

const KNOBS = { design: "", number: "" as const, level: "" as const, count: 6, speed: 180, size: 0.42 };

const PLAY: Act[] = [
  ticking(16),
  ticking(400),
  staging({ w: 500, h: 300, dpr: 2, lock: 0 }),
  FIT,
  ticking(1000),
  setting({ ...KNOBS, count: 3 }),
];

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const replay = (acts: Act[], config = CONFIG) => acts.reduce(step, boot(config));

const inside = (state: ReturnType<typeof boot>) =>
  state.crowd.every((one) => one.x >= 0 && one.y >= 0 && one.x + one.side <= state.w && one.y + one.side <= state.h);

test("boot gathers the crowd on whole cells inside the stage, the generator kept in the state", () => {
  const state = boot(CONFIG);
  expect([state.crowd.length, inside(state), Number.isInteger(state.rand)]).toEqual([6, true, true]);
  expect(state.crowd.every((one) => one.side % one.number ** one.level === 0)).toBe(true);
});

test("a tick runs every sleeper by speed, dpr, time and depth, and a wall hit bounces it and draws a new mark", () => {
  const state = boot({ ...CONFIG, count: 1 });
  const [one] = state.crowd;
  const [moved] = step(state, ticking(10)).crowd;
  expect([moved.x, moved.y]).toEqual([one.x + one.vx * 1.8 * one.d, one.y + one.vy * 1.8 * one.d]);
  const hit = step(state, ticking(60000));
  expect([hit.rand === state.rand, Math.sign(hit.crowd[0].vx) === Math.sign(one.vx), inside(hit)]).toEqual([
    false,
    false,
    true,
  ]);
});

test("a stage change waits for the fit, which shrinks the crowd onto the new stage", () => {
  const staged = step(boot(CONFIG), staging({ w: 300, h: 200, dpr: 1, lock: 0 }));
  expect([staged.w, inside(staged), inside(step(staged, FIT))]).toEqual([300, false, true]);
});

test("a set rerolls every mark on a new design, regathers to the count, and a new seed boots the crowd afresh", () => {
  const state = boot(CONFIG);
  const net = step(state, setting({ ...KNOBS, design: "net", count: 9 }));
  expect([net.crowd.length, net.crowd.every((one) => one.design === "net")]).toEqual([9, true]);
  expect(step(state, setting({ ...KNOBS, seed: 11 }))).toEqual(boot({ ...CONFIG, seed: 11 }));
});

test("the same config and acts replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(PLAY))).toBe(JSON.stringify(replay(PLAY)));
  expect(JSON.stringify(replay(PLAY))).not.toBe(JSON.stringify(replay(PLAY, { ...CONFIG, seed: 8 })));
});

test("step leaves its input untouched and its output survives a JSON round trip", () => {
  let state = boot(CONFIG);
  for (const act of PLAY) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    state = next;
  }
});

test("moves lists a frame tick and the fit", () => {
  expect(moves(boot(CONFIG))).toEqual([TICK, FIT]);
});

test("read walks a slash path into the state", () => {
  const state = boot(CONFIG);
  expect([read(state), read(state, "count"), read(state, "crowd/0/d"), read(state, "nope/x")]).toEqual([
    state,
    6,
    0.4,
    undefined,
  ]);
});
