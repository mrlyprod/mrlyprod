import { setter } from "../../builders.ts"
import { fractalPanel, shadeBoard } from "../../components/fractal.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

type State = {
  steps: number
  settings: {
    width: number
    height: number
    zoom: number
    cycle: number
    band: number
    drift: number
    fade: number
    spin: number
    depth: number
    primary: string
    accent: string
  }
  shade?: Shade
}

const turn = setter("mandelbrot")

export function mandelbrot(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="mandelbrot">
      <card key="board">
        {shadeBoard("mandelbrot", s.shade, [s.settings.width, s.settings.height], s.steps)}
      </card>
      {fractalPanel(turn, s.settings)}
    </stack>
  )
}
