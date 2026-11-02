import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { deal, HOURS, read, run } from "./engine.ts";

const KNOBS = { choose: "near" as const, bots: 24 };

test("play finds wish, deals what the page deals, and steps an hour as the engine does", async () => {
  const start = await open("wish", 3, { choose: "near" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("wish"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays two hundred and forty hours to their end, as the engine runs them", async () => {
  const done = play(await open("wish", 3, { choose: "near" }), "random", HOURS * 2);
  expect([done.steps, done.session.state]).toEqual([HOURS, run(deal(KNOBS, { seed: 3 }), HOURS)]);
});
