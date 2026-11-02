import { expect, test } from "bun:test";
import * as snake from "../../../../apps/snake/engine.ts";
import { SPEC as SNAKE } from "../../../../apps/snake/scene.ts";
import { rng } from "../../../sdk/rand.ts";
import { tidy } from "../../../sdk/spec.ts";
import type { Session } from "./play.ts";
import { answer, open, reducers } from "./play.ts";

async function session(start: Session, lines: string[]) {
  let at = start;
  const outs = [];
  for (const line of lines) {
    const next = await answer(at, line);
    at = next.session;
    outs.push(next.out);
  }
  return outs;
}

test("discovery finds the reducer apps and no other", async () => {
  const found = await reducers();
  for (const id of ["chaos", "life", "matrix", "race", "sleep", "snake"]) expect(found).toContain(id);
  for (const id of ["apollonian", "ulam", "designs", "shared"]) expect(found).not.toContain(id);
});

test("a snake session reads as the engine called directly", async () => {
  let state = snake.deal(tidy(SNAKE, { size: "10" }) as snake.Config, { rand: rng(7) });
  const start = await open("snake", 7, { size: "10" });
  expect(start.engine.read(start.state)).toEqual(snake.read(state));
  const turn = snake.moves(state).find((act) => act.verb === "turn")!;
  const outs = await session(start, [
    '{"moves":true}',
    '{"step":{"verb":"tick"}}',
    JSON.stringify({ step: turn }),
    '{"step":{"verb":"tick"}}',
  ]);
  expect(outs[0]).toEqual({ moves: snake.moves(state) });
  for (const [i, act] of [snake.TICK, turn, snake.TICK].entries()) {
    state = snake.step(state, act);
    expect(outs[i + 1]).toEqual({ read: snake.read(state) });
  }
});

test("a malformed line answers an error and the session goes on", async () => {
  const start = await open("snake", 1);
  const [bad, odd, read] = await session(start, ["{oops", '{"step":{"verb":"fly"}}', '{"read":true}']);
  expect(bad.error).toContain("not JSON");
  expect(odd.error).toContain("no legal move");
  expect(read).toEqual({ read: start.engine.read(start.state) });
});
