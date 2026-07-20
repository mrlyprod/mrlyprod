import { call, raster } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Raster, Send } from "../../types.ts"

type State = { now: number; glyph?: Raster; stage: { h: number; m: number }; work: { h: number; m: number } }

const div = (a: number, n: number) => Math.floor(a / n)

const mod = (a: number, n: number) => ((a % n) + n) % n

const pad = (n: number, width = 2) => String(n).padStart(width, "0")

function face(now: number): string {
  if (now === 0) return "--:--:--"
  const s = mod(div(now, 1000), 86400)
  return `${pad(div(s, 3600))}:${pad(div(mod(s, 3600), 60))}:${pad(mod(s, 60))}`
}

function civil(days: number): [number, number, number] {
  const z = days + 719468
  const era = div(z, 146097)
  const doe = mod(z, 146097)
  const yoe = div(doe - div(doe, 1460) + div(doe, 36524) - div(doe, 146096), 365)
  const doy = doe - (365 * yoe + div(yoe, 4) - div(yoe, 100))
  const mp = div(5 * doy + 2, 153)
  const day = doy - div(153 * mp + 2, 5) + 1
  const month = mp < 10 ? mp + 3 : mp - 9
  const year = yoe + era * 400 + (month <= 2 ? 1 : 0)
  return [year, month, day]
}

function date(now: number): string {
  if (now === 0) return "waiting for time"
  const [year, month, day] = civil(div(now, 86400000))
  return `${pad(year, 4)}-${pad(month)}-${pad(day)} utc`
}

export function clock(state: unknown, _send: Send): Node {
  const s = state as State
  const shown = s.glyph !== undefined ? raster("face", "clock", s.glyph) : <symbol key="face" as="glyph" value={face(s.now)} />
  return (
    <stack key="clock">
      <card key="face">
        {shown}
        <text key="date" role="note">{date(s.now)}</text>
      </card>
      <Section keyName="stage" label="duration">
        <text key="staged">{`${pad(s.stage.h)}:${pad(s.stage.m)}`}</text>
        <grid key="steppers" cols={2}>
          <button key="h-dec" call={call("clock.set", { key: "hour", value: Math.max(0, s.stage.h - 1) })}>- hr</button>
          <button key="h-inc" call={call("clock.set", { key: "hour", value: Math.min(23, s.stage.h + 1) })}>+ hr</button>
          <button key="m-dec" call={call("clock.set", { key: "minute", value: Math.max(0, s.stage.m - 1) })}>- min</button>
          <button key="m-inc" call={call("clock.set", { key: "minute", value: Math.min(59, s.stage.m + 1) })}>+ min</button>
        </grid>
      </Section>
    </stack>
  )
}
