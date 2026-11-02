import { expect, test } from "bun:test";
import { lang } from "../../sdk/git/view.ts";
import { paint } from "./code.ts";

const shiki = await import("@shikijs/core").then(
  () => true,
  () => false,
);

test.skipIf(!shiki)("a rust snippet paints one string per line", async () => {
  const out = await paint("fn main() {\n    let a = 1;\n}\n", "rust");
  expect(out).not.toBeNull();
  expect(out!.length).toBe(3);
  expect(out![1]).toContain('<span class="tk-keyword">let</span>');
  expect(out!.join("")).not.toContain("style=");
});

test("an unknown extension paints nothing", async () => {
  expect(await paint("hello\n", lang("notes.bin"))).toBeNull();
  expect(await paint("hello\n", "text")).toBeNull();
});
