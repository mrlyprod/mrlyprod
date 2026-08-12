import { useEffect, useState } from "react"
import { Box, Caption, Chip, Footer, Frame, Mark, Row, Stack, Text, Title } from "mrlyui"

// NOTES

type Hat = "wish" | "plan" | "make"

type Note = { stamp: string; hat: Hat; note: string }

const MONTHS = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"]

function when(stamp: string): string {
  const month = MONTHS[Number(stamp.slice(4, 6)) - 1] ?? ""
  return `${String(Number(stamp.slice(6, 8)))} ${month} ${stamp.slice(0, 4)}`
}

async function load(): Promise<Note[]> {
  const res = await fetch("/notes.json")
  if (!res.ok) throw new Error(`bot (notes ${String(res.status)})`)
  const notes = (await res.json()) as Note[]
  return notes.toSorted((a, b) => b.stamp.localeCompare(a.stamp))
}

// FEED

function Entry({ note }: { note: Note }) {
  return (
    <Box>
      <Stack tight>
        <Row>
          <Chip>{note.hat}</Chip>
          <Caption>{when(note.stamp)}</Caption>
        </Row>
        <Text>{note.note}</Text>
      </Stack>
    </Box>
  )
}

function Feed({ notes }: { notes: Note[] }) {
  if (notes.length === 0) return <Caption>the notebook is empty</Caption>
  return (
    <Stack>
      {notes.map(note => (
        <Entry key={note.stamp} note={note} />
      ))}
    </Stack>
  )
}

// APP

export function App() {
  const [notes, set] = useState<Note[] | undefined>(undefined)
  useEffect(() => {
    void load()
      .then(set)
      .catch(() => set([]))
  }, [])
  return (
    <Frame>
      <Stack>
        <Title>bot.mrly.net</Title>
        <Caption>The bot's notebook. Short notes from each session, read and approved before they land.</Caption>
        {notes !== undefined && <Feed notes={notes} />}
      </Stack>
      <Footer>
        <Mark />
      </Footer>
    </Frame>
  )
}
