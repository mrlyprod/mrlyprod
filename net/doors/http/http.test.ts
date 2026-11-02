import { expect, test } from "bun:test";
import type { Act, Config, World as Kept } from "../../kernel/index.ts";
import { app, kernel } from "../../kernel/index.ts";
import type { World, Worlds } from "./http.ts";
import { handler } from "./http.ts";

const dial = app<{ n: number }>({
  id: "dial",
  title: "Dial",
  line: "Zero to nine.",
  boot: () => ({ n: 0 }),
  verbs: {
    turn: {
      line: "Set the dial.",
      args: { type: "object", properties: { n: { type: "integer", minimum: 0, maximum: 9 } }, required: ["n"] },
      run: (_s, a) => ({ n: Number(a.n) }),
    },
  },
});

function memory(): Worlds {
  const k = kernel([dial]);
  const kept = new Map<string, World>();
  const make = (config: Config): World => {
    let world: Kept = k.boot(config);
    return {
      list: () => k.list(world),
      read: (path, shape) => k.read(world, path, shape),
      call: (act: Act) => {
        const done = k.call(world, act);
        world = done.world;
        return done.envelope;
      },
      at: () => 0,
    };
  };
  return {
    boot: async (config) => {
      const one = make(config);
      kept.set(config.name, one);
      return { id: config.name, world: one };
    },
    open: async (id) => kept.get(id) ?? null,
  };
}

const ask = (fetch: (r: Request) => Promise<Response>, method: string, path: string, body?: unknown) =>
  fetch(
    new Request(`http://local${path}`, { method, body: body === undefined ? undefined : JSON.stringify(body) }),
  ).then(async (r) => [r.status, await r.json()] as const);

test("the four calls are routes; a broken request is a fault with its status, a refusal stays a 200", async () => {
  const fetch = handler(memory(), { id: "me", kind: "agent" });
  const [booted] = await ask(fetch, "POST", "/api/1/worlds", { name: "w", apps: ["dial"] });
  expect(booted).toBe(200);
  const [, list] = await ask(fetch, "GET", "/api/1/worlds/w/list");
  expect(list.moves.map((one: { verb: string }) => one.verb)).toEqual(["dial.turn", "nav.open"]);
  const [, called] = await ask(fetch, "POST", "/api/1/worlds/w/call", { verb: "dial.turn", args: { n: 7 } });
  expect([called.tick, called.state]).toEqual([1, { n: 7 }]);
  expect((await ask(fetch, "GET", "/api/1/worlds/w/read/dial/n"))[1].state).toBe(7);
  expect(await ask(fetch, "POST", "/api/1/worlds/w/call", { verb: "dial.turn", args: { n: 12 } })).toEqual([
    400,
    { error: { code: "bad-act", note: "dial.turn: n must be between 0 and 9" } },
  ]);
  expect((await ask(fetch, "GET", "/api/1/worlds/nope/list"))[0]).toBe(404);
  expect((await ask(fetch, "POST", "/api/1/worlds/w/call", { verb: "dial.spin" }))[1].error.code).toBe("unknown-verb");
});
