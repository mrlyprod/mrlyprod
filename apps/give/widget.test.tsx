import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show fed, idle and top in thousandths, top beside its fair share, the hungry bots and the clock", () => {
  const told = {
    fed: 812,
    idle: 45,
    top: 312,
    fair: 250,
    made: 120,
    used: 98,
    spilled: 6,
    posted: 121,
    hungry: 1,
    tick: 40,
    length: 168,
    bots: 8,
    economy: "gift" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["fed", "0.812"],
    ["idle", "0.045"],
    ["top", "0.312, fair 0.250"],
    ["hungry", "1 of 8"],
    ["tick", "40 of 168"],
    ["things", "120 made, 98 used, 6 spilled"],
  ]);
  expect([rows({ ...told, over: "done" })[4][0], rows(null)]).toEqual(["ran", []]);
});
