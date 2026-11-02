import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { REQUIRED } from "../src/shell/design.ts";
import { choose, sheets } from "./design.ts";

const part = (name: string) => `${name}: () => null`;

const shelf = (designs: Record<string, string[]>) => {
  const dir = mkdtempSync(join(tmpdir(), "designs-"));
  for (const [name, parts] of Object.entries(designs)) {
    mkdirSync(join(dir, name));
    writeFileSync(
      join(dir, name, "index.ts"),
      `export const ${name} = { id: "${name}", css: [], parts: { ${parts.map(part).join(", ")} } };\n`,
    );
  }
  return dir;
};

test("the draw never picks a design that misses a part, and naming one fails with the parts it misses", async () => {
  const dir = shelf({ whole: [...REQUIRED], half: REQUIRED.filter((one) => one !== "app" && one !== "launcher") });
  try {
    const drawn = await Promise.all([0, 0.5, 0.99].map((at) => choose("", "", () => at, dir)));
    expect(drawn.map((one) => one.name)).toEqual(["whole", "whole", "whole"]);
    expect(choose("half", "", Math.random, dir)).rejects.toThrow("the design half does not draw app, launcher");
    expect((await choose("", "whole", Math.random, dir)).by).toBe("pin");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("the tokens sheet wears the skin, light on the root and dark under the scheme and the theme, and an empty skin adds nothing", () => {
  const skin = { light: { "--paper": "#fff" }, dark: { "--paper": "#000" } };
  expect(sheets("one", [], skin)["site:design/tokens.css"]).toBe(
    [
      ":root {\n  --paper: #fff;\n}",
      '@media (prefers-color-scheme: dark) {\n  :root:not([data-theme="light"]) {\n    --paper: #000;\n  }\n}',
      ':root[data-theme="dark"] {\n  --paper: #000;\n}',
    ].join("\n\n"),
  );
  expect(sheets("one", ["tokens.css"])).toEqual(sheets("one", ["tokens.css"], { light: {}, dark: {} }));
  expect(sheets("one", ["tokens.css"])["site:design/tokens.css"]).not.toContain(":root");
});
