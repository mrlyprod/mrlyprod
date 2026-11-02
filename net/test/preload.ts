import { mock } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { draw, runtime } from "../hosts/web/scripts/design.ts";
import { SITE, UNITS, wasm } from "../hosts/web/scripts/home.ts";

const lost = UNITS.filter((unit) => !existsSync(wasm(unit)));
if (lost.length)
  throw new Error(`no js/pkg in ${lost.map((unit) => `../${unit}`).join(", ")}: run scripts/wasm.sh there first`);

for (const unit of UNITS) {
  const door = await import(`@mrlyprod/${unit}`);
  door.initSync({ module: readFileSync(wasm(unit)) });
}

const drawn = await draw();
runtime(drawn.name, drawn.design.pulldown);

mock.module("site:routes", () => ({ site: SITE, rows: [] }));
mock.module("site:widgets", () => ({ order: ["clock", "continue"], widgets: {} }));
