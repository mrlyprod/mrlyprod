import { expect, test } from "bun:test";
import type { Act, World as Kept } from "../../kernel/index.ts";
import { app, kernel } from "../../kernel/index.ts";
import type { World } from "./mcp.ts";
import { handle } from "./mcp.ts";

const lamp = app<{ on: boolean }>({
  id: "lamp",
  user: "test",
  title: "Lamp",
  line: "On or off.",
  boot: () => ({ on: false }),
  verbs: {
    on: { line: "Light it.", run: () => ({ on: true }) },
    off: { line: "Dark again.", run: () => ({ on: false }) },
  },
  moves: (s) => (s.on ? ["off"] : ["on"]),
});

function served(): { world: World; said: object[] } {
  const k = kernel([lamp]);
  let kept: Kept = k.boot({ name: "lamp", apps: ["lamp"] });
  const world: World = {
    list: () => k.list(kept),
    read: (path, shape) => k.read(kept, path, shape),
    call: (act: Act) => {
      const done = k.call(kept, act);
      kept = done.world;
      return done.envelope;
    },
    at: () => 0,
  };
  return { world, said: [] };
}

const OPTS = { name: "mrlynet", version: "0", who: { id: "me", kind: "agent" as const }, note: "" };

const ask = (method: string, params?: Record<string, unknown>) => ({ jsonrpc: "2.0" as const, id: 1, method, params });

test("each legal verb is a tool with its args schema; a call that shifts the moves says the list changed", () => {
  const { world, said } = served();
  const say = (one: object) => said.push(one);
  const names = () =>
    (handle(world, OPTS, ask("tools/list"), say) as { tools: { name: string }[] }).tools.map((t) => t.name);
  expect(names()).toEqual(["test_lamp_on", "nav_open", "read"]);
  const out = handle(world, OPTS, ask("tools/call", { name: "test_lamp_on", arguments: {} }), say) as {
    structuredContent: { state: unknown };
    isError: boolean;
  };
  expect([out.structuredContent.state, out.isError]).toEqual([{ on: true }, false]);
  expect(said).toEqual([{ jsonrpc: "2.0", method: "notifications/tools/list_changed" }]);
  expect(names()).toEqual(["test_lamp_off", "nav_open", "read"]);
  const stale = handle(world, OPTS, ask("tools/call", { name: "test_lamp_on", arguments: {} }), say) as {
    isError: boolean;
  };
  expect(stale.isError).toBe(true);
  const read = handle(world, OPTS, ask("resources/read", { uri: "mrlynet://world/lamp/on" }), say) as {
    contents: { text: string }[];
  };
  expect(read.contents[0].text).toBe("true");
});
