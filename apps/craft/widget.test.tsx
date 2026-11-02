import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show fed, loved and idle in thousandths, shipped, the day and the town in words", () => {
  const told = {
    fed: 943,
    loved: 365,
    idle: 57,
    busy: 302,
    basics: 1000,
    shipped: 34,
    best: 461,
    dropped: 0,
    working: 30,
    resting: 2,
    office: 16,
    hungry: 1,
    tick: 30,
    length: 60,
    bots: 48,
    teams: 1,
    work: "earn" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["fed", "0.943"],
    ["loved", "0.365"],
    ["idle", "0.057"],
    ["shipped", "34"],
    ["day", "30 of 60"],
    [
      "town",
      "30 at a craft, 16 at desks, 2 on the plaza, 1 hungry bot; busy 0.302, basics 1.000, best 0.461, dropped 0.000",
    ],
  ]);
  expect([rows({ ...told, over: "done" })[4][0], rows(null)]).toEqual(["ran", []]);
});
