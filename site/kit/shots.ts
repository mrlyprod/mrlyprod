import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { decode } from "./git/view.ts";
import { decide } from "./edge.ts";
import { kind } from "./types.ts";

/* WHERE */

export type Size = [number, number, boolean];

export type Block = { routes: string[]; sizes: Record<string, Size> };

const CHROME = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const PORT = 9335;
const SERVE = 3335;
const LIVE = (process.env.SITE_URL ?? "").replace(/\/$/, "");
const CSP = process.env.CSP ?? "";
const TALL = 6000;
const PATIENCE = 15000;
const REPLY = 30000;
const ACT = 900000;
const FIRST = 5000;
const PAUSE = 300;
const SETTLE = 600;
const TAP = 60;

export function block(config: Record<string, unknown>): Block {
  const found = config.shots as Block | undefined;
  if (!found?.routes?.length || !found.sizes) throw new Error("shots: site.json has no shots block");
  return found;
}

/* SERVER */

const doc = (bytes: Uint8Array) => new TextDecoder().decode(bytes.subarray(0, 15)).toLowerCase().startsWith("<!doctype html");

function serve(dist: string) {
  const file = (path: string): Response | null => {
    const want = join(dist, path.endsWith("/") ? `${path}index.html` : path);
    if (!want.startsWith(dist) || !existsSync(want) || !statSync(want).isFile()) return null;
    const bytes = new Uint8Array(readFileSync(want));
    const type = kind(doc(bytes) ? ".html" : want);
    const headers: Record<string, string> = { "content-type": type };
    if (CSP) headers["content-security-policy"] = CSP;
    return new Response(bytes, { headers });
  };
  return Bun.serve({
    port: SERVE,
    fetch(req) {
      const url = new URL(req.url);
      const step = decide(decode(url.pathname));
      if (step.redirect) return Response.redirect(`${step.redirect}${url.search}`, 301);
      const hit = file(step.uri!);
      if (hit) return hit;
      const lost = file("/404.html");
      return lost ? new Response(lost.body, { status: 404, headers: lost.headers }) : new Response("not found", { status: 404 });
    },
  });
}

/* CHROME */

const wait = (ms: number) => new Promise((r) => setTimeout(r, ms));

function deadline<T>(job: Promise<T>, ms: number, what: string): Promise<T> {
  return new Promise<T>((ok, no) => {
    const timer = setTimeout(() => no(new Error(`shots: ${what} did not answer within ${ms / 1000} s`)), ms);
    const clear = (run: () => void) => {
      clearTimeout(timer);
      run();
    };
    job.then(
      (value) => clear(() => ok(value)),
      (error) => clear(() => no(error)),
    );
  });
}

type Target = { type: string; webSocketDebuggerUrl: string };

async function targets(): Promise<Target[] | null> {
  try {
    return (await (await fetch(`http://127.0.0.1:${PORT}/json`)).json()) as Target[];
  } catch {
    return null;
  }
}

async function launch(profile: string) {
  if (await targets()) throw new Error(`shots: something already answers on port ${PORT}; kill it first`);
  if (!LIVE) rmSync(profile, { recursive: true, force: true });
  mkdirSync(profile, { recursive: true });
  const proc = Bun.spawn(
    [CHROME, "--headless=new", "--disable-gpu", "--enable-unsafe-swiftshader", "--use-angle=swiftshader", "--no-first-run", "--hide-scrollbars", `--user-data-dir=${profile}`, `--remote-debugging-port=${PORT}`, "about:blank"],
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

const flat = (one: any) => (one?.value !== undefined ? String(one.value) : (one?.description ?? one?.preview?.description ?? one?.unserializableValue ?? one?.type ?? ""));

function driver(ws: WebSocket) {
  let id = 0;
  const pending = new Map<number, (v: any) => void>();
  const events = new Map<string, () => void>();
  const heard = new Map<string, (params: any) => void>();
  const noise: string[] = [];
  const said = (text: string) => {
    const line = text.trim().replace(/\s+/g, " ").slice(0, 400);
    if (line) noise.push(line);
  };
  ws.onmessage = (event) => {
    const m = JSON.parse(String(event.data));
    if (m.id && pending.has(m.id)) {
      pending.get(m.id)!(m.result ?? m.error);
      pending.delete(m.id);
    } else if (m.method === "Runtime.consoleAPICalled" && /error|warning|assert/.test(m.params.type)) {
      said(`${m.params.type}: ${(m.params.args ?? []).map(flat).join(" ")}`);
    } else if (m.method === "Runtime.exceptionThrown") {
      said(`exception: ${m.params.exceptionDetails.exception?.description ?? m.params.exceptionDetails.text}`);
    } else if (m.method === "Log.entryAdded" && /error|warning/.test(m.params.entry.level)) {
      said(`${m.params.entry.level}: ${m.params.entry.text} ${m.params.entry.url ?? ""}`);
    } else if (m.method && heard.has(m.method)) {
      heard.get(m.method)!(m.params);
    } else if (m.method && events.has(m.method)) {
      events.get(m.method)!();
      events.delete(m.method);
    }
  };
  const send = (method: string, params = {}, ms = REPLY) =>
    deadline(
      new Promise<any>((r) => {
        pending.set(++id, r);
        ws.send(JSON.stringify({ id, method, params }));
      }),
      ms,
      `${method} reply`,
    );
  const once = (method: string) => deadline(new Promise<void>((r) => events.set(method, r)), PATIENCE, method);
  const on = (method: string, fn: (params: any) => void) => heard.set(method, fn);
  return { send, once, on, noise };
}

type Send = ReturnType<typeof driver>["send"];

async function until(send: Send, expression: string, ms: number): Promise<boolean> {
  const end = Date.now() + ms;
  for (;;) {
    const reply = await send("Runtime.evaluate", { expression, returnByValue: true }).catch(() => null);
    if (reply?.result?.value === true) return true;
    if (Date.now() >= end) return false;
    await wait(50);
  }
}

/* PROBES */

const MOUNTED = `new Promise((r) => { const root = document.querySelector("#root, #app"); const t0 = Date.now(); const poll = () => (((!root || root.children.length) && !document.querySelector('[aria-busy="true"]')) || Date.now() - t0 > 8000 ? r() : setTimeout(poll, 50)); poll(); })`;

const STILL = `document.head.insertAdjacentHTML("beforeend", "<style>canvas { visibility: hidden !important; }</style>")`;

const ready = (motion: boolean) =>
  `${MOUNTED}.then(() => Promise.all([...document.images].map((i) => { i.loading = "eager"; return (i.complete ? Promise.resolve() : new Promise((r) => { i.onload = i.onerror = r; })).then(() => i.decode().catch(() => 0)); }))).then(() => document.fonts.ready)${motion ? "" : `.then(() => { ${STILL}; })`}.then(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))).then(() => 1)`;

const QUIET = `(() => { for (const i of document.images) { i.loading = "eager"; i.decoding = "sync"; } return document.readyState === "complete" && [...document.images].every((i) => i.complete) && document.fonts.status !== "loading"; })()`;

const painted = (born: number) => `performance.timeOrigin !== ${born} && document.readyState !== "loading" && [...document.querySelectorAll('link[rel="stylesheet"]')].every((one) => one.sheet)`;

const WIDE = `JSON.stringify([...document.querySelectorAll("body *")].filter((el) => !el.closest(".pane, .scrim") && getComputedStyle(el).visibility !== "hidden").map((el) => [el, el.getBoundingClientRect()]).filter(([, r]) => r.right > innerWidth + 1 && r.width > 0).sort((a, b) => b[1].right - a[1].right).slice(0, 4).map(([el, r]) => el.tagName.toLowerCase() + (typeof el.className === "string" && el.className ? "." + el.className.trim().split(/\\s+/).join(".") : "") + " right=" + Math.round(r.right) + " width=" + Math.round(r.width)))`;

const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex").slice(0, 8);

/* KEYS */

const KEYS: Record<string, [number, string?]> = {
  Tab: [9],
  Enter: [13, "\r"],
  Escape: [27],
  Space: [32, " "],
  Backspace: [8],
  Delete: [46],
  ArrowLeft: [37],
  ArrowUp: [38],
  ArrowRight: [39],
  ArrowDown: [40],
  Home: [36],
  End: [35],
  PageUp: [33],
  PageDown: [34],
};

const MODS: Record<string, number> = { Alt: 1, Control: 2, Meta: 4, Shift: 8 };

async function stroke(send: Send, step: string) {
  const mods = step.split("+");
  const name = mods.pop()!;
  if (!Object.hasOwn(KEYS, name) || !mods.every((one) => Object.hasOwn(MODS, one))) {
    for (const char of step) {
      await send("Input.dispatchKeyEvent", { type: "keyDown", key: char, text: char, unmodifiedText: char });
      await send("Input.dispatchKeyEvent", { type: "keyUp", key: char });
    }
    return;
  }
  const [code, text] = KEYS[name]!;
  const modifiers = mods.reduce((sum, one) => sum | MODS[one]!, 0);
  const key = { key: name === "Space" ? " " : name, code: name, windowsVirtualKeyCode: code, nativeVirtualKeyCode: code, modifiers };
  const types = text !== undefined && !(modifiers & 7);
  await send("Input.dispatchKeyEvent", types ? { ...key, type: "keyDown", text, unmodifiedText: text } : { ...key, type: "rawKeyDown" });
  await send("Input.dispatchKeyEvent", { ...key, type: "keyUp" });
}

/* WALK */

const AIM = (to: string) => `const want = new URL(${JSON.stringify(to)}, location.origin); const same = (one) => one.origin === want.origin && one.pathname === want.pathname && one.search === want.search && one.hash === want.hash;`;

const OVER = `document.querySelector("dialog[open]")`;

const hop = (to: string) =>
  `(() => { ${AIM(to)} const all = [...(${OVER} ?? document).querySelectorAll("a[href]")].filter(same); const a = all.find((one) => one.getClientRects().length) ?? all[0]; if (!a) return "none"; window.__main = document.querySelector("main"); window.__over = Boolean(${OVER}); a.scrollIntoView({ block: "center", behavior: "instant" }); const r = a.getBoundingClientRect(); const x = r.left + r.width / 2; const y = r.top + r.height / 2; const top = document.elementFromPoint(x, y); if (top && a.contains(top)) return JSON.stringify([x, y]); a.click(); return "synthetic"; })()`;

const HERE = `location.pathname + location.search + location.hash`;

const landed = (to: string, from: string) =>
  `(() => { ${AIM(to)} const was = new URL(${JSON.stringify(from)}, location.origin); const on = location.pathname === want.pathname; const self = was.pathname === want.pathname && !want.hash; const fresh = window.__main?.isConnected !== true || (window.__over === true && !${OVER}); return ((self ? on && fresh : same(location) || (on && ${HERE} !== ${JSON.stringify(from)})) || ${OVER}?.dataset.page === want.pathname) && document.readyState === "complete"; })()`;

const moved = (from: string) => `(${HERE} !== ${JSON.stringify(from)} || window.__popped === true) && document.readyState === "complete"`;

/* RUN */

const VALUED = ["--js", "--theme", "--size", "--keys", "--frames"];

export async function main(root: string, config?: Record<string, unknown>, dist = join(root, "dist")): Promise<void> {
  const conf = config ?? (JSON.parse(readFileSync(join(root, "site.json"), "utf8")) as Record<string, unknown>);
  const { routes, sizes } = block(conf);
  const desk = resolve(root, "../..");
  const DATA_DIR = join(desk, "data", relative(desk, root), "scripts");
  const args = process.argv.slice(2);
  const flag = (name: string) => (args.includes(name) ? (args[args.indexOf(name) + 1] ?? "") : "");
  const print = args.includes("--print");
  const baseline = args.includes("--baseline");
  const motion = args.includes("--motion");
  const nojs = args.includes("--nojs");
  const first = args.includes("--first");
  const walk = args.includes("--walk");
  const probe = flag("--js");
  const scheme = flag("--theme");
  const only = flag("--size");
  const keys = flag("--keys") ? flag("--keys").split(",") : [];
  const frames = flag("--frames") ? flag("--frames").split(",").map(Number) : [];
  const asked = args.filter((a, i) => !a.startsWith("--") && !VALUED.includes(args[i - 1] ?? ""));
  const pages = asked.length ? asked : routes;
  const sized = Object.entries(sizes).filter(([size]) => !only || size === only);
  if (!sized.length) throw new Error(`shots: --size ${only} is none of ${Object.keys(sizes).join(", ")}`);
  if (frames.some((at) => !Number.isFinite(at) || at < 0)) throw new Error("shots: --frames takes milliseconds, as 0,200,400");
  if (first && (walk || keys.length || frames.length)) throw new Error("shots: --first shoots alone, without --walk, --keys or --frames");
  const out = join(DATA_DIR, "shots", baseline ? "baseline" : "latest");
  const base = join(DATA_DIR, "shots", "baseline");
  const origin = LIVE || `http://127.0.0.1:${SERVE}`;
  const name = (route: string, size: string, act: number, stage = "") =>
    `${route.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "") || "home"}${act >= 0 ? `-open${act}` : ""}-${size}${print ? "-print" : ""}${motion ? "-motion" : ""}${nojs ? "-nojs" : ""}${scheme ? `-${scheme}` : ""}${keys.length ? "-keys" : ""}${walk ? "-walk" : ""}${stage}.png`;
  mkdirSync(out, { recursive: true });
  const server = LIVE ? null : serve(resolve(dist));
  const { proc, page } = await launch(join(DATA_DIR, "profile"));
  let shot = 0;
  let changed = 0;
  let fresh = 0;
  try {
    const ws = new WebSocket(page.webSocketDebuggerUrl);
    await new Promise((r) => (ws.onopen = r));
    const { send, once, on, noise } = driver(ws);
    await send("Page.enable");
    await send("Runtime.enable");
    await send("Log.enable");
    await send("Browser.setDownloadBehavior", { behavior: "allow", downloadPath: out });
    await send("Emulation.setEmulatedMedia", { media: print ? "print" : "", features: [{ name: "prefers-reduced-motion", value: motion ? "no-preference" : "reduce" }, ...(scheme ? [{ name: "prefers-color-scheme", value: scheme }] : [])] });
    if (nojs) await send("Emulation.setScriptExecutionDisabled", { value: true });
    if (keys.length) await send("Emulation.setFocusEmulationEnabled", { enabled: true });
    const held: string[] = [];
    let holding = false;
    let loads = 0;
    const pass = (requestId: string) => send("Fetch.continueRequest", { requestId }).catch(() => 0);
    if (first) {
      on("Fetch.requestPaused", ({ requestId }) => (holding ? held.push(requestId) : pass(requestId)));
      await send("Fetch.enable", { patterns: [{ resourceType: "Script" }] });
    }
    on("Page.frameNavigated", ({ frame }) => {
      if (!frame.parentId) loads++;
    });
    const value = async (expression: string) => (await send("Runtime.evaluate", { expression, returnByValue: true })).result?.value;
    const said = () => {
      for (const line of [...new Set(noise)]) console.log(`  ${line}`);
    };
    const probed = async () => {
      if (!probe) return;
      const { result, exceptionDetails } = await send("Runtime.evaluate", { expression: probe, returnByValue: true, awaitPromise: true });
      console.log(`  ${exceptionDetails ? `probe failed: ${exceptionDetails.text}` : JSON.stringify(result.value)}`);
    };
    const take = async ([width, height]: Size, file: string, view: boolean, note = "", tell = true) => {
      const { cssContentSize } = await send("Page.getLayoutMetrics");
      const full = view ? height : Math.min(Math.ceil(cssContentSize.height), TALL);
      const cap = await send("Page.captureScreenshot", view ? { format: "png" } : { format: "png", captureBeyondViewport: true, clip: { x: 0, y: 0, width, height: full, scale: 1 } });
      const bytes = new Uint8Array(Buffer.from(cap.data, "base64"));
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
      console.log(`shots: ${file} ${width}x${full} ${sha(bytes)}${note}${wide ? " OVERFLOW" : ""}${verdict}${tell && noise.length ? ` ${noise.length} NOISE` : ""}`);
      if (tell) {
        said();
        await probed();
      }
      if (wide) {
        const { result } = await send("Runtime.evaluate", { expression: WIDE, returnByValue: true });
        for (const line of JSON.parse(result.value) as string[]) console.log(`  ${line}`);
      }
    };
    const settle = async () => {
      if (nojs) {
        await until(send, QUIET, PATIENCE);
        if (!motion) await send("Runtime.evaluate", { expression: STILL });
      } else await Promise.race([send("Runtime.evaluate", { expression: ready(motion), awaitPromise: true, returnByValue: true }).catch(() => 0), wait(PATIENCE)]);
      await wait(PAUSE);
    };
    const run = async (act: string) => {
      const { result, exceptionDetails } = await send("Runtime.evaluate", { expression: act, returnByValue: true, awaitPromise: true }, ACT);
      if (exceptionDetails) console.log(`  act failed: ${exceptionDetails.exception?.description ?? exceptionDetails.text}`);
      else if (result?.value !== undefined && result.value !== 0) console.log(`  act: ${JSON.stringify(result.value)}`);
    };
    const finish = async (dims: Size, file: (stage: string) => string, moved: boolean, tell = true) => {
      for (const step of keys) {
        await stroke(send, step);
        await wait(TAP);
      }
      if (!frames.length) {
        if (moved || keys.length) await wait(SETTLE);
        return take(dims, file(""), moved || keys.length > 0, "", tell);
      }
      const from = Date.now();
      for (const [n, at] of frames.entries()) {
        await wait(Math.max(0, at - (Date.now() - from)));
        await take(dims, file(`-f${n}`), true, ` t=${Date.now() - from}ms`, tell && n === frames.length - 1);
      }
    };
    const shoot = async (turn: number, want: string, size: string, dims: Size) => {
      const [route = "/", act] = want.split("@");
      const born = first ? Number(await value("performance.timeOrigin")) : 0;
      const loaded = once("Page.loadEventFired").then(() => "", (error: Error) => error.message);
      noise.length = 0;
      held.length = 0;
      holding = first;
      await send("Page.navigate", { url: `${origin}${route}` });
      let top = 0;
      if (first) {
        const early = name(route, size, -1, "-first");
        if (await until(send, painted(born), FIRST)) {
          await wait(PAUSE);
          top = Number(await value("Math.round(scrollY)"));
          await take(dims, early, true, ` scroll=${top}`);
        } else console.log(`shots: ${early} FAILED nothing parsed within ${FIRST / 1000} s with every script held`);
        holding = false;
        for (const id of held.splice(0)) pass(id);
      }
      const file = (stage: string) => name(route, size, !first && act ? turn : -1, first ? "-after" : stage);
      const lost = await loaded;
      if (lost) {
        console.log(`shots: ${file("")} FAILED ${lost}`);
        return;
      }
      await settle();
      if (act) await run(act);
      if (!first) return finish(dims, file, Boolean(act));
      if (act) await wait(SETTLE);
      const now = Number(await value("Math.round(scrollY)"));
      await take(dims, file(""), true, ` scroll=${now}${now === top ? "" : " JUMP"}`);
    };
    const stroll = async (size: string, dims: Size) => {
      loads = 0;
      for (const [n, want] of pages.entries()) {
        const [to = "/", act] = want.split("@");
        const from = n ? String(await value(HERE)) : "";
        let how = "load";
        noise.length = 0;
        if (!n) {
          const loaded = once("Page.loadEventFired").then(() => "", (error: Error) => error.message);
          await send("Page.navigate", { url: `${origin}${to}` });
          const lost = await loaded;
          if (lost) return console.log(`walk: ${n} ${to} FAILED ${lost}`);
        } else if (to === "reload") {
          how = to;
          const loaded = once("Page.loadEventFired").then(() => "", (error: Error) => error.message);
          await send("Page.reload");
          const lost = await loaded;
          if (lost) return console.log(`walk: ${n} ${to} FAILED ${lost}`);
        } else if (to === "back" || to === "forward") {
          how = to;
          await send("Runtime.evaluate", { expression: `window.__popped = false; addEventListener("popstate", () => { window.__popped = true; }, { once: true }); history.${to}()` });
          if (!(await until(send, moved(from), PATIENCE))) return console.log(`walk: ${n} ${to} FAILED ${from} never left`);
        } else {
          const spot = String(await value(hop(to)));
          if (spot === "none") return console.log(`walk: ${n} ${to} FAILED ${from} holds no link to it`);
          how = spot === "synthetic" ? spot : "click";
          if (how === "click") {
            const [x, y] = JSON.parse(spot) as [number, number];
            await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
            await send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", clickCount: 1 });
            await send("Input.dispatchMouseEvent", { type: "mouseReleased", x, y, button: "left", clickCount: 1 });
          }
          if (!(await until(send, landed(to, from), PATIENCE))) return console.log(`walk: ${n} ${to} FAILED the ${how} on ${from} never landed`);
        }
        await settle();
        if (act) await run(act);
        const here = String(await value(HERE));
        if (n === pages.length - 1) await finish(dims, (stage) => name(here, size, -1, stage), true, false);
        else if (act) await wait(PAUSE);
        const over = await value(`${OVER}?.dataset.page ?? ""`);
        console.log(`walk: ${n} ${here} ${how} loads=${loads} scroll=${await value("Math.round(scrollY)")}${over ? ` dialog=${over}` : ""}${noise.length ? ` ${noise.length} NOISE` : ""}`);
        said();
        await probed();
      }
      console.log(`walk: ${pages.length} hops at ${size}, ${loads} page loads`);
    };
    for (const [size, dims] of walk ? sized : []) {
      await send("Emulation.setDeviceMetricsOverride", { width: dims[0], height: dims[1], deviceScaleFactor: 1, mobile: dims[2] });
      await stroll(size, dims);
    }
    for (const [turn, want] of walk ? [] : pages.entries()) {
      for (const [size, dims] of sized) {
        await send("Emulation.setDeviceMetricsOverride", { width: dims[0], height: dims[1], deviceScaleFactor: 1, mobile: dims[2] });
        await shoot(turn, want, size, dims);
      }
    }
    ws.close();
  } finally {
    proc.kill();
    await proc.exited;
    if (!LIVE) rmSync(join(DATA_DIR, "profile"), { recursive: true, force: true });
    server?.stop(true);
  }
  const tail = baseline ? "baseline written" : `${changed} differ from baseline, ${fresh} new`;
  console.log(`shots: ${shot} shots in ${out}, ${tail}, chrome killed`);
  process.exit(0);
}
