import {
  Box,
  Caption,
  Chip,
  Cluster,
  Grid,
  Label,
  Search,
  Stack,
} from "mrlyui"
import { call } from "../../builders"
import { useApps, useSend } from "../../send"
import type { Manifest } from "../../types"

type State = { apps: Manifest[]; query: string; mode: "grid" | "list" }

export function Menu({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const apps = useApps()
  const groups = [...new Set(apps.filter(app => !app.hidden).map(app => app.category))]
  const list = s.mode === "list"
  const seek = (q: string) => send(call("menu.search", { q }))
  return (
    <Stack>
      <Box>
        <Search value={s.query} onChange={seek} placeholder="search" />
      </Box>
      <Box>
        <Cluster>
          {groups.map(group => (
            <Chip key={group} active={s.query === group} onClick={() => seek(s.query === group ? "" : group)}>
              {group}
            </Chip>
          ))}
        </Cluster>
      </Box>
      {s.apps.length === 0 ? (
        <Box>
          <Caption>nothing answers to that</Caption>
        </Box>
      ) : (
        <Grid cols={list ? 1 : 3}>
          {s.apps.map(app => (
            <Box key={app.route} onClick={() => send(call("nav.open", { app: app.route }))}>
              <Label
                mode={list ? "row" : "stack"}
                symbol={{ as: "emoji", value: app.emoji }}
                text={app.title}
                note={list ? app.category : undefined}
              />
            </Box>
          ))}
        </Grid>
      )}
      <Box>
        <Cluster>
          <Label mode="text" text="privacy" href="/privacy" />
          <Label mode="text" text="terms" href="/terms" />
        </Cluster>
      </Box>
    </Stack>
  )
}
