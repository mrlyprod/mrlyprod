import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show granted, waiting, oldest wait, nobody can grant, let go, the hour and the world in words", () => {
  const told = {
    granted: 120,
    waiting: 9,
    oldest: 41,
    nobody: 2,
    gone: 7,
    wait: 8350,
    hour: 100,
    length: 240,
    choose: "near" as const,
    bots: 24,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["granted", "120"],
    ["waiting", "9"],
    ["oldest wait", "41 hours"],
    ["nobody can grant", "2"],
    ["let go", "7"],
    ["hour", "100 of 240"],
    ["world", "nearest first, 24 bots; a granted wish waited 8.3 hours"],
  ]);
  expect([
    rows({ ...told, over: "done" })[5][0],
    rows({ ...told, oldest: 1 })[2][1],
    rows({ ...told, granted: 0 })[6][1],
    rows(null),
  ]).toEqual(["ran", "1 hour", "nearest first, 24 bots; a granted wish waited -", []]);
});
