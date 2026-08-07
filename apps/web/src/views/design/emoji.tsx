import {
  Box,
  Button,
  Caption,
  Choice,
  Emoji,
  Grid,
  Section,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { opts } from "../../components/options"
import { useSend } from "../../send"

type State = { category: string; categories: string[]; grid: string[]; library: string[] }

export function Emojis({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Box>
        <Choice
          label="category"
          options={opts(s.categories)}
          value={s.category}
          onChange={value => send(call("emoji.set", { key: "category", value }))}
        />
        <Caption>tap to keep</Caption>
      </Box>
      <Section label={s.category}>
        <Grid cols={8}>
          {s.grid.map(glyph => (
            <Button key={glyph} onClick={() => send(call("emoji.keep", { value: glyph }))}>
              <Emoji glyph={glyph} label={glyph} />
            </Button>
          ))}
        </Grid>
      </Section>
      <Section label="library">
        <Caption>tap to drop</Caption>
        <Grid cols={8}>
          {s.library.map(glyph => (
            <Button key={glyph} onClick={() => send(call("emoji.drop", { value: glyph }))}>
              <Emoji glyph={glyph} label={glyph} />
            </Button>
          ))}
        </Grid>
      </Section>
    </Stack>
  )
}
