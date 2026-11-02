import { expect, test } from "bun:test";
import { act } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Schema } from "../kernel/index.ts";
import { app, kernel } from "../kernel/index.ts";
import type { FakeNode } from "../test/dom.ts";
import { all, find, mount, text } from "../test/dom.ts";
import { View } from "./view.tsx";
import type { Port } from "./world.ts";
import { plug } from "./world.ts";

const ALL: Record<string, Schema> = {
  n: { type: "integer", minimum: 1, maximum: 3 },
  tag: { type: "string", maxLength: 8 },
  kind: { enum: ["red", "blue"] },
  loud: { type: "boolean" },
};

const jar = (properties: Record<string, Schema>) =>
  app<{ beans: number }>({
    id: "jar",
    title: "Jar",
    line: "Beans in, beans out.",
    boot: () => ({ beans: 0 }),
    verbs: {
      put: {
        line: "Put some in.",
        args: { type: "object", properties, required: ["n"], additionalProperties: false },
        run: (state, args) => ({ beans: state.beans + Number(args.n) }),
      },
      take: {
        line: "Take one out.",
        run: (state, _args, ctx) => (state.beans ? { beans: state.beans - 1 } : ctx.refuse("the jar is empty")),
      },
    },
  });

function port(properties = ALL): Port {
  const k = kernel([jar(properties)]);
  let world = k.boot({ name: "w", apps: ["jar"] });
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
      const done = k.call(world, one);
      world = done.world;
      for (const fn of heard) fn();
      return { ok: done.envelope.last.ok, note: done.envelope.last.note, envelope: done.envelope };
    },
    who: () => ({ id: "test-1", kind: "human" }),
    actor: () => true,
    at: () => world.at,
  };
}

test("the generic view draws the state as a tree and each legal verb as a form, one design control a property", () => {
  plug(port());
  const html = renderToStaticMarkup(<View id="jar" />);
  plug(null);
  const controls = [
    ...html.matchAll(/<span>(n|tag|kind|loud)<\/span>(?:<span class="num">\d<\/span>)?<(input type="\w+"|select)/g),
  ];
  expect(controls.map(([, key, tag]) => `${key} ${tag}`)).toEqual([
    'n input type="range"',
    'tag input type="text"',
    "kind select",
    'loud input type="checkbox"',
  ]);
  expect([...html.matchAll(/<button[^>]*>(\w+)<\/button>/g)].map(([, verb]) => verb)).toEqual(["put", "take"]);
  expect(html).toContain('<dt>beans</dt><dd><span class="leaf">0</span></dd>');
});

test("a pressed verb acts in the world, and a refusal comes back as the note", async () => {
  plug(port({ n: ALL.n! }));
  const press = async (host: FakeNode, verb: string) => {
    const button = all(host, (at) => at.tagName === "BUTTON").find((at) => text(at) === verb)!;
    const props = Object.entries<unknown>(button).find(([key]) => key.startsWith("__reactProps"))![1] as {
      onClick: (e: object) => void;
    };
    await act(async () => props.onClick({ currentTarget: button, target: button }));
  };
  const notes = await mount(async (draw, host) => {
    await draw(<View id="jar" />);
    const note = () => text(find(host, (at) => at.attributes?.role === "status")!);
    await press(host, "take");
    const empty = note();
    await press(host, "put");
    await press(host, "take");
    return [empty, note(), text(find(host, (at) => at.tagName === "DD")!)];
  });
  plug(null);
  expect(notes).toEqual(["the jar is empty", "", "0"]);
});
