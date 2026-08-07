import {
  Box,
  Button,
  Caption,
  Choice,
  Grid,
  Section,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { opts, SKINS, SURFACES } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const SIDES = [2, 4, 6, 8, 10, 12, 20]

const HISTORY = 8

type State = {
  steps: number
  face: number
  nonce: number
  rolls: number[]
  settings: { sides: number; surface: string; skin: string }
  cells: Deck
}

export function Dice({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const roll = () => send(call("dice.roll"))
  const tail = s.rolls.slice(-HISTORY)
  return (
    <Stack>
      <Box>
        <Cells app="dice" cells={s.cells} crisp={s.settings.surface === "grid"} onTap={roll} />
        <Grid cols={2}>
          <Button onClick={roll}>roll</Button>
          <Shot />
        </Grid>
      </Box>
      <Box>
        <Grid cols={7}>
          {SIDES.map(n => (
            <Button key={n} active={n === s.settings.sides} onClick={() => send(set("dice", "sides", n))}>
              {`d${String(n)}`}
            </Button>
          ))}
        </Grid>
        <Caption>{tail.length === 0 ? "unrolled" : `rolls ${tail.join(" ")}`}</Caption>
      </Box>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("dice", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("dice", "skin", v))}
        />
      </Section>
    </Stack>
  )
}
