import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { DAYS, deal, read, run } from "./engine.ts";

const KNOBS = { bots: 64, own: "shared" as const, city: 16, borders: "open" as const };

test("play finds settle, deals what the page deals, and steps a day as the engine does", async () => {
  const start = await open("settle", 3, { own: "shared" });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("settle"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays a hundred days to their end, as the engine runs them", async () => {
  const done = play(await open("settle", 3, { own: "shared" }), "random", DAYS * 2);
  expect([done.steps, done.session.state]).toEqual([DAYS, run(deal(KNOBS, { seed: 3 }), DAYS)]);
});
