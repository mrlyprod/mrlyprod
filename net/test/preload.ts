import { mock } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { draw, runtime } from "../hosts/web/scripts/design.ts";
import { SITE } from "../hosts/web/scripts/home.ts";

const CRATES = new URL("../../crates/", import.meta.url);

const units = readdirSync(CRATES).filter((unit) => existsSync(new URL(`${unit}/js/package.json`, CRATES)));
const wasm = (unit: string) => new URL(`${unit}/js/pkg/mrly_${unit}_js_bg.wasm`, CRATES);
const lost = units.filter((unit) => !existsSync(wasm(unit)));
if (lost.length) throw new Error(`crates/*/js/pkg holds no ${lost.join(", ")}: run scripts/wasm.sh first`);

for (const unit of units) {
  const door = await import(`@mrlyprod/${unit}`);
  door.initSync({ module: readFileSync(wasm(unit)) });
}

const drawn = await draw();
runtime(drawn.name, drawn.design.pulldown);

mock.module("site:routes", () => ({ site: SITE, rows: [] }));
