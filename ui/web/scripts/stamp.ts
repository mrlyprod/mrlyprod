import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import { EMOJIS, ICONS, MAP, OPTIONAL, REQUIRED } from "../../shell/design.ts";

const TOP = resolve(import.meta.dir, "..", "..", "..");
const DESIGNS = join(TOP, "designs");

const path = (kind: keyof typeof MAP.each) => (name: string) => MAP.each[kind].replace("*", name);

const GIVES = { ...MAP.required, ...MAP.optional };
const NEEDS = [
  ...Object.keys(MAP.required),
  ...REQUIRED.map(path("parts")),
  ...ICONS.map(path("icons")),
  ...Object.keys(EMOJIS).map(path("emojis")),
];
const KNOWN = new Set([MAP.index, ...NEEDS, ...Object.keys(MAP.optional), ...OPTIONAL.map(path("parts"))]);

const walk = (dir: string, under = ""): string[] =>
  readdirSync(join(dir, under), { withFileTypes: true }).flatMap((one) => {
    if (one.name.startsWith(".") || (!under && MAP.free.includes(one.name))) return [];
    const next = join(under, one.name);
    return one.isDirectory() ? walk(dir, next) : [next];
  });

const capital = (name: string) => name[0]!.toUpperCase() + name.slice(1);

export function stamped(dir: string): string {
  const has = (file: string) => NEEDS.includes(file) || existsSync(join(dir, file));
  const gives = Object.entries(GIVES).filter(([file, fields]) => has(file) && Object.keys(fields).length);
  const parts = [...REQUIRED, ...OPTIONAL].filter((one) => has(path("parts")(one)));
  const imports = [
    ...gives.map(([file, fields]) => [file, Object.values(fields).sort().join(", ")]),
    ...parts.map((one) => [path("parts")(one), capital(one)]),
  ].sort(([a], [b]) => (a! < b! ? -1 : 1));
  return [
    'import type { Design } from "@mrlyprod/ui/shell/design.ts";',
    ...imports.map(([file, names]) => `import { ${names} } from "./${file}";`),
    "",
    "export const design: Design = {",
    `  id: ${JSON.stringify(basename(dir))},`,
    `  title: ${JSON.stringify(capital(basename(dir)))},`,
    ...gives.flatMap(([, fields]) =>
      Object.entries(fields).map(([field, name]) => `  ${field === name ? field : `${field}: ${name}`},`),
    ),
    "  parts: {",
    ...parts.map((one) => `    ${one}: ${capital(one)},`),
    "  },",
    "};",
    "",
  ].join("\n");
}

export function faults(dir: string): string[] {
  const at = relative(TOP, dir);
  const index = join(dir, MAP.index);
  return [
    ...NEEDS.filter((one) => !existsSync(join(dir, one))).map((one) => `stamp: ${at} lacks ${one}`),
    ...walk(dir)
      .filter((one) => !KNOWN.has(one))
      .map((one) => `stamp: ${at} holds ${one}, which the map does not name`),
    ...(existsSync(index) && readFileSync(index, "utf8") === stamped(dir)
      ? []
      : [`stamp: ${at}/${MAP.index} is stale; run bun run stamp in ui`]),
  ];
}

if (import.meta.main) {
  const dirs = readdirSync(DESIGNS, { withFileTypes: true })
    .filter((one) => one.isDirectory() && one.name !== "node_modules")
    .map((one) => join(DESIGNS, one.name))
    .sort();
  if (!process.argv.includes("--check")) for (const dir of dirs) writeFileSync(join(dir, MAP.index), stamped(dir));
  const bad = dirs.flatMap(faults);
  if (bad.length) {
    console.error(bad.join("\n"));
    process.exit(1);
  }
  console.log(`stamp: ${dirs.map((one) => basename(one)).join(" ")} keep to the map`);
}
