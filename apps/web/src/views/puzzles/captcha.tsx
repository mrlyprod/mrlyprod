import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const SKINS = ["tiles", "digits"]

type Sprite = { rows: number[][]; palette: string[] }

type State = {
  score: number
  steps: number
  over: boolean
  prompt: string
  sprites: Sprite[]
  settings: { cols: number; rows: number; size: number; surface: string; skin: string }
  frame: { rows: number[][]; palette: string[] }
}

export function captcha(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  const meterText = `solved ${s.score} · tries ${s.steps}`
  return (
    <stack key="captcha">
      <card key="board">
        {grid
          ? <grid key="grid" cols={s.settings.cols}>
              {s.sprites.map((sprite, i) => (
                <cell key={`c-${i}`} call={s.over ? undefined : call("captcha.pick", { cell: i })}>
                  <canvas key="face" handle={`captcha-${i}`} rows={sprite.rows} palette={sprite.palette} />
                </cell>
              ))}
            </grid>
          : <Board app="captcha" rows={s.frame.rows} palette={s.frame.palette} />}
        <text key="prompt" role="note">{`find: ${s.prompt}`}</text>
      </card>
      {s.over
        ? <GameOver app="captcha" emoji="🧩" status={`solved ${s.score}`} />
        : grid
          ? undefined
          : <card key="controls">
              <field key="answer" value="" live={false} call={call("captcha.answer")} arg="text" hint="type what you see" />
            </card>}
      <card key="meter">
        {!s.over && <text key="meter" role="note">{meterText}</text>}
        <Shot />
      </card>
      <Section keyName="rules" label="rules">
        <range key="cols" value={s.settings.cols} min={2} max={5} call={set("captcha", "cols")} arg="value" step={1} label="cols" />
        <range key="rows" value={s.settings.rows} min={2} max={5} call={set("captcha", "rows")} arg="value" step={1} label="rows" />
        <range key="size" value={s.settings.size} min={2} max={16} call={set("captcha", "size")} arg="value" step={1} label="size" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("captcha", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("captcha", "skin")} arg="value" label="skin" mode="row" />
      </Section>
    </stack>
  )
}
