import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { deal, HOURS, read, run } from "./engine.ts";

const KNOBS = { prison: "repair" as const, scarcity: 2, bots: 24 };

test("play finds fence, deals what the page deals, and steps an hour as the engine does", async () => {
  const start = await open("fence", 3, { prison: "repair" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("fence"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays two hundred and forty hours to their end, as the engine runs them", async () => {
  const done = play(await open("fence", 3, { prison: "repair" }), "random", HOURS * 2);
  expect([done.steps, done.session.state]).toEqual([HOURS, run(deal(KNOBS, { seed: 3 }), HOURS)]);
});
