import { call } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Cell = { midi: number; name: string; held: boolean } | null

type State = { cols: number; cells: Cell[]; held: number[] }

export function piano(state: unknown, _send: Send): Node {
  const s = state as State
  const names = s.cells.filter((c): c is Exclude<Cell, null> => c !== null && c.held).map(c => c.name)
  return (
    <stack key="piano">
      <Section keyName="keys" label="keys">
        <grid key="board" cols={s.cols}>
          {s.cells.map((cell, i) =>
            cell === null ? (
              <text key={`gap-${i}`}></text>
            ) : (
              <button
                key={`key-${cell.midi}`}
                call={call("piano.press", { midi: cell.midi })}
                press={call("piano.press", { midi: cell.midi })}
                lift={call("piano.lift", { midi: cell.midi })}
                bg={cell.held ? "var(--accent-color)" : undefined}
              >
                {cell.name}
              </button>
            ),
          )}
        </grid>
      </Section>
      <Section keyName="held" label="held">
        <text key="held-notes">{names.length === 0 ? "silence" : names.join(" ")}</text>
      </Section>
    </stack>
  )
}
