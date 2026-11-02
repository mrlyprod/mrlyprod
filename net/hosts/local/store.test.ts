import { afterAll, expect, test } from "bun:test";
import { existsSync, rmSync } from "node:fs";
import { join } from "node:path";
import { app, digest } from "../../kernel/index.ts";
import { boot, DATA_DIR, open } from "./store.ts";

const DIR = join(DATA_DIR, "test", String(process.pid));

const step = app<{ n: number }>({
  id: "step",
  title: "Step",
  line: "Counts acts.",
  boot: () => ({ n: 0 }),
  verbs: { up: { line: "One more.", run: (s, _a, ctx) => ({ n: s.n + 1 + (ctx.draw() % 2) }) } },
});

afterAll(() => rmSync(join(DATA_DIR, "test"), { recursive: true, force: true }));

test("a world on disk is its config and its journal: reopened, past a snapshot, it is the same world", () => {
  const store = boot([step], DIR, { name: "disk", seed: 3, apps: ["step"] });
  for (let i = 0; i < 520; i++) store.call({ verb: "step.up", who: { id: "t", kind: "bot" }, at: i });
  expect(existsSync(join(DIR, "snapshot.json"))).toBe(true);
  const again = open([step], DIR);
  expect([again.world().tick, digest(again.world())]).toEqual([520, digest(store.world())]);
});
