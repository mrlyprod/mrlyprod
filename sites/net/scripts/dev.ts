import { existsSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { statics } from "./site.ts";

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const { papers, research, blog } = await statics();

const routes: Record<string, unknown> = { "/demos": (await import(join(org, "demos", "index.html"))).default };
for (const name of readdirSync(join(org, "demos"))) {
  const html = join(org, "demos", name, "index.html");
  if (existsSync(html)) routes[`/demos/${name}`] = (await import(html)).default;
}

const server = Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  development: true,
  routes,
  fetch(req) {
    let path = decodeURIComponent(new URL(req.url).pathname);
    if (path.endsWith("/")) path += "index.html";
    const file = Bun.file(join(dist, path));
    return file.size ? new Response(file) : new Response(Bun.file(join(dist, "404.html")), { status: 404 });
  },
});
console.log(`dev: ${Object.keys(routes).length} pages, ${papers} papers, ${research} research pages, ${blog} posts at ${server.url}`);
