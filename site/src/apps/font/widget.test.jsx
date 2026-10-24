import { expect, test } from "bun:test";
import { added, status } from "./widget.jsx";

const PLAN = { text: "hi", rows: 7, cols: 11, frames: 100, fps: 25 };

const FACTS = { pens: 108, strokes: 328, flagged: "D L" };

test("the status reads the pens, their strokes and the flagged ones on the wall, and the board, frames and time of the text", () => {
  expect(status({ view: "pens" }, PLAN, FACTS)).toEqual([
    ["pens", 108],
    ["strokes", 328],
    ["flagged", "D L"],
  ]);
  expect(status({ view: "text", speed: 2 }, PLAN, FACTS)).toEqual([
    ["board", "11x7"],
    ["frames", 100],
    ["time", "2.0 s"],
  ]);
  expect(status({ view: "pens" }, PLAN, null)).toEqual([]);
  expect(status({ view: "text", speed: 1 }, null, FACTS)).toEqual([]);
});

test("a picked glyph goes on the end of the text and shows the text; from the wall a blank text starts with it", () => {
  expect(added({ view: "text", text: "" }, PLAN, "A")).toEqual({ view: "text", text: "hiA" });
  expect(added({ view: "text", text: "ok" }, PLAN, "!")).toEqual({ view: "text", text: "ok!" });
  expect(added({ view: "pens", text: "" }, PLAN, "A")).toEqual({ view: "text", text: "A" });
  expect(added({ view: "pens", text: "ok" }, PLAN, "A")).toEqual({ view: "text", text: "okA" });
});
