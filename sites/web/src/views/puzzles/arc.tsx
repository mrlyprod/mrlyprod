import {
  Box,
  Button,
  Caption,
  Choice,
  Cluster,
  Field,
  Section,
  Stack,
  Stepper,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { opts } from "../../components/options"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const SETS = ["one", "two", "three"]

const SPLITS = ["train", "eval"]

const PENS = 10

type Grid = number[][]

type Task = {
  index: number
  id: string
  pair: number
  pairs: { input: Grid; output: Grid }[]
  test: { i: number; input: Grid; tries: number }
  tests: number
  solved: boolean[]
}

type State = {
  score: number
  steps: number
  over: boolean
  settings: { set: string; split: string; pen: number }
  size: { w: number; h: number }
  task: Task | null
  cells: Deck
}

const shape = (grid: Grid): [number, number] => [grid[0]?.length ?? 1, grid.length]

export function Arc({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const task = s.task
  const live = s.settings.set === "three"
  const shown = task?.pairs[task.pair]
  const deck = (grid: Grid): Deck => ({ ...s.cells, ids: grid })
  return (
    <Stack>
      {task === null ? (
        <Box>
          <Caption>{live ? "the live arcade needs a session" : "load a task to play"}</Caption>
          {live ? (
            <Button onClick={() => send(call("arc.fetch"))}>fetch</Button>
          ) : (
            <Button primary onClick={() => send(call("arc.load", { task: 0 }))}>
              load
            </Button>
          )}
        </Box>
      ) : (
        <Section label={`examples · ${task.pairs.length}`}>
          <Cluster>
            {task.pairs.map((_, i) => (
              <Button key={i} active={i === task.pair} onClick={() => send(call("arc.pair", { i }))}>
                {String(i + 1)}
              </Button>
            ))}
          </Cluster>
          {shown === undefined ? null : (
            <Cluster>
              <Cells app="arc" cells={deck(shown.input)} grid={shape(shown.input)} />
              <Cells app="arc" cells={deck(shown.output)} grid={shape(shown.output)} />
            </Cluster>
          )}
        </Section>
      )}
      {task === null ? null : (
        <Box>
          <Cells
            app="arc"
            cells={s.cells}
            grid={[s.size.w, s.size.h]}
            handle="arc"
            onTap={(x, y) => send(call("arc.fill", { x, y, pen: s.settings.pen }))}
            onDrag={points => send(call("arc.paint", { points }))}
          />
        </Box>
      )}
      {s.over ? <GameOver app="arc" emoji="🏆" status={`task ${task?.id ?? ""} solved`} /> : null}
      {task === null || s.over ? null : (
        <Box>
          <Cluster>
            {Array.from({ length: PENS }, (_, i) => (
              <Button
                key={i}
                bg={s.cells.pens[i]}
                active={s.settings.pen === i}
                onClick={() => send(set("arc", "pen", i))}
              >
                {String(i)}
              </Button>
            ))}
          </Cluster>
          {task.tests > 1 ? (
            <Cluster>
              {Array.from({ length: task.tests }, (_, i) => (
                <Button key={i} active={i === task.test.i} onClick={() => send(call("arc.test", { i }))}>
                  {`test ${i + 1}${task.solved[i] === true ? " ✓" : ""}`}
                </Button>
              ))}
            </Cluster>
          ) : null}
          <Cluster>
            <Button onClick={() => send(call("arc.copy"))}>copy</Button>
            <Button onClick={() => send(call("arc.clear"))}>clear</Button>
            <Button primary onClick={() => send(call("arc.submit"))}>
              submit
            </Button>
          </Cluster>
          <Caption>
            {`${task.id} · ${s.size.w}x${s.size.h} · solved ${s.score}/${task.tests} · tries ${task.test.tries} · steps ${s.steps}`}
          </Caption>
        </Box>
      )}
      <Section label="task">
        <Field label="index">
          <Stepper
            value={task?.index ?? 0}
            min={0}
            max={999}
            onChange={v => send(call("arc.load", { task: v }))}
          />
        </Field>
        <Choice
          label="set"
          mode="row"
          options={opts(SETS)}
          value={s.settings.set}
          onChange={v => send(set("arc", "set", v))}
        />
        <Choice
          label="split"
          mode="row"
          options={opts(SPLITS)}
          value={s.settings.split}
          onChange={v => send(set("arc", "split", v))}
        />
      </Section>
    </Stack>
  )
}
