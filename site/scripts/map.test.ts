import { expect, test } from "bun:test";
import type { Site } from "../kit/ssg/build.ts";
import { sections, type Lists } from "./map.ts";

/* SITE */

const site = {
  root: "/repo/site",
  config: { git: { root: ".." } },
  routes: [{ route: "/raw/research/wiki/a.md", kind: "raw", urls: [{ route: "/raw/research/wiki/a.md" }] }],
} as unknown as Site;

const lists = (some: Partial<Lists>): Lists => ({ wiki: [], notes: [], claims: [], papers: [], lanes: [], posts: [], math: [], pages: [], ...some });

const rows = (all: Lists, name: string) => sections(site, all).find((one) => one.name === name)!.rows;

/* ROWS */

test("a served source links its raw path and an unserved one its page", () => {
  const wiki = [
    { name: "A", lead: "first", file: "/repo/research/wiki/a.md", href: "/research/wiki/a/" },
    { name: "B", lead: "second", file: "/repo/research/wiki/b.md", href: "/research/wiki/b/" },
  ];
  expect(rows(lists({ wiki }), "Wiki, in prerequisite order")).toEqual([
    { name: "A", note: "first", href: "/raw/research/wiki/a.md" },
    { name: "B", note: "second", href: "/research/wiki/b/" },
  ]);
});

test("a claim file is one row with its claim count and its newest date", () => {
  const md = "# C\n\n- 2026-01-02 [Proved] one\n- 2026-03-04 [Conjecture] two\n- 2026-02-03 [Verified] three\n";
  const claims = [{ title: "C", md, file: "/repo/research/claims/c.md", href: "/research/claims/c/" }];
  expect(rows(lists({ claims }), "Claims")).toEqual([{ name: "C", note: "3 claims, newest 2026-03-04", href: "/research/claims/c/" }]);
});
