import { existsSync, mkdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { main } from "../kit/shots.ts";
import { palette } from "../kit/theme/palette.js";
import SITE from "../site.json";
import { SIZE, sheet } from "./film/page.js";

const root = resolve(import.meta.dir, "..");
const desk = resolve(root, "../..");
const DATA_DIR = join(desk, "data", relative(desk, import.meta.dir));
const PAGE = join(import.meta.dir, "film", "page.js");
const FPS = 30;
const LONGEST = 30;
const TALL = 6000;
const VALUED = ["--name", "--seed", "--opts", "--cues", "--from", "--step", "--frames"];

/* ARGS */

function cues(text: string): [number, string][] {
  if (!text) return [];
  return text.split(",").map((one) => {
    const [at = "", name = ""] = one.split(":");
    const t = Number(at);
    if (!at.trim() || !Number.isFinite(t) || !/^\w+$/.test(name)) throw new Error(`film: --cues takes t:name pairs, as 600:trigger,4900:exit, not ${one}`);
    return [t, name];
  });
}

function parse(argv: string[]) {
  const flag = (name: string, fallback = "") => (argv.includes(name) ? (argv[argv.indexOf(name) + 1] ?? "") : fallback);
  const scene = argv.find((one, i) => !one.startsWith("--") && !VALUED.includes(argv[i - 1] ?? ""));
  const name = flag("--name");
  const numbers = { seed: Number(flag("--seed", "7")), from: Number(flag("--from", "0")), step: Number(flag("--step", "100")), frames: Number(flag("--frames", "60")) };
  if (!scene) throw new Error("film: name a scene module, as apps/lightspeed/scene.js");
  if (!/^[a-z0-9-]+$/.test(name)) throw new Error("film: --name takes lowercase letters, digits and hyphens");
  if (!Number.isInteger(numbers.seed) || numbers.seed < 0) throw new Error("film: --seed takes a whole number");
  if (!Number.isFinite(numbers.from) || !(numbers.step > 0) || !Number.isInteger(numbers.frames) || numbers.frames < 1) throw new Error("film: --from, --step and --frames take numbers, step above 0 and at least one frame");
  return { scene, name, ...numbers, opts: JSON.parse(flag("--opts", "{}")) as Record<string, unknown>, cues: cues(flag("--cues")), video: argv.includes("--video") };
}

/* PAGE */

const html = (name: string, config: unknown) =>
  `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>film ${name}</title>
<link rel="icon" href="data:,">
<style>html, body { margin: 0; background: #000; } canvas { display: block; }</style>
<script type="application/json" id="film">${JSON.stringify(config).replace(/</g, "\\u003c")}</script>
<script type="module" src="/${name}/film.js"></script>
</head>
<body></body>
</html>
`;

async function pack(scene: string, out: string) {
  const entry = join(out, "entry.js");
  writeFileSync(entry, `import { make } from ${JSON.stringify(scene)};\nimport { film } from ${JSON.stringify(PAGE)};\nwindow.film = film(make);\n`);
  const built = await Bun.build({ entrypoints: [entry], target: "browser", minify: true });
  if (!built.success) throw new Error(`film: the page failed to bundle\n${built.logs.join("\n")}`);
  await Bun.write(join(out, "film.js"), built.outputs[0]!);
}

/* RUN */

if (import.meta.main) {
  const run = parse(process.argv.slice(2));
  const scene = resolve(process.cwd(), run.scene);
  if (!existsSync(scene)) throw new Error(`film: no scene at ${scene}`);
  if (run.video && run.frames / FPS > LONGEST) throw new Error(`film: --video takes at most ${LONGEST * FPS} frames, ${LONGEST} s`);
  const films = process.env.MRLY_DIST ? `${resolve(process.env.MRLY_DIST)}-film` : join(DATA_DIR, "film");
  const out = join(films, run.name);
  mkdirSync(out, { recursive: true });
  const hues = palette as Record<string, string>;
  const look = { paper: hues.black, accent: hues[String(run.opts.tint ?? "")] ?? hues[SITE.tint] };
  const shared = { name: run.name, seed: run.seed, opts: run.opts, cues: run.cues, from: run.from, frames: run.frames, look };
  writeFileSync(join(out, "index.html"), html(run.name, run.video ? { ...shared, video: true, fps: FPS } : { ...shared, step: run.step }));
  await pack(scene, out);
  const [w, h] = run.video ? SIZE : sheet(run.frames);
  console.log(`film: ${run.name} ${run.video ? `${run.frames} frames at ${FPS} fps, ${w}x${h} webm` : `${run.frames} frames every ${run.step} ms from ${run.from}, sheet ${w}x${h}`} in ${out}`);
  const latest = join(DATA_DIR, "shots", "latest");
  const took = join(latest, run.video ? `${run.name}-16x9.webm` : `${run.name}-open0-film-motion.png`);
  const kept = run.video ? join(out, `${run.name}.webm`) : join(latest, `${run.name}-film-motion.png`);
  rmSync(took, { force: true });
  if (run.video) rmSync(kept, { force: true });
  process.once("exit", () => {
    if (!existsSync(took)) return console.log(`film: FAILED no ${run.video ? "webm" : "sheet"} came back`);
    renameSync(took, kept);
    console.log(`film: ${kept}`);
  });
  process.argv = [process.argv[0]!, process.argv[1]!, "--motion"];
  const routes = [`/${run.name}/@window.film`];
  await main(root, { shots: { routes, sizes: run.video ? { video: [SIZE[0], SIZE[1], false] } : { film: [w, Math.min(h, TALL), false] } } }, films);
}
