import { expect, test } from "bun:test";
import { answer, open, play, reducers } from "../../net/hosts/web/scripts/play.ts";
import { DAYS, deal, read, run } from "./engine.ts";

const KNOBS = { currencies: 3, fee: 3, traders: 48 };

test("play finds coins, deals what the page deals, and steps a day as the engine does", async () => {
  const start = await open("coins", 3, { currencies: 3 });
  const state = deal(KNOBS, { seed: 3 });
  const { out } = await answer(start, '{"step":{"verb":"tick","args":{}}}');
  expect([(await reducers()).includes("coins"), start.app.bots, start.state, out]).toEqual([
    true,
    ["first", "random"],
    state,
    { read: read(run(state, 1)) },
  ]);
});

test("a bot plays a hundred and twenty days to their end, as the engine runs them", async () => {
  const done = play(await open("coins", 3, { currencies: 3 }), "random", DAYS * 2);
  expect([done.steps, done.session.state]).toEqual([DAYS, run(deal(KNOBS, { seed: 3 }), DAYS)]);
});
