import { copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { live } from "./build.ts";

/* PATHS */

const site = resolve(import.meta.dir, "..");
const repo = resolve(site, "..");
const PKG = join(repo, "bindings", "js", "pkg");
const WASM = join(site, "wasm");
const FONT = join(site, "src", "lib", "font", "font.gen.json");

const run = (cmd: string[]) => {
  const done = Bun.spawnSync(cmd, { cwd: repo, stdio: ["inherit", "inherit", "inherit"] });
  if (done.exitCode !== 0) throw new Error(`wasm: ${cmd.join(" ")} exited ${done.exitCode}`);
};

/* COPY */

export function copy(units = live(), from = PKG, to = WASM): string[] {
  const lost = units.filter((unit) => !existsSync(join(from, unit, `mrly_js_${unit}_bg.wasm`)));
  if (lost.length) throw new Error(`wasm: ${from} holds no ${lost.join(", ")}; scripts/wasm.sh builds it`);
  rmSync(to, { recursive: true, force: true });
  for (const unit of units) {
    mkdirSync(join(to, unit), { recursive: true });
    for (const name of [`mrly_js_${unit}.js`, `mrly_js_${unit}_bg.wasm`])
      copyFileSync(join(from, unit, name), join(to, unit, name));
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
  console.log(`wasm: copied ${copy().join(" ")} into site/wasm`);
  font();
  console.log("wasm: wrote src/lib/font/font.gen.json");
}
