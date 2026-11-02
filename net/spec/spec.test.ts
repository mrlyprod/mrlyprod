import { expect, test } from "bun:test";
import { app, check, type Fault, kernel, schemaNote } from "../kernel/index.ts";
import { goose } from "../sim/goose.ts";
import { SCHEMAS } from "./index.ts";

const tally = app<{ n: number }>({
  id: "tally",
  user: "spec",
  title: "Tally",
  line: "Counts.",
  boot: () => ({ n: 0 }),
  verbs: {
    up: { line: "Up one.", run: (s, _a, ctx) => (s.n < 3 ? { n: s.n + 1 } : ctx.refuse("full")) },
    keep: {
      line: "Wish it kept.",
      run: (s, _a, ctx) => {
        ctx.wish({ kind: "file", data: { n: s.n }, reply: { verb: "spec.tally.up" } });
        return s;
      },
    },
  },
  beat: "up",
});

const K = kernel([tally]);

const ME = { id: "u1", kind: "human" as const };

test("every schema of the spec stays inside the subset any kernel can check", () => {
  for (const [name, schema] of Object.entries(SCHEMAS)) expect([name, schemaNote(schema)]).toEqual([name, null]);
});

test("what the kernel speaks passes the spec: config, act, line, envelope, move, manifest, fault", () => {
  const config = { name: "spec", seed: 1, apps: ["tally"] };
  expect(check(SCHEMAS.config, config)).toBeNull();
  let world = K.boot(config);
  let s = 9;
  const draw = () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s;
  };
  for (let i = 0; i < 12; i++) {
    const act = { ...goose(K.list(world).moves, draw)!, who: ME, at: i };
    expect(check(SCHEMAS.act, act)).toBeNull();
    const done = K.call(world, act);
    world = done.world;
    expect(check(SCHEMAS.line, done.line)).toBeNull();
    expect(check(SCHEMAS.envelope, done.envelope)).toBeNull();
  }
  for (const move of K.list(world).moves) expect(check(SCHEMAS.move, move)).toBeNull();
  for (const one of K.list(world).apps) expect(check(SCHEMAS.manifest, one)).toBeNull();
  expect(check(SCHEMAS.envelope, K.read(world, "tally/n"))).toBeNull();
  try {
    K.call(world, { verb: "spec.tally.down", who: ME });
  } catch (thrown) {
    const { code, note } = thrown as Fault;
    expect(check(SCHEMAS.fault, { error: { code, note } })).toBeNull();
  }
});
