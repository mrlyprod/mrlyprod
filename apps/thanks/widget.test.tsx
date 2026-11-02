import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show made by robots, the top fifth, nothing to do, robots built, the day and the world in words", () => {
  const told = {
    robots: 18,
    hand: 6,
    share: 750,
    top: 412,
    idle: 9,
    building: 3,
    built: 14,
    alive: 9,
    makers: 125,
    paid: 96,
    day: 60,
    length: 120,
    credit: "runner" as const,
    bots: 32,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["made by robots", "75.0%"],
    ["top fifth", "41.2%"],
    ["nothing to do", "9"],
    ["robots built", "14"],
    ["day", "60 of 120"],
    [
      "world",
      "thanks to the runner, 32 bots, 9 robots now, 3 building; makers got 12.5% of robot thanks, 96 thanks paid for robots",
    ],
  ]);
  expect([
    rows({ ...told, over: "done" })[4][0],
    String(rows({ ...told, credit: "all" })[5][1]).startsWith("thanks to everyone,"),
    rows(null),
  ]).toEqual(["ran", true, []]);
});
