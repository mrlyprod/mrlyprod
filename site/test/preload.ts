import { existsSync, readdirSync, readFileSync } from "node:fs";

const BINDINGS = new URL("../../bindings/js/", import.meta.url);

const units = readdirSync(new URL("units", BINDINGS)).filter((unit) => unit !== "all");
const lost = units.filter((unit) => !existsSync(new URL(`pkg/${unit}/mrly_js_${unit}_bg.wasm`, BINDINGS)));
if (lost.length) throw new Error(`bindings/js/pkg holds no ${lost.join(", ")}: run scripts/wasm.sh first`);

for (const unit of units) {
  const door = await import(`@mrlyprod/mrly/${unit}`);
  door.initSync({ module: readFileSync(new URL(`pkg/${unit}/mrly_js_${unit}_bg.wasm`, BINDINGS)) });
}
