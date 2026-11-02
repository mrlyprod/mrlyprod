import { expect, test } from "bun:test";
import { act } from "react";
import type { Act } from "../kernel/index.ts";
import { app, kernel } from "../kernel/index.ts";
import { mount } from "../test/dom.ts";
import type { Port } from "./world.ts";
import { plug, useCall, useWorld } from "./world.ts";

const box = (id: string) =>
  app<{ n: number }>({
    id,
    title: id,
    line: id,
    boot: () => ({ n: 0 }),
    verbs: { up: { line: "Up.", run: (state) => ({ n: state.n + 1 }) } },
  });

function port(seen: Act[] = []): Port {
  const k = kernel([box("a"), box("b")]);
  let world = k.boot({ name: "w", apps: ["a", "b"] });
  const heard = new Set<() => void>();
  return {
    subscribe: (one) => {
      heard.add(one);
      return () => {
        heard.delete(one);
      };
    },
    read: (path, shape) => k.read(world, path, shape),
    list: () => k.list(world),
    call: (one) => {
      seen.push(one);
      const done = k.call(world, one);
      world = done.world;
      for (const fn of heard) fn();
      return { ok: true, note: null, envelope: done.envelope };
    },
    who: () => ({ id: "test-1", kind: "human" }),
    actor: () => true,
    at: () => world.at,
  };
}

test("a reader of one path re-renders when that read changes, never on a call elsewhere", async () => {
  const one = port();
  plug(one);
  let renders = 0;
  function Reader() {
    renders++;
    return <span>{JSON.stringify(useWorld("a", ["n"]))}</span>;
  }
  await mount(async (draw) => {
    await draw(<Reader />);
    const drawn = renders;
    await act(async () => void one.call({ verb: "b.up" }));
    const elsewhere = renders;
    await act(async () => void one.call({ verb: "a.up" }));
    expect([elsewhere - drawn, renders - elsewhere]).toEqual([0, 1]);
  });
  plug(null);
});

test("useCall stamps the signed-in user and the wall clock on the act", async () => {
  const seen: Act[] = [];
  plug(port(seen));
  let call: ReturnType<typeof useCall> = () => ({ ok: false, note: null, envelope: null });
  function Caller() {
    call = useCall();
    return null;
  }
  const before = Date.now();
  await mount(async (draw) => {
    await draw(<Caller />);
    call("a.up");
  });
  plug(null);
  expect([seen[0]!.who, seen[0]!.at! >= before]).toEqual([{ id: "test-1", kind: "human" }, true]);
});
