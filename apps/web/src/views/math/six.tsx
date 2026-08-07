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
import { colors, NUMBERS, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Carve } from "../../eyes/Carve"
import { floats } from "../../reads"
import { useSend } from "../../send"

const VIEWS = ["iso", "pro", "cut"]

const LEVELS = ["1", "2"]

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  view: string
  fill: string
  census: { grid: number; fill: number; void: number }
}

export function Six({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("six")
  return (
    <Stack>
      <Box>
        <Carve tris={floats("six/tris") ?? []} handle="six" />
      </Box>
      <Box>
        <Pager
          current={s.index + 1}
          total={s.count}
          onPrev={() => send(call("six.page", { dir: "prev" }))}
          onNext={() => send(call("six.page", { dir: "next" }))}
        />
        <Cluster>
          <Button onClick={() => send(call("six.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label={s.design}>
        <Cluster>
          <Chip>{`grid ${s.census.grid}^3`}</Chip>
          <Chip>{`fill ${s.census.fill}`}</Chip>
          <Chip>{`void ${s.census.void}`}</Chip>
        </Cluster>
      </Section>
      <Section label="shape">
        <Choice
          label="number"
          mode="row"
          value={String(s.number)}
          options={opts(NUMBERS)}
          onChange={v => send(turn("number", v))}
        />
        <Choice
          label="level"
          mode="row"
          value={String(s.level)}
          options={opts(LEVELS)}
          onChange={v => send(turn("level", v))}
        />
        <Choice
          label="view"
          mode="row"
          value={s.view}
          options={opts(VIEWS)}
          onChange={v => send(turn("view", v))}
        />
      </Section>
      <Section label="paint">
        <Choice label="fill" value={s.fill} options={opts(colors())} onChange={v => send(turn("fill", v))} />
      </Section>
    </Stack>
  )
}
