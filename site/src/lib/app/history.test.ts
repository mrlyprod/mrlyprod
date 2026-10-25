import { expect, test } from "bun:test";
import { fresh, MERGE, patch, record, redo, undo } from "./history.ts";

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

test("a patch holds every key whose value differs, gone keys as undefined", () => {
  expect(patch({ a: 1, b: 2, c: 3 }, { a: 1, b: 5, d: 4 })).toEqual({ b: 5, c: undefined, d: 4 });
});
