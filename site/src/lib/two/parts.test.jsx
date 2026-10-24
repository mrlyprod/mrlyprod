import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { mount } from "../../../test/dom.js";
import { Rolls, rolls } from "../app/hooks.js";
import { roll } from "../designs/engine.js";
import { rng } from "../rand.js";
import { DIM } from "./engine.js";
import { useReroll } from "./parts.jsx";

function Probe({ value, onChange }) {
  useReroll(value, onChange, math);
  return null;
}

test("a reroll draws a code of the base from the new seed, and a seed that moves any other way draws nothing", async () => {
  const told = rolls();
  const seen = [];
  await mount(async (draw) => {
    const view = (seed) => (
      <Rolls.Provider value={told}>
        <Probe value={{ seed, base: 3 }} onChange={(patch) => seen.push(patch)} />
      </Rolls.Provider>
    );
    await draw(view(1));
    await draw(view(2));
    expect(seen).toEqual([]);
    told.tell(5);
  });
  expect(seen).toEqual([{ code: roll(math, DIM, 3, rng(5)) }]);
});
