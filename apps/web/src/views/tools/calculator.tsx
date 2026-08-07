import {
  Box,
  Button,
  Grid,
  Letters,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"
import type { Call } from "../../types"

type State = { display: string }

export function Calculator({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const pad = (glyph: string, made: Call) => (
    <Button onClick={() => send(made)}>
      <Letters text={glyph} scramble={false} />
    </Button>
  )
  const digit = (d: number) => pad(String(d), call("calculator.digit", { d }))
  const op = (name: string, glyph: string) => pad(glyph, call("calculator.op", { op: name }))
  return (
    <Stack>
      <Box>
        <Button wide onClick={() => send(call("calculator.copy"))}>
          <Letters text={s.display} />
        </Button>
        <Grid cols={4}>
          {pad("AC", call("calculator.clear"))}
          {pad("+/-", call("calculator.negate"))}
          {pad("%", call("calculator.percent"))}
          {op("div", "÷")}
          {digit(7)}
          {digit(8)}
          {digit(9)}
          {op("mul", "×")}
          {digit(4)}
          {digit(5)}
          {digit(6)}
          {op("sub", "−")}
          {digit(1)}
          {digit(2)}
          {digit(3)}
          {op("add", "+")}
        </Grid>
        <Grid cols={2}>
          {digit(0)}
          <Grid cols={2}>
            {pad(".", call("calculator.dot"))}
            {pad("=", call("calculator.equals"))}
          </Grid>
        </Grid>
      </Box>
    </Stack>
  )
}
