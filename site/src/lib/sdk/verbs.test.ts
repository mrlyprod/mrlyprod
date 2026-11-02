import { expect, test } from "bun:test";
import { createElement } from "react";
import type { Row } from "../app/spec.ts";
import type { Model } from "./verbs.ts";
import { call, list, read, schema } from "./verbs.ts";

const of = (row: Row) => schema(row, { [row.key]: row.def });

test("a slider gives its bounds and its step", () => {
  expect(of({ key: "size", label: "Size", kind: "slider", def: 16, min: 8, max: 32, step: 2 })).toEqual({
    title: "Size",
    default: 16,
    type: "integer",
    minimum: 8,
    maximum: 32,
    multipleOf: 2,
  });
});

test("a number reads live bounds, and a step off its grid stays a plain number", () => {
  expect(
    of({ key: "x", kind: "number", def: 0.5, min: () => 0.25, max: (value) => Number(value.x) * 4, step: 0.5 }),
  ).toEqual({ title: "x", default: 0.5, type: "number", minimum: 0.25, maximum: 2 });
});

test("a toggle is 0 or 1", () => {
  expect(of({ key: "wrap", kind: "toggle", def: 1 })).toEqual({
    title: "wrap",
    default: 1,
    type: "integer",
    enum: [0, 1],
  });
});

test("a pick enumerates its options, labelled or not", () => {
  expect(of({ key: "tint", kind: "pick", def: "", options: ["", ["red", "Red"], 3] }).enum).toEqual(["", "red", 3]);
});

test("a segment enumerates its options", () => {
  expect(of({ key: "base", kind: "segment", def: 2, options: [2, 3] }).enum).toEqual([2, 3]);
});

test("a text is a string", () => {
  expect(of({ key: "word", kind: "text", def: "mrly" })).toEqual({ title: "word", default: "mrly", type: "string" });
});

test("a list is a string of its items joined by its sep", () => {
  const dotted = new RegExp(of({ key: "l", kind: "list", def: "a", options: ["a", "b+"] }).pattern!);
  const tight = new RegExp(of({ key: "l", kind: "list", def: "a", sep: "", options: ["l", "d"] }).pattern!);
  expect([dotted.test("a.b+"), dotted.test("a.bb"), dotted.test("a.x"), tight.test("ldl"), tight.test("lx")]).toEqual([
    true,
    false,
    false,
    true,
    false,
  ]);
});

const SNAKE: Model = {
  id: "snake",
  spec: [{ key: "size", kind: "slider", def: 16, min: 8, max: 32, step: 2 }],
  keys: [
    { key: "ArrowLeft", label: "Turn", act: "turn" },
    { key: ["ArrowRight", "ArrowUp"], label: "Turn", act: "turn" },
    { key: "g", label: () => "Grow", act: "grow" },
    { key: "h", label: "Hidden", act: "hide", when: () => false },
    { key: " ", label: "Pause", act: "pause" },
  ],
  value: { seed: 0, size: 16 },
  status: [
    ["score", 3],
    ["best", createElement("b", null, "1", 2)],
  ],
};

test("list names set, reroll, pause and one verb per live key act, a key arg when the act has several", () => {
  const verbs = list(SNAKE);
  expect(verbs.map((one) => [one.name, one.title, one.args.properties?.key?.enum ?? []])).toEqual([
    ["mrlyprod.snake.set", "Set", []],
    ["mrlyprod.snake.reroll", "Reroll", []],
    ["mrlyprod.snake.pause", "Pause", []],
    ["mrlyprod.snake.turn", "Turn", ["ArrowLeft", "ArrowRight", "ArrowUp"]],
    ["mrlyprod.snake.grow", "Grow", []],
  ]);
  expect(Object.keys(verbs[0].args.properties!)).toEqual(["seed", "size"]);
});

test("read tidies the value and writes each readout as text", () => {
  expect(read({ ...SNAKE, value: { seed: 4, size: 99 } })).toEqual({
    value: { seed: 4, size: 32 },
    status: { score: "3", best: "12" },
  });
});

test("call sends set to change, any other verb to run with its key, and refuses what list does not name", () => {
  const said: unknown[] = [];
  const ports = { change: (patch: object) => said.push(patch), run: (act: string) => said.push(act) };
  call(SNAKE, ports, "mrlyprod.snake.set", { size: 24 });
  call(SNAKE, ports, "mrlyprod.snake.pause");
  call(SNAKE, ports, "mrlyprod.snake.turn", { key: "ArrowUp" });
  call(SNAKE, ports, "mrlyprod.snake.grow");
  expect(said).toEqual([{ size: 24 }, "pause", "turn@ArrowUp", "grow@g"]);
  expect(() => call(SNAKE, ports, "mrlyprod.snake.hide")).toThrow("no verb");
  expect(() => call(SNAKE, ports, "mrlyprod.snake.set", { speed: 2 })).toThrow("takes no speed");
  expect(() => call(SNAKE, ports, "mrlyprod.snake.turn", { key: "x" })).toThrow("takes key");
});
