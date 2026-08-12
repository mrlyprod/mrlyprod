import {
  Box,
  Button,
  Caption,
  Cluster,
  Field,
  GraphemeInput,
  Grid,
  Section,
  Stack,
  Symbol,
  Title,
} from "mrlyui"
import { call } from "../../builders"
import { Bits } from "../../eyes/Bits"
import { useSend } from "../../send"
import type { Raster } from "../../types"

type State = {
  char: string
  name: string
  index: number
  total: number
  revealing: boolean
  glyph: Raster
  library: string[]
}

const FORMATS = ["json", "ttf", "woff", "woff2"]

export function Fonts({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const rows = s.glyph.rows.map(row => row.map(cell => (cell === 0 ? 0 : 255)))
  const pick = (char: string) => {
    if (char !== "") send(call("fonts.pick", { char }))
  }
  return (
    <Stack>
      <Box>
        <Bits rows={rows} handle="fonts" crisp />
        <Title>{s.char}</Title>
        <Caption>{`${s.name.toLowerCase()} · ${s.glyph.width}x${s.glyph.height} · ${s.index + 1} of ${s.total}`}</Caption>
      </Box>
      <Box>
        <Cluster>
          <Button onClick={() => send(call("fonts.page", { dir: "prev" }))}>
            <Symbol name="chevron_left" />
          </Button>
          <Button onClick={() => send(call("fonts.page", { dir: "next" }))}>
            <Symbol name="chevron_right" />
          </Button>
          <Button active={s.revealing} onClick={() => send(call("fonts.scramble"))}>
            scramble
          </Button>
          <Button onClick={() => send(call("fonts.keep"))}>keep</Button>
        </Cluster>
        <Field label="jump" hint="type any glyph">
          <GraphemeInput value={s.char} onChange={pick} />
        </Field>
      </Box>
      <Section label="export">
        <Grid cols={4}>
          {FORMATS.map(format => (
            <Button key={format} onClick={() => send(call("fonts.export", { format }))}>
              {format}
            </Button>
          ))}
        </Grid>
      </Section>
      <Section label="library">
        <Caption>tap to drop</Caption>
        <Grid cols={8}>
          {s.library.map(char => (
            <Button key={char} onClick={() => send(call("fonts.drop", { char }))}>
              {char}
            </Button>
          ))}
        </Grid>
      </Section>
    </Stack>
  )
}
