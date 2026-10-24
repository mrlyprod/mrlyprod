import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { values } from "../../lib/app/page.jsx";
import { CARDS } from "./cards.js";
import { SPEC } from "./scene.js";
import { rolled, turned, Widget, walked } from "./widget.jsx";

test("a seed rolls a card and a step inside its range, the same every time", () => {
  for (const seed of [1, 2, 3, 99, 4294967295]) {
    const { card, step } = rolled(seed);
    expect(rolled(seed)).toEqual({ card, step });
    expect(step >= CARDS[card].min && step <= CARDS[card].max).toBe(true);
  }
  expect(new Set([1, 2, 3, 4, 5, 6, 7, 8].map((seed) => rolled(seed).card)).size).toBeGreaterThan(2);
});

test("the cards turn round and the steps wrap inside the card", () => {
  expect(turned(12, 1)).toEqual({ card: 0, step: CARDS[0].first });
  expect(turned(0, -1)).toEqual({ card: 12, step: CARDS[12].first });
  expect(walked(1, CARDS[1].max, 1)).toEqual({ step: CARDS[1].min });
  expect(walked(1, CARDS[1].min, -1)).toEqual({ step: CARDS[1].max });
});

test("before the unit loads the widget renders nothing on the server", () => {
  const value = values({ seed: 0, card: 0, step: 4 }, SPEC);
  expect(renderToStaticMarkup(<Widget value={value} onChange={() => {}} onReady={() => {}} />)).toBe("");
});
