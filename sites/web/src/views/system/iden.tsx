import {
  Badge,
  Box,
  Caption,
  Emoji,
  Section,
  Setting,
  Stack,
  Text,
  Title,
} from "mrlyui"

type State = { handle: string; id: string; verified: boolean }

export function Iden({ state }: { state: unknown }) {
  const s = state as State
  const name = s.handle === "" ? "guest" : s.handle
  return (
    <Stack>
      <Box>
        <Emoji glyph="👤" label={name} size="var(--font-xl)" />
        <Title>{name}</Title>
        <Caption>{s.verified ? "a known face" : "this device only"}</Caption>
      </Box>
      <Section label="account">
        <Setting label="handle">
          <Text>{s.handle}</Text>
        </Setting>
        <Setting label="id">
          <Caption>{s.id}</Caption>
        </Setting>
        <Setting label="status">
          <Badge variant={s.verified ? "success" : "info"}>{s.verified ? "verified" : "guest"}</Badge>
        </Setting>
      </Section>
    </Stack>
  )
}
