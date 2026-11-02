import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { DAYS, deal, read, run } from "./engine.ts";

const KNOBS = { bots: 48, work: "free" as const, teams: 1 };

test("play finds craft, deals what the page deals, and steps a day as the engine does", async () => {
  const start = await open("craft", 3, { work: "free" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("craft"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays sixty days to their end, as the engine runs them", async () => {
  const done = play(await open("craft", 3, { work: "free" }), "random", DAYS * 2);
  expect([done.steps, done.session.state]).toEqual([DAYS, run(deal(KNOBS, { seed: 3 }), DAYS)]);
});
