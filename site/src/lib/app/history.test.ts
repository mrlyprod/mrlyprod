import { expect, test } from "bun:test";
import { fresh, MERGE, patch, record, redo, state, undo } from "./history.ts";

test("changes closer than the merge window are one step, and a new change clears the redo side", () => {
  let stack = record(fresh(), { a: 1 }, 0);
  stack = record(stack, { a: 2 }, MERGE - 1);
  stack = record(stack, { a: 3 }, 3 * MERGE);
  expect(stack.past).toEqual([{ a: 1 }, { a: 3 }]);
  const back = undo(stack, { a: 4 })!;
  expect([back.to, back.stack.future]).toEqual([{ a: 3 }, [{ a: 4 }]]);
  expect(record(back.stack, { a: 3 }, 9 * MERGE).future).toEqual([]);
});

test("undo then redo comes back to where it was, and an empty side answers null", () => {
  const stack = record(fresh(), { a: 1 }, 0);
  const back = undo(stack, { a: 2 })!;
  const ahead = redo(back.stack, back.to)!;
  expect([ahead.to, ahead.stack.past, undo(fresh(), {}), redo(fresh(), {})]).toEqual([
    { a: 2 },
    [{ a: 1 }],
    null,
    null,
  ]);
});

test("history.state reads as itself when well formed and as absent when null or malformed", () => {
  const was = Object.getOwnPropertyDescriptor(globalThis, "history");
  const read = (one: unknown) => {
    Object.defineProperty(globalThis, "history", { configurable: true, value: { state: one } });
    return state();
  };
  try {
    expect([read({ depth: 2, y: 40, up: true }), read(null), read({ depth: "3", y: "far" }), read(7)]).toEqual([
      { depth: 2, y: 40, up: true },
      {},
      {},
      {},
    ]);
  } finally {
    if (was) Object.defineProperty(globalThis, "history", was);
    else Reflect.deleteProperty(globalThis, "history");
  }
});

test("a patch holds every key whose value differs, gone keys as undefined", () => {
  expect(patch({ a: 1, b: 2, c: 3 }, { a: 1, b: 5, d: 4 })).toEqual({ b: 5, c: undefined, d: 4 });
});
