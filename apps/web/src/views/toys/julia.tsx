import { Choice, Field, Section, Slider, Stack } from "mrlyui"
import { setter } from "../../builders"
import { Fractal, Panel } from "../../components/Fractal"
import { opts } from "../../components/options"
import { useSend } from "../../send"
import type { Shade } from "../../types"

const FEMTO = 1e15

const PRESETS = [
  "-0.4+0.6i",
  "-0.8+0.156i",
  "0.285+0.01i",
  "-0.727+0.189i",
  "-0.1+0.651i",
  "0.355+0.355i",
  "custom",
]

type State = {
  steps: number
  settings: {
    width: number
    height: number
    preset: string
    cre: number
    cim: number
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

export function Julia({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("julia")
  return (
    <Stack>
      <Fractal app="julia" shade={s.shade} grid={[s.settings.width, s.settings.height]} steps={s.steps} />
      <Section label="seed">
        <Choice
          label="preset"
          value={s.settings.preset}
          options={opts(PRESETS)}
          onChange={v => send(turn("preset", v))}
        />
        <Field label="cre">
          <Slider
            min={-2}
            max={2}
            step={0.01}
            value={s.settings.cre / FEMTO}
            format={v => v.toFixed(2)}
            onChange={v => send(turn("cre", Math.round(v * FEMTO)))}
          />
        </Field>
        <Field label="cim">
          <Slider
            min={-2}
            max={2}
            step={0.01}
            value={s.settings.cim / FEMTO}
            format={v => v.toFixed(2)}
            onChange={v => send(turn("cim", Math.round(v * FEMTO)))}
          />
        </Field>
      </Section>
      <Panel app="julia" dials={s.settings} />
    </Stack>
  )
}
