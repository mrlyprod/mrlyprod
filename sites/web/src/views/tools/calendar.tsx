import {
  Board,
  Box,
  Button,
  Caption,
  Cell,
  DatePicker,
  Grid,
  Section,
  Setting,
  Stack,
  Symbol,
  Title,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type Slot = { day: number; faded: boolean }

type State = {
  title: string
  days: string[]
  weeks: Slot[][]
  today: number | null
  year: number
  month: number
  picked: { year: number; month: number; day: number }
}

function staged(s: State, slot: Slot): boolean {
  if (slot.faded) return false
  return s.picked.year === s.year && s.picked.month === s.month && s.picked.day === slot.day
}

export function Calendar({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const slots = s.weeks.flat()
  const stage = new Date(s.year, s.month - 1, 1)
  const jump = (date: Date) => {
    send(call("calendar.goto", { year: date.getFullYear(), month: date.getMonth() + 1 }))
    send(call("calendar.pick", { day: date.getDate() }))
  }
  return (
    <Stack>
      <Box>
        <Title>{s.title}</Title>
        <Grid cols={3}>
          <Button onClick={() => send(call("calendar.flip", { n: -1 }))}>
            <Symbol name="chevron_left" />
          </Button>
          <Button onClick={() => send(call("calendar.today"))}>today</Button>
          <Button onClick={() => send(call("calendar.flip", { n: 1 }))}>
            <Symbol name="chevron_right" />
          </Button>
        </Grid>
      </Box>
      <Box>
        <Board cols={7} rows={s.weeks.length + 1}>
          {s.days.map((day, i) => (
            <Cell key={`head-${String(i)}`}>
              <Caption>{day}</Caption>
            </Cell>
          ))}
          {slots.map((slot, i) => (
            <Cell
              key={`slot-${String(i)}`}
              on={!slot.faded && slot.day === s.today}
              bg={staged(s, slot) ? "var(--accent-color)" : undefined}
              onClick={slot.faded ? undefined : () => send(call("calendar.pick", { day: slot.day }))}
            >
              {slot.faded ? <Caption>{slot.day}</Caption> : slot.day}
            </Cell>
          ))}
        </Board>
      </Box>
      <Section label="jump">
        <Setting label="month">
          <DatePicker value={stage} onChange={jump} />
        </Setting>
      </Section>
    </Stack>
  )
}
