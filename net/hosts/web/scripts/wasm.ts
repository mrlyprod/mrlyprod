import { copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { live } from "./build.ts";
import { HOME, REPO } from "./home.ts";

/* PATHS */

const repo = REPO;
const CRATES = join(repo, "crates");
const WASM = join(HOME, "wasm");
const FONT = join(repo, "apps", "shared", "font", "font.gen.json");

const run = (cmd: string[]) => {
  const done = Bun.spawnSync(cmd, { cwd: repo, stdio: ["inherit", "inherit", "inherit"] });
  if (done.exitCode !== 0) throw new Error(`wasm: ${cmd.join(" ")} exited ${done.exitCode}`);
};

/* COPY */

const pkg = (from: string, unit: string) => join(from, unit, "js", "pkg");

export function copy(units = live(), from = CRATES, to = WASM): string[] {
  const lost = units.filter((unit) => !existsSync(join(pkg(from, unit), `mrly_${unit}_js_bg.wasm`)));
  if (lost.length) throw new Error(`wasm: ${from}/*/js/pkg holds no ${lost.join(", ")}; scripts/wasm.sh builds it`);
  rmSync(to, { recursive: true, force: true });
  for (const unit of units) {
    mkdirSync(join(to, unit), { recursive: true });
    for (const name of [`mrly_${unit}_js.js`, `mrly_${unit}_js_bg.wasm`])
      copyFileSync(join(pkg(from, unit), name), join(to, unit, name));
  }
  return units;
}

/* FONT */

export function font(out = FONT) {
  const done = Bun.spawnSync(
    ["bash", join(repo, "scripts", "cargo.sh"), "cargo", "run", "-q", "-p", "mrly-font", "--example", "book"],
    { cwd: repo, stderr: "inherit" },
  );
  if (done.exitCode !== 0) throw new Error(`wasm: the font book exited ${done.exitCode}`);
  const text = done.stdout.toString();
  JSON.parse(text);
  writeFileSync(out, text);
}

/* MAIN */

if (import.meta.main) {
  if (!process.argv.includes("--no-build")) run(["bash", join(repo, "scripts", "wasm.sh")]);
  console.log(`wasm: copied ${copy().join(" ")} into ${WASM}`);
  font();
  console.log("wasm: wrote apps/shared/font/font.gen.json");
}
