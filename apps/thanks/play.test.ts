import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { DAYS, deal, read, run } from "./engine.ts";

const KNOBS = { credit: "maker", bots: 32 } as const;

test("play finds thanks, deals what the page deals, and steps a day as the engine does", async () => {
  const start = await open("thanks", 3, { credit: "maker" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("thanks"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays a hundred and twenty days to their end, as the engine runs them", async () => {
  const done = play(await open("thanks", 3, { credit: "maker" }), "random", DAYS * 2);
  expect([done.steps, done.session.state]).toEqual([DAYS, run(deal(KNOBS, { seed: 3 }), DAYS)]);
});
