import { readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { build, digest, globals, today, type Manifest, type Output } from "../../../pkgs/js/mrlyjs/ssg/build.ts";
import { client, del, getText, list, putBytes, DEV_BUCKET, NET_BUCKET } from "../../../aws/s3.ts";
import { spec } from "./site.ts";

/* WHERE */

const REMOTE = "build/net/manifest.json";
const ASSET = "@";
const BATCH = 32;

/* HEADERS */

const IMMUTABLE = "public, max-age=31536000, immutable";
const REVALIDATE = "public, max-age=0, must-revalidate";

const HASHED = [/(^|\/)lib-[^/]+\.js$/, /(^|\/)lib-[^/]+\.css$/, /\.wasm$/, /-[0-9a-f]{8}\.[^./]+$/];

const TYPES: Record<string, string> = {
  html: "text/html; charset=utf-8",
  js: "text/javascript; charset=utf-8",
  mjs: "text/javascript; charset=utf-8",
  css: "text/css; charset=utf-8",
  json: "application/json",
  map: "application/json",
  webmanifest: "application/manifest+json",
  xml: "application/xml",
  txt: "text/plain; charset=utf-8",
  md: "text/markdown; charset=utf-8",
  tex: "text/plain; charset=utf-8",
  wasm: "application/wasm",
  svg: "image/svg+xml",
  png: "image/png",
  jpg: "image/jpeg",
  jpeg: "image/jpeg",
  gif: "image/gif",
  webp: "image/webp",
  ico: "image/x-icon",
  pdf: "application/pdf",
  woff2: "font/woff2",
};

const kind = (path: string) => TYPES[(path.match(/\.([^./]+)$/)?.[1] ?? "").toLowerCase()] ?? "application/octet-stream";

const cache = (path: string) => (HASHED.some((re) => re.test(path)) ? IMMUTABLE : REVALIDATE);

/* MANIFEST */

function assets(items: Output[], old: Manifest): Manifest {
  const out: Manifest = {};
  for (const item of items) {
    const body = typeof item.bytes === "string" ? new TextEncoder().encode(item.bytes) : item.bytes;
    const hash = digest([body]).slice(0, 16);
    const was = old[ASSET + item.path];
    out[ASSET + item.path] = was && was.hash === hash ? was : { hash, at: today(), outputs: [item.path] };
  }
  return out;
}

function spread(manifest: Manifest): Map<string, string> {
  const out = new Map<string, string>();
  for (const [key, record] of Object.entries(manifest)) for (const path of record.outputs) out.set(path, key);
  return out;
}

/* PUSH */

export async function push(options: { dry?: boolean } = {}): Promise<{ rendered: number; uploaded: number; deleted: number }> {
  const dev = client(DEV_BUCKET);
  const net = client(NET_BUCKET);
  const found = await getText(dev, REMOTE);
  const old: Manifest = found ? JSON.parse(found) : {};
  const carry = join(tmpdir(), `mrlynet-remote-${process.pid}.json`);
  writeFileSync(carry, JSON.stringify(old, null, 2) + "\n");
  const done = await build(spec, { manifest: carry });
  rmSync(carry, { force: true });
  const next: Manifest = { ...done.manifest, ...assets(await globals(done.site, spec), old) };
  const want = spread(next);
  const had = spread(old);
  const upload: string[] = [];
  for (const [path, key] of want) {
    const was = old[key];
    if (was && was.hash === next[key]!.hash && had.has(path)) continue;
    upload.push(path);
  }
  const seen = found ? [...had.keys()] : await list(net);
  const remove = seen.filter((path) => !want.has(path));
  if (!options.dry) {
    for (let i = 0; i < upload.length; i += BATCH) {
      await Promise.all(
        upload.slice(i, i + BATCH).map((path) =>
          putBytes(net, path, new Uint8Array(readFileSync(join(done.site.out, path))), {
            type: kind(path),
            cacheControl: cache(path),
          }),
        ),
      );
    }
    if (remove.length) await del(net, remove, BATCH);
    await putBytes(dev, REMOTE, JSON.stringify(next, null, 2) + "\n", { type: "application/json", cacheControl: REVALIDATE });
  }
  return { rendered: done.rendered, uploaded: upload.length, deleted: remove.length };
}

/* MAIN */

if (import.meta.main) {
  const dry = process.argv.includes("--dry");
  const done = await push({ dry });
  console.log(`push${dry ? " --dry" : ""}: ${done.rendered} rendered, ${done.uploaded} uploaded, ${done.deleted} deleted`);
}
