import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { client, list, need } from "./s3.ts";

/* HASH */

function wasmFiles(dir: string): string[] {
  return (readdirSync(dir, { recursive: true }) as string[]).filter((rel) => statSync(join(dir, rel)).isFile()).sort((a, b) => Buffer.compare(Buffer.from(a, "utf8"), Buffer.from(b, "utf8")));
}

function wasmHash(dir: string): string {
  const manifest = new Bun.CryptoHasher("sha256");
  for (const rel of wasmFiles(dir)) {
    const one = new Bun.CryptoHasher("sha256");
    one.update(readFileSync(join(dir, rel)));
    manifest.update(`${one.digest("hex")}  ${rel}\n`);
  }
  return manifest.digest("hex");
}

/* FETCH */

export async function ensureWasm(root = resolve(import.meta.dir, "..")): Promise<string> {
  const dir = join(root, "wasm");
  if (existsSync(dir) && wasmFiles(dir).length > 0) return dir;
  const hash = readFileSync(join(root, "wasm.lock"), "utf8").trim();
  const prefix = `pkg/${hash}/`;
  const bucket = need("MRLYPROD_BUCKET");
  const s3 = client(bucket);
  const keys = (await list(s3, prefix)).filter((key) => key.length > prefix.length);
  if (keys.length === 0) throw new Error(`no objects at s3://${bucket}/${prefix}`);
  const stage = join(root, "wasm.next");
  rmSync(stage, { recursive: true, force: true });
  await Promise.all(
    keys.map(async (key) => {
      const to = join(stage, key.slice(prefix.length));
      const body = Buffer.from(await s3.file(key).arrayBuffer());
      mkdirSync(dirname(to), { recursive: true });
      writeFileSync(to, body);
    }),
  );
  if (existsSync(dir)) rmSync(dir, { recursive: true });
  renameSync(stage, dir);
  return dir;
}

/* MAIN */

if (import.meta.main) {
  const dir = await ensureWasm();
  console.log(`${wasmHash(dir)} ${wasmFiles(dir).length} ${dir}`);
}
