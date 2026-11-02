import { expect, test } from "bun:test";
import { app, check, kernel, seedOf, stream } from "../kernel/index.ts";
import { goose } from "./goose.ts";

const zoo = app<{ seen: number }>({
  id: "zoo",
  title: "Zoo",
  line: "One verb per shape of argument.",
  boot: () => ({ seen: 0 }),
  verbs: {
    feed: {
      line: "Every shape at once.",
      args: {
        type: "object",
        properties: {
          n: { type: "integer", minimum: 2, maximum: 9, multipleOf: 2 },
          x: { type: "number", minimum: 0.5, maximum: 2, multipleOf: 0.25 },
          y: { type: "number", maximum: 1 },
          word: { type: "string", minLength: 2, maxLength: 4 },
          pick: { enum: ["a", "b", "c"] },
          on: { type: "boolean" },
          none: { type: "null" },
          nest: { type: "object", properties: { k: { const: 1 } }, required: ["k"] },
          list: { type: "array", items: { type: "integer", minimum: 0, maximum: 1 }, minItems: 1, maxItems: 2 },
        },
        required: ["n", "x", "word", "pick", "on", "none", "nest", "list"],
        additionalProperties: false,
      },
      run: (s) => ({ seen: s.seen + 1 }),
    },
    name: {
      line: "A pattern the goose cannot fill.",
      args: { type: "object", properties: { id: { type: "string", pattern: "^x" } }, required: ["id"] },
      run: (s) => s,
    },
  },
});

const K = kernel([zoo]);

test("the goose fills every verb it can from its schema, each act passing the schema, and skips the verb it cannot fill", () => {
  const world = K.boot({ name: "zoo", apps: ["zoo"] });
  const draw = stream(seedOf(1, "goose")).draw;
  const verbs = new Set<string>();
  for (let i = 0; i < 200; i++) {
    const act = goose(K.list(world).moves, draw)!;
    const move = K.list(world).moves.find((one) => one.verb === act.verb)!;
    expect(check(move.args, act.args)).toBeNull();
    verbs.add(act.verb);
  }
  expect([...verbs].sort()).toEqual(["nav.open", "zoo.feed"]);
});
