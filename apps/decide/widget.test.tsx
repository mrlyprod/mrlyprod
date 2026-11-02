import { expect, test } from "bun:test";
import { rows, ticks } from "./widget.tsx";

test("the readouts show granted in thousandths, the median wait in ticks, reach out of the bots, and the clock", () => {
  const told = {
    posted: 31,
    granted: 742,
    wait: 2500,
    reach: 4,
    dropped: 2,
    tick: 40,
    length: 168,
    bots: 5,
    decide: "vote" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["granted", "0.742"],
    ["wait", "2.5 ticks"],
    ["reach", "4 of 5"],
    ["tick", "40 of 168"],
    ["wishes", "31 posted, 2 taken back"],
  ]);
  expect([
    rows({ ...told, granted: 50, over: "done" })[0][1],
    rows({ ...told, over: "done" })[3][0],
    ticks(3000),
  ]).toEqual(["0.050", "ran", "3 ticks"]);
});
