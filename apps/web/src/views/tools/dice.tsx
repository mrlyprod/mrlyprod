import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const SIDES = [2, 4, 6, 8, 10, 12, 20]

type State = {
  steps: number
  face: number
  nonce: number
  rolls: number[]
  sprite: { rows: number[][]; palette: string[] }
  settings: { sides: number; surface: string; skin: string }
  frame: { rows: number[][]; palette: string[] }
}

export function dice(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  return (
    <stack key="dice">
      <card key="face">
        {grid
          ? <cell key="die" call={call("dice.roll")}>
              <canvas key={`face-${s.nonce}`} handle="dice" rows={s.sprite.rows} palette={s.sprite.palette} />
            </cell>
          : [
              <Board app="dice" rows={s.frame.rows} palette={s.frame.palette} />,
              <button key="roll" call={call("dice.roll")}>roll</button>,
            ]}
        <Shot />
      </card>
      <card key="sides">
        <grid key="sides" cols={7}>
          {SIDES.map(n =>
            n === s.settings.sides ? (
              <text key={`d-${n}`} role="note">{`d${n}`}</text>
            ) : (
              <button key={`d-${n}`} call={call("dice.set", { key: "sides", value: n })}>{`d${n}`}</button>
            ),
          )}
        </grid>
        <text key="meter" role="note">{s.rolls.length === 0 ? "unrolled" : `rolls ${s.rolls.join(" ")}`}</text>
      </card>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("dice", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("dice", "skin")} arg="value" label="skin" mode="row" />
      </Section>
    </stack>
  )
}
