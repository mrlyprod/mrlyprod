import type { Held, Node, Send } from "../types.ts"
import { create, patch } from "./nodes.ts"

let sender: Send | undefined

export function courier(): Send | undefined {
  return sender
}

function identity(node: Node, index: number): string {
  const id = `${node.kind}:${node.key ?? `#${index}`}`
  return node.kind === "Choice" ? `${id}:${node.mode ?? "select"}` : id
}

export function reconcile(parent: HTMLElement, nodes: Node[], send: Send): void {
  sender = send
  const existing = new Map<string, Held>()
  for (const child of Array.from(parent.children)) {
    const el = child as Held
    if (el.dataset.id !== undefined) existing.set(el.dataset.id, el)
  }
  const seasoned = existing.size > 0
  const els = nodes.map((node, index) => {
    const id = identity(node, index)
    const found = existing.get(id)
    if (found !== undefined) {
      existing.delete(id)
      return found
    }
    const el = create(node, send)
    el.dataset.id = id
    if (seasoned) enter(el)
    return el
  })
  for (const el of existing.values()) depart(el)
  els.forEach((el, index) => {
    const live = Array.from(parent.children).filter(child => !child.classList.contains("leaving"))
    const before = live[index] ?? null
    if (el !== before) parent.insertBefore(el, before)
    patch(el, nodes[index] as Node, send)
  })
}

function enter(el: Held): void {
  el.classList.add("enter")
  el.addEventListener("animationend", event => {
    if (event.target === el && event.animationName === "enter") el.classList.remove("enter")
  })
}

function depart(el: Held): void {
  delete el.dataset.id
  el.classList.add("leaving")
  el.addEventListener("animationend", event => {
    if (event.target === el && event.animationName === "leave") el.remove()
  })
  const pace = parseFloat(getComputedStyle(document.documentElement).getPropertyValue("--pace"))
  setTimeout(() => el.remove(), (Number.isNaN(pace) ? 600 : pace) + 200)
}
