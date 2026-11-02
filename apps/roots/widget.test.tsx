import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show top, mobility (a dash before any child has ended its life), made and from nothing in thousandths, the year and the world in words", () => {
  const told = {
    top: 712,
    mobility: 480,
    pairs: 25,
    made: 455,
    rented: 545,
    shared: 0,
    nothing: 0,
    commons: 0,
    generations: 3,
    richest: 1977,
    poorest: 40,
    year: 90,
    length: 200,
    families: 16,
    inherit: "heir" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["top", "0.712"],
    ["mobility", "0.480"],
    ["made", "0.455"],
    ["from nothing", "0.000"],
    ["year", "90 of 200"],
    ["world", "16 families, generation 3; rent 0.545, shared 0.000, commons 0.000; richest 1977, poorest 40"],
  ]);
  expect([rows({ ...told, over: "done" })[4][0], rows({ ...told, pairs: 0 })[1], rows(null)]).toEqual([
    "ran",
    ["mobility", "-"],
    [],
  ]);
});
