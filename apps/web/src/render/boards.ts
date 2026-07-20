import type { Node } from "../types.ts"

type Board = Extract<Node, { kind: "Canvas" }>

const boards = new Map<HTMLCanvasElement, Board>()

export function remember(el: HTMLCanvasElement, node: Board): void {
  boards.set(el, node)
}

export function prune(): void {
  for (const held of boards.keys()) {
    if (!held.isConnected) boards.delete(held)
  }
}

export function entries(): [HTMLCanvasElement, Board][] {
  return Array.from(boards)
}

export function forget(el: HTMLCanvasElement): void {
  boards.delete(el)
}
