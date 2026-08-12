import {
  Box,
  Caption,
  Letters,
  Stack,
} from "mrlyui"

type State = { now: number }

const WEEK = ["thu", "fri", "sat", "sun", "mon", "tue", "wed"]

const div = (a: number, n: number) => Math.floor(a / n)

const mod = (a: number, n: number) => ((a % n) + n) % n

const pad = (n: number, width = 2) => String(n).padStart(width, "0")

// FACE

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
  const days = div(now, 86400000)
  const [year, month, day] = civil(days)
  return `${WEEK[mod(days, 7)] ?? ""} ${pad(year, 4)}-${pad(month)}-${pad(day)} utc`
}

// CLOCK

export function Clock({ state }: { state: unknown }) {
  const s = state as State
  return (
    <Stack>
      <Box>
        <Letters text={face(s.now)} scramble={false} />
        <Caption>{date(s.now)}</Caption>
      </Box>
    </Stack>
  )
}
