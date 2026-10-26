import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { mount } from "../../../test/dom.ts";
import { Rolls, rolls } from "../app/hooks.ts";
import type { Patch, Seeded } from "../app/spec.ts";
import { roll } from "../designs/engine.ts";
import { rng } from "../rand.ts";
import { DIM } from "./engine.ts";
import { useReroll } from "./parts.tsx";

type ProbeProps = { value: Seeded; onChange: (patch: Patch) => void };

function Probe({ value, onChange }: ProbeProps) {
  useReroll(value, onChange, math);
  return null;
}

test("a reroll draws a code of the base from the new seed, and a seed that moves any other way draws nothing", async () => {
  const told = rolls();
  const seen: Patch[] = [];
  await mount(async (draw) => {
    const view = (seed: number) => (
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
