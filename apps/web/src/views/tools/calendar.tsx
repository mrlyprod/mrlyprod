import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const MONTHS = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]

type Cell = { day: number; faded: boolean }

type State = {
  title: string
  days: string[]
  weeks: Cell[][]
  today: number | null
  year: number
  month: number
  picked: { year: number; month: number; day: number }
}

function cell(c: Cell, i: number, s: State): Node {
  const key = `c-${i}`
  if (c.faded) return <text key={key} role="note">{String(c.day)}</text>
  const staged = s.picked.year === s.year && s.picked.month === s.month && c.day === s.picked.day
  const bg = staged ? "var(--accent-color)" : c.day === s.today ? "var(--muted-color)" : undefined
  return <button key={key} call={call("calendar.pick", { day: c.day })} bg={bg}>{String(c.day)}</button>
}

export function calendar(state: unknown, _send: Send): Node {
  const s = state as State
  const cells = s.weeks.flat()
  return (
    <stack key="calendar">
      <card key="month">
        <text key="month" role="title">{s.title}</text>
        <grid key="controls" cols={3}>
          <button key="prev" call={call("calendar.flip", { n: -1 })}>‹</button>
          <button key="now" call={call("calendar.today")}>today</button>
          <button key="next" call={call("calendar.flip", { n: 1 })}>›</button>
        </grid>
        <grid key="jump" cols={2}>
          <choice key="jump-month" value={MONTHS[s.month - 1] ?? ""} options={MONTHS} call={call("calendar.goto", { year: s.year })} arg="month" mode="select" />
          <field key="jump-year" value={String(s.year)} live={false} call={call("calendar.goto", { month: s.month })} arg="year" label="year" />
        </grid>
      </card>
      <card key="sheet">
        <grid key="sheet" cols={7}>
          {s.days.map((d, i) => (
            <text key={`h-${i}`} role="note">{d}</text>
          ))}
          {cells.map((c, i) => cell(c, i, s))}
        </grid>
      </card>
    </stack>
  )
}
