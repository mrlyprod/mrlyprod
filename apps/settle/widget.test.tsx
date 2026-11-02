import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show housed, fed, green and reach in thousandths, the day and the world in words", () => {
  const told = {
    ashore: 40,
    housed: 975,
    fed: 1000,
    green: 612,
    reach: 80,
    tents: 1,
    towns: 3,
    fenced: 12,
    tick: 30,
    length: 100,
    bots: 64,
    own: "private" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["housed", "0.975"],
    ["fed", "1.000"],
    ["green", "0.612"],
    ["reach", "0.080"],
    ["day", "30 of 100"],
    ["world", "40 of 64 bots ashore, 3 towns, 1 tent, 12 fenced idle"],
  ]);
  expect([rows({ ...told, over: "done" })[4][0], rows(null)]).toEqual(["ran", []]);
});
