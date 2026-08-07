import {
  Box,
  Button,
  Caption,
  Emoji,
  Title,
} from "mrlyui"
import { call } from "../builders"
import { useSend } from "../send"

export function GameOver({ app, emoji, status }: { app: string; emoji: string; status: string }) {
  const send = useSend()
  return (
    <Box>
      <Emoji glyph={emoji} />
      <Title>game over</Title>
      <Caption>{status}</Caption>
      <Button primary onClick={() => send(call(`${app}.reset`))}>play again</Button>
    </Box>
  )
}
