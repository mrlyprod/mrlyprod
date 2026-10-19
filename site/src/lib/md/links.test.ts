import { expect, test } from "bun:test";
import { index, resolve } from "./links.ts";

const rows = [
  { route: "/about/", source: "site/src/pages/about/index.md" },
  { route: "/blog/x/", source: "site/src/pages/blog/x/index.md" },
  { route: "/", source: null },
];

const idx = index(rows);

const post = (href: string) => resolve("site/src/pages/blog/x/index.md", href, idx);

test("a link to a page's source lands on its route, with or without .md, its tail kept", () => {
  expect([post("../../about/index.md"), post("../../about/index"), post("../../about/index.md#who")]).toEqual(["/about/", "/about/", "/about/#who"]);
});

test("a README with no route of its own lands in the code viewer", () => {
  expect(post("../../../../README.md")).toBe("/git/site/README.md");
});

test("a folder with its slash lands on its viewer listing", () => {
  expect(post("../../../lib/")).toBe("/git/site/src/lib/");
});

test("an absolute link, a fragment and another origin pass unchanged", () => {
  expect([post("/git/"), post("#top"), post("https://mrly.net/x/"), post("mailto:help@mrly.net")]).toEqual(["/git/", "#top", "https://mrly.net/x/", "mailto:help@mrly.net"]);
});

test("an image lands on its raw bytes", () => {
  expect(post("./plot.png")).toBe("/raw/site/src/pages/blog/x/plot.png");
});

test("a target in research lands in the code viewer like any other file", () => {
  expect([post("../../../../../research/farey/README.md"), resolve("research/farey/README.md", "notes/farey.md#top", idx)]).toEqual(["/git/research/farey/README.md", "/git/research/farey/notes/farey.md#top"]);
});

test("a script url goes nowhere and a path above the repo is left as written", () => {
  expect([post("java\tscript:alert(1)"), post("../../../../../../x.md")]).toEqual(["#", "../../../../../../x.md"]);
});
