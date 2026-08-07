import { useState } from "react"
import {
  Box,
  Button,
  Caption,
  Cluster,
  Field,
  Input,
  Search,
  Stack,
  Symbol,
} from "mrlyui"
import { call } from "../../builders"
import { useSend } from "../../send"

type Item = { id: number; text: string }

type State = { query: string; found: Item[] }

function Line({ item }: { item: Item }) {
  const send = useSend()
  const [text, setText] = useState(item.text)
  const ready = text.trim() !== "" && text.trim() !== item.text
  return (
    <Box>
      <Input value={text} onChange={setText} />
      <Cluster>
        <Button disabled={!ready} onClick={() => send(call("notes.edit", { id: item.id, text }))}>
          save
        </Button>
        <Button onClick={() => send(call("notes.remove", { id: item.id }))}>
          <Symbol name="close" />
        </Button>
      </Cluster>
    </Box>
  )
}

export function Notes({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const [draft, setDraft] = useState("")
  const write = () => {
    if (draft.trim() === "") return
    send(call("notes.add", { text: draft }))
    setDraft("")
  }
  return (
    <Stack>
      <Box>
        <Field label="write">
          <Input value={draft} onChange={setDraft} placeholder="write a note" />
        </Field>
        <Button primary disabled={draft.trim() === ""} onClick={write}>
          add
        </Button>
        <Search value={s.query} onChange={q => send(call("notes.search", { q }))} placeholder="search" />
      </Box>
      {s.found.length === 0 && (
        <Box>
          <Caption>{s.query === "" ? "no notes yet" : `nothing matches ${s.query}`}</Caption>
        </Box>
      )}
      {s.found.map(item => (
        <Line key={item.id} item={item} />
      ))}
      <Box>
        <Cluster>
          <Button onClick={() => send(call("notes.export"))}>export</Button>
          <Button onClick={() => send(call("notes.clear"))}>clear all</Button>
        </Cluster>
      </Box>
    </Stack>
  )
}
