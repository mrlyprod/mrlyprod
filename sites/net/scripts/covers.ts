import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const cache = join(dist, "_shots");
const CHROME = process.env.MRLY_CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const LANES = Math.max(1, Number(process.env.MRLY_COVER_LANES ?? 3));
const WIDE = 1200;
const TALL = 630;
const PATIENCE = 45000;
const READY =
  "location.href !== 'about:blank' && document.readyState === 'complete' && !document.documentElement.dataset.wait && Array.from(document.images).every((img) => img.complete)";

type Shot = { key: string; route: string; url: string; sources: string[] };
type Job = { png: string; url: string; wide: number; tall: number; scale: number; route?: string };

const mode = process.env.MRLY_COVERS ?? "";
if (mode === "0") process.exit(0);

const list = join(dist, "_covers", "list.json");
if (!existsSync(list)) throw new Error("dist/_covers/list.json missing: run bun scripts/site.ts first");
const shots = JSON.parse(readFileSync(list, "utf8")) as Shot[];

const rest = (ms: number) => new Promise((go) => setTimeout(go, ms));
const stamp = (p: string) => (existsSync(p) ? statSync(p).mtimeMs : 0);
const fresh = (png: string, sources: string[]) => {
  if (mode === "fresh") return false;
  const made = stamp(png);
  return made > 0 && sources.every((s) => stamp(s) <= made);
};

/* SERVER */

const server = Bun.serve({
  port: 0,
  fetch(req) {
    let path = decodeURIComponent(new URL(req.url).pathname);
    if (path.endsWith("/")) path += "index.html";
    const file = Bun.file(join(dist, path));
    return file.size ? new Response(file) : new Response("not found", { status: 404 });
  },
});
const site = `http://localhost:${server.port}`;

/* BROWSER */

const seat = mkdtempSync(join(tmpdir(), "mrly-shot-"));
const chrome = Bun.spawn(
  [
    CHROME,
    "--headless=new",
    "--disable-gpu",
    "--hide-scrollbars",
    "--no-first-run",
    "--no-default-browser-check",
    "--disable-extensions",
    "--disable-background-networking",
    "--disable-sync",
    "--mute-audio",
    "--disable-background-timer-throttling",
    "--disable-backgrounding-occluded-windows",
    "--disable-renderer-backgrounding",
    `--user-data-dir=${seat}`,
    "--remote-debugging-port=0",
    "about:blank",
  ],
  { stdout: "ignore", stderr: "ignore" },
);

async function port() {
  const file = join(seat, "DevToolsActivePort");
  const clock = Date.now();
  while (Date.now() - clock < 30000) {
    if (existsSync(file)) {
      const first = readFileSync(file, "utf8").split("\n")[0].trim();
      if (first) return Number(first);
    }
    await rest(100);
  }
  throw new Error("chrome never opened a debugging port");
}

const debug = await port();
const version = (await (await fetch(`http://127.0.0.1:${debug}/json/version`)).json()) as { webSocketDebuggerUrl: string };
const socket = new WebSocket(version.webSocketDebuggerUrl);
await new Promise((go, stop) => {
  socket.onopen = go;
  socket.onerror = stop;
});

let seq = 0;
const pending = new Map<number, (msg: { error?: { message: string }; result?: unknown }) => void>();
socket.onmessage = (event) => {
  const msg = JSON.parse(String(event.data));
  const hit = pending.get(msg.id);
  if (hit) {
    pending.delete(msg.id);
    hit(msg);
  }
};

function send(method: string, params: object = {}, sessionId?: string): Promise<Record<string, unknown>> {
  return new Promise((done, fail) => {
    const id = ++seq;
    pending.set(id, (msg) => (msg.error ? fail(new Error(`${method}: ${msg.error.message}`)) : done((msg.result ?? {}) as Record<string, unknown>)));
    socket.send(JSON.stringify(sessionId ? { id, method, params, sessionId } : { id, method, params }));
  });
}

async function tab() {
  const { targetId } = (await send("Target.createTarget", { url: "about:blank" })) as { targetId: string };
  const { sessionId } = (await send("Target.attachToTarget", { targetId, flatten: true })) as { sessionId: string };
  return sessionId;
}

async function shoot(session: string, job: Job) {
  await send("Emulation.setDeviceMetricsOverride", { width: job.wide, height: job.tall, deviceScaleFactor: job.scale, mobile: false }, session);
  await send("Page.navigate", { url: "about:blank" }, session);
  await rest(40);
  await send("Page.navigate", { url: job.url }, session);
  const clock = Date.now();
  for (;;) {
    await rest(80);
    const answer = (await send("Runtime.evaluate", { expression: READY, returnByValue: true }, session)) as { result?: { value?: boolean } };
    if (answer.result?.value) break;
    if (Date.now() - clock > PATIENCE) throw new Error(`cover never settled: ${job.url}`);
  }
  const { data } = (await send("Page.captureScreenshot", { format: "png" }, session)) as { data: string };
  mkdirSync(dirname(job.png), { recursive: true });
  writeFileSync(job.png, Buffer.from(data, "base64"));
}

/* RUN */

const clock = Date.now();
mkdirSync(cache, { recursive: true });
const made = { drawn: 0, kept: 0 };
const queue: Job[] = [];

for (const job of shots) {
  const png = join(cache, `${job.key}.png`);
  if (fresh(png, job.sources)) made.kept++;
  else queue.push({ png, url: site + job.url, wide: WIDE, tall: TALL, scale: 1, route: job.route });
}

const marks = [join(org, "lib", "logo.js"), join(org, "public", "pages.css")];
for (const icon of [
  { name: "icon-512.png", wide: 512, tall: 512 },
  { name: "apple-touch-icon.png", wide: 180, tall: 180 },
]) {
  const png = join(cache, icon.name);
  if (!fresh(png, marks)) queue.push({ png, url: `${site}/_covers/icon/`, wide: icon.wide, tall: icon.tall, scale: 1 });
}

async function lane() {
  const session = await tab();
  for (let job = queue.pop(); job; job = queue.pop()) {
    await shoot(session, job);
    if (job.route) made.drawn++;
  }
}

await Promise.all(Array.from({ length: LANES }, () => lane()));
socket.close();
chrome.kill(9);
Bun.spawnSync(["pkill", "-9", "-f", seat], { stdout: "ignore", stderr: "ignore" });
await chrome.exited;
rmSync(seat, { recursive: true, force: true });
server.stop(true);

for (const job of shots) {
  const png = join(cache, `${job.key}.png`);
  const out = join(dist, job.route.replace(/^\//, ""), "cover.png");
  mkdirSync(dirname(out), { recursive: true });
  copyFileSync(png, out);
}
for (const name of ["icon-512.png", "apple-touch-icon.png"]) copyFileSync(join(cache, name), join(dist, name));

rmSync(join(dist, "_covers"), { recursive: true, force: true });
rmSync(join(dist, "lib"), { recursive: true, force: true });
console.log(`covers: ${made.drawn} drawn, ${made.kept} kept, 2 icons, ${((Date.now() - clock) / 1000).toFixed(1)}s`);
