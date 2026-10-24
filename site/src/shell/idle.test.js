import { expect, test } from "bun:test";
import { wakes } from "./idle.js";

test("the saver wakes on any input after its first second, but not on a pointer drift of 8 px or less", () => {
  const from = { x: 10, y: 10 };
  expect([
    wakes({ type: "keydown" }, 500, from),
    wakes({ type: "keydown" }, 1500, from),
    wakes({ type: "pointermove", clientX: 16, clientY: 15 }, 1500, from),
    wakes({ type: "pointermove", clientX: 19, clientY: 10 }, 1500, from),
    wakes({ type: "pointermove", clientX: 90, clientY: 90 }, 1500, null),
  ]).toEqual([false, true, false, true, false]);
});
