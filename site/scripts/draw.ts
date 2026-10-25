import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";
import { crc32, deflateSync } from "node:zlib";
import SITE from "../site.json";
import { inked, SIZE } from "../src/lib/figure.ts";
import { palette } from "../src/lib/theme/palette.gen.js";
import { type Color, type Ink, type Pen, raster, svg } from "../src/lib/view/index.ts";
import { mrly, plates, roster } from "./build.ts";

/* PATHS */

const org = resolve(import.meta.dir, "..");
const repo = resolve(org, "..");
const MRLY = join(repo, "bindings", "js");
const DATA_DIR = join(repo, "data", relative(repo, import.meta.dir));
const SHEET = readFileSync(join(org, "src", "styles", "palette.gen.css"), "utf8");
const THEMES = ["dark", "light"] as const;
const KINDS = ["svg", "png"] as const;
const FLAGS = ["--all", "--svg", "--png", "--theme", "--size", "--out"];
const VALUED = ["--theme", "--size", "--out"];
const USAGE = "usage: bun run draw <name...> | --all [--svg] [--png] [--theme dark|light] [--size px] [--out dir]";

/* TYPES */

type Theme = (typeof THEMES)[number];
type Kind = (typeof KINDS)[number];
type Unit = { initSync: (options: { module: WebAssembly.Module }) => unknown };
type Figure = {
  default: (pen: Pen, ink: Ink, t: number) => void;
  size?: number[];
  still?: number;
  units?: Record<string, Unit>;
};
type Pixels = { shape: [number, number]; colors: Uint8Array | Uint8ClampedArray };
export type Ask = { names: string[]; all?: boolean; kinds: Kind[]; themes: Theme[]; size: number | null; out: string };

/* INK */

function tint(theme: Theme): Ink {
  const links = [...SHEET.matchAll(new RegExp(`--${SITE.tint}-link: (#[0-9a-f]{6})`, "g"))];
  return inked(theme, palette[SITE.tint as keyof typeof palette], links.at(theme === "dark" ? -1 : 0)?.[1]);
}

/* PNG */

const SIGNATURE = Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10]);

function chunk(type: string, data: Uint8Array) {
  const out = new Uint8Array(data.length + 12);
  const view = new DataView(out.buffer);
  view.setUint32(0, data.length);
  out.set(new TextEncoder().encode(type), 4);
  out.set(data, 8);
  view.setUint32(data.length + 8, crc32(out.subarray(4, data.length + 8)));
  return out;
}

function png({ shape: [height, width], colors }: Pixels) {
  const line = width * 4 + 1;
  const rows = new Uint8Array(line * height);
  for (let y = 0; y < height; y++) rows.set(colors.subarray(y * width * 4, (y + 1) * width * 4), y * line + 1);
  const head = new Uint8Array(13);
  const view = new DataView(head.buffer);
  view.setUint32(0, width);
  view.setUint32(4, height);
  head.set([8, 6], 8);
  return Buffer.concat([
    SIGNATURE,
    chunk("IHDR", head),
    chunk("IDAT", deflateSync(rows)),
    chunk("IEND", new Uint8Array()),
  ]);
}

function straight({ shape, colors }: Pixels): Pixels {
  const out = Uint8ClampedArray.from(colors);
  for (let i = 0; i < out.length; i += 4) {
    const a = out[i + 3]!;
    if (a > 0 && a < 255) for (let k = i; k < i + 3; k++) out[k] = (out[k]! * 255) / a;
  }
  return { shape, colors: out };
}

/* LOAD */

const compiled = new Map<string, WebAssembly.Module>();

function wasm(unit: string) {
  if (!compiled.has(unit))
    compiled.set(unit, new WebAssembly.Module(readFileSync(join(MRLY, "pkg", unit, `mrly_js_${unit}_bg.wasm`))));
  return compiled.get(unit)!;
}

async function load(files: string[]): Promise<Figure[]> {
  const out = join(tmpdir(), `draw-${process.pid}`);
  try {
    const built = await Bun.build({ entrypoints: files, root: repo, outdir: out, target: "bun", plugins: [mrly] });
    if (!built.success) throw new Error(`draw: the figures failed to bundle\n${built.logs.join("\n")}`);
    const figures: Figure[] = [];
    for (const file of files) {
      const figure: Figure = await import(join(out, relative(repo, file).replace(/\.ts$/, ".js")));
      for (const [unit, door] of Object.entries(figure.units ?? {})) door.initSync({ module: wasm(unit) });
      figures.push(figure);
    }
    return figures;
  } finally {
    rmSync(out, { recursive: true, force: true });
  }
}

/* DRAW */

async function paint(
  figure: Figure,
  kind: Kind,
  [width, height]: [number, number],
  ink: Ink,
  ground: Color | undefined,
) {
  const t = figure.still ?? 0;
  if (kind === "png") {
    const pen = raster(width, height, ground);
    figure.default(pen, ink, t);
    return png(straight(pen.pixels()));
  }
  const pen = svg(width, height, ground);
  figure.default(pen, ink, t);
  return pen.text({ png });
}

export async function* draw({ names, all, kinds, themes, size, out }: Ask) {
  const list = roster();
  const lost = names.find((name) => !list.some((one) => one.name === name));
  if (lost) throw new Error(`draw: ${lost} is no figure; the roster draws ${list.length}, as research-arcs`);
  const picked = all ? list : names.map((name) => list.find((one) => one.name === name)!);
  const clear = new Set(plates(list));
  const figures = await load(picked.map((one) => one.file));
  const failed: string[] = [];
  mkdirSync(out, { recursive: true });
  for (const [i, one] of picked.entries()) {
    const figure = figures[i]!;
    const [w, h] = (figure.size ?? SIZE) as [number, number];
    const scale = size ? size / Math.max(w, h) : 1;
    const box: [number, number] = [Math.max(1, Math.round(w * scale)), Math.max(1, Math.round(h * scale))];
    for (const theme of themes) {
      const ink = tint(theme);
      for (const kind of kinds) {
        const file = join(out, `${one.name}.${theme}.${kind}`);
        try {
          writeFileSync(file, await paint(figure, kind, box, ink, clear.has(one.name) ? undefined : ink.ground));
        } catch (error) {
          failed.push(
            `draw: ${one.name} failed in ${theme} ${kind}: ${error instanceof Error ? error.message : String(error)}`,
          );
          continue;
        }
        yield file;
      }
    }
  }
  if (failed.length) throw new Error(failed.join("\n"));
}

/* ARGS */

export function parse(argv: string[]): Ask {
  const flag = (name: string) => (argv.includes(name) ? (argv[argv.indexOf(name) + 1] ?? "") : null);
  const names = argv.filter((one, i) => !one.startsWith("--") && !VALUED.includes(argv[i - 1] ?? ""));
  const all = argv.includes("--all");
  const theme = flag("--theme") ?? "dark";
  const size = flag("--size");
  const out = flag("--out");
  const odd = argv.find((one) => one.startsWith("--") && !FLAGS.includes(one));
  if (odd) throw new Error(`draw: ${odd} is no flag; ${USAGE}`);
  if (all === names.length > 0) throw new Error(`draw: name figures or pass --all; ${USAGE}`);
  if (!THEMES.includes(theme as Theme)) throw new Error(`draw: --theme takes dark or light, not ${theme}`);
  if (size !== null && !(Number.isInteger(Number(size)) && Number(size) > 0))
    throw new Error(`draw: --size takes a whole number of pixels, not ${size}`);
  if (out !== null && (!out || out.startsWith("--")))
    throw new Error(`draw: --out takes a folder, not ${out || "nothing"}`);
  const kinds = KINDS.filter((kind) => argv.includes(`--${kind}`));
  return {
    names,
    all,
    kinds: kinds.length ? kinds : ["svg"],
    themes: [theme as Theme],
    size: size === null ? null : Number(size),
    out: resolve(out ?? join(DATA_DIR, "draw")),
  };
}

/* MAIN */

if (import.meta.main) {
  try {
    for await (const file of draw(parse(process.argv.slice(2)))) console.log(file);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
