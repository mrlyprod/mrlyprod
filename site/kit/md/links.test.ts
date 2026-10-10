import { expect, test } from "bun:test";
import { index, resolve } from "./links.ts";

const BLOB = "https://github.com/mrlyprod/mrlyprod/blob/main/";

const rows = [
  { route: "/about/", source: "site/pages/about.md" },
  { route: "/blog/x/", source: "site/blog/x/index.md" },
  { route: "/", source: null },
];

const idx = index(rows, BLOB);

const post = (href: string) => resolve("site/blog/x/index.md", href, idx);

test("a link to a page's source lands on its route, with or without .md, its tail kept", () => {
  expect([post("../../pages/about.md"), post("../../pages/about"), post("../../pages/about.md#who")]).toEqual(["/about/", "/about/", "/about/#who"]);
});

test("a README with no route of its own lands in the code viewer", () => {
  expect(post("../../README.md")).toBe("/git/site/README.md");
});

test("a folder with its slash lands on its viewer listing", () => {
  expect(post("../../kit/")).toBe("/git/site/kit/");
});

test("an absolute link, a fragment and another origin pass unchanged", () => {
  expect([post("/git/"), post("#top"), post("https://mrly.net/x/"), post("mailto:help@mrly.net")]).toEqual(["/git/", "#top", "https://mrly.net/x/", "mailto:help@mrly.net"]);
});

test("an image lands on its raw bytes", () => {
  expect(post("./plot.png")).toBe("/raw/site/blog/x/plot.png");
});

test("a target in the archive lands on its public reading copy", () => {
  expect(post("../../../research/claims/README.md")).toBe(`${BLOB}research/claims/README.md`);
});

test("a script url goes nowhere and a path above the repo is left as written", () => {
  expect([post("java\tscript:alert(1)"), post("../../../../x.md")]).toEqual(["#", "../../../../x.md"]);
});
