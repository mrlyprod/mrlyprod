import { setter } from "../../builders.ts"
import { fractalBoard, fractalPanel } from "../../components/fractal.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

type State = {
  steps: number
  settings: {
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
  frame: { width: number; height: number; rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("mandelbrot")

export function mandelbrot(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="mandelbrot">
      <card key="board">
        {fractalBoard("mandelbrot", "mandelbrot", s.frame, { steps: s.steps, shade: s.shade })}
      </card>
      {fractalPanel(turn, s.settings)}
    </stack>
  )
}
