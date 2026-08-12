import { useEffect, useState } from "react"
import {
  Box,
  Button,
  Caption,
  Choice,
  Field,
  Grid,
  Input,
  Section,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const RULES = ["life", "maze", "replicator", "anneal"]

type State = {
  text: string
  hex: string
  rule: string
  cells: Deck
}

export function Hash({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const [draft, setDraft] = useState(s.text)
  useEffect(() => setDraft(s.text), [s.text])
  const digest = () => {
    if (draft.trim() !== "") send(call("hash.digest", { text: draft }))
  }
  return (
    <Stack>
      <Box>
        <Cells app="hash" cells={s.cells} />
      </Box>
      <Section label="digest">
        <Field label="text" hint="up to 256 characters">
          <Input value={draft} onChange={setDraft} />
        </Field>
        <Grid cols={2}>
          <Button primary onClick={digest}>
            digest
          </Button>
          <Button onClick={() => send(call("hash.reset"))}>reset</Button>
        </Grid>
        <Choice
          label="rule"
          mode="row"
          options={opts(RULES)}
          value={s.rule}
          onChange={v => send(set("hash", "rule", v))}
        />
        <Shot />
      </Section>
      <Box>
        <Caption>{s.hex.slice(0, 32)}</Caption>
        <Caption>{s.hex.slice(32)}</Caption>
      </Box>
    </Stack>
  )
}
