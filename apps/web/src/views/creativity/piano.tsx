import { call } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Key = { midi: number; name: string; held: boolean } | null

type State = { cols: number; keys: Key[]; held: number[] }

export function piano(state: unknown, _send: Send): Node {
  const s = state as State
  const names = s.keys.filter((k): k is Exclude<Key, null> => k !== null && k.held).map(k => k.name)
  return (
    <stack key="piano">
      <Section keyName="keys" label="keys">
        <grid key="board" cols={s.cols}>
          {s.keys.map((key, i) =>
            key === null ? (
              <text key={`gap-${i}`}></text>
            ) : (
              <button
                key={`key-${key.midi}`}
                call={call("piano.press", { midi: key.midi })}
                press={call("piano.press", { midi: key.midi })}
                lift={call("piano.lift", { midi: key.midi })}
                bg={key.held ? "var(--accent-color)" : undefined}
              >
                {key.name}
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
