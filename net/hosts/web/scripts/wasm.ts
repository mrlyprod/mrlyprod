import { copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { live } from "./build.ts";
import { HOME, REPO, SIBLINGS, unit, wasm } from "./home.ts";

/* PATHS */

const WASM = join(HOME, "wasm");
const FONT = join(REPO, "apps", "shared", "font", "font.gen.json");

/* COPY */

export function copy(units = live(), to = WASM): string[] {
  const lost = units.filter((name) => !existsSync(wasm(name)));
  if (lost.length)
    throw new Error(
      `wasm: no js/pkg in ${lost.map((name) => `../${name}`).join(", ")}; run scripts/wasm.sh in ${lost.length > 1 ? "each" : "it"}`,
    );
  rmSync(to, { recursive: true, force: true });
  for (const name of units) {
    mkdirSync(join(to, name), { recursive: true });
    for (const file of [`mrly_${name}_js.js`, `mrly_${name}_js_bg.wasm`])
      copyFileSync(join(unit(name), "pkg", file), join(to, name, file));
  }
  return units;
}

/* FONT */

export function font(out = FONT) {
  const done = Bun.spawnSync(["cargo", "run", "-q", "--example", "book"], {
    cwd: join(SIBLINGS, "font"),
    stderr: "inherit",
  });
  if (done.exitCode !== 0) throw new Error(`wasm: the font book exited ${done.exitCode}`);
  const text = done.stdout.toString();
  JSON.parse(text);
  writeFileSync(out, text);
}

/* MAIN */

if (import.meta.main) {
  console.log(`wasm: copied ${copy().join(" ") || "no unit"} into ${WASM}`);
  if (!process.argv.includes("--copy")) {
    font();
    console.log("wasm: wrote apps/shared/font/font.gen.json");
  }
}
