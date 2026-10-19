import { expect, test } from "bun:test";
import { llms, sections, type Listed } from "./llms.ts";

const row = (route: string, more: Partial<Listed> = {}): Listed => ({ route, kind: "prose", title: route, source: null, lead: "", hidden: false, meta: {}, ...more });

const rows = [
  row("/about/", { title: "About", source: "site/src/pages/about/index.md", lead: "One mark.", meta: { id: "about", prose: "page" } }),
  row("/blog/x/", { title: "X", source: "site/src/pages/blog/x/index.md", meta: { prose: "post" } }),
  row("/julia/", { kind: "app", title: "Julia", meta: { id: "julia" } }),
  row("/settings/", { kind: "app", title: "Settings", meta: { id: "settings" } }),
  row("/", { kind: "app", title: "Home", meta: { id: "home" } }),
  row("/figures/", { kind: "app", hidden: true, meta: { id: "figures" } }),
];

const shelves = [{ name: "Play", apps: ["julia"] }, { name: "Read", apps: ["about"] }, { name: "Tools", apps: ["settings"] }];

test("a row whose source was copied links its raw file, any other row its page", () => {
  const raw = new Set(["site/src/pages/about/index.md"]);
  expect(sections(rows, raw, shelves).flatMap((one) => one.lines.map((line) => line.href))).toEqual(["/julia/", "/raw/site/src/pages/about/index.md", "/settings/", "/blog/x/", "/"]);
});

test("every shown row lands in one section, an app by its Launchpad section, a hidden row in none", () => {
  expect(sections(rows, new Set(), shelves).map((one) => [one.name, one.lines.length])).toEqual([["Play", 1], ["Read", 1], ["Tools", 1], ["Blog", 1], ["Doors", 1]]);
});

test("the topics make a Research section: the folder, then each README by its title", () => {
  const topics = [{ slug: "beneath", title: "Beneath" }, { slug: "farey", title: "Farey" }];
  expect(sections(rows, new Set(), shelves, topics).at(-1)).toEqual({ name: "Research", lines: [{ name: "Research", note: "", href: "/raw/research/README.md" }, { name: "Beneath", note: "", href: "/raw/research/beneath/README.md" }, { name: "Farey", note: "", href: "/raw/research/farey/README.md" }] });
  expect(sections(rows, new Set()).map((one) => one.name)).not.toContain("Research");
});

test("llms.txt opens on the title, the root and the words, then one list per section", () => {
  const text = llms("T", "https://x.net", { about: "A.", legend: "L." }, rows.slice(0, 1), new Set(["site/src/pages/about/index.md"]), shelves);
  expect(text).toBe("# T\n\n> https://x.net\n\nA.\n\nL.\n\n## Read\n\n- [About](https://x.net/raw/site/src/pages/about/index.md): One mark.\n");
});
