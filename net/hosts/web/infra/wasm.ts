import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { HOME } from "../scripts/home.ts";
import { client, list, need } from "./s3.ts";

/* HASH */

function wasmFiles(dir: string): string[] {
  return (readdirSync(dir, { recursive: true }) as string[])
    .filter((rel) => statSync(join(dir, rel)).isFile())
    .sort((a, b) => Buffer.compare(Buffer.from(a, "utf8"), Buffer.from(b, "utf8")));
}

export function wasmHash(dir: string): string {
  const manifest = new Bun.CryptoHasher("sha256");
  for (const rel of wasmFiles(dir)) {
    const one = new Bun.CryptoHasher("sha256");
    one.update(readFileSync(join(dir, rel)));
    manifest.update(`${one.digest("hex")}  ${rel}\n`);
  }
  return manifest.digest("hex");
}

/* PIN */

export type Pin = { lock: string; local: string | null };

export function pin(root = HOME): Pin {
  const dir = join(root, "wasm");
  const lock = readFileSync(join(root, "wasm.lock"), "utf8").trim();
  return { lock, local: existsSync(dir) && wasmFiles(dir).length > 0 ? wasmHash(dir) : null };
}

/* FETCH */

export async function ensureWasm(root = HOME): Promise<string> {
  const dir = join(root, "wasm");
  if (existsSync(dir) && wasmFiles(dir).length > 0) return dir;
  const hash = readFileSync(join(root, "wasm.lock"), "utf8").trim();
  const prefix = `pkg/${hash}/`;
  const named = (JSON.parse(readFileSync(join(root, "site.json"), "utf8")) as { push?: { wasm?: string } }).push?.wasm;
  if (!named) throw new Error("wasm: site.json names no push.wasm, the env var that holds the wasm bucket");
  const bucket = need(named);
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
  const { lock, local } = pin();
  console.log(`${local} ${wasmFiles(dir).length} ${dir}`);
  if (local !== lock) {
    console.error(`wasm: ${dir} hashes to ${local}, and wasm.lock pins ${lock}; the deploy ships only the pinned copy`);
    process.exit(1);
  }
}
