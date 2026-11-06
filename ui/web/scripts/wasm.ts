import { copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { live } from "./build.ts";
import { blobs, HOME, home, REPO, SITE, UNITS, unit } from "./home.ts";

const WASM = join(HOME, "wasm");
const FONT = join(REPO, "kit", "font", "font.gen.json");

export function copy(units = live(), to = WASM): string[] {
  if (SITE.wasm && !UNITS.length) throw new Error("wasm: no bindgen table; run scripts/build.sh from the repo root");
  const held = [...new Set(units.map(home))];
  const lost = held.filter((name) => !existsSync(join(blobs, `${name}.wasm`)));
  if (lost.length) throw new Error(`wasm: no build of ${lost.join(", ")}; run scripts/build.sh from the repo root`);
  rmSync(to, { recursive: true, force: true });
  for (const name of units) {
    const at = home(name);
    mkdirSync(join(to, name), { recursive: true });
    copyFileSync(join(unit(name), "index.js"), join(to, name, "index.js"));
    writeFileSync(join(to, name, "home.js"), `export { default } from "../blobs/${at}.js";\n`);
  }
  mkdirSync(join(to, "blobs"), { recursive: true });
  for (const name of held) {
    copyFileSync(join(blobs, `${name}.js`), join(to, "blobs", `${name}.js`));
    copyFileSync(join(blobs, `${name}.wasm`), join(to, "blobs", `${name}.wasm`));
    copyFileSync(join(blobs, `${name}.load.ts`), join(to, "blobs", `${name}.load.ts`));
  }
  return units;
}

export async function font(out = FONT) {
  const door = await import("@mrlyprod/font");
  const glyphs = Object.entries(door.map()).sort(([a], [b]) => a.codePointAt(0)! - b.codePointAt(0)!);
  const rows = glyphs.map(
    ([char, rows]) => `${JSON.stringify(char)}:${JSON.stringify({ rows, path: door.path(char) })}`,
  );
  writeFileSync(out, `{${rows.join(",")}}\n`);
}

if (import.meta.main) {
  console.log(`wasm: copied ${copy().join(" ") || "no unit"} into ${WASM}`);
  if (!process.argv.includes("--copy")) {
    await font();
    console.log("wasm: wrote kit/font/font.gen.json");
  }
}
