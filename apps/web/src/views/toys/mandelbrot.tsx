import { Stack } from "mrlyui"
import { Fractal, Panel } from "../../components/Fractal"
import type { Shade } from "../../types"

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

export function Mandelbrot({ state }: { state: unknown }) {
  const s = state as State
  return (
    <Stack>
      <Fractal
        app="mandelbrot"
        shade={s.shade}
        grid={[s.settings.width, s.settings.height]}
        steps={s.steps}
      />
      <Panel app="mandelbrot" dials={s.settings} />
    </Stack>
  )
}
