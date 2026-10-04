import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { availableParallelism } from "node:os";
import { basename, extname, join, relative, resolve } from "node:path";
import sharp from "sharp";

/* ARGS */

const USAGE = "usage: bun diff.ts <a> <b> [--levels N], two images or two folders\n       bun diff.ts --baseline [name ...] [--levels N], the press against the Rust baseline, 2 levels by default";
const HERE = import.meta.dir;
const DESK = resolve(HERE, "../..");
const DATA_DIR = join(DESK, "data", relative(DESK, HERE));
const STORE = join(DATA_DIR, "store");
const LOCK = join(DATA_DIR, "figures.lock");
const BASELINE = join(DATA_DIR, "baseline");
const THEMES = ["dark", "light"];
const IMAGE = new Set([".png", ".webp", ".svg", ".jpg", ".jpeg", ".gif", ".avif", ".tif", ".tiff"]);

type Pair = { label: string; a: string; b: string };
type Raw = { data: Buffer; width: number; height: number };
type Row = { label: string; max: number; count: number; share: number; size: string };

function args(argv: string[]) {
  const rest: string[] = [];
  let levels: number | undefined;
  let baseline = false;
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--levels") levels = Number(argv[++i]);
    else if (argv[i] === "--baseline") baseline = true;
    else rest.push(argv[i]);
  }
  if (levels !== undefined && !(Number.isInteger(levels) && levels >= 0)) throw new Error(USAGE);
  if (baseline) return { baseline: true as const, names: [...new Set(rest.map((name) => basename(name).replace(/\.ts$/, "")))], levels: levels ?? 2 };
  if (rest.length !== 2) throw new Error(USAGE);
  return { baseline: false as const, a: resolve(rest[0]), b: resolve(rest[1]), levels: levels ?? 0 };
}

/* PAIRS */

const isImage = (name: string) => IMAGE.has(extname(name).toLowerCase());
const stem = (name: string) => name.slice(0, name.length - extname(name).length);
const images = (dir: string) => readdirSync(dir).filter(isImage).sort();

function byStem(names: string[]) {
  const found = new Map<string, string[]>();
  for (const name of names) found.set(stem(name), [...(found.get(stem(name)) ?? []), name]);
  return found;
}

function match(a: string, b: string) {
  const folders = [statSync(a).isDirectory(), statSync(b).isDirectory()];
  if (!folders[0] && !folders[1]) {
    const label = basename(a) === basename(b) ? basename(a) : `${basename(a)} ${basename(b)}`;
    return { pairs: [{ label, a, b }], lone: [] as string[] };
  }
  if (!folders[0] || !folders[1]) throw new Error(USAGE);
  const left = images(a);
  const right = images(b);
  const shared = new Set(left.filter((name) => right.includes(name)));
  const pairs: Pair[] = [...shared].map((name) => ({ label: name, a: join(a, name), b: join(b, name) }));
  const restLeft = byStem(left.filter((name) => !shared.has(name)));
  const restRight = byStem(right.filter((name) => !shared.has(name)));
  const lone: string[] = [];
  for (const [key, names] of restLeft) {
    const other = restRight.get(key);
    if (names.length === 1 && other?.length === 1) {
      pairs.push({ label: `${names[0]} ${other[0]}`, a: join(a, names[0]), b: join(b, other[0]) });
      restRight.delete(key);
    } else lone.push(...names.map((name) => `only a  ${name}`));
  }
  for (const names of restRight.values()) lone.push(...names.map((name) => `only b  ${name}`));
  return { pairs: pairs.sort((x, y) => (x.label < y.label ? -1 : 1)), lone: lone.sort() };
}

function pressed(names: string[]) {
  if (!existsSync(LOCK)) throw new Error(`${relative(DESK, LOCK)} is missing, press first`);
  const lock: Record<string, string> = JSON.parse(readFileSync(LOCK, "utf8"));
  const shelf = existsSync(BASELINE) ? readdirSync(BASELINE) : [];
  const old = new Set(shelf.filter((file) => /-(dark|light)\.png$/.test(file)).map((file) => file.replace(/-(dark|light)\.png$/, "")));
  const wanted = names.length ? names : Object.keys(lock);
  const pairs: Pair[] = [];
  const lone: string[] = [];
  for (const name of wanted) {
    if (!lock[name]) lone.push(`no press  ${name}`);
    else if (!old.has(name)) lone.push(`no baseline  ${name}`);
    else {
      for (const theme of THEMES) {
        const label = `${name}-${theme}.png`;
        const a = join(STORE, lock[name], label);
        const b = join(BASELINE, label);
        if (!existsSync(b)) lone.push(`no baseline  ${label}`);
        else if (!existsSync(a)) lone.push(`no png  ${label}, press with --png`);
        else pairs.push({ label, a, b });
      }
    }
  }
  if (!names.length) for (const name of [...old].sort()) if (!lock[name]) lone.push(`no press  ${name}`);
  return { pairs: pairs.sort((x, y) => (x.label < y.label ? -1 : 1)), lone };
}

/* PIXELS */

const isSvg = (path: string) => extname(path).toLowerCase() === ".svg";

async function decode(path: string, width?: number): Promise<Raw> {
  let options: sharp.SharpOptions = {};
  if (width && isSvg(path)) {
    const meta = await sharp(path).metadata();
    if (meta.width && meta.width !== width) options = { density: (72 * width) / meta.width };
  }
  const { data, info } = await sharp(path, options).toColourspace("srgb").ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  if (info.channels !== 4) throw new Error(`${path}: ${info.channels} channels after decode, want 4`);
  return { data, width: info.width, height: info.height };
}

async function load(a: string, b: string) {
  if (isSvg(a) && !isSvg(b)) {
    const right = await decode(b);
    return [await decode(a, right.width), right];
  }
  if (isSvg(b) && !isSvg(a)) {
    const left = await decode(a);
    return [left, await decode(b, left.width)];
  }
  return Promise.all([decode(a), decode(b)]);
}

function compare(x: Buffer, y: Buffer) {
  if (x.equals(y)) return { max: 0, count: 0 };
  let max = 0;
  let count = 0;
  for (let i = 0; i < x.length; i += 4) {
    const delta = Math.max(Math.abs(x[i] - y[i]), Math.abs(x[i + 1] - y[i + 1]), Math.abs(x[i + 2] - y[i + 2]), Math.abs(x[i + 3] - y[i + 3]));
    if (delta === 0) continue;
    count++;
    if (delta > max) max = delta;
  }
  return { max, count };
}

async function measure(pair: Pair): Promise<Row> {
  const [left, right] = await load(pair.a, pair.b);
  if (left.width !== right.width || left.height !== right.height) {
    return { label: pair.label, max: -1, count: 0, share: 0, size: `${left.width}x${left.height} ${right.width}x${right.height}` };
  }
  const { max, count } = compare(left.data, right.data);
  return { label: pair.label, max, count, share: count / (left.width * left.height), size: "" };
}

async function pool<T, R>(items: T[], run: (item: T) => Promise<R>) {
  const out: R[] = new Array(items.length);
  let next = 0;
  const lane = async () => {
    while (next < items.length) {
      const i = next++;
      out[i] = await run(items[i]);
    }
  };
  await Promise.all(Array.from({ length: Math.min(availableParallelism(), items.length) }, lane));
  return out;
}

/* MAIN */

async function main() {
  const given = args(process.argv.slice(2));
  const { levels } = given;
  const { pairs, lone } = given.baseline ? pressed(given.names) : match(given.a, given.b);
  const rows = await pool(pairs, measure);
  for (const row of rows) {
    if (row.size) console.log(`size  ${row.size}  ${row.label}`);
    else console.log(`${String(row.max).padStart(3)}  ${(row.share * 100).toFixed(4).padStart(8)}%  ${row.label}`);
  }
  for (const line of lone) console.log(line);
  const sized = rows.filter((row) => row.size).length;
  const over = rows.filter((row) => row.max > levels).length;
  const worst = rows.reduce((top, row) => (row.max > top.max ? row : top), { label: "", max: 0 });
  const at = worst.max > 0 ? ` at ${worst.label}` : "";
  console.log(`${rows.length} pairs, ${over} over ${levels} levels, ${sized} size and ${lone.length} name mismatches, worst ${worst.max}${at}`);
  process.exit(over + sized + lone.length > 0 ? 1 : 0);
}

if (import.meta.main) await main();
