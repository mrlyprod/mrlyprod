import { existsSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { statics } from "./site.ts";

const demos = resolve(import.meta.dir, "..");
const dist = join(demos, "dist");
const { papers, research } = statics();

const routes: Record<string, unknown> = { "/": (await import(join(demos, "index.html"))).default };
for (const name of readdirSync(demos)) {
  const html = join(demos, name, "index.html");
  if (name !== "dist" && name !== "node_modules" && existsSync(html)) routes[`/${name}`] = (await import(html)).default;
}

const server = Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  development: true,
  routes,
  fetch(req) {
    let path = new URL(req.url).pathname;
    if (path.endsWith("/")) path += "index.html";
    const file = Bun.file(join(dist, path));
    return file.size ? new Response(file) : new Response("not found", { status: 404 });
  },
});
console.log(`dev: ${Object.keys(routes).length} pages, ${papers} papers, ${research} research pages at ${server.url}`);
