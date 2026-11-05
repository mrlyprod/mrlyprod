import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, extname, join, relative, resolve } from "node:path";
import { HALL, PAGE, ROBOTS } from "@mrlyprod/os/hosts/cloud/guard.js";
import type { File } from "@mrlyprod/os/hosts/cloud/push.ts";
import type { BunPlugin } from "bun";
import { esc } from "../../kit/md/text.ts";
import { tintCss } from "../../kit/theme/tint.ts";
import { draw, sheets, source } from "./design.ts";
import { HOME, REPO, SITE } from "./home.ts";
import { packages } from "./packages.ts";

export type Lock = { realm: string };

export type Site = { title: string; wordmark?: string; root: string; tint: string; mark: string; lock?: Lock };

const NAMING = { entry: "page-[hash].[ext]", chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" };
const SHARED = join(REPO, "shared");
const UNLOCK = join(REPO, "shell", "unlock.tsx");
const BOTS = "User-agent: *\nDisallow: /\n";

const sha = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");

const here = (url: string) => url.slice(1);

export const lockOf = (site: Site = SITE as Site) => site.lock ?? null;

export function modules(name: string, css: string[], skin: Parameters<typeof sheets>[2], site: Site) {
  const word = site.wordmark || site.title.toUpperCase();
  const pull = (file: string) => `@import ${JSON.stringify(file)} (scripting: enabled);`;
  return {
    "lock:page": [
      'import { design } from "site:design";',
      `import { open } from ${JSON.stringify(UNLOCK)};`,
      `open(design.parts.lock, ${JSON.stringify(word)});`,
    ].join("\n"),
    "lock:page.css": [
      pull(join(SHARED, "palette.gen.css")),
      pull("site:design/tokens.css"),
      pull(join(SHARED, "base.css")),
      pull("site:design.css"),
      pull(join(SHARED, "fonts", "fonts.css")),
      `@media (scripting: enabled) {\n${tintCss(site.tint)}.domain {\n  display: none;\n}\n}`,
    ].join("\n"),
    "site:design": source(name),
    ...sheets(name, css, skin),
  } as Record<string, string>;
}

export const only = (table: Record<string, string>, fonts: Map<string, string>): BunPlugin => ({
  name: "lock",
  setup(build) {
    packages.setup(build);
    build.onResolve({ filter: /\.woff2$/ }, ({ path, importer }) => {
      const file = resolve(dirname(importer), path);
      if (!fonts.has(file))
        fonts.set(file, `${HALL}${basename(file, ".woff2")}-${sha(readFileSync(file)).slice(0, 8)}.woff2`);
      return { path: fonts.get(file)!, external: true };
    });
    build.onResolve({ filter: /^(site|lock|apps?):/ }, ({ path }) => ({ path, namespace: "lock" }));
    build.onLoad({ filter: /.*/, namespace: "lock" }, ({ path }) => {
      const text = table[path];
      if (text === undefined)
        throw new Error(
          `lock: the lock page imports ${path}; it shows the design and the wordmark, nothing of the site`,
        );
      return { contents: text, loader: path.endsWith(".css") ? "css" : "js" };
    });
  },
});

const html = (realm: string, js: string, css: string, mark: string | null) =>
  [
    "<!doctype html>",
    '<html lang="en">',
    "<head>",
    '<meta charset="utf-8">',
    '<meta name="viewport" content="width=device-width, initial-scale=1">',
    '<meta name="robots" content="noindex, nofollow">',
    `<title>${esc(realm)}</title>`,
    ...(mark ? [`<link rel="icon" href="${mark}">`] : []),
    `<link rel="stylesheet" href="${css}">`,
    `<script type="module" src="${js}"></script>`,
    "</head>",
    "<body>",
    `<p class="domain">${esc(realm)}</p>`,
    "</body>",
    "</html>",
    "",
  ].join("\n");

type Out = { path: string; bytes: string | Uint8Array; fixed: boolean };

export async function hall(lock: Lock, out: string, site: Site = SITE as Site, home = HOME): Promise<File[]> {
  const drawn = await draw();
  if (typeof drawn.design.parts.lock !== "function")
    throw new Error(`lock: the design ${drawn.name} draws no lock part, so it cannot lock a site`);
  const fonts = new Map<string, string>();
  const table = modules(drawn.name, drawn.design.css, drawn.design.skin, site);
  const built = await Bun.build({
    entrypoints: ["lock:page", "lock:page.css"],
    minify: true,
    publicPath: HALL,
    naming: NAMING,
    define: { "process.env.NODE_ENV": '"production"' },
    plugins: [only(table, fonts)],
  });
  if (!built.success) throw new Error(`lock: the lock page failed to bundle\n${built.logs.join("\n")}`);
  const made: Out[] = await Promise.all(
    built.outputs.map(async (one) => ({
      path: `${here(HALL)}${one.path.replace(/^\.\//, "")}`,
      bytes: new Uint8Array(await one.arrayBuffer()),
      fixed: true,
    })),
  );
  const faces: Out[] = [...fonts].map(([file, url]) => ({ path: here(url), bytes: readFileSync(file), fixed: true }));
  const icon = join(home, "public", site.mark);
  const shown = existsSync(icon) ? readFileSync(icon) : null;
  const mark = shown ? `${HALL}mark-${sha(shown).slice(0, 8)}${extname(icon)}` : null;
  const find = (ext: string) =>
    `/${made.find((one) => one.path.startsWith(`${here(HALL)}page-`) && one.path.endsWith(ext))!.path}`;
  const all: Out[] = [
    ...made,
    ...faces,
    ...(shown && mark ? [{ path: here(mark), bytes: shown, fixed: true }] : []),
    { path: here(PAGE), bytes: html(lock.realm, find(".js"), find(".css"), mark), fixed: false },
    { path: here(ROBOTS), bytes: BOTS, fixed: false },
  ];
  return all.map((one) => {
    const file = join(out, one.path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, one.bytes);
    return { path: one.path, hash: sha(one.bytes).slice(0, 16), fixed: one.fixed || undefined };
  });
}

export async function locked<T extends { out: string; files: File[] }>(done: T, site: Site = SITE as Site): Promise<T> {
  const lock = lockOf(site);
  return lock ? { ...done, files: [...done.files, ...(await hall(lock, done.out, site))] } : done;
}

if (import.meta.main) {
  const out = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(HOME, "dist");
  const site = SITE as Site;
  const files = await hall(lockOf(site) ?? { realm: new URL(site.root).host }, out, site);
  console.log(`lock: ${files.length} files under ${relative(REPO, join(out, here(HALL)))}`);
}
