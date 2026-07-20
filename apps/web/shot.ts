export {}

const chrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
const app = process.argv[2] ?? "clock"
const out = process.argv[3] ?? `data/web/${app}.png`

const proc = Bun.spawn([
  chrome,
  "--headless=new",
  "--remote-debugging-port=9333",
  "--window-size=1400,900",
  "--user-data-dir=/tmp/shot-profile",
  "about:blank",
])
await new Promise(r => setTimeout(r, 1500))

const targets = (await (await fetch("http://localhost:9333/json")).json()) as { type: string; webSocketDebuggerUrl: string }[]
const page = targets.find(t => t.type === "page")
if (page === undefined) throw new Error("no page target")

const ws = new WebSocket(page.webSocketDebuggerUrl)
let id = 0
const pending = new Map<number, (v: any) => void>()
ws.addEventListener("message", e => {
  const msg = JSON.parse(String(e.data))
  if (msg.id !== undefined) pending.get(msg.id)?.(msg.result)
})
const send = (method: string, params = {}) =>
  new Promise<any>(resolve => {
    id += 1
    pending.set(id, resolve)
    ws.send(JSON.stringify({ id, method, params }))
  })
await new Promise(r => ws.addEventListener("open", r))

await send("Emulation.setDeviceMetricsOverride", { width: 1400, height: 900, deviceScaleFactor: 1, mobile: false })
await send("Page.navigate", { url: `http://localhost:3000/${app}` })
await new Promise(r => setTimeout(r, 3000))
await send("Input.dispatchMouseEvent", { type: "mousePressed", x: 700, y: 450, button: "left", clickCount: 1 })
await send("Input.dispatchMouseEvent", { type: "mouseReleased", x: 700, y: 450, button: "left", clickCount: 1 })
await new Promise(r => setTimeout(r, 800))
const shot = await send("Page.captureScreenshot", { format: "png" })
await Bun.write(out, Uint8Array.from(atob(shot.data), c => c.charCodeAt(0)))
console.log(`wrote ${out}`)
proc.kill()
process.exit(0)
