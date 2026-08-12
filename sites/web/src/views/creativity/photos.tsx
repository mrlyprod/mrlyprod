import {
  Box,
  Button,
  Caption,
  Grid,
  Section,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { Bits } from "../../eyes/Bits"
import { useSend } from "../../send"
import type { Flip } from "../../types"

type State = {
  shots: number
  photos: Flip[]
}

export function Photos({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const wall = s.photos.length === 0 ? "an empty wall" : `${s.photos.length} on the wall`
  return (
    <Stack>
      <Box>
        <Button onClick={() => send(call("photos.clear"))}>clear</Button>
        <Caption>{`${wall} · ${s.shots} taken`}</Caption>
      </Box>
      <Section label="wall">
        {s.photos.length === 0 ? (
          <Caption>take a screenshot in any app and it hangs here</Caption>
        ) : (
          <Grid cols={3}>
            {s.photos.map((photo, i) => (
              <Bits
                key={`photo-${i}`}
                rows={photo.rows}
                palette={photo.palette}
                handle={`photo-${i}`}
                crisp
              />
            ))}
          </Grid>
        )}
      </Section>
    </Stack>
  )
}
