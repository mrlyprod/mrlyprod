import {
  Box,
  Button,
  Caption,
  Section,
  Setting,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type Entry = { verb: string; args: unknown; now: number; tick: number }

type State = { entries: Entry[] }

function show(args: unknown): string {
  const body = JSON.stringify(args)
  if (body === "{}") return ""
  return body.length > 80 ? `${body.slice(0, 77)}...` : body
}

export function Log({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Section label="calls">
        {s.entries.length === 0 ? <Caption>no calls yet</Caption> : null}
        {s.entries.map(entry => (
          <Setting key={`${String(entry.tick)}-${entry.verb}`} label={entry.verb} hint={show(entry.args)}>
            <Caption>{entry.tick}</Caption>
          </Setting>
        ))}
      </Section>
      <Box>
        <Button wide onClick={() => send(call("log.export"))}>
          export
        </Button>
      </Box>
    </Stack>
  )
}
