import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { main } from "../kit/dev.ts";
import type { Site } from "../kit/ssg/build.ts";
import { counted, demoShell, demoTree, spec } from "./site.ts";

const org = resolve(import.meta.dir, "..");
const cached = join(org, "data", "shelf", "research");
if (!process.env.MRLY_SHELF && existsSync(join(cached, "README.md"))) process.env.MRLY_SHELF = cached;

const demos = (site: Site, tail: string, ext: string) => {
  const home = site.input("demos");
  return home.files.filter((file) => file.endsWith(tail)).map((file) => ({ route: `/demos/${file.slice(home.path.length + 1, -tail.length)}${ext}`, file }));
};

await main(spec, {
  html: (site) => [{ route: "/demos/", file: demoShell(site, "") }, ...demos(site, "/index.html", "/")],
  scripts: (site) => demos(site, "/widget.jsx", "/widget.js"),
  disk: (site) => [["/figures/", site.input("figures").path], ["/research/", site.input("research").path]],
  extra: (site, path) => (path === "/demos/tree.json" ? Response.json(demoTree(site)) : null),
  line: () => {
    const count = counted();
    return `${count.papers} papers, ${count.research} research pages, ${count.blog} posts`;
  },
});
