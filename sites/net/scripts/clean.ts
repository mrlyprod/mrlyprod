import { existsSync, readdirSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";

const dist = join(resolve(import.meta.dir, ".."), "dist");
const keep = new Set(["_shots"]);

if (existsSync(dist)) {
  for (const name of readdirSync(dist)) if (!keep.has(name)) rmSync(join(dist, name), { recursive: true, force: true });
}
