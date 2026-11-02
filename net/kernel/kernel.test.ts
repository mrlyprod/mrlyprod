import { expect, test } from "bun:test";
import { goose } from "../sim/goose.ts";
import { SCHEMAS } from "../spec/index.ts";
import { check } from "./check.ts";
import { replay } from "./journal.ts";
import type { Json } from "./json.ts";
import { canon, digest } from "./json.ts";
import type { Line, World } from "./kernel.ts";
import { app, Fault, kernel } from "./kernel.ts";
import { below } from "./rand.ts";

/* FIXTURES */

type Count = { n: number; max: number; rolls: number[] };

const count = app<Count>({
  id: "count",
  user: "test",
  title: "Count",
  line: "A number that climbs to its max.",
  boot: ({ config }) => ({ n: 0, max: Number(config.max ?? 3), rolls: [] }),
  verbs: {
    add: {
      line: "Add a step.",
      args: (s) => ({
        type: "object",
        properties: { by: { type: "integer", minimum: 1, maximum: s.max - s.n } },
        required: ["by"],
        additionalProperties: false,
      }),
      run: (s, a) => ({ ...s, n: s.n + Number(a.by) }),
    },
    roll: { line: "Roll a die.", run: (s, _a, ctx) => ({ ...s, rolls: [...s.rolls, 1 + below(ctx.draw, 6)] }) },
    reset: { line: "Back to zero.", run: (s) => ({ ...s, n: 0 }) },
    odd: {
      line: "Refuses even numbers.",
      args: { type: "object", properties: { k: { type: "integer" } }, required: ["k"], additionalProperties: false },
      run: (s, a, ctx) => (Number(a.k) % 2 ? s : ctx.refuse(`${a.k} is even`)),
    },
  },
  moves: (s) => (s.n >= s.max ? ["reset", "roll"] : ["add", "roll", "odd"]),
});

const echo = app<{ said: string[] }>({
  id: "echo",
  title: "Echo",
  line: "Says it back, then wishes it kept.",
  boot: () => ({ said: [] }),
  verbs: {
    say: {
      line: "Say a word.",
      args: { type: "object", properties: { word: { type: "string", maxLength: 12 } }, required: ["word"] },
      run: (s, a, ctx) => {
        ctx.wish({ kind: "file", data: { name: `${a.word}.txt` }, reply: { verb: "echo.kept" } });
        return { said: [...s.said, String(a.word)] };
      },
    },
  },
});

type Bank = { balances: Record<string, number>; moves: string[] };

const bank = app<Bank>({
  id: "wallet",
  title: "Bank",
  line: "Holds the money.",
  boot: ({ actors }) => ({ balances: Object.fromEntries((actors ?? []).map((id) => [id, 10])), moves: [] }),
  verbs: {},
  money: {
    move: (s, { from, to, amount, verb }) => {
      const held = s.balances[from] ?? 0;
      if (held < amount) return `${from} holds ${held}, less than ${amount}`;
      return {
        balances: { ...s.balances, [from]: held - amount, [to]: (s.balances[to] ?? 0) + amount },
        moves: [...s.moves, `${from}>${to}:${amount}:${verb}`],
      };
    },
  },
});

type Shop = { sold: Record<string, number> };

const shop = app<Shop>({
  id: "shop",
  title: "Shop",
  line: "Sells apples for 3, and a lemon it never hands over.",
  boot: () => ({ sold: {} }),
  verbs: {
    apple: {
      line: "Buy an apple.",
      pay: () => ({ to: "shop", amount: 3 }),
      run: (s, _a, ctx) => ({ sold: { ...s.sold, [ctx.who.id]: (s.sold[ctx.who.id] ?? 0) + 1 } }),
    },
    lemon: {
      line: "Pay for a lemon, then be refused.",
      pay: () => ({ to: "shop", amount: 1 }),
      run: (_s, _a, ctx) => ctx.refuse("no lemons today"),
    },
    refund: {
      line: "The shop pays you back 1.",
      pay: (_s, _a, ctx) => ({ from: "shop", to: ctx.who.id, amount: 1 }),
      run: (s) => s,
    },
    tip: {
      line: "Move money between two others.",
      pay: () => ({ from: "u2", to: "shop", amount: 1 }),
      run: (s) => s,
    },
    look: { line: "Just look.", pay: () => null, run: (s) => s },
  },
});

const K = kernel([count, echo]);

const PAID = kernel([bank, shop]);

const MARKET = { name: "market", seed: 2, apps: ["wallet", "shop"], actors: ["u1", "u2"] };

const CONFIG = { name: "test", seed: 7, apps: [{ id: "count", config: { max: 5 } }, "echo"] };

const ME = { id: "u1", handle: "@me", kind: "human" as const };

function fault(run: () => unknown): string {
  try {
    run();
  } catch (thrown) {
    if (thrown instanceof Fault) return thrown.code;
    throw thrown;
  }
  return "none";
}

/* BOOT */

test("boot builds one JSON world from a config, with no IO and no clock", () => {
  const world = K.boot(CONFIG);
  expect(world.apps).toEqual({ count: { n: 0, max: 5, rolls: [] }, echo: { said: [] } });
  expect([world.tick, world.at, world.route.app]).toEqual([0, 0, "count"]);
  expect(canon(K.boot(CONFIG))).toBe(canon(world));
});

test("boot refuses a config it cannot run, and names the nearest app", () => {
  expect(fault(() => K.boot({ name: "x", apps: ["cuont"] }))).toBe("bad-config");
  expect(() => K.boot({ name: "x", apps: ["cuont"] })).toThrow("nearest: count");
  expect(fault(() => K.boot({ name: "x", spec: "1.0", apps: ["count"] }))).toBe("spec-mismatch");
  expect(fault(() => K.boot({ name: "x", apps: [] }))).toBe("bad-config");
});

/* LIST */

test("list shows every legal verb now with its schema, and shifts as state does", () => {
  let world = K.boot(CONFIG);
  const verbs = () => K.list(world).moves.map((move) => move.verb);
  expect(verbs()).toEqual(["test.count.add", "test.count.roll", "test.count.odd", "echo.say", "nav.open"]);
  expect(K.list(world).moves[0].args.properties?.by.maximum).toBe(5);
  world = K.call(world, { verb: "test.count.add", args: { by: 5 }, who: ME }).world;
  expect(verbs()).toEqual(["test.count.roll", "test.count.reset", "echo.say", "nav.open"]);
  expect(K.list(world).apps.map((one) => one.id)).toEqual(["count", "echo"]);
});

/* CALL */

test("call takes one act and returns a fresh envelope; the old world is untouched", () => {
  const world = K.boot(CONFIG);
  const before = canon(world);
  const { world: next, envelope, line } = K.call(world, { verb: "test.count.add", args: { by: 2 }, who: ME, at: 10 });
  expect(canon(world)).toBe(before);
  expect(envelope.state).toEqual({ n: 2, max: 5, rolls: [] });
  expect([envelope.tick, envelope.last.ok, next.at]).toEqual([1, true, 10]);
  expect(line).toEqual({ tick: 1, at: 10, who: ME, verb: "test.count.add", args: { by: 2 }, n: 1 });
});

test("a broken act is a fault with a plain note; nothing changes", () => {
  const world = K.boot(CONFIG);
  expect(fault(() => K.call(world, { verb: "test.count.add", args: { by: 9 }, who: ME }))).toBe("bad-act");
  expect(() => K.call(world, { verb: "test.count.add", args: { by: 9 }, who: ME })).toThrow(
    "by must be between 1 and 5",
  );
  expect(() => K.call(world, { verb: "test.count.ad", who: ME })).toThrow("nearest: test.count.add");
  expect(fault(() => K.call(world, { verb: "test.count.add", args: { by: 1 }, who: { id: "", kind: "human" } }))).toBe(
    "bad-act",
  );
  expect(fault(() => K.call(world, { verb: "test.count.add", args: { by: 1 }, who: ME, at: -1 }))).toBe("bad-act");
});

test("a refusal is an outcome: journaled, the state kept, the note names the way", () => {
  let world = K.boot(CONFIG);
  const odd = K.call(world, { verb: "test.count.odd", args: { k: 2 }, who: ME });
  expect([odd.envelope.last, odd.world.tick, odd.world.apps.count]).toEqual([
    { ok: false, note: "2 is even" },
    1,
    world.apps.count,
  ]);
  world = K.call(world, { verb: "test.count.add", args: { by: 5 }, who: ME }).world;
  const late = K.call(world, { verb: "test.count.odd", args: { k: 1 }, who: ME });
  expect(late.envelope.last.note).toBe("test.count.odd is not legal now; try test.count.roll");
});

test("an app reaches outside only through wishes, carried in the envelope", () => {
  const { envelope } = K.call(K.boot(CONFIG), { verb: "echo.say", args: { word: "hi" }, who: ME });
  expect(envelope.wishes).toEqual([{ kind: "file", data: { name: "hi.txt" }, reply: { verb: "echo.kept" } }]);
  expect(envelope.route.app).toBe("echo");
});

test("a watcher may read but not act", () => {
  const world = K.boot({ ...CONFIG, actors: ["u1"] });
  expect(fault(() => K.call(world, { verb: "echo.say", args: { word: "x" }, who: { id: "u2", kind: "bot" } }))).toBe(
    "refused",
  );
  expect(K.call(world, { verb: "echo.say", args: { word: "x" }, who: ME }).envelope.last.ok).toBe(true);
});

/* PAY */

test("a paid act settles through the wallet before it runs: money moves, the verb runs, the envelope says who paid whom", () => {
  const world = PAID.boot(MARKET);
  const { world: next, envelope, line } = PAID.call(world, { verb: "shop.apple", who: ME, at: 5 });
  expect(next.apps).toEqual({
    wallet: { balances: { u1: 7, u2: 10, shop: 3 }, moves: ["u1>shop:3:shop.apple"] },
    shop: { sold: { u1: 1 } },
  });
  expect([envelope.paid, envelope.last.ok, "paid" in line]).toEqual([
    { from: "u1", to: "shop", amount: 3 },
    true,
    false,
  ]);
  expect(check(SCHEMAS.envelope, envelope)).toBeNull();
  const back = PAID.call(next, { verb: "shop.refund", who: ME, at: 6 });
  expect([back.envelope.paid, (back.world.apps.wallet as Bank).balances]).toEqual([
    { from: "shop", to: "u1", amount: 1 },
    { u1: 8, u2: 10, shop: 2 },
  ]);
  expect(PAID.call(world, { verb: "shop.look", who: ME }).envelope.paid).toBeUndefined();
});

test("a payment the wallet refuses, one the caller is on neither side of, or one in a world without a wallet is a refusal with its note, and nothing changes", () => {
  const U2 = { id: "u2", kind: "human" as const };
  const apples = (world: World, n: number): World =>
    n ? apples(PAID.call(world, { verb: "shop.apple", who: U2 }).world, n - 1) : world;
  const world = PAID.boot(MARKET);
  const broke = apples(world, 3);
  const alone = kernel([shop]);
  const refused = [
    PAID.call(broke, { verb: "shop.apple", who: U2 }),
    PAID.call(world, { verb: "shop.tip", who: ME }),
    alone.call(alone.boot({ ...MARKET, apps: ["shop"] }), { verb: "shop.apple", who: ME }),
  ];
  expect(refused.map((done) => done.envelope.last)).toEqual([
    { ok: false, note: "u2 holds 1, less than 3" },
    { ok: false, note: "u1 is on neither side of this payment" },
    { ok: false, note: "this world has no wallet, so nothing can be paid" },
  ]);
  expect([refused[0]!.world.tick, digest(refused[0]!.world.apps), refused[0]!.envelope.paid]).toEqual([
    4,
    digest(broke.apps),
    undefined,
  ]);
});

test("when the verb refuses after the payment settled, the payment rolls back: both or neither", () => {
  const world = PAID.boot(MARKET);
  const done = PAID.call(world, { verb: "shop.lemon", who: ME });
  expect([done.envelope.last, done.world.tick, done.envelope.paid, digest(done.world.apps)]).toEqual([
    { ok: false, note: "no lemons today" },
    1,
    undefined,
    digest(world.apps),
  ]);
});

test("a journal of paid acts replays to the same digest across 200 seeded worlds", () => {
  for (let seed = 0; seed < 200; seed++) {
    const config = { ...MARKET, seed };
    let world = PAID.boot(config);
    const lines: Line[] = [];
    let s = seed + 1;
    const draw = () => {
      s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
      return s;
    };
    for (let i = 0; i < 30; i++) {
      const act = goose(PAID.list(world).moves, draw)!;
      const who = { id: draw() & 1 ? "u1" : "u2", kind: "human" as const };
      const done = PAID.call(world, { ...act, who, at: i * 125 });
      world = done.world;
      lines.push(done.line);
    }
    expect(digest(replay(PAID, config, lines))).toBe(digest(world));
    const held = Object.values((world.apps.wallet as Bank).balances);
    expect([held.reduce((a, b) => a + b, 0), held.every((one) => one >= 0)]).toEqual([20, true]);
  }
});

/* RULES */

test("a rule is pinned or picked: a pick is drawn from the world's seed at boot, the same for the same seed", () => {
  const rules = { payments: { pick: ["on", "off"] }, gifts: "on", users: 8 };
  const drawn = Array.from({ length: 40 }, (_, seed) => K.boot({ ...CONFIG, seed, rules }).rules);
  expect(drawn.every((one) => one.gifts === "on" && one.users === 8)).toBe(true);
  expect(new Set(drawn.map((one) => one.payments))).toEqual(new Set(["on", "off"]));
  expect(K.boot({ ...CONFIG, seed: 3, rules }).rules).toEqual(drawn[3]);
});

/* LOOK */

test("an app may read another app's state at call time, never write it", () => {
  const peek = app<{ seen: Json }>({
    id: "peek",
    title: "Peek",
    line: "Looks at the count.",
    boot: () => ({ seen: null }),
    verbs: { look: { line: "Look.", run: (_s, _a, ctx) => ({ seen: ctx.read("count/n") }) } },
  });
  const K2 = kernel([count, peek]);
  let world = K2.boot({ name: "look", apps: [{ id: "count", config: { max: 5 } }, "peek"] });
  world = K2.call(world, { verb: "test.count.add", args: { by: 4 }, who: ME }).world;
  world = K2.call(world, { verb: "peek.look", who: ME }).world;
  expect([world.apps.peek, world.apps.count]).toEqual([{ seen: 4 }, { n: 4, max: 5, rolls: [] }]);
});

/* READ */

test("read walks a slash path and keeps only the keys a shape names", () => {
  let world = K.boot(CONFIG);
  world = K.call(world, { verb: "test.count.add", args: { by: 3 }, who: ME }).world;
  expect(K.read(world, "count/n").state).toBe(3);
  expect(K.read(world, "count", ["n"]).state).toEqual({ n: 3 });
  expect(K.read(world, "", { count: { max: true } }).state).toEqual({ count: { max: 5 } });
  expect(K.read(world, "nope/x").state).toBeNull();
});

/* JOURNAL */

function run(seed: number, acts: number): { world: World; lines: Line[] } {
  const config = { ...CONFIG, seed };
  let world = K.boot(config);
  const lines: Line[] = [];
  let s = seed + 1;
  const draw = () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s;
  };
  for (let i = 0; i < acts; i++) {
    const act = goose(K.list(world).moves, draw);
    if (!act) break;
    const done = K.call(world, { ...act, who: ME, at: i * 125 });
    world = done.world;
    lines.push(done.line);
  }
  return { world, lines };
}

test("replaying the journal from the seed rebuilds the world to the bit, across 1000 seeded worlds", () => {
  for (let seed = 0; seed < 1000; seed++) {
    const { world, lines } = run(seed, 24);
    expect(digest(replay(K, { ...CONFIG, seed }, lines))).toBe(digest(world));
  }
});

test("seek is a replay that stops early, and a thaw with another act forks", () => {
  const { lines } = run(3, 20);
  const mid = replay(K, { ...CONFIG, seed: 3 }, lines, 10);
  expect(mid.tick).toBe(10);
  const thawed = JSON.parse(canon(mid)) as World;
  const fork = K.call(thawed, { verb: "test.count.roll", who: ME, at: thawed.at }).world;
  expect(fork.tick).toBe(11);
  expect(digest(fork)).not.toBe(digest(replay(K, { ...CONFIG, seed: 3 }, lines, 11)));
});
