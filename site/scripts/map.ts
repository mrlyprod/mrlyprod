import { readFileSync } from "node:fs";
import { basename, join, relative } from "node:path";
import { config } from "../kit/git/git.ts";
import { rawPath } from "../kit/git/view.ts";
import type { Row, Section, Site } from "../kit/ssg/build.ts";
import { front, summary, title } from "../kit/ssg/md.ts";

/* TYPES */

type Wiki = { name: string; lead: string; file: string; href: string };

type Note = { title: string; lead: string; file: string; href: string };

type Claim = { title: string; md: string; file: string; href: string };

type Paper = { name: string; lead: string; file: string; href: string };

type Lane = { name: string; blurb: string; md: string; href: string };

type Post = { slug: string; name: string; lead: string };

export type Lists = { wiki: Wiki[]; notes: Note[]; claims: Claim[]; papers: Paper[]; lanes: Lane[]; posts: Post[]; math: string[]; pages: string[] };

/* CLAIMS */

const CLAIM = /^- (\d{4}-\d{2}-\d{2}) \[(?:Proved|Verified|Conjecture|Refuted)\] /gm;

function claimed(md: string) {
  const dates = [...md.matchAll(CLAIM)].map((m) => m[1]!).sort();
  return dates.length ? `${dates.length} claim${dates.length > 1 ? "s" : ""}, newest ${dates.at(-1)}` : "no claim yet";
}

/* SECTIONS */

export function sections(site: Site, lists: Lists): Section[] {
  const git = config(site);
  const served = new Set(site.routes.flatMap((route) => (route.urls ?? []).map((one) => one.route)));
  const row = (name: string, note: string, file: string, page: string): Row => {
    const raw = git ? `/${rawPath(relative(git.root, file))}` : "";
    return { name, note, href: served.has(raw) ? raw : page };
  };
  const read = (file: string, page: string, name?: string) => {
    const { data, body } = front(readFileSync(file, "utf8"));
    return row(name || data.title || title(body) || basename(file, ".md"), data.lead || summary(body), file, page);
  };
  return [
    { name: "Wiki, in prerequisite order", rows: lists.wiki.map((e) => row(e.name, e.lead, e.file, e.href)) },
    { name: "Research notes", rows: lists.notes.map((n) => row(n.title, n.lead, join(site.input("research").path, n.file), n.href)) },
    { name: "Claims", rows: lists.claims.map((c) => row(c.title, claimed(c.md), c.file, c.href)) },
    {
      name: "Papers",
      rows: [...lists.papers.map((p) => row(p.name, p.lead, p.file, p.href)), ...lists.lanes.map((p) => ({ name: p.name, note: p.blurb || summary(p.md), href: p.href }))],
    },
    { name: "Blog", rows: lists.posts.map((p) => row(p.name, p.lead, join(site.input("blog").path, p.slug, "index.md"), `/blog/${p.slug}/`)) },
    { name: "MrlyMath", rows: lists.math.map((file) => read(file, "/mrlymath/", "MrlyMath")) },
    { name: "About", rows: lists.pages.map((file) => read(file, `/${basename(file, ".md")}/`)) },
  ];
}
