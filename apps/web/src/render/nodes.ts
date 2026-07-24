import { spell } from "../glyphs.ts"
import { html } from "../kernel.ts"
import * as gpu from "../gpu.ts"
import { icon } from "../icons.ts"
import type { Call, Flip, Held, Node, Send, Sym } from "../types.ts"
import { prune, remember } from "./boards.ts"
import { scramble } from "./fx.ts"
import { markCanvas } from "./mark.ts"
import { make, paint } from "./paint.ts"
import { reconcile } from "./reconcile.ts"

// HELPERS

let render = "cpu"

export function engine(value: unknown): void {
  render = value === "gpu" ? "gpu" : "cpu"
}

function mark(sym: Sym): string {
  return sym.as === "icon" ? icon(sym.value) : sym.value
}

function fire(node: { call: Call; arg?: string }, value?: unknown): Call {
  if (node.arg === undefined) return node.call
  return { verb: node.call.verb, args: { ...node.call.args, [node.arg]: value } }
}

function held(el: Element): Node {
  return (el as Held).__node as Node
}

function labelled(tag: string, cls: string): Held {
  const el = make(tag, cls)
  el.append(make("span", "label"))
  return el
}

function relabel(el: Held, label: string | undefined): void {
  const span = el.querySelector("span.label") as HTMLElement
  span.textContent = label ?? ""
  span.style.display = label ? "" : "none"
}

// KINDS

export function create(node: Node, send: Send): Held {
  switch (node.kind) {
    case "Stack":
      return make("div", "list-box")
    case "Grid":
      return make("div", "grid-box")
    case "Card":
      return make("section", "mrly-box border-box")
    case "Pills":
      return make("div", "pills-box")
    case "Text":
      return make("p", "mrly-box text-box")
    case "Symbol": {
      const el = make("div", "mrly-box card-box emoji-container")
      el.append(make("span", "emoji-large"))
      return el
    }
    case "Label": {
      const el = make("button", "mrly-box label-box")
      el.append(make("span", "symbol"), make("span", "text"), make("span", "note"))
      el.addEventListener("click", () => {
        const node = held(el) as Extract<Node, { kind: "Label" }>
        if (node.href !== undefined) {
          const a = document.createElement("a")
          a.href = node.href
          if (node.href.startsWith("data:")) {
            a.download = node.text ?? "download"
          } else {
            a.target = "_blank"
            a.rel = "noopener"
          }
          a.click()
          return
        }
        if (node.call !== undefined) send(node.call)
      })
      return el
    }
    case "Image":
      return make("img", "image")
    case "Canvas": {
      const el = make("canvas", "mrly-box canvas")
      const cell = (event: PointerEvent): [number, number] | null => {
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        const [cols, rows] = node.grid ?? [node.rows[0]?.length ?? 0, node.rows.length]
        if (cols === 0 || rows === 0) return null
        const rect = el.getBoundingClientRect()
        const x = Math.floor(((event.clientX - rect.left) / rect.width) * cols)
        const y = Math.floor(((event.clientY - rect.top) / rect.height) * rows)
        return [Math.max(0, Math.min(cols - 1, x)), Math.max(0, Math.min(rows - 1, y))]
      }
      let trail: [number, number][] | null = null
      el.addEventListener("pointerdown", event => {
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        if (node.tap === undefined && node.drag === undefined) return
        const at = cell(event)
        if (at === null) return
        el.setPointerCapture(event.pointerId)
        trail = [at]
      })
      el.addEventListener("pointermove", event => {
        if (trail === null) return
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        if (node.drag === undefined) return
        const at = cell(event)
        const last = trail[trail.length - 1]
        if (at === null || (last !== undefined && at[0] === last[0] && at[1] === last[1])) return
        trail.push(at)
      })
      el.addEventListener("pointerup", () => {
        if (trail === null) return
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        const gesture = trail
        trail = null
        if (node.drag !== undefined && (gesture.length > 1 || node.tap === undefined)) {
          send({ verb: node.drag.verb, args: { ...node.drag.args, points: gesture } })
          return
        }
        if (node.tap !== undefined) {
          const [x, y] = gesture[0] as [number, number]
          send({ verb: node.tap.verb, args: { ...node.tap.args, x, y } })
        }
      })
      el.addEventListener("pointercancel", () => {
        trail = null
      })
      const TAU = Math.PI * 2
      type Grip = { pointers: Map<number, [number, number]>; turn: [number, number]; pan: [number, number]; zoom: number; panning: boolean }
      let grip: Grip | null = null
      let wheeling: ReturnType<typeof setTimeout> | null = null
      const steered = (): void => {
        if (grip === null) return
        gpu.steer(el as unknown as HTMLCanvasElement, {
          yaw: (grip.turn[0] * TAU) / 256,
          pitch: (grip.turn[1] * TAU) / 256,
          dist: grip.zoom / 4,
          panx: grip.pan[0] / 16,
          pany: grip.pan[1] / 16,
        })
      }
      const settle = (send: Send): void => {
        if (grip === null) return
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        const dyaw = Math.round(grip.turn[0])
        const dpitch = Math.round(grip.turn[1])
        const dx = Math.round(grip.pan[0])
        const dy = Math.round(grip.pan[1])
        const dz = Math.round(grip.zoom)
        grip = null
        gpu.steer(el as unknown as HTMLCanvasElement, null)
        if (node.turn !== undefined && (dyaw !== 0 || dpitch !== 0)) {
          send({ verb: node.turn.verb, args: { ...node.turn.args, dyaw, dpitch } })
        }
        if (node.pan !== undefined && (dx !== 0 || dy !== 0)) {
          send({ verb: node.pan.verb, args: { ...node.pan.args, dx, dy } })
        }
        if (node.zoom !== undefined && dz !== 0) {
          send({ verb: node.zoom.verb, args: { ...node.zoom.args, dir: dz < 0 ? "in" : "out", n: Math.abs(dz) } })
        }
      }
      el.addEventListener("pointerdown", event => {
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        if (node.turn === undefined && node.pan === undefined && node.zoom === undefined) return
        el.setPointerCapture(event.pointerId)
        grip ??= { pointers: new Map(), turn: [0, 0], pan: [0, 0], zoom: 0, panning: event.shiftKey }
        grip.pointers.set(event.pointerId, [event.clientX, event.clientY])
      })
      el.addEventListener("pointermove", event => {
        if (grip === null) return
        const node = held(el) as Extract<Node, { kind: "Canvas" }>
        const prev = grip.pointers.get(event.pointerId)
        if (prev === undefined) return
        const rect = el.getBoundingClientRect()
        if (rect.width === 0) return
        const now: [number, number] = [event.clientX, event.clientY]
        if (grip.pointers.size === 2) {
          const other = [...grip.pointers.entries()].find(([id]) => id !== event.pointerId)
          if (other !== undefined) {
            const o = other[1]
            const before = Math.hypot(prev[0] - o[0], prev[1] - o[1])
            const after = Math.hypot(now[0] - o[0], now[1] - o[1])
            if (node.zoom !== undefined) grip.zoom += ((before - after) / rect.width) * 16
            if (node.pan !== undefined) {
              grip.pan[0] += ((now[0] - prev[0]) / 2 / rect.width) * 40
              grip.pan[1] -= ((now[1] - prev[1]) / 2 / rect.width) * 40
            }
          }
        } else if (grip.panning && node.pan !== undefined) {
          grip.pan[0] += ((now[0] - prev[0]) / rect.width) * 40
          grip.pan[1] -= ((now[1] - prev[1]) / rect.width) * 40
        } else if (node.turn !== undefined) {
          grip.turn[0] += ((now[0] - prev[0]) / rect.width) * 128
          grip.turn[1] -= ((now[1] - prev[1]) / rect.width) * 128
        }
        grip.pointers.set(event.pointerId, now)
        steered()
      })
      const release = (event: PointerEvent): void => {
        if (grip === null) return
        grip.pointers.delete(event.pointerId)
        if (grip.pointers.size === 0) settle(send)
      }
      el.addEventListener("pointerup", release)
      el.addEventListener("pointercancel", release)
      el.addEventListener(
        "wheel",
        event => {
          const node = held(el) as Extract<Node, { kind: "Canvas" }>
          if (node.zoom === undefined) return
          event.preventDefault()
          grip ??= { pointers: new Map(), turn: [0, 0], pan: [0, 0], zoom: 0, panning: false }
          grip.zoom += event.deltaY / 100
          steered()
          if (wheeling !== null) clearTimeout(wheeling)
          wheeling = setTimeout(() => {
            wheeling = null
            if (grip !== null && grip.pointers.size === 0) settle(send)
          }, 250)
        },
        { passive: false },
      )
      return el
    }
    case "Button": {
      const el = make("button", "mrly-box link-box")
      el.addEventListener("click", () => {
        const node = held(el) as Extract<Node, { kind: "Button" }>
        if (node.press === undefined && node.call !== undefined) send(node.call)
      })
      let down = false
      el.addEventListener("pointerdown", () => {
        const node = held(el) as Extract<Node, { kind: "Button" }>
        if (node.press === undefined) return
        down = true
        send(node.press)
      })
      const up = () => {
        if (!down) return
        down = false
        const node = held(el) as Extract<Node, { kind: "Button" }>
        if (node.lift !== undefined) send(node.lift)
      }
      el.addEventListener("pointerup", up)
      el.addEventListener("pointerleave", up)
      el.addEventListener("pointercancel", up)
      return el
    }
    case "Field": {
      const el = labelled("label", "mrly-box form-box")
      el.prepend(make("span", "icon"))
      const input = document.createElement("input") as HTMLInputElement & { __committed?: string }
      input.type = "text"
      el.append(input)
      const wipe = make("button", "icon clear")
      wipe.textContent = icon("close")
      el.append(wipe)
      wipe.addEventListener("click", event => {
        event.preventDefault()
        const node = held(el) as Extract<Node, { kind: "Field" }>
        input.value = ""
        input.__committed = ""
        send(fire(node, ""))
      })
      const commit = () => {
        const node = held(el) as Extract<Node, { kind: "Field" }>
        if (input.value === (input.__committed ?? node.value)) return
        input.__committed = input.value
        send(fire(node, input.value))
      }
      input.addEventListener("input", () => {
        const node = held(el) as Extract<Node, { kind: "Field" }>
        if (node.live) send(fire(node, input.value))
      })
      input.addEventListener("keydown", event => {
        if (event.key !== "Enter") return
        const node = held(el) as Extract<Node, { kind: "Field" }>
        commit()
        if (node.enter !== undefined) send(node.enter)
      })
      input.addEventListener("blur", commit)
      return el
    }
    case "Toggle": {
      const el = labelled("label", "mrly-box setting-box toggle")
      const input = document.createElement("input")
      input.type = "checkbox"
      el.append(input)
      input.addEventListener("change", () => send(fire(held(el) as Extract<Node, { kind: "Toggle" }>, input.checked)))
      return el
    }
    case "Choice": {
      if (node.mode === "row") {
        const el = make("div", "choice-row")
        el.append(make("span", "mrly-box info-box muted label"))
        return el
      }
      if (node.mode === "cycle") {
        const el = make("button", "mrly-box link-box cycle")
        el.addEventListener("click", () => {
          const node = held(el) as Extract<Node, { kind: "Choice" }>
          const next = node.options[(node.options.indexOf(node.value) + 1) % node.options.length]
          send(fire(node, next))
        })
        return el
      }
      const el = labelled("label", "mrly-box setting-box choice")
      const select = document.createElement("select")
      el.append(select)
      select.addEventListener("change", () => send(fire(held(el) as Extract<Node, { kind: "Choice" }>, select.value)))
      return el
    }
    case "Range": {
      const el = labelled("label", "mrly-box setting-box range")
      const input = document.createElement("input")
      input.type = "range"
      const value = make("span", "value")
      el.append(input, value)
      input.addEventListener("input", () => {
        value.textContent = input.value
      })
      input.addEventListener("change", () => send(fire(held(el) as Extract<Node, { kind: "Range" }>, Number(input.value))))
      return el
    }
    case "Overlay": {
      const el = make("div", "overlay-box")
      el.addEventListener("click", event => {
        if (event.target !== el) return
        const node = held(el) as Extract<Node, { kind: "Overlay" }>
        if (node.close !== undefined) send(node.close)
      })
      return el
    }
    case "Cell": {
      const el = make("button", "mrly-box cell-box")
      el.addEventListener("click", () => {
        const node = held(el) as Extract<Node, { kind: "Cell" }>
        if (node.call !== undefined) send(node.call)
      })
      return el
    }
    case "Doc": {
      const el = make("article", "mrly-box doc-box")
      el.addEventListener("click", event => {
        const hit = (event.target as HTMLElement).closest("a[data-slug]") as HTMLElement | null
        if (hit === null) return
        event.preventDefault()
        const node = held(el) as Extract<Node, { kind: "Doc" }>
        const slug = hit.dataset.slug
        if (node.open !== undefined && slug !== undefined) send({ verb: node.open.verb, args: { ...node.open.args, slug } })
      })
      return el
    }
    case "Cells":
      return make("div", "text-grid")
    case "Mark":
      return markCanvas(node.doodle)
  }
}

export function patch(el: Held, node: Node, send: Send): void {
  el.__node = node
  switch (node.kind) {
    case "Stack":
    case "Pills":
      reconcile(el, node.children, send)
      break
    case "Card":
      el.style.setProperty("--tint", paint(el))
      reconcile(el, node.children, send)
      break
    case "Grid":
      el.style.setProperty("--cols", String(node.cols))
      el.classList.toggle("snap", node.mode === "snap")
      reconcile(el, node.children, send)
      break
    case "Text": {
      const role = node.role ?? "body"
      el.className = role === "title" ? "mrly-box text-box title" : role === "body" ? "mrly-box text-box" : "mrly-box info-box muted"
      if (node.fx === "scramble") {
        if (el.__committed !== node.text) {
          el.__committed = node.text
          scramble(el, node.text)
        }
        break
      }
      if (el.textContent !== node.text) el.textContent = node.text
      break
    }
    case "Symbol": {
      const glyph = el.querySelector("span.emoji-large") as HTMLElement
      glyph.className = `emoji-large ${node.as}`
      if (node.as === "glyph") {
        spell(glyph, node.value)
        break
      }
      delete glyph.dataset.glyphs
      const value = mark(node)
      if (glyph.textContent !== value) glyph.textContent = value
      break
    }
    case "Label": {
      el.className = `mrly-box label-box ${node.mode}`
      const symbol = el.querySelector("span.symbol") as HTMLElement
      const text = el.querySelector("span.text") as HTMLElement
      const sym = node.mode === "text" ? undefined : node.symbol
      symbol.className = sym === undefined ? "symbol" : `symbol ${sym.as}`
      if (sym !== undefined && sym.as === "glyph") {
        spell(symbol, sym.value)
        symbol.style.display = ""
      } else {
        delete symbol.dataset.glyphs
        const face = sym === undefined ? "" : mark(sym)
        if (symbol.textContent !== face) symbol.textContent = face
        symbol.style.display = face === "" ? "none" : ""
      }
      const words = node.mode === "icon" ? "" : (node.text ?? "")
      if (node.fx === "scramble") {
        if (el.__committed !== words) {
          el.__committed = words
          scramble(text, words)
        }
      } else if (text.textContent !== words) text.textContent = words
      text.style.display = words === "" ? "none" : ""
      const note = el.querySelector("span.note") as HTMLElement
      const gloss = node.note ?? ""
      if (note.textContent !== gloss) note.textContent = gloss
      note.style.display = gloss === "" ? "none" : ""
      if (node.mode === "icon" && node.text !== undefined) el.setAttribute("aria-label", node.text)
      else el.removeAttribute("aria-label")
      el.title = node.note ?? ""
      if (node.call !== undefined) el.dataset.call = node.call.verb
      else delete el.dataset.call
      if (node.href !== undefined) el.dataset.href = "1"
      else delete el.dataset.href
      el.tabIndex = node.call !== undefined || node.href !== undefined ? 0 : -1
      break
    }
    case "Image": {
      const img = el as unknown as HTMLImageElement
      if (img.getAttribute("src") !== node.src) img.src = node.src
      img.alt = node.alt
      break
    }
    case "Canvas": {
      el.dataset.handle = node.handle
      const surface = el as unknown as HTMLCanvasElement
      prune()
      remember(surface, node)
      const height = node.rows.length
      const width = node.rows[0]?.length ?? 0
      const [cols, lines] = node.grid ?? [width, height]
      surface.style.aspectRatio = `${Math.max(cols, 1)} / ${Math.max(lines, 1)}`
      surface.style.touchAction =
        node.tap !== undefined || node.drag !== undefined || node.turn !== undefined || node.pan !== undefined || node.zoom !== undefined
          ? "none"
          : ""
      const dark = document.body.classList.contains("darkmode")
      const sig = `${render}:${dark}:${node.shade?.uniforms.join(",") ?? ""}:${node.palette?.join(",") ?? ""}:${node.rows.map(row => row.join(",")).join(";")}`
      if (el.__committed === sig) break
      el.__committed = sig
      if (node.shade !== undefined && render === "gpu" && gpu.draw(surface, node)) break
      if (node.strip !== undefined && node.strip.length > 1 && !calm.matches) {
        play(surface, node.strip)
        break
      }
      halt(surface)
      fill(surface, node.rows, node.palette)
      break
    }
    case "Button":
      el.className = node.big ? "mrly-box link-box swatch-big" : "mrly-box link-box"
      el.classList.toggle("active", node.active === true)
      ;(el as HTMLButtonElement).disabled = node.call === undefined && node.press === undefined
      if (el.textContent !== node.label) el.textContent = node.label
      el.style.backgroundColor = node.bg ?? ""
      if (node.press !== undefined) el.dataset.press = "1"
      else delete el.dataset.press
      el.style.touchAction = node.press !== undefined ? "none" : ""
      break
    case "Field": {
      const input = el.querySelector("input") as HTMLInputElement & { __committed?: string }
      relabel(el, node.label)
      const glyph = el.querySelector("span.icon") as HTMLElement
      const face = node.icon === undefined ? "" : icon(node.icon)
      if (glyph.textContent !== face) glyph.textContent = face
      glyph.style.display = face === "" ? "none" : ""
      el.classList.toggle("lead", face !== "")
      const wipe = el.querySelector("button.clear") as HTMLElement
      wipe.style.display = node.clear === true && node.value !== "" ? "" : "none"
      input.placeholder = node.hint ?? ""
      if (document.activeElement !== input) input.value = node.value
      input.__committed = node.value
      break
    }
    case "Toggle": {
      const input = el.querySelector("input") as HTMLInputElement
      relabel(el, node.label)
      input.checked = node.on
      break
    }
    case "Choice": {
      if (node.mode === "row") {
        const span = el.querySelector("span.label") as HTMLElement
        relabel(el, node.label)
        const options = node.options.join(" ")
        if (el.dataset.options !== options) {
          el.dataset.options = options
          el.replaceChildren(
            span,
            ...node.options.map(option => {
              const item = make("button", "mrly-box link-box")
              item.textContent = option
              item.addEventListener("click", () => send(fire(held(el) as Extract<Node, { kind: "Choice" }>, option)))
              return item
            }),
          )
        }
        for (const item of el.querySelectorAll("button")) item.classList.toggle("active", item.textContent === node.value)
        break
      }
      if (node.mode === "cycle") {
        const reading = node.label === undefined ? node.value : `${node.label}: ${node.value}`
        if (el.textContent !== reading) el.textContent = reading
        break
      }
      const select = el.querySelector("select") as HTMLSelectElement
      relabel(el, node.label)
      const options = node.options.join(" ")
      if (el.dataset.options !== options) {
        el.dataset.options = options
        select.replaceChildren(
          ...node.options.map(option => {
            const item = document.createElement("option")
            item.value = option
            item.textContent = option
            return item
          }),
        )
      }
      select.value = node.value
      break
    }
    case "Range": {
      const input = el.querySelector("input") as HTMLInputElement
      const value = el.querySelector("span.value") as HTMLElement
      relabel(el, node.label)
      input.min = String(node.min)
      input.max = String(node.max)
      input.step = String(node.step ?? 1)
      if (document.activeElement !== input) input.value = String(node.value)
      value.textContent = input.value
      break
    }
    case "Overlay":
      reconcile(el, [node.child], send)
      break
    case "Cell": {
      el.classList.toggle("on", node.on === true)
      el.style.backgroundColor = node.bg ?? ""
      const button = el as unknown as HTMLButtonElement
      button.disabled = node.call === undefined
      reconcile(el, node.child === undefined ? [] : [node.child], send)
      break
    }
    case "Doc": {
      if (node.handle !== undefined) el.dataset.handle = node.handle
      else delete el.dataset.handle
      const sig = node.code !== undefined ? `code:${node.code}` : `md:${node.md}`
      if (el.__committed !== sig) {
        el.__committed = sig
        el.classList.toggle("code", node.code !== undefined)
        if (node.code !== undefined) el.textContent = node.code
        else el.innerHTML = html(node.md)
      }
      break
    }
    case "Cells": {
      el.style.setProperty("--cols", String(node.rows[0]?.length ?? 0))
      el.style.setProperty("--rows", String(node.rows.length))
      const sig = node.rows.map(row => row.join("")).join("\n")
      if (el.__committed === sig) break
      el.__committed = sig
      const cells: HTMLElement[] = []
      for (const row of node.rows) {
        for (const ch of row) {
          const cell = make("div", "text-cell")
          cell.textContent = ch
          cells.push(cell)
        }
      }
      el.replaceChildren(...cells)
      break
    }
    case "Mark":
      break
  }
}

const BEAT = 125
const calm = matchMedia("(prefers-reduced-motion: reduce)")
const reels = new WeakMap<HTMLCanvasElement, number>()

function halt(surface: HTMLCanvasElement): void {
  const id = reels.get(surface)
  if (id !== undefined) cancelAnimationFrame(id)
  reels.delete(surface)
}

function play(surface: HTMLCanvasElement, strip: Flip[]): void {
  halt(surface)
  const from = performance.now()
  let shown = -1
  const step = () => {
    reels.delete(surface)
    if (!surface.isConnected) return
    const t = (performance.now() - from) / BEAT
    const i = Math.min(strip.length - 1, Math.floor(t * strip.length))
    const frame = strip[i]
    if (frame !== undefined && i !== shown) {
      shown = i
      fill(surface, frame.rows, frame.palette)
    }
    if (i < strip.length - 1) reels.set(surface, requestAnimationFrame(step))
  }
  step()
}

function fill(surface: HTMLCanvasElement, rows: number[][], palette?: string[]): void {
  const height = rows.length
  const width = rows[0]?.length ?? 0
  if (surface.width !== width) surface.width = width
  if (surface.height !== height) surface.height = height
  const ctx = surface.getContext("2d")
  if (ctx === null || width === 0) return
  ctx.clearRect(0, 0, width, height)
  if (palette !== undefined) {
    for (let y = 0; y < height; y++) {
      const row = rows[y] as number[]
      for (let x = 0; x < width; x++) {
        ctx.fillStyle = palette[row[x] as number] ?? "#000000"
        ctx.fillRect(x, y, 1, 1)
      }
    }
    return
  }
  ctx.fillStyle = getComputedStyle(surface).color
  for (let y = 0; y < height; y++) {
    const row = rows[y] as number[]
    for (let x = 0; x < width; x++) {
      const cell = row[x] as number
      if (cell === 0) continue
      ctx.globalAlpha = cell / 255
      ctx.fillRect(x, y, 1, 1)
    }
  }
  ctx.globalAlpha = 1
}
