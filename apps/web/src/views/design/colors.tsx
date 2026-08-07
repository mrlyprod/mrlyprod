import {
  Box,
  Button,
  Caption,
  Cluster,
  ColorPicker,
  Section,
  Stack,
  Text,
  Title,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type State = {
  index: number
  count: number
  name: string
  hex: string
  rgb: { r: number; g: number; b: number }
  palette: { name: string; hex: string }[]
  library: string[]
}

export function Colors({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const swatches = s.palette.map(one => ({ name: one.name, color: one.hex }))
  const kept = swatches.filter(one => s.library.includes(one.name))
  const pick = (name: string | null) => {
    if (name !== null) send(call("colors.set", { key: "name", value: name }))
  }
  const drop = (name: string | null) => {
    if (name !== null) send(call("colors.drop", { name }))
  }
  return (
    <Stack>
      <Box>
        <Title>{s.name}</Title>
        <Button big bg={s.hex} onClick={() => send(call("colors.page", { dir: "next" }))}>
          {" "}
        </Button>
        <Text>{s.hex}</Text>
        <Caption>{`rgb ${s.rgb.r} ${s.rgb.g} ${s.rgb.b} · ${s.index + 1} of ${s.count}`}</Caption>
      </Box>
      <Section label="palette">
        <ColorPicker swatches={swatches} value={s.name} onChange={pick} big />
      </Section>
      <Section label="library">
        <Caption>tap to drop</Caption>
        <ColorPicker swatches={kept} value={s.library} onChange={drop} />
        <Cluster>
          <Button onClick={() => send(call("colors.keep"))}>keep</Button>
          <Button onClick={() => send(call("colors.export"))}>export</Button>
        </Cluster>
      </Section>
    </Stack>
  )
}
