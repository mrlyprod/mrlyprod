import { expect, test } from "bun:test";
import { llms, sections, type Listed } from "./map.ts";

const row = (route: string, more: Partial<Listed> = {}): Listed => ({ route, kind: "prose", title: route, source: null, lead: "", hidden: false, meta: {}, ...more });

const rows = [
  row("/about/", { title: "About", source: "site/pages/about.md", lead: "One mark.", meta: { was: "page" } }),
  row("/blog/x/", { title: "X", source: "site/blog/x/index.md", meta: { was: "post" } }),
  row("/julia/", { kind: "app", title: "Julia" }),
  row("/", { kind: "home", title: "Home" }),
  row("/cart/", { kind: "cart", hidden: true }),
];

test("a row whose source was copied links its raw file, any other row its page", () => {
  const raw = new Set(["site/pages/about.md"]);
  expect(sections(rows, raw).flatMap((one) => one.lines.map((line) => line.href))).toEqual(["/blog/x/", "/raw/site/pages/about.md", "/julia/", "/"]);
});

test("every shown row lands in one section, a hidden row in none", () => {
  expect(sections(rows, new Set()).map((one) => [one.name, one.lines.length])).toEqual([["Blog", 1], ["About", 1], ["Apps", 1], ["Doors", 1]]);
});

test("llms.txt opens on the title, the root and the words, then one list per section", () => {
  const text = llms("T", "https://x.net", { about: "A.", legend: "L." }, rows.slice(0, 1), new Set(["site/pages/about.md"]));
  expect(text).toBe("# T\n\n> https://x.net\n\nA.\n\nL.\n\n## About\n\n- [About](https://x.net/raw/site/pages/about.md): One mark.\n");
});
