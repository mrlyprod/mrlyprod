import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import * as num from "@mrlyprod/mrly/num";
import { ledger } from "../../lib/sequences/engine.js";
import { build, CARDS, pick, records, shown } from "./cards.js";

const units = { math, num, book: ledger(math) };

const card = (key) => CARDS.findIndex((one) => one.key === key);

test("every card builds a picture and lines of at least three terms at its first, lowest and highest step", () => {
  expect(CARDS).toHaveLength(13);
  CARDS.forEach((one, i) => {
    for (const step of [one.first, one.min, one.max]) {
      const made = build(units, i, step);
      expect([one.key, step, made.error, Boolean(made.art), made.lines.length > 0]).toEqual([
        one.key,
        step,
        "",
        true,
        true,
      ]);
      for (const line of made.lines)
        expect([one.key, step, line.label, line.terms.length >= 3]).toEqual([one.key, step, line.label, true]);
    }
  });
});

test("the level caps are the ones the picture budget allows", () => {
  expect(CARDS[card("carpet")].max).toBe(math.bang.level_cap(3, 2, 500000));
  expect(CARDS[card("sponge")].max).toBe(math.bang.level_cap(3, 3, 60000));
  expect(CARDS[card("gasket")].max).toBe(math.bang.level_cap(2, 3, 500000));
});

test("the odd-side law reads the polygonal numbers with their closed forms, and every ledger line links to the ledger", () => {
  const made = build(units, card("sides"), 4);
  expect(made.lines.map((line) => line.terms.join(", "))).toEqual([
    "4, 9, 16",
    "6, 15, 28",
    "8, 21, 40",
    "5, 13, 25",
    "7, 19, 37",
    "9, 25, 49",
  ]);
  expect(made.lines.map((line) => line.closed)).toEqual([
    "k^2",
    "k(2k - 1)",
    "k(3k - 2)",
    "2k^2 - 2k + 1",
    "3k^2 - 3k + 1",
    "(2k - 1)^2",
  ]);
  expect(made.lines[2].records.found[0].record.id).toBe("A000567");
  expect(made.lines[2].link).toBe("../sequences/?q=8%2C%2021%2C%2040");
  expect(made.art.rows[0].sheets.map((one) => one.number)).toEqual([3, 5, 7]);
});

test("the cards read their records: the carpet powers of 8, cuban primes marked, the gasket alike at every height, the Farey count as the totients say", () => {
  expect(build(units, card("carpet"), 3).lines[0]).toMatchObject({ terms: ["8", "64", "512"], closed: "8^level" });
  const vertices = build(units, card("vertices"), 4).lines[0];
  expect([vertices.terms, vertices.marks]).toEqual([
    ["7", "37", "91", "169"],
    [true, true, false, false],
  ]);
  expect(build(units, card("gasket"), 4)).toMatchObject({
    sub: "16 heights, all alike",
    lines: [{ terms: ["3", "9", "27", "81"] }],
  });
  expect(build(units, card("farey"), 5).sub).toBe("11 nodes lit, as the totients say, primes 2 3 5");
  expect(build(units, card("slices"), 2).lines[0].terms.slice(0, 3)).toEqual(["6", "42", "306"]);
  expect(records(["1", "2"])).toEqual({ found: [], formula: "", short: true });
});

test("a value tidies to a card and a step inside its range, and the step shows as the card says it", () => {
  expect(pick({ card: 99, step: 99 })).toEqual({ card: 12, step: 24 });
  expect(pick({ card: "x", step: "x" })).toEqual({ card: 0, step: 4 });
  expect(pick({ card: card("tile"), step: 1 })).toEqual({ card: card("tile"), step: 2 });
  expect([shown(card("sides"), 4), shown(card("carpet"), 2)]).toEqual(["side 7", "2"]);
});
