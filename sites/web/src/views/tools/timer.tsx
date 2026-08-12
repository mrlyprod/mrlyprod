import {
  Box,
  Button,
  Caption,
  Choice,
  Grid,
  Letters,
  Section,
  Setting,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { opts } from "../../components/options"
import { useSend } from "../../send"

const MODES = ["countdown", "stopwatch"]

const PRESETS = [1, 3, 5, 10]

const STEPS: { label: string; delta: number }[] = [
  { label: "-1h", delta: -60 },
  { label: "-1m", delta: -1 },
  { label: "+1m", delta: 1 },
  { label: "+1h", delta: 60 },
]

const clamp = (n: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, n))

const pad = (n: number) => String(n).padStart(2, "0")

type State = {
  mode: string
  armed: boolean
  remaining: number
  rung: boolean
  running: boolean
  elapsed: number
  laps: number[]
}

// FACE

const clock = (ms: number) => `${pad(Math.floor(ms / 60000))}:${pad(Math.floor(ms / 1000) % 60)}`

const fine = (ms: number) => `${clock(ms)}.${pad(Math.floor((ms % 1000) / 10))}`

function face(s: State): string {
  if (s.mode === "stopwatch") return clock(s.elapsed)
  if (!s.armed) return "--:--"
  return clock(Math.ceil(s.remaining / 1000) * 1000)
}

function status(s: State): string {
  if (s.mode === "stopwatch") return !s.armed ? "ready" : s.running ? "running" : "paused"
  if (!s.armed) return "set a timer"
  if (s.rung) return "time is up"
  return s.running ? "running" : "paused"
}

// TIMER

export function Timer({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const watch = s.mode === "stopwatch"
  const minutes = s.armed ? Math.max(1, Math.ceil(s.remaining / 60000)) : 0
  const hold = (target: number) =>
    call("timer.set", { key: "duration", value: { h: Math.floor(target / 60), m: target % 60 } })
  return (
    <Stack>
      <Box>
        <Letters text={face(s)} scramble={false} />
        <Caption>{status(s)}</Caption>
        <Caption>{fine(watch ? s.elapsed : s.remaining)}</Caption>
      </Box>
      <Box>
        {watch ? (
          <Grid cols={3}>
            {s.running ? (
              <Button onClick={() => send(call("timer.pause"))}>pause</Button>
            ) : (
              <Button primary onClick={() => send(call(s.armed ? "timer.resume" : "timer.start"))}>
                {s.armed ? "resume" : "start"}
              </Button>
            )}
            <Button disabled={!s.running} onClick={() => send(call("timer.lap"))}>
              lap
            </Button>
            <Button onClick={() => send(call("timer.clear"))}>clear</Button>
          </Grid>
        ) : (
          <Stack>
            <Grid cols={4}>
              {PRESETS.map(m => (
                <Button
                  key={m}
                  active={s.armed && !s.rung && minutes === m}
                  onClick={() => send(call("timer.start", { secs: m * 60 }))}
                >
                  {`${String(m)}m`}
                </Button>
              ))}
            </Grid>
            <Grid cols={4}>
              {STEPS.map(step => (
                <Button key={step.label} onClick={() => send(hold(clamp(minutes + step.delta, 1, 1440)))}>
                  {step.label}
                </Button>
              ))}
            </Grid>
            <Grid cols={2}>
              {s.running ? (
                <Button onClick={() => send(call("timer.pause"))}>pause</Button>
              ) : (
                <Button primary disabled={!s.armed || s.rung} onClick={() => send(call("timer.resume"))}>
                  resume
                </Button>
              )}
              <Button onClick={() => send(call("timer.clear"))}>clear</Button>
            </Grid>
          </Stack>
        )}
      </Box>
      {s.laps.length === 0 ? null : (
        <Section label="laps">
          {s.laps.map((ms, i) => (
            <Setting key={`lap-${String(i)}`} label={`lap ${String(i + 1)}`}>
              <Caption>{fine(ms)}</Caption>
            </Setting>
          ))}
        </Section>
      )}
      <Box>
        <Choice
          label="mode"
          mode="row"
          options={opts(MODES)}
          value={s.mode}
          onChange={v => send(call("timer.mode", { mode: v }))}
        />
      </Box>
    </Stack>
  )
}
