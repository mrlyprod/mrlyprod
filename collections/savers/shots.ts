import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

type Reply = { id?: number; method?: string; params?: Params; result?: Result; error?: { message: string } };

type Params = { exceptionDetails?: Fault; args?: { value?: unknown; description?: string }[] };

type Result = { result?: { value?: unknown }; exceptionDetails?: Fault };

type Fault = { text: string; exception?: { description?: string } };

type Ease = { png: string; curve: number[]; frames: string[] };

const HERE = import.meta.dir;
const REPO = resolve(HERE, "..", "..");
const DATA_DIR = join(REPO, "data", relative(REPO, HERE));
const CHROME = process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const SCREEN = [1440, 900];
const CELL = [480, 300];
const CHIP = [240, 150];
const FRAME = [360, 225];
const SMALL = [720, 450];
const PHONE = [390, 844];
const WHEN = 20000;
const LATER = 90000;
const BARS = "▁▂▃▄▅▆▇█";
const BEAT = 30;
const PATIENCE = 15000;

const wait = (ms: number) => new Promise((done) => setTimeout(done, ms));

async function deadline<T>(job: () => Promise<T>, fail: () => void): Promise<T> {
  const late = wait(PATIENCE).then(() => {
    throw new Error("shots: chrome did not answer");
  });
  try {
    return await Promise.race([job(), late]);
  } catch (error) {
    fail();
    throw error;
  }
}

const png = (file: string, url: string) => writeFileSync(file, Buffer.from(url.slice(url.indexOf(",") + 1), "base64"));

async function build(name: string) {
  const out = join(DATA_DIR, name);
  mkdirSync(out, { recursive: true });
  const entry = join(out, "page.ts");
  const from = (file: string) => JSON.stringify(join(HERE, file));
  writeFileSync(
    entry,
    `import saver from ${from(`${name}/saver.ts`)};\nimport { boot } from ${from("page.ts")};\nboot(saver);\n`,
  );
  const page = await Bun.build({ entrypoints: [entry], outdir: out, format: "iife", target: "browser" });
  const lean = await Bun.build({ entrypoints: [join(HERE, name, "saver.ts")], target: "browser", minify: true });
  for (const log of [...page.logs, ...lean.logs]) console.log(String(log));
  if (!page.success || !lean.success) throw new Error(`shots: ${name} does not build`);
  writeFileSync(
    join(out, "page.html"),
    `<!doctype html><meta charset="utf-8"><title>${name}</title><style>html,body{margin:0;background:#000;overflow:hidden}</style><body><script src="page.js"></script>`,
  );
  return Bun.gzipSync(await lean.outputs[0].arrayBuffer()).length;
}

async function chrome() {
  const profile = join(DATA_DIR, `chrome-${process.pid}`);
  rmSync(profile, { recursive: true, force: true });
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
      `--user-data-dir=${profile}`,
      "--remote-debugging-port=0",
      "about:blank",
    ],
    { stdout: "ignore", stderr: "ignore" },
  );
  const quit = () => {
    proc.kill(9);
    rmSync(profile, { recursive: true, force: true });
  };
  process.on("SIGINT", () => {
    quit();
    process.exit(130);
  });
  const port = join(profile, "DevToolsActivePort");
  for (let i = 0; i < 100 && !existsSync(port); i++) await wait(100);
  if (!existsSync(port)) {
    quit();
    throw new Error(`shots: no chrome at ${CHROME}; set CHROME to the binary`);
  }
  const ws = await deadline(async () => {
    const base = `http://127.0.0.1:${readFileSync(port, "utf8").split("\n")[0]}`;
    const pages = (await (await fetch(`${base}/json`)).json()) as { type: string; webSocketDebuggerUrl: string }[];
    const socket = new WebSocket(pages.find((one) => one.type === "page")!.webSocketDebuggerUrl);
    await new Promise((done) => socket.addEventListener("open", done, { once: true }));
    return socket;
  }, quit);
  const waits = new Map<number, (reply: Reply) => void>();
  const noise: string[] = [];
  let id = 0;
  ws.addEventListener("message", ({ data }) => {
    const reply = JSON.parse(String(data)) as Reply;
    if (reply.id) waits.get(reply.id)?.(reply);
    const fault = reply.params?.exceptionDetails;
    if (fault) noise.push(fault.exception?.description ?? fault.text);
    if (reply.method === "Runtime.consoleAPICalled")
      noise.push((reply.params?.args ?? []).map((arg) => arg.value ?? arg.description).join(" "));
  });
  const send = (method: string, params: object = {}) =>
    new Promise<Result>((done, fail) => {
      waits.set(++id, (reply) =>
        reply.error ? fail(new Error(`${method}: ${reply.error.message}`)) : done(reply.result!),
      );
      ws.send(JSON.stringify({ id, method, params }));
    });
  const call = async <T>(expression: string) => {
    const { result, exceptionDetails: fault } = await send("Runtime.evaluate", {
      expression,
      awaitPromise: true,
      returnByValue: true,
    });
    if (fault) throw new Error(`shots: ${fault.exception?.description ?? fault.text}`);
    return result?.value as T;
  };
  await send("Runtime.enable");
  await send("Page.enable");
  return { send, call, noise, quit };
}

function spark(curve: number[]) {
  const beats = Array.from({ length: Math.ceil(curve.length / BEAT) }, (_, i) => {
    const part = curve.slice(i * BEAT, (i + 1) * BEAT);
    return part.reduce((sum, v) => sum + v, 0) / part.length;
  });
  const top = Math.max(...beats, 1e-9);
  return beats.map((v) => (v ? BARS[Math.min(BARS.length - 1, Math.floor((v / top) * BARS.length))] : " ")).join("");
}

const names = process.argv.slice(2).length
  ? process.argv.slice(2)
  : readdirSync(HERE).filter((name) => existsSync(join(HERE, name, "saver.ts")));
const sizes = new Map<string, number>();
for (const name of names) sizes.set(name, await build(name));
const { send, call, noise, quit } = await chrome();
try {
  for (const name of names) {
    const out = join(DATA_DIR, name);
    const open = async (dpr: number) => {
      const [width, height] = SCREEN;
      await send("Emulation.setDeviceMetricsOverride", { width, height, deviceScaleFactor: dpr, mobile: false });
      await send("Page.navigate", { url: "about:blank" });
      await send("Page.navigate", { url: `file://${join(out, "page.html")}#shots` });
      for (let i = 0; i < 100 && !(await call<boolean>("typeof shots === 'object'")); i++) await wait(50);
    };
    const size = (pair: number[]) => JSON.stringify(pair);
    await open(1);
    png(join(out, "looks.png"), await call(`shots.looks(${size(CELL)}, ${WHEN})`));
    png(join(out, "hues.png"), await call(`shots.hues(${size(CHIP)}, ${WHEN})`));
    png(join(out, "times.png"), await call(`shots.times(${size(CELL)})`));
    const ease = await call<Ease>(`shots.eases(${size(FRAME)})`);
    png(join(out, "ease.png"), ease.png);
    const still = await call<{ start: string; later: string }>(`shots.first({}, ${size(CELL)}, true)`);
    const moved = await call<{ start: string; later: string }>(`shots.first({}, ${size(CELL)})`);
    const other = await call<{ start: string }>(`shots.first({ seed: 2 }, ${size(CELL)})`);
    const grown = await call<{ fit: boolean; blank: boolean; png: string }>(
      `shots.resize(${size(CELL)}, ${size(SMALL)})`,
    );
    png(join(out, "still.png"), still.later);
    png(join(out, "resize.png"), grown.png);
    await open(2);
    const costs: number[] = [];
    for (const theme of ["dark", "light"]) {
      const { png: url, cost } = await call<{ png: string; cost: number }>(
        `shots.full({ theme: "${theme}" }, ${size(SCREEN)}, ${WHEN})`,
      );
      png(join(out, `${theme}.png`), url);
      costs.push(cost);
    }
    png(join(out, "small.png"), (await call<{ png: string }>(`shots.full({ seed: 5 }, ${size(SMALL)}, ${LATER})`)).png);
    png(join(out, "phone.png"), (await call<{ png: string }>(`shots.full({ seed: 6 }, ${size(PHONE)}, ${LATER})`)).png);
    const held = ease.frames.slice(14, 17);
    const checks = {
      "still under reduced motion": still.start === still.later && still.start === moved.start,
      "moves otherwise": moved.start !== moved.later,
      "seed changes the picture": moved.start !== other.start,
      "freeze holds": new Set(held).size === 1,
      "resume moves": ease.frames[17] !== ease.frames[20],
      "resize refits": grown.fit && !grown.blank,
    };
    console.log(`${name}: ${sizes.get(name)} bytes gzipped, ${Math.max(...costs).toFixed(1)} ms a frame at 2x`);
    console.log(`  motion ${spark(ease.curve)}`);
    for (const [what, ok] of Object.entries(checks)) console.log(`  ${ok ? "ok  " : "FAIL"} ${what}`);
    for (const line of new Set(noise.splice(0))) console.log(`  ${line}`);
    console.log(`  ${out}`);
  }
} finally {
  quit();
}
