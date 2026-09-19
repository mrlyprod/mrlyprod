import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { extname, join, resolve } from "node:path";

/* WHERE */

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const DATA_DIR = resolve(org, "../../data/mrlyprod/site/scripts");
const SHOTS = join(DATA_DIR, "shots");
const PROFILE = join(DATA_DIR, "profile");
const CHROME = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const PORT = 9335;
const SERVE = 3335;

/* WHAT */

const ROUTES = ["/", "/demos/sponge/", "/papers/spin-harmonics/", "/research/discoveries/", "/git/site/kit/ssg/build.ts"];

const SIZES: [string, number, number, boolean][] = [
  ["phone", 390, 844, true],
  ["desktop", 1440, 900, false],
];

const TALL = 6000;

const PATIENCE = 15000;

/* ARGS */

const args = process.argv.slice(2);
const print = args.includes("--print");
const baseline = args.includes("--baseline");
const probe = args.includes("--js") ? args[args.indexOf("--js") + 1] ?? "" : "";
const routes = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--js");
const pages = routes.length ? routes : ROUTES;
const out = join(SHOTS, baseline ? "baseline" : "latest");
const base = join(SHOTS, "baseline");

/* SERVER */

const TYPES: Record<string, string> = {
  ".css": "text/css",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript",
  ".json": "application/json",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".wasm": "application/wasm",
  ".woff2": "font/woff2",
};

const html = (bytes: Uint8Array) => new TextDecoder().decode(bytes.subarray(0, 15)).toLowerCase().startsWith("<!doctype html");

function file(path: string): Response | null {
  const want = join(dist, path.endsWith("/") ? `${path}index.html` : path);
  if (!want.startsWith(dist) || !existsSync(want) || !statSync(want).isFile()) return null;
  const bytes = new Uint8Array(readFileSync(want));
  const type = html(bytes) ? TYPES[".html"]! : (TYPES[extname(want)] ?? "application/octet-stream");
  return new Response(bytes, { headers: { "content-type": type } });
}

const server = Bun.serve({
  port: SERVE,
  fetch(req) {
    const path = decodeURIComponent(new URL(req.url).pathname);
    const hit = file(path) ?? file(`${path}/`);
    if (hit) return hit;
    const lost = file("/404.html");
    return lost ? new Response(lost.body, { status: 404, headers: lost.headers }) : new Response("not found", { status: 404 });
  },
});

/* CHROME */

const wait = (ms: number) => new Promise((r) => setTimeout(r, ms));

type Target = { type: string; webSocketDebuggerUrl: string };

async function targets(): Promise<Target[] | null> {
  try {
    return (await (await fetch(`http://127.0.0.1:${PORT}/json`)).json()) as Target[];
  } catch {
    return null;
  }
}

async function launch() {
  if (await targets()) throw new Error(`shots: something already answers on port ${PORT}; kill it first`);
  rmSync(PROFILE, { recursive: true, force: true });
  mkdirSync(PROFILE, { recursive: true });
  const proc = Bun.spawn(
    [CHROME, "--headless=new", "--disable-gpu", "--enable-unsafe-swiftshader", "--use-angle=swiftshader", "--no-first-run", "--hide-scrollbars", `--user-data-dir=${PROFILE}`, `--remote-debugging-port=${PORT}`, "about:blank"],
    { stdout: "ignore", stderr: "ignore" },
  );
  for (let i = 0; i < 50; i++) {
    await wait(200);
    if (proc.exitCode !== null) break;
    const page = (await targets())?.find((one) => one.type === "page");
    if (page) return { proc, page };
  }
  proc.kill();
  throw new Error(`shots: no chrome at ${CHROME}; set CHROME to the binary`);
}

type Reply = { result?: Record<string, unknown>; exceptionDetails?: { text: string } };

function driver(ws: WebSocket) {
  let id = 0;
  const pending = new Map<number, (v: Reply) => void>();
  const events = new Map<string, () => void>();
  ws.onmessage = (event) => {
    const m = JSON.parse(String(event.data));
    if (m.id && pending.has(m.id)) {
      pending.get(m.id)!(m.result ?? m.error);
      pending.delete(m.id);
    } else if (m.method && events.has(m.method)) {
      events.get(m.method)!();
      events.delete(m.method);
    }
  };
  const send = (method: string, params = {}) =>
    new Promise<any>((r) => {
      pending.set(++id, r);
      ws.send(JSON.stringify({ id, method, params }));
    });
  const once = (method: string) =>
    new Promise<void>((r, fail) => {
      events.set(method, r);
      setTimeout(() => fail(new Error(`shots: ${method} never came within ${PATIENCE / 1000} s`)), PATIENCE);
    });
  return { send, once };
}

const MOUNTED = `new Promise((r) => { const root = document.getElementById("root"); if (!root) return r(); const t0 = Date.now(); const poll = () => (root.children.length || Date.now() - t0 > 8000 ? r() : setTimeout(poll, 50)); poll(); })`;

const STILL = `document.head.insertAdjacentHTML("beforeend", "<style>canvas:not(.mark) { visibility: hidden !important; }</style>")`;

const READY = `${MOUNTED}.then(() => Promise.all([...document.images].map((i) => { i.loading = "eager"; return (i.complete ? Promise.resolve() : new Promise((r) => { i.onload = i.onerror = r; })).then(() => i.decode().catch(() => 0)); }))).then(() => document.fonts.ready).then(() => { ${STILL}; }).then(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))).then(() => 1)`;

const WIDE = `JSON.stringify([...document.querySelectorAll("body *")].filter((el) => !el.closest(".pane, .scrim") && getComputedStyle(el).visibility !== "hidden").map((el) => [el, el.getBoundingClientRect()]).filter(([, r]) => r.right > innerWidth + 1 && r.width > 0).sort((a, b) => b[1].right - a[1].right).slice(0, 4).map(([el, r]) => el.tagName.toLowerCase() + (typeof el.className === "string" && el.className ? "." + el.className.trim().split(/\\s+/).join(".") : "") + " right=" + Math.round(r.right)))`;

const name = (route: string, size: string) => `${route.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "") || "home"}-${size}${print ? "-print" : ""}.png`;

const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex").slice(0, 8);

/* RUN */

mkdirSync(out, { recursive: true });
const { proc, page } = await launch();
let shot = 0;
let changed = 0;
let fresh = 0;
try {
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((r) => (ws.onopen = r));
  const { send, once } = driver(ws);
  await send("Page.enable");
  await send("Emulation.setEmulatedMedia", { media: print ? "print" : "", features: [{ name: "prefers-reduced-motion", value: "reduce" }] });
  for (const route of pages) {
    for (const [size, width, height, mobile] of SIZES) {
      await send("Emulation.setDeviceMetricsOverride", { width, height, deviceScaleFactor: 1, mobile });
      const loaded = once("Page.loadEventFired");
      await send("Page.navigate", { url: `http://127.0.0.1:${SERVE}${route}` });
      try {
        await loaded;
      } catch (error) {
        console.log(`shots: ${name(route, size)} FAILED ${(error as Error).message}`);
        continue;
      }
      await Promise.race([send("Runtime.evaluate", { expression: READY, awaitPromise: true, returnByValue: true }), wait(PATIENCE)]);
      await wait(300);
      const { cssContentSize } = await send("Page.getLayoutMetrics");
      const full = Math.min(Math.ceil(cssContentSize.height), TALL);
      const cap = await send("Page.captureScreenshot", { format: "png", captureBeyondViewport: true, clip: { x: 0, y: 0, width, height: full, scale: 1 } });
      const bytes = new Uint8Array(Buffer.from(cap.data, "base64"));
      const file = name(route, size);
      await Bun.write(join(out, file), bytes);
      shot++;
      const wide = Math.ceil(cssContentSize.width) > width;
      const was = join(base, file);
      let verdict = "";
      if (!baseline && existsSync(was)) {
        const same = sha(new Uint8Array(readFileSync(was))) === sha(bytes);
        verdict = same ? " same" : " DIFF";
        if (!same) changed++;
      } else if (!baseline) {
        verdict = " new";
        fresh++;
      }
      console.log(`shots: ${file} ${width}x${full} ${sha(bytes)}${wide ? " OVERFLOW" : ""}${verdict}`);
      if (probe) {
        const { result, exceptionDetails } = await send("Runtime.evaluate", { expression: probe, returnByValue: true, awaitPromise: true });
        console.log(`  ${exceptionDetails ? `probe failed: ${exceptionDetails.text}` : JSON.stringify(result.value)}`);
      }
      if (wide) {
        const { result } = await send("Runtime.evaluate", { expression: WIDE, returnByValue: true });
        for (const line of JSON.parse(result.value) as string[]) console.log(`  ${line}`);
      }
    }
  }
  ws.close();
} finally {
  proc.kill();
  await proc.exited;
  rmSync(PROFILE, { recursive: true, force: true });
  server.stop(true);
}
const tail = baseline ? "baseline written" : `${changed} differ from baseline, ${fresh} new`;
console.log(`shots: ${shot} shots in ${out}, ${tail}, chrome killed`);
process.exit(0);
