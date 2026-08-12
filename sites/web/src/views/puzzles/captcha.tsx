import { useState } from "react"
import {
  Board,
  Box,
  Button,
  Caption,
  Cell,
  Choice,
  Field,
  Input,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { SURFACES, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { Face } from "../../eyes/Face"
import { paint, visual } from "../../eyes/skin"
import { useSend } from "../../send"
import type { Cells as Deck, Visual } from "../../types"

const SKINS = ["tiles", "digits"]

type State = {
  score: number
  steps: number
  over: boolean
  prompt: string
  settings: { cols: number; rows: number; size: number; surface: string; skin: string }
  cells: Deck
}

function inked(v: Visual): boolean {
  const f = v.face
  return f !== undefined && f.as !== "sprite" && f.value !== undefined && f.value !== ""
}

export function Captcha({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const [text, setText] = useState("")
  const grid = s.settings.surface === "grid"
  const across = s.cells.ids[0]?.length ?? s.settings.cols
  const down = s.cells.ids.length

  const answer = () => {
    if (text === "") return
    send(call("captcha.answer", { text }))
    setText("")
  }

  const tile = (id: number, i: number) => {
    const v = visual("captcha", s.cells, id)
    const pick = s.over ? undefined : () => send(call("captcha.pick", { cell: i }))
    if (inked(v)) {
      return (
        <Cell key={i} bg={paint(v, s.cells)} onClick={pick}>
          <Face visual={v} />
        </Cell>
      )
    }
    return (
      <Cell key={i} onClick={pick}>
        <Cells app="captcha" cells={{ ...s.cells, ids: [[id]] }} />
      </Cell>
    )
  }

  return (
    <Stack>
      <Box>
        <Caption>{`find: ${s.prompt}`}</Caption>
        {grid ? (
          <Board cols={across} rows={down}>
            {s.cells.ids.flat().map((id, i) => tile(id, i))}
          </Board>
        ) : (
          <Cells
            app="captcha"
            cells={s.cells}
            grid={[across, down]}
            onTap={s.over ? undefined : (x, y) => send(call("captcha.pick", { x, y }))}
          />
        )}
      </Box>
      {s.over ? <GameOver app="captcha" emoji="🧩" status={`solved ${s.score}`} /> : null}
      {s.over || grid ? null : (
        <Box>
          <Field label="answer" hint="type what you see">
            <Input value={text} onChange={setText} />
          </Field>
          <Button onClick={answer}>answer</Button>
        </Box>
      )}
      <Box>
        {s.over ? null : <Caption>{`solved ${s.score} · tries ${s.steps}`}</Caption>}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="cols">
          <Slider min={2} max={5} step={1} value={s.settings.cols} onChange={v => send(set("captcha", "cols", v))} />
        </Field>
        <Field label="rows">
          <Slider min={2} max={5} step={1} value={s.settings.rows} onChange={v => send(set("captcha", "rows", v))} />
        </Field>
        <Field label="size">
          <Slider min={2} max={16} step={1} value={s.settings.size} onChange={v => send(set("captcha", "size", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("captcha", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("captcha", "skin", v))}
        />
      </Section>
    </Stack>
  )
}
