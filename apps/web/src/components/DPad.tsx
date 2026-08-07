import { Button, Grid } from "mrlyui"
import { call } from "../builders"
import { useSend } from "../send"

const DIRS = [
  ["left", "←"],
  ["up", "↑"],
  ["down", "↓"],
  ["right", "→"],
]

export function DPad({ app, verb }: { app: string; verb: string }) {
  const send = useSend()
  return (
    <Grid cols={4}>
      {DIRS.map(([dir, glyph]) => (
        <Button key={dir} onClick={() => send(call(`${app}.${verb}`, { dir }))}>
          {glyph}
        </Button>
      ))}
    </Grid>
  )
}
