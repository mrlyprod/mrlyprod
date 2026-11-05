import { readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { cells, type Hue, hue } from "../site/glyph.ts";
import { PAGES } from "./site.ts";

const HEX: Record<Hue, string> = {
  red: "#ff3d40",
  orange: "#ff8f2c",
  yellow: "#ffd100",
  green: "#32cc58",
  mint: "#00d1bb",
  teal: "#00cad8",
  cyan: "#1ec9f3",
  blue: "#008cff",
  indigo: "#6768fa",
  purple: "#d332e9",
  pink: "#ff325a",
  brown: "#b18462",
  gray: "#8e8e93",
};

export function icon(route: string): string {
  const unit = 96;
  const start = 512 - unit * 2.5;
  const rects = cells(route).flatMap((row, r) =>
    row.flatMap((on, c) =>
      on
        ? [
            `<rect x="${start + c * unit + 4}" y="${start + r * unit + 4}" width="${unit - 8}" height="${unit - 8}" rx="16" fill="#fff"/>`,
          ]
        : [],
    ),
  );
  return [
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">',
    `  <rect class="back" width="1024" height="1024" fill="${HEX[hue(route)]}"/>`,
    ...rects.map((one) => `  ${one}`),
    "</svg>",
    "",
  ].join("\n");
}

export const route = (dir: string) => {
  const data = JSON.parse(readFileSync(join(dir, "app.json"), "utf8")) as { route?: string };
  return data.route ?? (basename(dir) === "home" ? "/" : `/${basename(dir)}/`);
};

if (import.meta.main) {
  const glob = new Bun.Glob("**/app.json");
  for (const file of glob.scanSync(PAGES)) {
    const dir = dirname(join(PAGES, file));
    if (JSON.parse(readFileSync(join(dir, "app.json"), "utf8")).hidden) continue;
    writeFileSync(join(dir, "icon.svg"), icon(route(dir)));
  }
}
