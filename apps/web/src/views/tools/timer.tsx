import { call, raster } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Raster, Send } from "../../types.ts"

const MODES = ["countdown", "stopwatch"]

const PRESETS = [1, 3, 5, 10]

const STEPS: { label: string; delta: number }[] = [
  { label: "-1h", delta: -60 },
  { label: "-1m", delta: -1 },
  { label: "+1m", delta: 1 },
  { label: "+1h", delta: 60 },
]

const clamp = (n: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, n))

type State = {
  mode: string
  armed: boolean
  remaining: number
  rung: boolean
  running: boolean
  elapsed: number
  laps: number[]
  glyph?: Raster
}

const pad = (n: number) => String(n).padStart(2, "0")

const clock = (ms: number) => `${pad(Math.floor(ms / 60000))}:${pad(Math.floor(ms / 1000) % 60)}`

function face(s: State): string {
  if (s.mode === "stopwatch") return clock(s.elapsed)
  if (!s.armed) return "--:--"
  const secs = Math.floor((s.remaining + 999) / 1000)
  return `${pad(Math.floor(secs / 60))}:${pad(secs % 60)}`
}

function status(s: State): string {
  if (s.mode === "stopwatch") return !s.armed ? "ready" : s.running ? "running" : "paused"
  if (!s.armed) return "set a timer"
  if (s.rung) return "time is up"
  return s.running ? "running" : "paused"
}

function controls(s: State): Node[] {
  if (s.mode === "stopwatch") {
    return [
      <button key="start" call={call("timer.start")}>start</button>,
      s.running ? <button key="lap" call={call("timer.lap")}>lap</button> : null,
      s.running
        ? <button key="pause" call={call("timer.pause")}>pause</button>
        : s.armed
          ? <button key="resume" call={call("timer.resume")}>resume</button>
          : null,
      <button key="clear" call={call("timer.clear")}>clear</button>,
    ].filter((node): node is Node => node !== null)
  }
  const minutes = s.armed ? Math.max(1, Math.ceil(s.remaining / 60000)) : 0
  return [
    <field key="minutes" value="" live={false} call={call("timer.start")} arg="minutes" hint="minutes" />,
    <grid key="presets" cols={4}>
      {PRESETS.map(m => (
        <button key={`m${m}`} call={call("timer.start", { minutes: m })}>{`${m}m`}</button>
      ))}
    </grid>,
    <grid key="steps" cols={4}>
      {STEPS.map(step => {
        const target = clamp(minutes + step.delta, 1, 1440)
        return <button key={step.label} call={call("timer.set", { key: "duration", value: { h: Math.floor(target / 60), m: target % 60 } })}>{step.label}</button>
      })}
    </grid>,
    s.running ? <button key="pause" call={call("timer.pause")}>pause</button> : null,
    !s.running && s.armed && !s.rung
      ? <button key="resume" call={call("timer.resume")}>resume</button>
      : null,
    <button key="clear" call={call("timer.clear")}>clear</button>,
  ].filter((node): node is Node => node !== null)
}

export function timer(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="timer">
      <card key="panel">
        {s.glyph !== undefined ? raster("face", "timer", s.glyph) : <symbol key="face" as="glyph" value={face(s)} />}
        <text key="status" role="note">{status(s)}</text>
      </card>
      <card key="controls">{controls(s)}</card>
      {s.laps.length > 0 && (
        <card key="laps">
          {s.laps.map((ms, i) => (
            <text key={`lap${i}`} role="note">{`lap ${i + 1} · ${clock(ms)}`}</text>
          ))}
        </card>
      )}
      <card key="settings">
        <choice key="mode" value={s.mode} options={MODES} call={call("timer.mode")} arg="mode" label="mode" mode="row" />
      </card>
    </stack>
  )
}
