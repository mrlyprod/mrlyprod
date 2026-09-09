import { existsSync, mkdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const org = resolve(import.meta.dir, "..");
const data = join(org, "data");
const home = join(data, "shelf");
const REPO = process.env.SHELF_REPO ?? "";
const TARBALL = REPO ? `https://codeload.github.com/${REPO}/tar.gz/main` : "";
const KEEP = "research/";

/* TAR */

const text = (bytes: Uint8Array, from: number, to: number) => {
  const end = bytes.indexOf(0, from);
  return new TextDecoder().decode(bytes.subarray(from, end < 0 || end > to ? to : end));
};
const octal = (bytes: Uint8Array, from: number, to: number) => parseInt(text(bytes, from, to).trim() || "0", 8);

function pax(block: Uint8Array) {
  const out: Record<string, string> = {};
  const body = new TextDecoder().decode(block);
  for (const line of body.split("\n")) {
    const m = line.match(/^\d+ ([^=]+)=(.*)$/);
    if (m) out[m[1]] = m[2];
  }
  return out;
}

function untar(tar: Uint8Array, into: string) {
  let at = 0;
  let next: Record<string, string> = {};
  let files = 0;
  while (at + 512 <= tar.length) {
    const head = tar.subarray(at, at + 512);
    if (head.every((b) => b === 0)) break;
    const size = octal(head, 124, 136);
    const kind = String.fromCharCode(head[156]);
    const prefix = text(head, 345, 500);
    let name = next.path ?? (prefix ? `${prefix}/${text(head, 0, 100)}` : text(head, 0, 100));
    const body = tar.subarray(at + 512, at + 512 + size);
    at += 512 + Math.ceil(size / 512) * 512;
    if (kind === "x") {
      next = pax(body);
      continue;
    }
    next = {};
    if (kind === "g" || kind === "L") continue;
    name = name.replace(/^[^/]+\//, "");
    if (!name.startsWith(KEEP)) continue;
    const path = join(into, name);
    if (kind === "5" || name.endsWith("/")) {
      mkdirSync(path, { recursive: true });
    } else if (kind === "0" || kind === "\0") {
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, body);
      files++;
    }
  }
  return files;
}

/* FETCH */

export async function shelf(): Promise<string> {
  const local = process.env.MRLY_SHELF;
  if (local) return resolve(local);
  const cached = join(home, "research");
  const fallback = (why: string) => {
    if (!existsSync(join(cached, "README.md"))) throw new Error(`shelf: ${why} and no cache at ${cached}; set MRLY_SHELF to a local checkout or SHELF_REPO to an owner/repo`);
    console.warn(`shelf: ${why}, building from the cached copy`);
    return cached;
  };
  if (!TARBALL) return fallback("no SHELF_REPO in the environment");
  try {
    const res = await fetch(TARBALL);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const tar = Bun.gunzipSync(new Uint8Array(await res.arrayBuffer()));
    const fresh = join(data, "shelf.next");
    rmSync(fresh, { recursive: true, force: true });
    const files = untar(tar, fresh);
    if (!existsSync(join(fresh, "research", "README.md"))) throw new Error("tarball carries no research/README.md");
    rmSync(home, { recursive: true, force: true });
    renameSync(fresh, home);
    console.log(`shelf: fetched ${files} files from GitHub`);
  } catch (reason) {
    return fallback(`fetch failed (${reason})`);
  }
  return cached;
}
