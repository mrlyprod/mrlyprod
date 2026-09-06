import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { client, list, PROD_BUCKET } from "../../../aws/s3.ts";

/* HASH */

function crawl(dir: string, at: string, out: string[]) {
  for (const entry of readdirSync(join(dir, at), { withFileTypes: true })) {
    const rel = at ? `${at}/${entry.name}` : entry.name;
    if (entry.isDirectory()) crawl(dir, rel, out);
    else out.push(rel);
  }
}

export function pkgFiles(dir: string): string[] {
  const out: string[] = [];
  crawl(dir, "", out);
  return out.sort((a, b) => Buffer.compare(Buffer.from(a, "utf8"), Buffer.from(b, "utf8")));
}

export function pkgHash(dir: string): string {
  const manifest = new Bun.CryptoHasher("sha256");
  for (const rel of pkgFiles(dir)) {
    const one = new Bun.CryptoHasher("sha256");
    one.update(readFileSync(join(dir, rel)));
    manifest.update(`${one.digest("hex")}  ${rel}\n`);
  }
  return manifest.digest("hex");
}

/* FETCH */

export async function ensurePkg(root = resolve(import.meta.dir, "..")): Promise<string> {
  const dir = join(root, "pkg");
  if (existsSync(dir) && pkgFiles(dir).length > 0) return dir;
  const hash = readFileSync(join(root, "pkg.lock"), "utf8").trim();
  const prefix = `pkg/${hash}/`;
  const s3 = client(PROD_BUCKET);
  const keys = (await list(s3, prefix)).filter((key) => key.length > prefix.length);
  for (const key of keys) {
    const to = join(dir, key.slice(prefix.length));
    mkdirSync(dirname(to), { recursive: true });
    writeFileSync(to, Buffer.from(await s3.file(key).arrayBuffer()));
  }
  if (keys.length === 0) throw new Error(`no objects at s3://${PROD_BUCKET}/${prefix}`);
  return dir;
}

/* MAIN */

if (import.meta.main) {
  const dir = await ensurePkg();
  console.log(`${pkgHash(dir)} ${pkgFiles(dir).length} ${dir}`);
}
