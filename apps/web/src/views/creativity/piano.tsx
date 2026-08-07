import { Button, Caption, Grid, Section, Stack, Text } from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type Key = { midi: number; name: string; held: boolean }

type State = { cols: number; keys: (Key | null)[]; held: number[] }

export function Piano({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const names = s.keys.filter((k): k is Key => k !== null && k.held).map(k => k.name)
  return (
    <Stack>
      <Section label="keys">
        <Grid cols={s.cols}>
          {s.keys.map((key, i) =>
            key === null ? (
              <Caption key={`gap-${i}`} />
            ) : (
              <Button
                key={key.midi}
                active={key.held}
                onPress={() => send(call("piano.press", { midi: key.midi }))}
                onLift={() => send(call("piano.lift", { midi: key.midi }))}
              >
                {key.name}
              </Button>
            ),
          )}
        </Grid>
      </Section>
      <Section label="held">
        <Text>{names.length === 0 ? "quiet" : names.join(" ")}</Text>
        <Button disabled={names.length === 0} onClick={() => send(call("piano.silence"))}>
          silence
        </Button>
      </Section>
    </Stack>
  )
}
