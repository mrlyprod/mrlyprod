import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

export const top = (dir: string): string =>
  existsSync(join(dir, ".git")) || dirname(dir) === dir ? dir : top(dirname(dir));

export const REPO = top(import.meta.dir);
export const SIBLINGS = dirname(REPO);
export const UNITS = ["core", "font", "life", "math", "num", "tile"];
export const unit = (name: string) => join(SIBLINGS, name, "js");
export const wasm = (name: string) => join(unit(name), "pkg", `mrly_${name}_js_bg.wasm`);
export const HOME = resolve(process.env.MRLY_ROOT ?? join(REPO, "net", "mrly.net"));
export const SITE = JSON.parse(
  readFileSync(join(HOME, "site.json"), "utf8"),
) as typeof import("../../../mrly.net/site.json");
