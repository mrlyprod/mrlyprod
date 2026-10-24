import { expect, test } from "bun:test";
import { decode, encode, idle, KEY, minutes, offs, pack, read, savers, subscribe, unpack, write } from "./prefs.js";

const memory = () => {
  const box = new Map();
  return {
    getItem: (key) => (box.has(key) ? box.get(key) : null),
    setItem: (key, value) => box.set(key, String(value)),
    removeItem: (key) => box.delete(key),
  };
};

const fresh = (fn) => {
  globalThis.localStorage = memory();
  try {
    fn();
  } finally {
    delete globalThis.localStorage;
  }
};

test("a subscriber hears a write that changes the stored value, not one that keeps it, and nothing once it leaves", () => {
  fresh(() => {
    const heard = [];
    const leave = subscribe((key) => heard.push(key));
    write(KEY.theme, "dark");
    write(KEY.theme, "dark");
    leave();
    write(KEY.theme, "");
    expect(heard).toEqual([KEY.theme]);
  });
});

test("pack keeps the set settings and unpack writes them back, clearing the names it lacks and nothing else", () => {
  fresh(() => {
    write(KEY.theme, "dark");
    write(KEY.idle, "never");
    write(KEY.cart, "[1]");
    const data = pack();
    write(KEY.tint, "red");
    unpack(data);
    expect([data, pack(), read(KEY.cart)]).toEqual([
      { theme: "dark", idle: "never" },
      { theme: "dark", idle: "never" },
      "[1]",
    ]);
  });
});

test("unpack throws on anything but a plain object of known names and text values, and writes nothing then", () => {
  fresh(() => {
    write(KEY.theme, "dark");
    for (const bad of [
      null,
      "theme",
      7,
      [],
      ["dark"],
      new Date(),
      { theme: 1 },
      { nope: "x" },
      { theme: "light", cart: "[]" },
    ])
      expect(() => unpack(bad)).toThrow();
    expect(read(KEY.theme)).toBe("dark");
  });
});

test("encode and decode round trip any text as unpadded base64url, and decode turns junk into null", () => {
  const data = { theme: "dark", page: "Café ✓ 日本 🎨" };
  const code = encode(data);
  expect([/^[\w-]+$/.test(code), decode(code)]).toEqual([true, data]);
  expect(
    [
      "",
      "a",
      "!!!",
      `${code}=`,
      "_w",
      btoa("{bad").replace(/=+$/, ""),
      encode([1]),
      encode("x"),
      encode(null),
      undefined,
    ].map(decode),
  ).toEqual(Array(10).fill(null));
});

test("offs reads the saver ids switched off, and none on bad JSON", () => {
  fresh(() => {
    savers(["polygons/waves", "mrly/tiles"]);
    const set = offs();
    savers([]);
    const none = [read(KEY.off), offs()];
    write(KEY.off, "{bad");
    expect([set, none, offs()]).toEqual([["polygons/waves", "mrly/tiles"], ["", []], []]);
  });
});

test("minutes is 5 by default, the set delay otherwise and 0 for never", () => {
  fresh(() => {
    const unset = minutes();
    idle("2");
    const two = minutes();
    idle("never");
    const never = minutes();
    idle("soon");
    expect([unset, two, never, minutes()]).toEqual([5, 2, 0, 5]);
  });
});
