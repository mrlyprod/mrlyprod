import { readdirSync, readFileSync, realpathSync } from "node:fs";
import { cpus } from "node:os";
import { join, resolve } from "node:path";
import { ink, raster, type Ink, type Pen } from "mrlyjs/view";
import { dark, light } from "../site/kit/theme/theme.js";

/* RECIPE */

const HERE = import.meta.dir;
const MRLYJS = realpathSync(resolve(HERE, "../pkgs/mrlyjs"));
const SIZE = [1024, 1024];
const WARM = 3;
const GATE = { warm: 150, cold: 500 };
const FREE = new Set(["site-og", "site-icon"]);
const THEMES = { dark, light };

type Theme = keyof typeof THEMES;
type Unit = { initSync: (options: { module: Uint8Array }) => unknown };
type Figure = { default: (pen: Pen, ink: Ink, t: number) => void; size?: number[]; still?: number; units?: Record<string, Unit> };
type Job = { name: string; theme: Theme };
type Done = { cold: number; warm: number; units: string[] } | { error: string };

const isFigure = (file: string) => file.endsWith(".ts") && !file.endsWith(".test.ts") && !file.endsWith(".d.ts") && file !== "time.ts";

/* WORKER */

async function draw({ name, theme }: Job): Promise<Done> {
  try {
    const figure: Figure = await import(join(HERE, `${name}.ts`));
    const units = Object.entries(figure.units ?? {});
    for (const [unit, door] of units) door.initSync({ module: readFileSync(join(MRLYJS, "pkg", unit, `mrlyjs_${unit}_bg.wasm`)) });
    const [width, height] = figure.size ?? SIZE;
    const tint = ink(THEMES[theme]);
    const once = () => {
      const pen = raster(width, height, tint.ground);
      const start = performance.now();
      figure.default(pen, tint, figure.still ?? 0);
      return performance.now() - start;
    };
    const cold = once();
    const warm = Array.from({ length: WARM }, once).sort((a, b) => a - b)[WARM >> 1];
    return { cold, warm, units: units.map(([unit]) => unit) };
  } catch (error) {
    return { error: (error instanceof Error ? error.message : String(error)).split("\n")[0] };
  }
}

function serve() {
  self.onmessage = async ({ data }: MessageEvent<Job>) => postMessage(await draw(data));
}

/* MAIN */

function run(job: Job) {
  return new Promise<Done>((done) => {
    const worker = new Worker(import.meta.url);
    worker.onmessage = ({ data }: MessageEvent<Done>) => {
      worker.terminate();
      done(data);
    };
    worker.onerror = (event) => {
      worker.terminate();
      done({ error: event.message || "the worker died" });
    };
    worker.postMessage(job);
  });
}

const ms = (value: number) => value.toFixed(1).padStart(9);

async function main() {
  const wanted = process.argv.slice(2).map((arg) => arg.replace(/\.ts$/, ""));
  const names = readdirSync(HERE).filter(isFigure).map((file) => file.slice(0, -3)).filter((name) => !wanted.length || wanted.includes(name)).sort();
  const rows = [];
  for (const name of names) {
    const row = { name, cold: 0, warm: 0, units: [] as string[], error: "" };
    for (const theme of Object.keys(THEMES) as Theme[]) {
      const done = await run({ name, theme });
      if ("error" in done) row.error ||= `${theme}: ${done.error}`;
      else {
        row.cold = Math.max(row.cold, done.cold);
        row.warm = Math.max(row.warm, done.warm);
        row.units = done.units;
      }
    }
    rows.push(row);
  }
  rows.sort((a, b) => b.warm - a.warm || a.name.localeCompare(b.name));
  const flag = (row: (typeof rows)[number]) => (row.error ? `FAILED ${row.error}` : FREE.has(row.name) ? "excluded" : [row.warm > GATE.warm && "over warm", row.cold > GATE.cold && "over cold"].filter(Boolean).join(", "));
  const width = Math.max(...rows.map((row) => row.name.length));
  const over = rows.filter((row) => !FREE.has(row.name) && (row.error || row.warm > GATE.warm || row.cold > GATE.cold));
  const lines = [
    `figures/time.ts, bun ${Bun.version}, ${cpus()[0]?.model ?? "cpu"}`,
    "- The raster pen in Bun is a proxy for the browser's canvas() pen: same figures, same kit, a different paint.",
    `- cold: the first draw in a fresh worker; warm: the median of ${WARM} draws after it; each the slower of dark and light; pen build and unit init not timed.`,
    `- gate: ${GATE.warm} ms warm, ${GATE.cold} ms cold; site-og and site-icon excluded.`,
    "",
    `${"name".padEnd(width)}  ${"cold ms".padStart(9)}  ${"warm ms".padStart(9)}  ${"units".padEnd(6)}  flag`,
    ...rows.map((row) => `${row.name.padEnd(width)}  ${ms(row.cold)}  ${ms(row.warm)}  ${(row.units.join(",") || "-").padEnd(6)}  ${flag(row)}`.trimEnd()),
    "",
    `${rows.length} figures, ${over.length} over the gate`,
  ];
  console.log(lines.join("\n"));
  process.exit(over.length ? 1 : 0);
}

if (!Bun.isMainThread) serve();
else if (import.meta.main) await main();
