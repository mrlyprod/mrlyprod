import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { router } from "@mrlyprod/os/hosts/cloud/edge.ts";
import { kind } from "@mrlyprod/os/hosts/cloud/mime.ts";
import { decode } from "../../kit/git/view.ts";
import { COLS } from "./film/grid.ts";
import { HOME, SITE, top } from "./home.ts";

export type Screen = [number, number, number];

export type Block = { routes: string[]; screen: Screen };

export type Clock = "world" | "wall" | "still";

export type Verdict = "same" | "DIFF" | "new" | "baseline";

export type Frame = {
  file: string;
  route: string;
  n: number;
  t: number;
  clock: Clock;
  sha: string;
  verdict: Verdict;
  overflow: string[];
  noise: string[];
};

export type Answer = (path: string) => { uri?: string; redirect?: string };

export type Opts = {
  root: string;
  routes: string[];
  screen: Screen;
  dist?: string;
  answer?: Answer;
  run?: string;
  baseline?: boolean;
  print?: boolean;
  motion?: boolean;
  nojs?: boolean;
  first?: boolean;
  walk?: boolean;
  probe?: string;
  scheme?: string;
  keys?: string[];
  count?: number;
  every?: number;
};

export type Shot = { out: string; chrome: string; frames: Frame[]; failures: string[] };

const CHROME = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const PORT = 9335;
const SERVE = 3335;
const LIVE = (process.env.SITE_URL ?? "").replace(/\/$/, "");
const CSP = process.env.CSP ?? "";
const LOCALE = "en-US";
const ZONE = "Europe/Rome";
const GL = /GL Driver Message \(OpenGL, Performance,/;
const SIGNALS = ["SIGINT", "SIGTERM", "SIGHUP"] as const;
const CELL = 320;
const PATIENCE = 15000;
const REPLY = 30000;
const ACT = 900000;
const FIRST = 5000;
const PAUSE = 300;
const SETTLE = 600;
const TAP = 60;

export function block(config: Record<string, unknown>): Block {
  const routes = (config.shots as { routes?: string[] } | undefined)?.routes;
  const screen = config.screen as Screen | undefined;
  if (!routes?.length || screen?.length !== 3)
    throw new Error("shots: site.json needs shots.routes and a screen of [width, height, dpr]");
  return { routes, screen };
}

const doc = (bytes: Uint8Array) =>
  new TextDecoder().decode(bytes.subarray(0, 15)).toLowerCase().startsWith("<!doctype html");

const { decide, join: joined } = router(JSON.parse(readFileSync(join(HOME, "redirects.json"), "utf8")));

const edge: Answer = (path) => decide(decode(path));

export function screened(text: string): Screen {
  const m = /^(\d+)x(\d+)(?:@(\d))?$/.exec(text);
  if (!m) throw new Error(`shots: --screen takes WxH or WxH@dpr, as 1000x600@2, not ${text}`);
  return [Number(m[1]), Number(m[2]), Number(m[3] ?? 2)];
}

function serve(dist: string, answer: Answer) {
  const file = (path: string): Response | null => {
    const want = join(dist, path.endsWith("/") ? `${path}index.html` : path);
    if (!want.startsWith(dist + sep) || !existsSync(want) || !statSync(want).isFile()) return null;
    const bytes = new Uint8Array(readFileSync(want));
    const type = kind(doc(bytes) ? ".html" : want);
    const headers: Record<string, string> = { "content-type": type };
    if (CSP) headers["content-security-policy"] = CSP;
    return new Response(bytes, { headers });
  };
  return Bun.serve({
    hostname: "127.0.0.1",
    port: SERVE,
    fetch(req) {
      const url = new URL(req.url);
      const step = answer(url.pathname);
      if (step.redirect) return Response.redirect(joined(step.redirect, url.search), 301);
      const hit = file(step.uri!);
      if (hit) return hit;
      const lost = file("/404.html");
      return lost
        ? new Response(lost.body, { status: 404, headers: lost.headers })
        : new Response("not found", { status: 404 });
    },
  });
}

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
  if (!LIVE) rmSync(profile, { recursive: true, force: true });
  mkdirSync(profile, { recursive: true });
  const proc = Bun.spawn(
    [
      CHROME,
      "--headless=new",
      "--disable-gpu",
      "--enable-unsafe-swiftshader",
      "--use-angle=swiftshader",
      "--no-first-run",
      "--hide-scrollbars",
      `--lang=${LOCALE}`,
      `--user-data-dir=${profile}`,
      `--remote-debugging-port=${PORT}`,
      "about:blank",
    ],
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

function reaper(profile: string) {
  const marks = [`--user-data-dir=${profile}`, `--remote-debugging-port=${PORT}`];
  const alive = () => marks.some((mark) => Bun.spawnSync(["pgrep", "-f", "--", mark]).exitCode === 0);
  const reap = () => {
    for (const mark of marks) Bun.spawnSync(["pkill", "-9", "-f", "--", mark]);
  };
  return { alive, reap };
}

type Remote = {
  type?: string;
  value?: unknown;
  description?: string;
  unserializableValue?: string;
  preview?: { description?: string };
};

type Reply = {
  result: { value?: unknown };
  exceptionDetails?: { text: string; exception?: { description?: string } };
  cssContentSize: { width: number; height: number };
  data: string;
  product: string;
};

type Params = { requestId: string; frame: { parentId?: string } };

const flat = (one?: Remote) =>
  one?.value !== undefined
    ? String(one.value)
    : (one?.description ?? one?.preview?.description ?? one?.unserializableValue ?? one?.type ?? "");

function driver(ws: WebSocket) {
  let id = 0;
  const pending = new Map<number, (v: Reply) => void>();
  const events = new Map<string, () => void>();
  const heard = new Map<string, (params: Params) => void>();
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
    } else if (
      m.method === "Log.entryAdded" &&
      /error|warning/.test(m.params.entry.level) &&
      !GL.test(m.params.entry.text)
    ) {
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
      new Promise<Reply>((r) => {
        pending.set(++id, r);
        ws.send(JSON.stringify({ id, method, params }));
      }),
      ms,
      `${method} reply`,
    );
  const once = (method: string) => deadline(new Promise<void>((r) => events.set(method, r)), PATIENCE, method);
  const on = (method: string, fn: (params: Params) => void) => heard.set(method, fn);
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

const MOUNTED = `new Promise((r) => { const root = document.querySelector("#root, #app"); const t0 = performance.now(); const poll = () => (((!root || root.children.length) && !document.querySelector('[aria-busy="true"]')) || performance.now() - t0 > 8000 ? r() : setTimeout(poll, 50)); poll(); })`;

const STILL = `document.head.insertAdjacentHTML("beforeend", "<style>canvas { visibility: hidden !important; }</style>")`;

const SEEK = (s: number) =>
  `typeof window.world?.seek === "function" && window.world.seek(${Math.round(s * 1000)}) === true`;

const ready = (motion: boolean) =>
  `${MOUNTED}.then(() => Promise.all([...document.images].map((i) => { i.loading = "eager"; return (i.complete ? Promise.resolve() : new Promise((r) => { i.onload = i.onerror = r; })).then(() => i.decode().catch(() => 0)); }))).then(() => document.fonts.ready).then(() => ${motion ? "false" : `(${SEEK(0)}) || (${STILL}, false)`}).then((world) => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => r(world)))))`;

const PAINTED = `![...document.querySelectorAll('[aria-busy="true"]')].some((e) => { const r = e.getBoundingClientRect(); return r.width > 0 && r.bottom > 0 && r.top < innerHeight && r.right > 0 && r.left < innerWidth; })`;

const QUIET = `(() => { for (const i of document.images) { i.loading = "eager"; i.decoding = "sync"; } return document.readyState === "complete" && [...document.images].every((i) => i.complete) && document.fonts.status !== "loading"; })()`;

const painted = (born: number) =>
  `performance.timeOrigin !== ${born} && document.readyState !== "loading" && [...document.querySelectorAll('link[rel="stylesheet"]')].every((one) => one.sheet)`;

const WIDE = `JSON.stringify([...document.querySelectorAll("body *")].filter((el) => !el.closest(".pane, .scrim") && getComputedStyle(el).visibility !== "hidden").map((el) => [el, el.getBoundingClientRect()]).filter(([, r]) => r.right > innerWidth + 1 && r.width > 0).sort((a, b) => b[1].right - a[1].right).slice(0, 4).map(([el, r]) => el.tagName.toLowerCase() + (typeof el.className === "string" && el.className ? "." + el.className.trim().split(/\\s+/).join(".") : "") + " right=" + Math.round(r.right) + " width=" + Math.round(r.width)))`;

const FRAMED = "new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => r(1))))";

const pinned = (frozen: boolean) =>
  `(() => { const T = Date.UTC(2026, 0, 1, 8, 41); const t0 = performance.now(); const D = Date; const at = () => ${frozen ? "T" : "T + performance.now() - t0"}; class Pinned extends D { constructor(...a) { super(...(a.length ? a : [at()])); } static now() { return at(); } } globalThis.Date = Pinned; let s = 0x9e3779b9; Math.random = () => { s ^= s << 13; s ^= s >>> 17; s ^= s << 5; return (s >>> 0) / 4294967296; }; })()`;

export function every(text: string): number {
  const m = /^(\d+(?:\.\d+)?)(ms|s)?$/.exec(text.trim());
  if (!m) throw new Error(`shots: --every takes seconds, as 2s, 0.5 or 500ms, not ${text}`);
  return Math.round(Number(m[1]) * (m[2] === "ms" ? 1 : 1000)) / 1000;
}

const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex").slice(0, 8);

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
  const key = {
    key: name === "Space" ? " " : name,
    code: name,
    windowsVirtualKeyCode: code,
    nativeVirtualKeyCode: code,
    modifiers,
  };
  const types = text !== undefined && !(modifiers & 7);
  await send(
    "Input.dispatchKeyEvent",
    types ? { ...key, type: "keyDown", text, unmodifiedText: text } : { ...key, type: "rawKeyDown" },
  );
  await send("Input.dispatchKeyEvent", { ...key, type: "keyUp" });
}

const AIM = (to: string) =>
  `const want = new URL(${JSON.stringify(to)}, location.origin); const same = (one) => one.origin === want.origin && one.pathname === want.pathname && one.search === want.search && one.hash === want.hash;`;

const OVER = `document.querySelector("dialog[open]")`;

const hop = (to: string) =>
  `(() => { ${AIM(to)} const all = [...(${OVER} ?? document).querySelectorAll("a[href]")].filter(same); const a = all.find((one) => one.getClientRects().length) ?? all[0]; if (!a) return "none"; window.__main = document.querySelector("main"); window.__over = Boolean(${OVER}); a.scrollIntoView({ block: "center", behavior: "instant" }); const r = a.getBoundingClientRect(); const x = r.left + r.width / 2; const y = r.top + r.height / 2; const top = document.elementFromPoint(x, y); if (top && a.contains(top)) return JSON.stringify([x, y]); a.click(); return "synthetic"; })()`;

const HERE = `location.pathname + location.search + location.hash`;

const landed = (to: string, from: string) =>
  `(() => { ${AIM(to)} const was = new URL(${JSON.stringify(from)}, location.origin); const on = location.pathname === want.pathname; const self = was.pathname === want.pathname && !want.hash; const fresh = window.__main?.isConnected !== true || (window.__over === true && !${OVER}); return ((self ? on && fresh : same(location) || (on && ${HERE} !== ${JSON.stringify(from)})) || ${OVER}?.dataset.page === want.pathname) && document.readyState === "complete"; })()`;

const moved = (from: string) =>
  `(${HERE} !== ${JSON.stringify(from)} || window.__popped === true) && document.readyState === "complete"`;

export async function shoot(opts: Opts): Promise<Shot> {
  const { root, routes: pages, screen, answer = edge } = opts;
  const [width, height, dpr] = screen;
  const [print, baseline, motion, nojs, first, walk] = [
    opts.print,
    opts.baseline,
    opts.motion,
    opts.nojs,
    opts.first,
    opts.walk,
  ].map(Boolean);
  const probe = opts.probe ?? "";
  const scheme = opts.scheme ?? "";
  const keys = opts.keys ?? [];
  const count = opts.count ?? 0;
  const step = Math.round((opts.every ?? 1) * 1000);
  const label = opts.run || (baseline ? "baseline" : "latest");
  if (!(Number.isInteger(count) && count >= 0 && count <= 60))
    throw new Error("shots: --count takes a whole number from 1 to 60");
  if (!(step > 0)) throw new Error("shots: --every takes a time above 0");
  if (!/^[a-z0-9-]+$/.test(label)) throw new Error("shots: --run takes lowercase letters, digits and hyphens");
  if (first && (walk || keys.length || count))
    throw new Error("shots: --first shoots alone, without --walk, --keys or --count");
  if (await targets()) throw new Error(`shots: something already answers on port ${PORT}; kill it first`);
  const repo = top(root);
  const DATA_DIR = join(repo, "data", relative(repo, root), "scripts");
  const out = join(DATA_DIR, "shots", label);
  const base = join(DATA_DIR, "shots", "baseline");
  const profile = join(DATA_DIR, "profile");
  const origin = LIVE || `http://127.0.0.1:${SERVE}`;
  const frames: Frame[] = [];
  const failures: string[] = [];
  const fail = (line: string) => {
    console.log(line);
    failures.push(line);
  };
  const name = (route: string, act: number, stage = "") =>
    `${route.replace(/[^a-z0-9]+/gi, "-").replace(/^-|-$/g, "") || "home"}${act >= 0 ? `-open${act}` : ""}${print ? "-print" : ""}${motion ? "-motion" : ""}${nojs ? "-nojs" : ""}${scheme ? `-${scheme}` : ""}${keys.length ? "-keys" : ""}${walk ? "-walk" : ""}${stage}.png`;
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });
  const server = LIVE ? null : serve(resolve(opts.dist ?? join(root, "dist")), answer);
  const { alive, reap } = reaper(profile);
  const quit = (signal: NodeJS.Signals) => {
    reap();
    server?.stop(true);
    if (!LIVE) rmSync(profile, { recursive: true, force: true });
    for (const one of SIGNALS) process.off(one, quit);
    process.kill(process.pid, signal);
  };
  for (const one of SIGNALS) process.on(one, quit);
  let proc: Awaited<ReturnType<typeof launch>>["proc"] | null = null;
  let chrome = "";
  try {
    const launched = await launch(profile);
    proc = launched.proc;
    const ws = new WebSocket(launched.page.webSocketDebuggerUrl);
    await new Promise((r) => (ws.onopen = r));
    const { send, once, on, noise } = driver(ws);
    chrome = (await send("Browser.getVersion")).product;
    await send("Page.enable");
    await send("Runtime.enable");
    await send("Log.enable");
    await send("Page.addScriptToEvaluateOnNewDocument", { source: pinned(!motion && !count) });
    await send("Emulation.setTimezoneOverride", { timezoneId: ZONE });
    await send("Emulation.setLocaleOverride", { locale: LOCALE });
    await send("Browser.setDownloadBehavior", { behavior: "allow", downloadPath: out });
    await send("Emulation.setEmulatedMedia", {
      media: print ? "print" : "",
      features: [
        { name: "prefers-reduced-motion", value: motion ? "no-preference" : "reduce" },
        ...(scheme ? [{ name: "prefers-color-scheme", value: scheme }] : []),
      ],
    });
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
    const value = async (expression: string) =>
      (await send("Runtime.evaluate", { expression, returnByValue: true })).result?.value;
    const said = (since = 0) => {
      for (const line of [...new Set(noise.slice(since))]) console.log(`  ${line}`);
    };
    const probed = async () => {
      if (!probe) return;
      const { result, exceptionDetails } = await send("Runtime.evaluate", {
        expression: probe,
        returnByValue: true,
        awaitPromise: true,
      });
      console.log(`  ${exceptionDetails ? `probe failed: ${exceptionDetails.text}` : JSON.stringify(result.value)}`);
    };
    const fit = (w: number, h: number, scale = dpr) =>
      send("Emulation.setDeviceMetricsOverride", { width: w, height: h, deviceScaleFactor: scale, mobile: false });
    type At = Pick<Frame, "route" | "n" | "t" | "clock">;
    const take = async (file: string, note: string, tell: boolean, at: At) => {
      const { cssContentSize } = await send("Page.getLayoutMetrics");
      const cap = await send("Page.captureScreenshot", { format: "png" });
      const bytes = new Uint8Array(Buffer.from(cap.data, "base64"));
      await Bun.write(join(out, file), bytes);
      const hash = sha(bytes);
      const was = join(base, file);
      const verdict: Verdict = baseline
        ? "baseline"
        : !existsSync(was)
          ? "new"
          : sha(new Uint8Array(readFileSync(was))) === hash
            ? "same"
            : "DIFF";
      const wide = Math.ceil(cssContentSize.width);
      const overflow =
        wide > width ? [`content width=${wide}`, ...(JSON.parse(String(await value(WIDE))) as string[])] : [];
      const heard = [...new Set(noise)];
      frames.push({ file, ...at, sha: hash, verdict, overflow, noise: heard });
      console.log(
        `shots: ${file} ${width}x${height}@${dpr} ${hash}${note}${overflow.length ? " OVERFLOW" : ""}${baseline ? "" : ` ${verdict}`}${tell && heard.length ? ` ${heard.length} NOISE` : ""}`,
      );
      if (tell) {
        said();
        await probed();
      }
      for (const line of overflow.slice(1)) console.log(`  ${line}`);
      return bytes;
    };
    const calm = async () => {
      const glance = async () =>
        (
          await send("Page.captureScreenshot", {
            format: "png",
            clip: { x: 0, y: 0, width, height, scale: 0.25 },
          })
        ).data;
      let was = await glance();
      for (let i = 0; i < 40; i++) {
        await wait(150);
        const now = await glance();
        if (now === was) return;
        was = now;
      }
      console.log("  never still: frame 0 may differ between runs");
    };
    const contact = async (file: string, tiles: Uint8Array[]) => {
      const cols = Math.min(tiles.length, COLS);
      const cell = [CELL, Math.round((CELL * height) / width)] as const;
      const [w, h] = [cols * cell[0], Math.ceil(tiles.length / cols) * cell[1]];
      const imgs = tiles
        .map(
          (bytes, i) =>
            `<img src="data:image/png;base64,${Buffer.from(bytes).toString("base64")}" style="position:absolute;left:${(i % cols) * cell[0]}px;top:${Math.floor(i / cols) * cell[1]}px;width:${cell[0]}px;height:${cell[1]}px">`,
        )
        .join("");
      const tree = (await send("Page.getFrameTree")) as unknown as { frameTree: { frame: { id: string } } };
      await fit(w, h);
      await send("Page.setDocumentContent", {
        frameId: tree.frameTree.frame.id,
        html: `<!doctype html><html><body style="margin:0;background:#000">${imgs}</body></html>`,
      });
      await until(send, "[...document.images].every((i) => i.complete)", PATIENCE);
      const cap = await send("Page.captureScreenshot", { format: "png" });
      await Bun.write(join(out, file), Buffer.from(cap.data, "base64"));
      console.log(`shots: ${file} ${w}x${h}@${dpr} sheet of ${tiles.length}`);
      await fit(width, height);
    };
    const shown = motion || count > 0;
    const settle = async (): Promise<Clock> => {
      let world = false;
      if (nojs) {
        await until(send, QUIET, PATIENCE);
        if (!shown) await send("Runtime.evaluate", { expression: STILL });
      } else
        world = await Promise.race([
          send("Runtime.evaluate", { expression: ready(shown), awaitPromise: true, returnByValue: true }).then(
            (reply) => reply.result?.value === true,
            () => false,
          ),
          wait(PATIENCE).then(() => false),
        ]);
      await until(send, PAINTED, PATIENCE);
      await wait(PAUSE);
      return world ? "world" : "still";
    };
    const run = async (act: string) => {
      const { result, exceptionDetails } = await send(
        "Runtime.evaluate",
        { expression: act, returnByValue: true, awaitPromise: true },
        ACT,
      );
      if (exceptionDetails) fail(`  act FAILED: ${exceptionDetails.exception?.description ?? exceptionDetails.text}`);
      else if (result?.value !== undefined && result.value !== 0) console.log(`  act: ${JSON.stringify(result.value)}`);
    };
    const finish = async (route: string, file: (stage: string) => string, moved: boolean, tell: boolean, at: Clock) => {
      for (const one of keys) {
        await stroke(send, one);
        await wait(TAP);
      }
      if (!count) {
        if (moved || keys.length) await wait(SETTLE);
        await take(file(""), at === "world" ? " t=0s world" : "", tell, { route, n: 0, t: 0, clock: at });
        return;
      }
      const world = !LIVE && (await value(SEEK(0))) === true;
      if (world) await calm();
      const clock: Clock = world ? "world" : "wall";
      const from = Date.now();
      const tiles: Uint8Array[] = [];
      for (let n = 0; n < count; n++) {
        let ms = n * step;
        if (world) {
          if (n && (await value(SEEK(ms / 1000))) !== true) {
            fail(`shots: ${file(`-${n}`)} FAILED the world refused to seek to ${ms / 1000}s`);
            break;
          }
          await send("Runtime.evaluate", { expression: FRAMED, awaitPromise: true });
        } else {
          await wait(Math.max(0, ms - (Date.now() - from)));
          ms = Date.now() - from;
        }
        const t = ms / 1000;
        const one = count > 1 ? `-${n}` : "";
        tiles.push(await take(file(one), ` t=${t}s ${clock}`, tell && n === count - 1, { route, n, t, clock }));
      }
      if (count > 1) await contact(file("-sheet"), tiles);
    };
    const visit = async (turn: number, want: string) => {
      const [route = "/", act] = want.split("@");
      const born = first ? Number(await value("performance.timeOrigin")) : 0;
      const loaded = once("Page.loadEventFired").then(
        () => "",
        (error: Error) => error.message,
      );
      noise.length = 0;
      held.length = 0;
      holding = first;
      await send("Page.navigate", { url: `${origin}${route}` });
      let top = 0;
      if (first) {
        const early = name(route, -1, "-first");
        if (await until(send, painted(born), FIRST)) {
          await wait(PAUSE);
          top = Number(await value("Math.round(scrollY)"));
          await take(early, ` scroll=${top}`, true, { route, n: 0, t: 0, clock: "still" });
        } else fail(`shots: ${early} FAILED nothing parsed within ${FIRST / 1000} s with every script held`);
        holding = false;
        for (const id of held.splice(0)) pass(id);
      }
      const file = (stage: string) => name(route, !first && act ? turn : -1, first ? "-after" : stage);
      const lost = await loaded;
      if (lost) return fail(`shots: ${file("")} FAILED ${lost}`);
      const at = await settle();
      if (act) await run(act);
      if (!first) return finish(route, file, Boolean(act), true, at);
      if (act) await wait(SETTLE);
      const now = Number(await value("Math.round(scrollY)"));
      await take(file(""), ` scroll=${now}${now === top ? "" : " JUMP"}`, true, { route, n: 0, t: 0, clock: at });
    };
    const stroll = async () => {
      loads = 0;
      noise.length = 0;
      for (const [n, want] of pages.entries()) {
        const [to = "/", act] = want.split("@");
        const from = n ? String(await value(HERE)) : "";
        const since = noise.length;
        let how = "load";
        if (!n) {
          const loaded = once("Page.loadEventFired").then(
            () => "",
            (error: Error) => error.message,
          );
          await send("Page.navigate", { url: `${origin}${to}` });
          const lost = await loaded;
          if (lost) return fail(`walk: ${n} ${to} FAILED ${lost}`);
        } else if (to === "reload") {
          how = to;
          const loaded = once("Page.loadEventFired").then(
            () => "",
            (error: Error) => error.message,
          );
          await send("Page.reload");
          const lost = await loaded;
          if (lost) return fail(`walk: ${n} ${to} FAILED ${lost}`);
        } else if (to === "back" || to === "forward") {
          how = to;
          await send("Runtime.evaluate", {
            expression: `window.__popped = false; addEventListener("popstate", () => { window.__popped = true; }, { once: true }); history.${to}()`,
          });
          if (!(await until(send, moved(from), PATIENCE))) return fail(`walk: ${n} ${to} FAILED ${from} never left`);
        } else {
          const spot = String(await value(hop(to)));
          if (spot === "none") return fail(`walk: ${n} ${to} FAILED ${from} holds no link to it`);
          how = spot === "synthetic" ? spot : "click";
          if (how === "click") {
            const [x, y] = JSON.parse(spot) as [number, number];
            await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
            await send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", clickCount: 1 });
            await send("Input.dispatchMouseEvent", { type: "mouseReleased", x, y, button: "left", clickCount: 1 });
          }
          if (!(await until(send, landed(to, from), PATIENCE)))
            return fail(`walk: ${n} ${to} FAILED the ${how} on ${from} never landed`);
        }
        const at = await settle();
        if (act) await run(act);
        const here = String(await value(HERE));
        if (n === pages.length - 1) await finish(here, (stage) => name(here, -1, stage), true, false, at);
        else if (act) await wait(PAUSE);
        const over = await value(`${OVER}?.dataset.page ?? ""`);
        const fresh = new Set(noise.slice(since)).size;
        console.log(
          `walk: ${n} ${here} ${how} loads=${loads} scroll=${await value("Math.round(scrollY)")}${over ? ` dialog=${over}` : ""}${fresh ? ` ${fresh} NOISE` : ""}`,
        );
        said(since);
        await probed();
      }
      console.log(`walk: ${pages.length} hops, ${loads} page loads`);
    };
    await fit(width, height);
    if (walk) await stroll();
    for (const [turn, want] of walk ? [] : pages.entries()) await visit(turn, want);
    ws.close();
  } finally {
    for (const one of SIGNALS) process.off(one, quit);
    proc?.kill();
    await proc?.exited;
    for (let i = 0; i < 50 && alive(); i++) await wait(100);
    reap();
    for (let i = 0; i < 50 && alive(); i++) await wait(100);
    if (alive()) console.log("shots: WARNING a chrome process outlived the kill");
    if (!LIVE) rmSync(profile, { recursive: true, force: true });
    server?.stop(true);
  }
  writeFileSync(
    join(out, "run.json"),
    `${JSON.stringify({ origin, chrome, locale: LOCALE, zone: ZONE, screen, unit: "s", frames, failures }, null, 2)}\n`,
  );
  const tally = (verdict: Verdict) => frames.filter((one) => one.verdict === verdict).length;
  const tail = baseline ? "baseline written" : `${tally("DIFF")} differ from baseline, ${tally("new")} new`;
  console.log(`shots: ${frames.length} shots in ${out}, ${tail}, chrome killed`);
  return { out, chrome, frames, failures };
}

export function faults(shot: Pick<Shot, "frames" | "failures">, same = false): string[] {
  return [
    ...shot.failures.map((line) => line.trim()),
    ...shot.frames.flatMap((one) => [
      ...(one.overflow.length ? [`${one.file} OVERFLOW`] : []),
      ...(one.noise.length ? [`${one.file} NOISE`] : []),
      ...(same && (one.verdict === "DIFF" || one.verdict === "new") ? [`${one.file} ${one.verdict}`] : []),
    ]),
  ];
}

const VALUED = ["--js", "--theme", "--keys", "--count", "--every", "--run", "--screen"];

async function main(args: string[]) {
  const told = block(SITE as Record<string, unknown>);
  const flag = (name: string) => (args.includes(name) ? (args[args.indexOf(name) + 1] ?? "") : "");
  const asked = args.filter((a, i) => !a.startsWith("--") && !VALUED.includes(args[i - 1] ?? ""));
  if (args.includes("--count") && !(Number(flag("--count")) >= 1)) throw new Error("shots: --count takes 1 to 60");
  const shot = await shoot({
    root: HOME,
    dist: process.env.MRLY_DIST || undefined,
    routes: asked.length ? asked : told.routes,
    screen: args.includes("--screen") ? screened(flag("--screen")) : told.screen,
    run: flag("--run"),
    baseline: args.includes("--baseline"),
    print: args.includes("--print"),
    motion: args.includes("--motion"),
    nojs: args.includes("--nojs"),
    first: args.includes("--first"),
    walk: args.includes("--walk"),
    probe: flag("--js"),
    scheme: flag("--theme"),
    keys: flag("--keys") ? flag("--keys").split(",") : [],
    count: args.includes("--count") ? Number(flag("--count")) : 0,
    every: args.includes("--every") ? every(flag("--every")) : 1,
  });
  const bad = faults(shot, args.includes("--same"));
  if (bad.length) console.log(`shots: exit 1, ${bad.length} faults: ${bad.join("; ")}`);
  process.exit(bad.length ? 1 : 0);
}

if (import.meta.main) await main(process.argv.slice(2));
