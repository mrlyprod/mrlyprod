import { expect, test } from "bun:test";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { animate, cycle } from "../../pkgs/js/mrlyjs/ui/font.js";

const pkg = join(import.meta.dir, "pkg");

test("the kit's font choreography matches the crate frame for frame", async () => {
  if (!existsSync(join(pkg, "mrlyweb.js"))) return;
  const wasm = await import(join(pkg, "mrlyweb.js"));
  await wasm.default({ module_or_path: await Bun.file(join(pkg, "mrlyweb_bg.wasm")).arrayBuffer() });
  for (const text of ["MRLYPROD", "CARLOMITCHENER", "mrly.net", "(1)", "Hi 42", "A"]) {
    expect(animate(text, 1)).toEqual(JSON.parse(wasm.font_animate(text, 1)));
    expect(cycle(text, 1, 25)).toEqual(JSON.parse(wasm.font_cycle(text, 1, 25)));
  }
});
