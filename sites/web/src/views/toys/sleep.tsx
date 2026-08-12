import { Field, Section, Slider, Stack } from "mrlyui"
import { setter } from "../../builders"
import { Fractal } from "../../components/Fractal"
import { Palette } from "../../components/Palette"
import { useSend } from "../../send"
import type { Shade } from "../../types"

type State = {
  steps: number
  settings: { cols: number; rows: number; size: number; speed: number; palette: string[] }
  shade?: Shade
}

export function Sleep({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("sleep")
  return (
    <Stack>
      <Fractal app="sleep" shade={s.shade} grid={[s.settings.cols, s.settings.rows]} steps={s.steps} />
      <Section label="field">
        <Field label="cols">
          <Slider min={8} max={64} step={1} value={s.settings.cols} onChange={v => send(turn("cols", v))} />
        </Field>
        <Field label="rows">
          <Slider min={8} max={48} step={1} value={s.settings.rows} onChange={v => send(turn("rows", v))} />
        </Field>
        <Field label="size">
          <Slider min={2} max={12} step={1} value={s.settings.size} onChange={v => send(turn("size", v))} />
        </Field>
      </Section>
      <Section label="drift">
        <Field label="speed">
          <Slider
            min={0.1}
            max={2}
            step={0.1}
            value={s.settings.speed / 100}
            format={v => v.toFixed(1)}
            onChange={v => send(turn("speed", Math.round(v * 100)))}
          />
        </Field>
      </Section>
      <Section label="palette">
        <Palette app="sleep" colors={s.settings.palette} />
      </Section>
    </Stack>
  )
}
