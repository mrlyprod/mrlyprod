import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show traded, failed, lost in thousandths, stuck, the day and the world in words", () => {
  const told = {
    traded: 212,
    failed: 9,
    stuck: 4,
    lost: 23,
    pace: 13500,
    spread: 341,
    booths: 10,
    day: 60,
    length: 120,
    currencies: 5,
    fee: 3,
    traders: 48,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["traded", "212"],
    ["failed", "9"],
    ["lost", "0.023"],
    ["stuck", "4"],
    ["day", "60 of 120"],
    ["world", "5 coins, 10 changers, fee 3%, 48 traders; days per trade 13.500, top fifth holds 0.341"],
  ]);
  expect([
    rows({ ...told, over: "done" })[4][0],
    rows({ ...told, currencies: 1, booths: 0, traded: 0 })[5][1],
    String(rows({ ...told, currencies: 2, booths: 1 })[5][1]).startsWith("2 coins, 1 changer,"),
    rows(null),
  ]).toEqual(["ran", "1 coin, 0 changers, fee 3%, 48 traders; days per trade -, top fifth holds 0.341", true, []]);
});
