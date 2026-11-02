import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { deal, read, run, WEEK } from "./engine.ts";

const KNOBS = { bots: 8, economy: "pool" as const, plenty: 40, generous: 70, purse: 3 };

test("play finds give, deals what the page deals, and steps a tick as the engine does", async () => {
  const start = await open("give", 3, { economy: "pool" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("give"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays a week to its end, as the engine runs it", async () => {
  const done = play(await open("give", 3, { economy: "pool" }), "first", WEEK * 2);
  expect([done.steps, done.session.state]).toEqual([WEEK, run(deal(KNOBS, { seed: 3 }), WEEK)]);
});
