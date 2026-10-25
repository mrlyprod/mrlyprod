import { expect, mock, test } from "bun:test";
import { act } from "react";
import type { FakeNode } from "../../test/dom.ts";
import { all, doc, mount, text } from "../../test/dom.ts";

type Keyed = { key: string; shiftKey?: boolean; preventDefault: () => void; currentTarget?: unknown };

type Props = { onKeyDown: (e: Keyed) => void; onClick: () => void };

type More = { focus: () => void } | null;

const SITE = {
  launchpad: {
    sections: [
      { name: "Make", apps: ["ink", "saw"] },
      { name: "Play", apps: ["dice"] },
    ],
    dock: [[]],
  },
};

mock.module("site:icons", () => ({ arts: {}, defs: "" }));
mock.module("site:routes", () => ({
  rows: ["ink", "saw", "dice"].map((id) => ({ route: `/${id}/`, title: id, lead: "", meta: { id } })),
  site: SITE,
}));

const { Launchpad } = await import("./launchpad.tsx");
const { KEY } = await import("./prefs.ts");

const props = (el: FakeNode): Props =>
  Reflect.get(el, Object.keys(el).find((key) => key.startsWith("__reactProps$")) ?? "");

const find = (host: FakeNode, tag: string, words: string) =>
  all(host, (at) => at.tagName === tag && (text(at) === words || at.attributes["aria-label"] === words))[0];

const pad = async (run: (host: FakeNode) => Promise<void>, more: More = null) => {
  const store = new Map<string, string>();
  const make = doc.createElement;
  Object.assign(globalThis, {
    localStorage: {
      getItem: (key: string) => store.get(key) ?? null,
      setItem: (key: string, value: string) => store.set(key, value),
      removeItem: (key: string) => store.delete(key),
    },
    location: { search: "" },
  });
  const made = (tag: string) =>
    Object.defineProperty(make(tag), "children", {
      get(this: FakeNode) {
        return this.childNodes;
      },
    });
  doc.createElement = (tag: string) =>
    Object.assign(made(tag), {
      querySelector: (sel: string) => (sel === ".more" ? more : null),
      querySelectorAll: () => [],
      getBoundingClientRect: () => ({ left: 0, bottom: 0 }),
      focus: () => {},
    });
  Object.assign(doc.body, { prepend: (kid: FakeNode) => doc.body.insertBefore(kid, doc.body.firstChild) });
  Object.assign(doc, { getElementById: () => null });
  try {
    await mount(async (draw, host) => {
      await draw(<Launchpad />);
      await run(host);
    });
  } finally {
    doc.createElement = make;
    doc.activeElement = null;
    Reflect.deleteProperty(doc.body, "prepend");
    Reflect.deleteProperty(doc, "getElementById");
    Reflect.deleteProperty(globalThis, "localStorage");
    Reflect.deleteProperty(globalThis, "location");
  }
  return store;
};

test("moving an app to another section from its tile's menu leaves it in that section alone", async () => {
  const store = await pad(async (host) => {
    const tile = all(host, (at) => at.tagName === "LI" && at.attributes["data-id"] === "ink")[0];
    await act(async () =>
      props(tile).onKeyDown({ key: "F10", shiftKey: true, preventDefault: () => {}, currentTarget: tile }),
    );
    await act(async () => props(find(host, "BUTTON", "Move to Play")).onClick());
  });
  expect(JSON.parse(store.get(KEY.launchpad)!)).toEqual([
    { name: "Make", apps: ["saw"] },
    { name: "Play", apps: ["dice", "ink"] },
  ]);
});

test("confirming a section rename with Enter cancels the key so it cannot click the chip", async () => {
  const stop = mock(() => {});
  await pad(async (host) => {
    await act(async () => props(find(host, "BUTTON", "Make")).onKeyDown({ key: "F2", preventDefault: () => {} }));
    await act(async () =>
      props(find(host, "INPUT", "Rename Make")).onKeyDown({
        key: "Enter",
        preventDefault: stop,
        currentTarget: { value: "Build" },
      }),
    );
  });
  expect(stop).toHaveBeenCalled();
});

test("undoing from the toast by keyboard hands focus back to More", async () => {
  const focus = mock(() => {});
  await pad(
    async (host) => {
      await act(async () => props(find(host, "BUTTON", "Reset Layout")).onClick());
      const undo = find(host, "BUTTON", "Undo");
      doc.activeElement = undo;
      await act(async () => props(undo).onClick());
    },
    { focus },
  );
  expect(focus).toHaveBeenCalled();
});
