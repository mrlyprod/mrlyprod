import {
  Box,
  Button,
  Caption,
  Chip,
  Cluster,
  Field,
  Section,
  Slider,
  Stack,
  Symbol,
  Toggle,
} from "mrlyui"
import { call, setter } from "../../builders"
import { Library } from "../../components/Library"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const SEEDS = ["seed", "clear", "soup", "glider", "pulsar", "pentomino"]

const SEQS = ["evens", "odds", "primes", "fibonacci"]

type Work = unknown

type State = {
  cells: Deck
  generation: number
  population: number
  entropy: number
  fate: string | null
  period: number
  running: boolean
  cursor: number
  length: number
  max_neighbors: number
  settings: {
    size: number
    wrap: boolean
    speed: number
    tiling: number
    padding: number
    density: number
    birth: number[]
    survive: number[]
    zeros: boolean
    ones: boolean
    seed: Work
    mask: Work
    fill: string
    void: string
  }
}

// RULE

function Rule({ label, which, counts, worn }: {
  label: string
  which: "birth" | "survive"
  counts: number[]
  worn: number[]
}) {
  const send = useSend()
  return (
    <Section label={label}>
      <Cluster>
        {counts.map(n => (
          <Chip
            key={n}
            active={worn.includes(n)}
            onClick={() => send(call("life.rule", { which, n, on: !worn.includes(n) }))}
          >
            {n}
          </Chip>
        ))}
      </Cluster>
      <Cluster>
        {SEQS.map(seq => (
          <Button key={seq} onClick={() => send(call("life.fill", { which, seq }))}>
            {seq}
          </Button>
        ))}
      </Cluster>
    </Section>
  )
}

// LIFE

export function Life({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("life")
  const counts = Array.from({ length: s.max_neighbors + 1 }, (_, n) => n)
  const settled = s.fate !== null && s.cursor === s.length - 1
  return (
    <Stack>
      <Box>
        <Cells
          app="life"
          cells={s.cells}
          grid={[s.settings.size, s.settings.size]}
          handle="life"
          onDrag={s.running ? undefined : points => send(call("life.paint", { points }))}
        />
      </Box>
      <Box>
        <Cluster>
          <Button disabled={s.cursor === 0} onClick={() => send(call("life.start"))}>
            <Symbol name="keyboard_double_arrow_left" />
          </Button>
          <Button disabled={s.cursor === 0} onClick={() => send(call("life.back"))}>
            <Symbol name="chevron_left" />
          </Button>
          <Button
            active={s.running}
            disabled={settled && !s.running}
            onClick={() => send(call("life.run", { on: !s.running }))}
          >
            <Symbol name={s.running ? "pause" : "play_arrow"} />
          </Button>
          <Button disabled={settled} onClick={() => send(call("life.step"))}>
            <Symbol name="chevron_right" />
          </Button>
          <Button disabled={s.cursor === s.length - 1} onClick={() => send(call("life.end"))}>
            <Symbol name="keyboard_double_arrow_right" />
          </Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="timeline">
        <Cluster>
          <Chip>{`gen ${s.generation}`}</Chip>
          <Chip>{`pop ${s.population}`}</Chip>
          <Chip>{`H ${(s.entropy / 1000).toFixed(3)}`}</Chip>
          <Chip>{`${s.cursor + 1} / ${s.length}`}</Chip>
          {s.fate !== null && <Chip active>{s.period > 1 ? `${s.fate} p${s.period}` : s.fate}</Chip>}
        </Cluster>
        <Field label="speed">
          <Slider min={1} max={32} step={1} value={s.settings.speed} onChange={v => send(turn("speed", v))} />
        </Field>
      </Section>
      <Section label="seed">
        <Cluster>
          {SEEDS.map(pattern => (
            <Button key={pattern} onClick={() => send(call("life.reset", { pattern }))}>
              {pattern}
            </Button>
          ))}
        </Cluster>
        <Field label="density">
          <Slider min={1} max={99} step={1} value={s.settings.density} onChange={v => send(turn("density", v))} />
        </Field>
      </Section>
      <Section label="tile">
        <Library kind="tile" host="life" name="seed" current={s.settings.seed} />
        <Field label="tiling">
          <Slider min={1} max={8} step={1} value={s.settings.tiling} onChange={v => send(turn("tiling", v))} />
        </Field>
        <Field label="padding">
          <Slider min={0} max={8} step={1} value={s.settings.padding} onChange={v => send(turn("padding", v))} />
        </Field>
      </Section>
      <Section label="neighborhood">
        <Caption>{`${s.max_neighbors} neighbors`}</Caption>
        <Library kind="tile" host="life" name="mask" current={s.settings.mask} />
      </Section>
      <Rule label="birth" which="birth" counts={counts} worn={s.settings.birth} />
      <Rule label="survive" which="survive" counts={counts} worn={s.settings.survive} />
      <Section label="board">
        <Field label="size">
          <Slider min={8} max={64} step={1} value={s.settings.size} onChange={v => send(turn("size", v))} />
        </Field>
        <Field label="wrap">
          <Toggle value={s.settings.wrap} onChange={v => send(turn("wrap", v))} />
        </Field>
        <Field label="zeros">
          <Toggle value={s.settings.zeros} onChange={v => send(turn("zeros", v))} />
        </Field>
        <Field label="ones">
          <Toggle value={s.settings.ones} onChange={v => send(turn("ones", v))} />
        </Field>
      </Section>
      <Section label="fill">
        <Library kind="colors" host="life" name="fill" current={s.settings.fill} />
      </Section>
      <Section label="void">
        <Library kind="colors" host="life" name="void" current={s.settings.void} />
      </Section>
    </Stack>
  )
}
