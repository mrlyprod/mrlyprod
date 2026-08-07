import {
  Box,
  Button,
  Chip,
  Choice,
  Cluster,
  Pager,
  Section,
  Stack,
} from "mrlyui"
import { call, setter } from "../../builders"
import { colors, LEVELS, NUMBERS, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  settings: { design: string; number: number; level: number; fill: string; void: string }
  index: number
  count: number
  census: { grid: number; fill: number; void: number }
  cells: Deck
}

export function Two({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("two")
  return (
    <Stack>
      <Box>
        <Cells app="two" cells={s.cells} handle="two" />
      </Box>
      <Box>
        <Pager
          current={s.index + 1}
          total={s.count}
          onPrev={() => send(call("two.page", { dir: "prev" }))}
          onNext={() => send(call("two.page", { dir: "next" }))}
        />
        <Cluster>
          <Button onClick={() => send(call("two.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label={s.settings.design}>
        <Cluster>
          <Chip>{`grid ${s.census.grid}x${s.census.grid}`}</Chip>
          <Chip>{`fill ${s.census.fill}`}</Chip>
          <Chip>{`void ${s.census.void}`}</Chip>
        </Cluster>
      </Section>
      <Section label="shape">
        <Choice
          label="number"
          mode="row"
          value={String(s.settings.number)}
          options={opts(NUMBERS)}
          onChange={v => send(turn("number", v))}
        />
        <Choice
          label="level"
          mode="row"
          value={String(s.settings.level)}
          options={opts(LEVELS)}
          onChange={v => send(turn("level", v))}
        />
      </Section>
      <Section label="paint">
        <Choice
          label="fill"
          value={s.settings.fill}
          options={opts(colors())}
          onChange={v => send(turn("fill", v))}
        />
        <Choice
          label="void"
          value={s.settings.void}
          options={opts(colors())}
          onChange={v => send(turn("void", v))}
        />
      </Section>
    </Stack>
  )
}
