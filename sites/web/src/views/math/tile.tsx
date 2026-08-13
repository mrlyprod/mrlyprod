import {
  Box,
  Caption,
  Card,
  Button,
  Choice,
  Cluster,
  Field,
  Grid,
  Input,
  Section,
  Stack,
  Toggle,
} from "mrlyui"
import { call, setter } from "../../builders"
import { opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type Source = { design?: string; code?: number | string }

type Paint = { edition: string; scheme: string; target: string; primary: string }

type Slab = { id: number; name: string; value: unknown; cells: Deck }

type State = {
  tile: {
    group: string
    factor: number
    sources: Source[]
    numbers: number[]
    levels: number[]
    rotations: number[]
    anti: boolean[]
    invert: boolean
    flip: boolean
    name: string | null
  }
  paint: Paint | null
  catalog: string
  parity: string
  budget: number
  labels: string[]
  note: string | null
  options: {
    groups: string[]
    catalogs: string[]
    parities: string[]
    budgets: number[]
    editions: string[]
    schemes: string[]
    targets: string[]
    primaries: string[]
    sources: { label: string; value: string }[]
    rotations: number[]
    numbers: number[][]
    levels: number[]
    counts: number[]
    factors: number[]
  }
  thumbs: { level: number; cells: Deck }[]
  library: Slab[]
  cells: Deck
}

const strings = (ns: number[]): string[] => ns.map(String)

// SLOT

function Slot({ s, i }: { s: State; i: number }) {
  const send = useSend()
  const t = s.tile
  const pick = (key: string, value: unknown) => send(call("tile.set", { key, slot: i, value }))
  const thumbs = t.group === "Fractal" && i === 0 && s.thumbs.length > 0
  return (
    <Section label={`slot ${i + 1}`}>
      <Choice
        label="source"
        value={s.labels?.[i] ?? ""}
        options={s.options.sources}
        onChange={v => pick("source", v)}
      />
      {t.group === "Magic" && (
        <Choice
          label="number"
          value={String(t.numbers[i] ?? 0)}
          options={opts(strings(s.options.numbers[i] ?? []))}
          onChange={v => pick("number", v)}
        />
      )}
      {thumbs && (
        <Grid cols={3}>
          {s.thumbs.map(thumb => (
            <Card
              key={thumb.level}
              active={t.levels[0] === thumb.level}
              onClick={() => send(call("tile.set", { key: "level", slot: 0, value: thumb.level }))}
            >
              <Cells app="tile" cells={thumb.cells} handle={`thumb-${thumb.level}`} />
            </Card>
          ))}
        </Grid>
      )}
      {!thumbs && t.group === "Fractal" && i === 0 && (
        <Choice
          label="level"
          value={String(t.levels[i] ?? 0)}
          options={opts(strings(s.options.levels))}
          onChange={v => pick("level", v)}
        />
      )}
      <Choice
        label="rotation"
        mode="row"
        value={String(t.rotations[i] ?? 0)}
        options={opts(strings(s.options.rotations))}
        onChange={v => pick("rotation", v)}
      />
      {t.group !== "Special" && (
        <Field label="anti">
          <Toggle value={t.anti[i] ?? false} onChange={v => pick("anti", v)} />
        </Field>
      )}
    </Section>
  )
}

// TILE

export function Tile({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("tile")
  const t = s.tile
  const paint = s.paint
  return (
    <Stack>
      <Box>
        <Cells app="tile" cells={s.cells} handle="tile" />
        {t.name != null && <Caption>{t.name}</Caption>}
        {s.note != null && <Caption>{s.note}</Caption>}
      </Box>
      <Box>
        <Cluster>
          <Button onClick={() => send(call("tile.roll"))}>roll</Button>
          <Button onClick={() => send(call("tile.reset"))}>reset</Button>
          <Button onClick={() => send(call("tile.save"))}>save</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="shape">
        <Choice
          label="group"
          value={t.group}
          options={opts(s.options.groups)}
          onChange={v => send(turn("group", v))}
        />
        {s.options.counts.length > 0 && (
          <Choice
            label="count"
            value={String(t.numbers.length)}
            options={opts(strings(s.options.counts))}
            onChange={v => send(turn("count", v))}
          />
        )}
        {s.options.factors.length > 0 && (
          <Choice
            label="factor"
            value={String(t.factor)}
            options={opts(strings(s.options.factors))}
            onChange={v => send(turn("factor", v))}
          />
        )}
        {t.group !== "Magic" && (
          <Choice
            label="number"
            value={String(t.numbers[0] ?? 0)}
            options={opts(strings(s.options.numbers[0] ?? []))}
            onChange={v => send(turn("number", v))}
          />
        )}
      </Section>
      {t.sources.map((_, i) => (
        <Slot key={i} s={s} i={i} />
      ))}
      <Section label="tile">
        <Choice
          label="catalog"
          mode="row"
          value={s.catalog}
          options={opts(s.options.catalogs)}
          onChange={v => send(turn("catalog", v))}
        />
        <Choice
          label="parity"
          mode="row"
          value={s.parity}
          options={opts(s.options.parities)}
          onChange={v => send(turn("parity", v))}
        />
        <Choice
          label="budget"
          mode="row"
          value={String(s.budget)}
          options={opts(strings(s.options.budgets))}
          onChange={v => send(turn("budget", v))}
        />
        <Field label="invert">
          <Toggle value={t.invert} onChange={v => send(turn("invert", v))} />
        </Field>
        {t.group === "Special" && (
          <Field label="flip">
            <Toggle value={t.flip} onChange={v => send(turn("flip", v))} />
          </Field>
        )}
      </Section>
      <Section label="paint">
        <Choice
          label="edition"
          value={paint?.edition ?? ""}
          options={opts(s.options.editions)}
          onChange={v => send(turn("edition", v))}
        />
        <Choice
          label="scheme"
          value={paint?.scheme ?? ""}
          options={opts(s.options.schemes)}
          onChange={v => send(turn("scheme", v))}
        />
        <Choice
          label="target"
          mode="row"
          value={paint?.target ?? ""}
          options={opts(s.options.targets)}
          onChange={v => send(turn("target", v))}
        />
        <Choice
          label="primary"
          mode="row"
          value={paint?.primary ?? ""}
          options={opts(s.options.primaries)}
          onChange={v => send(turn("primary", v))}
        />
        <Cluster>
          <Button onClick={() => send(call("tile.paint"))}>{paint === null ? "paint" : "repaint"}</Button>
          {paint !== null && <Button onClick={() => send(call("tile.strip"))}>strip</Button>}
        </Cluster>
      </Section>
      <Section label="library">
        <Grid cols={3}>
          {s.library.map(entry => (
            <Box key={entry.id}>
              <Cells app="tile" cells={entry.cells} handle={`lib-${entry.id}`} />
              <Input
                value={entry.name}
                onChange={v => send(call("tile.name", { id: entry.id, name: v }))}
              />
              <Button onClick={() => send(call("tile.drop", { id: entry.id }))}>drop</Button>
            </Box>
          ))}
        </Grid>
      </Section>
    </Stack>
  )
}
