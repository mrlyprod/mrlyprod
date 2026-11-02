import { afterAll, expect, test } from "bun:test";
import { rmSync } from "node:fs";
import { join } from "node:path";
import type { Who } from "../../kernel/index.ts";
import { app } from "../../kernel/index.ts";
import { guarded, report, simulate } from "./sim.ts";
import { DATA_DIR, faults, open } from "./store.ts";

const ROOT = join(DATA_DIR, "test-sim", String(process.pid));

const jar = app<{ beans: number; log: string[] }>({
  id: "jar",
  title: "Jar",
  line: "Beans in, beans out.",
  boot: () => ({ beans: 0, log: [] }),
  verbs: {
    put: {
      line: "Put some in.",
      args: { type: "object", properties: { n: { type: "integer", minimum: 1, maximum: 3 } }, required: ["n"] },
      run: (s, a, ctx) => ({ beans: s.beans + Number(a.n), log: [...s.log.slice(-9), ctx.who.id] }),
    },
    take: {
      line: "Take one out.",
      run: (s, _a, ctx) => (s.beans ? { ...s, beans: s.beans - 1 } : ctx.refuse("the jar is empty")),
    },
  },
});

afterAll(() => rmSync(join(DATA_DIR, "test-sim"), { recursive: true, force: true }));

test("a simulation of bots is a pure function of its seed: replayed from its journal, the same in one run or two, its report carrying the settled rules and each player's kind", () => {
  const config = { name: "jar", seed: 9, apps: ["jar"], rules: { lid: { pick: ["open", "shut"] }, beans: 3 } };
  const plan = { bots: 4, rounds: 30, beat: 1000 };
  const once = simulate([jar], join(ROOT, "once"), config, plan);
  simulate([jar], join(ROOT, "twice"), config, { ...plan, rounds: 12 });
  const twice = simulate([jar], join(ROOT, "twice"), config, { ...plan, rounds: 18 });
  expect([once.tick, once.at, once.replayed, once.bots.length]).toEqual([120, 30000, true, 4]);
  expect([once.rules.beans, ["open", "shut"].includes(String(once.rules.lid)), once.bots[0]!.kind]).toEqual([
    3,
    true,
    "bot",
  ]);
  expect(twice.digest).toBe(once.digest);
});

test("a malformed act by a player lands in faults.jsonl and never in the journal: the digest stays, and the report counts the faults by note", () => {
  const dir = join(ROOT, "faults");
  const config = { name: "jar", seed: 9, apps: ["jar"] };
  const before = simulate([jar], dir, config, { bots: 2, rounds: 5, beat: 1000 });
  const table = guarded(open([jar], dir));
  const who: Who = { id: "bot-1", kind: "bot" };
  table.call({ verb: "jar.put", args: { n: 99 }, who, at: 6000 });
  table.call({ verb: "jar.nope", args: {}, who, at: 6000 });
  table.call({ verb: "jar.put", args: { n: 99 }, who, at: 6000 });
  const after = report([jar], dir);
  expect([before.faults, after.digest === before.digest, after.tick === before.tick, after.replayed]).toEqual([
    0,
    true,
    true,
    true,
  ]);
  expect([
    after.faults,
    Object.values(after.faulty),
    faults(dir).map((one) => [one.fault, one.verb, one.tick]),
  ]).toEqual([
    3,
    [2, 1],
    [
      ["bad-act", "jar.put", before.tick],
      ["unknown-verb", "jar.nope", before.tick],
      ["bad-act", "jar.put", before.tick],
    ],
  ]);
});
