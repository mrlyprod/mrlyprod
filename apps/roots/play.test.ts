import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { deal, read, run, YEARS } from "./engine.ts";

const KNOBS = { families: 16, inherit: "commons" as const };

test("play finds roots, deals what the page deals, and steps a year as the engine does", async () => {
  const start = await open("roots", 3, { inherit: "commons" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("roots"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays two hundred years to their end, as the engine runs them", async () => {
  const done = play(await open("roots", 3, { inherit: "commons" }), "random", YEARS * 2);
  expect([done.steps, done.session.state]).toEqual([YEARS, run(deal(KNOBS, { seed: 3 }), YEARS)]);
});
