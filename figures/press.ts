import { createHash, randomBytes } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { availableParallelism } from "node:os";
import { basename, dirname, join, relative, resolve } from "node:path";
import sharp from "sharp";
import { ink, raster, svg, type Ink, type Palette, type Pen, type Pixels } from "mrlyjs/view";

/* WHERE */

const USAGE = "usage: bun press.ts [name ...] [--svg] [--png]; PRESS_BUDGET=<ms> lifts the draw budget";
const HERE = import.meta.dir;
const DESK = resolve(HERE, "../..");
const MRLYJS = resolve(HERE, "../pkgs/mrlyjs");
const DATA_DIR = join(DESK, "data", relative(DESK, HERE));
const STORE = join(DATA_DIR, "store");
const LOCK = join(DATA_DIR, "figures.lock");
const BENCH = join(DATA_DIR, "press.txt");
const MUTEX = `${LOCK}.d`;

/* RECIPE */

const NOT_FIGURES = new Set(["press.ts", "diff.ts"]);
const isFigure = (file: string) => file.endsWith(".ts") && !file.endsWith(".test.ts") && !file.endsWith(".d.ts") && !NOT_FIGURES.has(file);
const KIT = /^(pkg\/.+\.(js|wasm)|(view\/)?[^/]+\.js)$/;
const THEMES = ["dark", "light"] as const;
const SIZE = [1024, 1024];
const BUDGET = Number(process.env.PRESS_BUDGET) > 0 ? Number(process.env.PRESS_BUDGET) : 5000;
const WIDTH = 1024;
const WEBP = { quality: 90, effort: 6 };
const PNG = { compressionLevel: 9 };

type Theme = (typeof THEMES)[number];
type Flags = { svg: boolean; png: boolean };
type Job = { name: string; file: string; dir: string; missing: string[]; palettes: Record<Theme, Palette> };
type Row = { state: "drawn" | "cached" | "failed"; name: string; theme: string; draw?: number; encode?: number; extra?: number; note?: string };
type Done = { rows: Row[]; ms: number };
type Unit = { initSync: (options: { module: Uint8Array }) => unknown };
type Figure = { default: (pen: Pen, ink: Ink, t: number) => void; size?: number[]; still?: number; units?: Record<string, Unit> };

const reason = (error: unknown) => (error instanceof Error ? error.message : String(error)).split("\n")[0];

/* KEY */

const sha = (data: string | Uint8Array) => createHash("sha256").update(data).digest("hex");
const scanner = new Bun.Transpiler({ loader: "ts" });

function digest(files: string[], root: string) {
  const lines = files.map((file) => `${sha(readFileSync(file))}  ${relative(root, file)}\n`);
  return sha(lines.sort().join(""));
}

function kit() {
  const files = (readdirSync(MRLYJS, { recursive: true }) as string[]).filter((file) => KIT.test(file) && !file.endsWith(".test.js"));
  return digest(files.map((file) => join(MRLYJS, file)), MRLYJS);
}

function sources(file: string, seen = new Set<string>()) {
  if (seen.has(file)) return seen;
  seen.add(file);
  if (!/\.[cm]?[jt]s$/.test(file)) return seen;
  for (const { path } of scanner.scanImports(readFileSync(file, "utf8"))) {
    if (path.startsWith(".")) sources(Bun.resolveSync(path, dirname(file)), seen);
  }
  return seen;
}

const key = (base: string, name: string, file: string) => sha(`${base}\n${name}\n${digest([...sources(file)], HERE)}`);

/* WORKER */

const pixels = (p: Pixels) => sharp(p.colors, { raw: { width: p.shape[1], height: p.shape[0], channels: 4 } });

async function clock(work: () => Promise<void>) {
  const start = performance.now();
  await work();
  return performance.now() - start;
}

async function press(job: Job, ready: Set<string>) {
  const rows: Row[] = [];
  let theme = "-";
  try {
    const figure: Figure = await import(job.file);
    if (typeof figure.default !== "function") throw new Error("no default draw");
    for (const [unit, door] of Object.entries(figure.units ?? {})) {
      if (ready.has(unit)) continue;
      door.initSync({ module: readFileSync(join(MRLYJS, "pkg", unit, `mrlyjs_${unit}_bg.wasm`)) });
      ready.add(unit);
    }
    const [width, height] = figure.size ?? SIZE;
    const missing = new Set(job.missing);
    mkdirSync(job.dir, { recursive: true });
    const write = (file: string, bytes: Uint8Array | string) => {
      const tmp = join(job.dir, `${file}.${randomBytes(6).toString("hex")}.tmp`);
      writeFileSync(tmp, bytes);
      renameSync(tmp, join(job.dir, file));
    };
    const draw = (pen: Pen, tint: Ink) => {
      postMessage("draw");
      const start = performance.now();
      figure.default(pen, tint, figure.still ?? 0);
      const ms = performance.now() - start;
      postMessage("drawn");
      if (ms > BUDGET) throw new Error(`draw took ${ms.toFixed(0)} ms, over ${BUDGET}`);
      return ms;
    };
    for (theme of THEMES) {
      const tint = ink(job.palettes[theme as Theme]);
      const row = { state: "drawn" as const, name: job.name, theme, draw: undefined as number | undefined, encode: 0, extra: 0 };
      const webp = `${job.name}-${theme}.webp`;
      const png = `${job.name}-${theme}.png`;
      const vector = `${job.name}-${theme}.svg`;
      const og = theme === "dark" && missing.has(`${job.name}.png`);
      if (missing.has(webp) || missing.has(png) || og) {
        const pen = raster(width, height, tint.ground);
        row.draw = draw(pen, tint);
        if (missing.has(webp)) {
          row.encode += await clock(async () => {
            const image = pixels(pen.pixels());
            write(webp, await (width > WIDTH ? image.resize({ width: WIDTH, kernel: "lanczos3" }) : image).webp(WEBP).toBuffer());
          });
        }
        if (og || missing.has(png)) {
          row[og ? "encode" : "extra"] += await clock(async () => {
            const bytes = await pixels(pen.pixels()).png(PNG).toBuffer();
            if (og) write(`${job.name}.png`, bytes);
            if (missing.has(png)) write(png, bytes);
          });
        }
      }
      if (missing.has(vector)) {
        row.extra += await clock(async () => {
          const pen = svg(width, height, tint.ground);
          draw(pen, tint);
          write(vector, await pen.text({ png: (p) => pixels(p).png().toBuffer() }));
        });
      }
      rows.push(row);
    }
  } catch (error) {
    rows.push({ state: "failed", name: job.name, theme, note: reason(error) });
  }
  return rows;
}

function serve() {
  const ready = new Set<string>();
  self.onmessage = async ({ data }: MessageEvent<Job>) => {
    const start = performance.now();
    const rows = await press(data, ready);
    postMessage({ rows, ms: performance.now() - start });
  };
}

/* CREW */

function run(worker: Worker, job: Job) {
  return new Promise<Done | string>((done) => {
    let timer: ReturnType<typeof setTimeout>;
    const arm = () => {
      clearTimeout(timer);
      timer = setTimeout(() => done(`over ${BUDGET} ms`), BUDGET);
    };
    worker.onmessage = ({ data }: MessageEvent<"draw" | "drawn" | Done>) => {
      if (data === "draw") return arm();
      clearTimeout(timer);
      if (data !== "drawn") done(data);
    };
    worker.onerror = (event) => {
      clearTimeout(timer);
      done(reason(event.message || "the worker died"));
    };
    arm();
    worker.postMessage(job);
  });
}

function crew(jobs: Job[], cores: number, note: (done: Done) => void) {
  let next = 0;
  const lane = async () => {
    let worker = new Worker(import.meta.url);
    while (next < jobs.length) {
      const job = jobs[next++];
      const done = await run(worker, job);
      if (typeof done !== "string") note(done);
      else {
        worker.terminate();
        worker = new Worker(import.meta.url);
        note({ rows: [{ state: "failed", name: job.name, theme: "-", note: done }], ms: 0 });
      }
    }
    worker.terminate();
  };
  return Promise.all(Array.from({ length: Math.min(cores, jobs.length) }, lane));
}

/* LOCK */

const atomic = (path: string, text: string) => {
  const tmp = `${path}.${randomBytes(6).toString("hex")}.tmp`;
  writeFileSync(tmp, text);
  renameSync(tmp, path);
};

async function exclusive<T>(work: () => T) {
  mkdirSync(DATA_DIR, { recursive: true });
  const give = Date.now() + 20000;
  for (;;) {
    try {
      mkdirSync(MUTEX);
      break;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      if (Date.now() > give) throw new Error(`${MUTEX} is held by another press`);
      try {
        if (Date.now() - statSync(MUTEX).mtimeMs > 10000) rmSync(MUTEX, { recursive: true, force: true });
      } catch {}
      await Bun.sleep(10 + Math.random() * 20);
    }
  }
  try {
    return work();
  } finally {
    rmSync(MUTEX, { recursive: true, force: true });
  }
}

function readLock(): Record<string, string> {
  if (!existsSync(LOCK)) return {};
  try {
    return JSON.parse(readFileSync(LOCK, "utf8"));
  } catch {
    throw new Error(`${LOCK} is not JSON`);
  }
}

function settle(keys: Record<string, string>, failed: Set<string>, all: string[], full: boolean) {
  const lock = readLock();
  if (full) for (const name of Object.keys(lock)) if (!all.includes(name)) delete lock[name];
  for (const [name, hash] of Object.entries(keys)) {
    if (!failed.has(name)) lock[name] = hash;
    else if (full) delete lock[name];
  }
  atomic(LOCK, `${JSON.stringify(Object.fromEntries(Object.entries(lock).sort()), null, 2)}\n`);
  if (full && existsSync(STORE)) {
    const held = new Set(Object.values(lock));
    for (const dir of readdirSync(STORE)) {
      const path = join(STORE, dir);
      if (!held.has(dir) && statSync(path).mtimeMs < performance.timeOrigin) rmSync(path, { recursive: true, force: true });
    }
  }
}

/* MAIN */

function args(argv: string[]) {
  const flags: Flags = { svg: false, png: false };
  const names = new Set<string>();
  for (const arg of argv) {
    if (arg === "--svg") flags.svg = true;
    else if (arg === "--png") flags.png = true;
    else if (arg.startsWith("-")) throw new Error(USAGE);
    else names.add(basename(arg).replace(/\.ts$/, ""));
  }
  return { flags, names: [...names] };
}

function outputs(name: string, flags: Flags) {
  const out = [`${name}-dark.webp`, `${name}-light.webp`, `${name}.png`];
  if (flags.svg) out.push(`${name}-dark.svg`, `${name}-light.svg`);
  if (flags.png) out.push(`${name}-dark.png`, `${name}-light.png`);
  return out;
}

const ms = (value?: number) => (value === undefined ? "-" : value.toFixed(1)).padStart(8);

function line(row: Row) {
  const extra = row.extra ? `  extra ${row.extra.toFixed(1)}` : "";
  return `${row.state.padEnd(6)}  ${ms(row.draw)}  ${ms(row.encode)}  ${row.name}  ${row.theme}${extra}${row.note ? `  ${row.note}` : ""}`;
}

async function main() {
  const { flags, names } = args(process.argv.slice(2));
  const cores = availableParallelism();
  const all = readdirSync(HERE).filter(isFigure).map((file) => file.slice(0, -3)).sort();
  const core = await import("mrlyjs/core");
  core.initSync({ module: readFileSync(join(MRLYJS, "pkg/core/mrlyjs_core_bg.wasm")) });
  const palettes: Record<Theme, Palette> = { dark: core.colors.DARK().toJSON(), light: core.colors.LIGHT().toJSON() };
  const base = sha(JSON.stringify([sha(readFileSync(import.meta.path)), kit(), palettes]));
  const rows: Row[] = [];
  const keys: Record<string, string> = {};
  const jobs: Job[] = [];
  let work = 0;
  const note = (row: Row) => {
    rows.push(row);
    console.log(line(row));
  };
  const wanted = names.length ? names : all;
  for (const name of wanted) {
    try {
      if (!all.includes(name)) throw new Error("no such figure");
      const file = join(HERE, `${name}.ts`);
      keys[name] = key(base, name, file);
      const dir = join(STORE, keys[name]);
      const missing = outputs(name, flags).filter((out) => !existsSync(join(dir, out)));
      if (missing.length) jobs.push({ name, file, dir, missing, palettes });
      else for (const theme of THEMES) note({ state: "cached", name, theme });
    } catch (error) {
      note({ state: "failed", name, theme: "-", note: reason(error) });
    }
  }
  await crew(jobs, cores, (done) => {
    work += done.ms;
    done.rows.forEach(note);
  });
  const named = (state: Row["state"]) => new Set(rows.filter((row) => row.state === state).map((row) => row.name));
  const failed = named("failed");
  const drawn = [...named("drawn")].filter((name) => !failed.has(name));
  const full = names.length === 0;
  const last = `${wanted.length} figures: ${drawn.length} drawn, ${named("cached").size} cached, ${failed.size} failed, wall ${performance.now().toFixed(0)} ms, work ${work.toFixed(0)} ms, ${cores} cores`;
  console.log(last);
  await exclusive(() => settle(keys, failed, all, full));
  if (full) {
    const sorted = [...rows].sort((a, b) => (a.name + a.theme < b.name + b.theme ? -1 : 1));
    atomic(BENCH, `${[...sorted.map(line), last].join("\n")}\n`);
  }
  process.exit(failed.size ? 1 : 0);
}

if (!Bun.isMainThread) serve();
else if (import.meta.main) await main();
