import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const UA =
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0 Safari/537.36";
const CSS2 = "https://fonts.googleapis.com/css2";
const OFL = "https://raw.githubusercontent.com/google/fonts/main/ofl";
const MATERIAL = "https://raw.githubusercontent.com/google/material-design-icons/master";

export type Family = {
  family: string;
  file: string;
  axes?: string;
  weight: string;
  display?: string;
  licence: string;
};

export type Fonts = { out: string; faces: Family[]; keep?: string };

export type Icons = { names: string; out: string };

/* FETCH */

async function get(url: string): Promise<Uint8Array> {
  const res = await fetch(url, { headers: { "User-Agent": UA } });
  if (!res.ok) throw new Error(`${res.status} ${res.statusText} for ${url}`);
  return new Uint8Array(await res.arrayBuffer());
}

async function text(url: string): Promise<string> {
  return new TextDecoder().decode(await get(url));
}

const query = (one: Family) => `family=${one.family.replace(/ /g, "+")}${one.axes ? `:${one.axes}` : ""}`;

const licence = (name: string) => (/^https?:/.test(name) ? name : `${OFL}/${name}/OFL.txt`);

/* PARSE */

type Face = { url: string; range: string | null };

function faces(css: string): Face[] {
  const out: Face[] = [];
  for (const [, body] of css.matchAll(/@font-face\s*\{([^}]+)\}/g)) {
    const url = /url\((\S+?)\)\s*format/.exec(body);
    if (!url) continue;
    const range = /unicode-range:\s*([^;]+);/.exec(body);
    out.push({ url: url[1], range: range ? range[1].trim() : null });
  }
  return out;
}

function symbol(svg: string): { box: string; d: string } {
  const box = /viewBox="([^"]+)"/.exec(svg);
  const paths = [...svg.matchAll(/<path d="([^"]+)"\/>/g)];
  if (!box || paths.length !== 1) throw new Error("vendor: expected one viewBox and one path");
  return { box: box[1], d: paths[0][1] };
}

function source(box: string, icons: [string, string][]): string {
  const rows = icons.map(([name, d]) => `  ${name}: '${d}',`);
  return `export const VIEWBOX = '${box}';\n\nexport const ICONS = {\n${rows.join("\n")}\n};\n`;
}

/* FACES */

function rule(family: string, file: string, weight: string, display: string, range: string | null) {
  const lines = [
    "@font-face {",
    `  font-family: "${family}";`,
    "  font-style: normal;",
    `  font-weight: ${weight};`,
    `  src: url("${file}") format("woff2");`,
    `  font-display: ${display};`,
  ];
  if (range) lines.push(`  unicode-range: ${range};`);
  lines.push("}");
  return lines.join("\n");
}

/* MAIN */

export async function main(root: string) {
  const config = JSON.parse(await readFile(join(root, "site.json"), "utf8")) as { fonts?: Fonts; icons?: Icons };
  const { fonts, icons } = config;
  if (!fonts || !icons) throw new Error("vendor: site.json needs a fonts and an icons block");
  const out = resolve(root, fonts.out);
  await mkdir(out, { recursive: true });

  const sizes: [string, number][] = [];
  const save = async (file: string, data: Uint8Array | string) => {
    const body = typeof data === "string" ? new TextEncoder().encode(data) : data;
    await writeFile(file, body);
    sizes.push([file.slice(root.length + 1), body.byteLength]);
  };
  const font = async (name: string, url: string) => {
    const data = await get(url);
    const tag = String.fromCharCode(data[0], data[1], data[2], data[3]);
    if (tag !== "wOF2") throw new Error(`${name}: expected wOF2 magic, got ${JSON.stringify(tag)}`);
    await save(join(out, name), data);
  };

  const sheet: string[] = [];
  const licences: [string, string][] = [];

  for (const one of fonts.faces) {
    const display = one.display ?? "swap";
    for (const [i, shard] of faces(await text(`${CSS2}?${query(one)}`)).entries()) {
      const name = `${one.file}.${i}.woff2`;
      await font(name, shard.url);
      sheet.push(rule(one.family, name, one.weight, display, shard.range));
    }
    licences.push([`LICENSE-${one.file}.txt`, licence(one.licence)]);
  }

  const kept = fonts.keep ? (await readFile(resolve(root, fonts.keep), "utf8")).trim() : "";
  await save(join(out, "fonts.css"), sheet.join("\n\n") + (kept ? `\n\n${kept}` : "") + "\n");

  const names = JSON.parse(await readFile(resolve(root, icons.names), "utf8")) as string[];
  const drawn: [string, string][] = [];
  let box = "";
  for (const name of names) {
    const one = symbol(await text(`${MATERIAL}/symbols/web/${name}/materialsymbolsoutlined/${name}_24px.svg`));
    if (box && one.box !== box) throw new Error(`vendor: ${name} has viewBox ${one.box}, not ${box}`);
    box = one.box;
    drawn.push([name, one.d]);
  }
  await save(resolve(root, icons.out), source(box, drawn));
  licences.push(["LICENSE-icons.txt", `${MATERIAL}/LICENSE`]);

  for (const [name, url] of licences) await save(join(out, name), await get(url));

  const width = Math.max(...sizes.map(([name]) => name.length));
  for (const [name, n] of sizes) console.log(`${name.padEnd(width)}  ${n} bytes`);
}
