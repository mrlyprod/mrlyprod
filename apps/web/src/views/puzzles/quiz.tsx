import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

const SKINS = ["tiles", "digits"]

type State = {
  score: number
  steps: number
  over: boolean
  position: number
  total: number
  options: string[]
  settings: { options: number; length: number; surface: string; skin: string }
  cells: Cells
}

export function quiz(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  const prompt = grid
    ? <canvas key="prompt" handle="quiz-prompt" cells={{ app: "quiz", ...s.cells }} />
    : <Board app="quiz" cells={s.cells} />
  const settings = [
    <Section keyName="rules" label="rules">
      <range key="options" value={s.settings.options} min={2} max={8} call={set("quiz", "options")} arg="value" step={1} label="options" />
      <range key="length" value={s.settings.length} min={2} max={32} call={set("quiz", "length")} arg="value" step={1} label="length" />
    </Section>,
    <Section keyName="look" label="look">
      <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("quiz", "surface")} arg="value" label="surface" mode="row" />
      <choice key="skin" value={s.settings.skin} options={SKINS} call={set("quiz", "skin")} arg="value" label="skin" mode="row" />
    </Section>,
  ]
  return (
    <stack key="quiz">
      <card key="board">{prompt}</card>
      {s.over
        ? <GameOver app="quiz" emoji="❓" status={`score ${s.score}`} />
        : <card key="options">
            {s.options.map((text, i) => (
              <button key={`option-${i}`} call={call("quiz.answer", { text })}>{text}</button>
            ))}
          </card>}
      <card key="meter">
        {!s.over && <text key="meter" role="note">{`score ${s.score} · ${s.position} / ${s.total}`}</text>}
        <Shot />
      </card>
      {settings}
    </stack>
  )
}
