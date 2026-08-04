import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs"
import { tmpdir } from "node:os"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"

const root = fileURLToPath(new URL("..", import.meta.url))

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms))

// SITES

type Site = {
  port: number
  dir: string
  cmd: string[]
  settle: number
  enter?: boolean
  routes: (base: string) => string[] | Promise<string[]>
}

const vite = (port: number) => ["bun", "x", "--bun", "vite", "--port", String(port), "--strictPort"]

const SITES: Record<string, Site> = {
  web: {
    port: 3000,
    dir: "apps/web",
    cmd: ["bun", "run", "index.ts"],
    settle: 3000,
    enter: true,
    routes: () =>
      readdirSync(join(root, "apps/web/fixtures"))
        .filter(name => name.endsWith(".json"))
        .map(name => name.slice(0, -5)),
  },
  net: {
    port: 5173,
    dir: "apps/net",
    cmd: vite(5173),
    settle: 1500,
    routes: async base => {
      const site = (await (await fetch(`${base}/site.json`)).json()) as {
        products: Record<string, unknown>
        pages: Record<string, unknown>
      }
      return ["", ...Object.keys(site.products), ...Object.keys(site.pages)]
    },
  },
  git: {
    port: 5174,
    dir: "apps/git",
    cmd: vite(5174),
    settle: 1500,
    routes: () => [""],
  },
}

// CHROME

function chrome(): string {
  const fixed = [
    process.env["MRLY_CHROME"],
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
  ]
  for (const path of fixed) if (path !== undefined && existsSync(path)) return path
  for (const name of ["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"]) {
    const hit = Bun.which(name)
    if (hit !== null) return hit
  }
  throw new Error("no chrome or chromium found; set MRLY_CHROME to a binary")
}

let id = 0
const pending = new Map<number, (value: any) => void>()
const listeners: ((msg: any) => void)[] = []
let ws: WebSocket

function send(method: string, params = {}): Promise<any> {
  return new Promise(resolve => {
    id += 1
    pending.set(id, resolve)
    ws.send(JSON.stringify({ id, method, params }))
  })
}

function once(method: string, timeout: number): Promise<void> {
  return new Promise(resolve => {
    const done = (fn: (msg: any) => void) => {
      listeners.splice(listeners.indexOf(fn), 1)
      resolve()
    }
    const fn = (msg: any) => {
      if (msg.method === method) {
        clearTimeout(timer)
        done(fn)
      }
    }
    const timer = setTimeout(() => done(fn), timeout)
    listeners.push(fn)
  })
}

async function launch(width: number, height: number) {
  const profile = mkdtempSync(join(tmpdir(), "mrlyshot-"))
  const flags = [
    "--headless=new",
    "--remote-debugging-port=0",
    `--user-data-dir=${profile}`,
    `--window-size=${width},${height}`,
    "--hide-scrollbars",
    "--mute-audio",
    "--no-first-run",
    "--no-default-browser-check",
    "--use-angle=swiftshader",
  ]
  if (process.getuid?.() === 0) flags.push("--no-sandbox")
  const proc = Bun.spawn([chrome(), ...flags, "about:blank"], { stdout: "ignore", stderr: "ignore" })
  const portFile = join(profile, "DevToolsActivePort")
  let port = 0
  for (let i = 0; i < 100; i += 1) {
    if (existsSync(portFile)) {
      port = Number(readFileSync(portFile, "utf8").split("\n")[0])
      if (port > 0) break
    }
    await sleep(100)
  }
  if (port === 0) throw new Error("chrome did not expose a devtools port")
  const targets = (await (await fetch(`http://127.0.0.1:${port}/json/list`)).json()) as {
    type: string
    webSocketDebuggerUrl: string
  }[]
  const page = targets.find(t => t.type === "page")
  if (page === undefined) throw new Error("no page target")
  ws = new WebSocket(page.webSocketDebuggerUrl)
  ws.addEventListener("message", e => {
    const msg = JSON.parse(String(e.data))
    if (msg.id !== undefined) pending.get(msg.id)?.(msg.result)
    else for (const fn of [...listeners]) fn(msg)
  })
  await new Promise(r => ws.addEventListener("open", r))
  await send("Page.enable")
  await send("Emulation.setDeviceMetricsOverride", { width, height, deviceScaleFactor: 1, mobile: false })
  return () => {
    proc.kill()
    rmSync(profile, { recursive: true, force: true })
  }
}

// SHOOT

async function shoot(url: string, out: string, settle: number, enter: boolean, width: number, height: number) {
  const loaded = once("Page.loadEventFired", 10000)
  await send("Page.navigate", { url })
  await loaded
  await sleep(settle)
  if (enter) {
    const x = Math.floor(width / 2)
    const y = Math.floor(height / 2)
    await send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", clickCount: 1 })
    await send("Input.dispatchMouseEvent", { type: "mouseReleased", x, y, button: "left", clickCount: 1 })
    await sleep(800)
  }
  const shot = await send("Page.captureScreenshot", { format: "png" })
  mkdirSync(dirname(out), { recursive: true })
  await Bun.write(out, Buffer.from(shot.data, "base64"))
  console.log(`wrote ${out}`)
}

// SERVERS

async function alive(base: string): Promise<boolean> {
  try {
    return (await fetch(base, { signal: AbortSignal.timeout(1000) })).ok
  } catch {
    return false
  }
}

async function boot(site: Site): Promise<(() => void) | undefined> {
  const base = `http://localhost:${site.port}`
  if (await alive(base)) return undefined
  const proc = Bun.spawn(site.cmd, { cwd: join(root, site.dir), stdout: "ignore", stderr: "ignore" })
  for (let i = 0; i < 100; i += 1) {
    if (await alive(base)) return () => proc.kill()
    await sleep(300)
  }
  proc.kill()
  throw new Error(`${site.dir} did not come up on :${site.port}`)
}

// MAIN

function usage(): never {
  console.log("bun utils/shot.ts [--size WxH]                    every site, every route")
  console.log("bun utils/shot.ts [--size WxH] <site> [route...]  one of: " + Object.keys(SITES).join(" "))
  console.log("bun utils/shot.ts [--size WxH] <url> [out.png]    any url")
  process.exit(1)
}

let width = 1400
let height = 900
const args = process.argv.slice(2).filter((arg, i, all) => {
  if (arg === "--size") return false
  if (i > 0 && all[i - 1] === "--size") {
    const [w, h] = arg.split("x").map(Number)
    if (w === undefined || h === undefined || !(w > 0) || !(h > 0)) usage()
    width = w
    height = h
    return false
  }
  return true
})

const head = args[0]
if (head !== undefined && head.startsWith("http")) {
  const kill = await launch(width, height)
  await shoot(head, args[1] ?? join(root, "data/shot.png"), 1500, false, width, height)
  kill()
} else {
  const names = head === undefined ? Object.keys(SITES) : [head]
  for (const name of names) if (SITES[name] === undefined) usage()
  const kill = await launch(width, height)
  for (const name of names) {
    const site = SITES[name]
    if (site === undefined) continue
    const base = `http://localhost:${site.port}`
    const stop = await boot(site)
    const routes = args.length > 1 ? args.slice(1) : await site.routes(base)
    for (const route of routes) {
      const file = route === "" ? "home" : route.replaceAll("/", "-")
      await shoot(`${base}/${route}`, join(root, "data", name, `${file}.png`), site.settle, site.enter === true, width, height)
    }
    stop?.()
  }
  kill()
}
process.exit(0)
