import { expect, test } from "bun:test";
import type { Site } from "../kit/ssg/build.ts";
import { sections, type Lists } from "./map.ts";

/* SITE */

const site = {
  root: "/repo/site",
  config: { git: { root: ".." } },
  routes: [{ route: "/git/wiki/a.md", urls: [{ route: "/git/wiki/a.md" }, { route: "/raw/wiki/a.md" }] }],
} as unknown as Site;

const lists = (some: Partial<Lists>): Lists => ({ wiki: [], notes: [], claims: [], papers: [], lanes: [], posts: [], math: [], pages: [], ...some });

const rows = (all: Lists, name: string) => sections(site, all).find((one) => one.name === name)!.rows;

/* ROWS */

test("a served source links its raw path and an unserved one its page", () => {
  const wiki = [
    { slug: "a", name: "A", lead: "first", file: "/repo/wiki/a.md" },
    { slug: "b", name: "B", lead: "second", file: "/repo/wiki/b.md" },
  ];
  expect(rows(lists({ wiki }), "Wiki, in prerequisite order")).toEqual([
    { name: "A", note: "first", href: "/raw/wiki/a.md" },
    { name: "B", note: "second", href: "/wiki/b/" },
  ]);
});

test("a claim file is one row with its claim count and its newest date", () => {
  const md = "# C\n\n- 2026-01-02 [Proved] one\n- 2026-03-04 [Conjecture] two\n- 2026-02-03 [Verified] three\n";
  const claims = [{ slug: "c", title: "C", md, file: "/repo/research/claims/c.md" }];
  expect(rows(lists({ claims }), "Discoveries")).toEqual([{ name: "C", note: "3 claims, newest 2026-03-04", href: "/research/discoveries/#c" }]);
});
