import { readdirSync, readFileSync } from "node:fs";

const BINDINGS = new URL("../../bindings/js/", import.meta.url);

for (const unit of readdirSync(new URL("units", BINDINGS)).filter((unit) => unit !== "all")) {
  const door = await import(`@mrlyprod/mrly/${unit}`);
  door.initSync({ module: readFileSync(new URL(`pkg/${unit}/mrly_js_${unit}_bg.wasm`, BINDINGS)) });
}
