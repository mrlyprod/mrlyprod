import {
  Box,
  Button,
  Caption,
  Cluster,
  Label,
  Stack,
  Symbol,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type Doc = { name: string; mime: string; uri: string; size: number; tick: number }

type State = { count: number; files: Doc[] }

const UNITS = ["b", "kb", "mb", "gb"]

function weigh(size: number): string {
  let left = size
  let step = 0
  while (left >= 1024 && step < UNITS.length - 1) {
    left = left / 1024
    step += 1
  }
  const unit = UNITS[step] ?? "b"
  return step === 0 ? `${String(left)} ${unit}` : `${left.toFixed(1)} ${unit}`
}

export function Files({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      {s.files.map((doc, i) => (
        <Box key={`${doc.name}-${String(doc.tick)}-${String(i)}`}>
          <Cluster>
            <Label
              mode="row"
              symbol={{ as: "icon", value: "download" }}
              text={doc.name}
              note={`${weigh(doc.size)} · ${doc.mime}`}
              href={doc.uri}
            />
            <Button onClick={() => send(call("files.drop", { index: i }))}>
              <Symbol name="close" />
            </Button>
          </Cluster>
        </Box>
      ))}
      <Box>
        <Caption>{s.count === 0 ? "no documents yet" : `${String(s.count)} on the shelf`}</Caption>
        {s.count === 0 ? null : (
          <Button wide onClick={() => send(call("files.clear"))}>
            clear all
          </Button>
        )}
      </Box>
    </Stack>
  )
}
