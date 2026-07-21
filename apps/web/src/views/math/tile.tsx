import { call, setter } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Fact = { rows: number[][]; palette: string[] }

type Source = { design?: string; code?: number }

type Paint = { edition: string; scheme: string; target: string; primary: string }

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
  }
  paint: Paint | null
  catalog: string
  parity: string
  budget: number
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
  thumbs: { level: number; frame: Fact }[]
  frame: Fact
}

const set = setter("tile")

const slotted = (key: string, slot: number) => call("tile.set", { key, slot })

const strings = (ns: number[]) => ns.map(String)

const label = (source: Source) =>
  source.design ?? `mrly_${String(source.code ?? 0).padStart(2, "0")}`

function slotCard(s: State, i: number): Node {
  const tile = s.tile
  const thumbs = tile.group === "Fractal" && i === 0 && s.thumbs.length > 0
  return (
    <Section keyName={`slot-${i}`} label={`slot ${i}`}>
      <choice
        key={`slot-${i}-source`}
        value={label(tile.sources[i] ?? {})}
        options={s.options.sources.map(o => o.label)}
        call={slotted("source", i)}
        arg="value"
        label="source"
      />
      {tile.group === "Magic" && (
        <choice
          key={`slot-${i}-number`}
          value={String(tile.numbers[i])}
          options={strings(s.options.numbers[i] ?? [])}
          call={slotted("number", i)}
          arg="value"
          label="number"
        />
      )}
      {thumbs && (
        <grid key={`slot-${i}-thumbs`} cols={3}>
          {s.thumbs.map(thumb => (
            <canvas
              key={`thumb-${thumb.level}`}
              handle="tile"
              rows={thumb.frame.rows}
              palette={thumb.frame.palette}
              tap={call("tile.set", { key: "level", slot: 0, value: thumb.level })}
            />
          ))}
        </grid>
      )}
      {!thumbs && tile.group === "Fractal" && i === 0 && (
        <choice
          key={`slot-${i}-level`}
          value={String(tile.levels[i])}
          options={strings(s.options.levels)}
          call={slotted("level", i)}
          arg="value"
          label="level"
        />
      )}
      <choice
        key={`slot-${i}-rotation`}
        value={String(tile.rotations[i])}
        options={strings(s.options.rotations)}
        call={slotted("rotation", i)}
        arg="value"
        label="rotation"
        mode="row"
      />
      <toggle
        key={`slot-${i}-anti`}
        on={tile.anti[i] ?? false}
        call={slotted("anti", i)}
        arg="value"
        label="anti"
      />
    </Section>
  )
}

export function tile(state: unknown, _send: Send): Node {
  const s = state as State
  const t = s.tile
  const paint = s.paint
  return (
    <stack key="tile">
      <card key="preview">
        <canvas key="frame" handle="tile" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      <Section keyName="shape" label="shape">
        <choice
          key="group"
          value={t.group}
          options={s.options.groups}
          call={set("group")}
          arg="value"
          label="group"
        />
        {s.options.counts.length > 0 && (
          <choice
            key="count"
            value={String(t.numbers.length)}
            options={strings(s.options.counts)}
            call={set("count")}
            arg="value"
            label="count"
          />
        )}
        {s.options.factors.length > 0 && (
          <choice
            key="factor"
            value={String(t.factor)}
            options={strings(s.options.factors)}
            call={set("factor")}
            arg="value"
            label="factor"
          />
        )}
        {t.group !== "Magic" && (
          <choice
            key="number"
            value={String(t.numbers[0])}
            options={strings(s.options.numbers[0] ?? [])}
            call={set("number")}
            arg="value"
            label="number"
          />
        )}
      </Section>
      {t.sources.map((_, i) => slotCard(s, i))}
      <Section keyName="tile" label="tile">
        <toggle key="invert" on={t.invert} call={set("invert")} arg="value" label="invert" />
        {t.group === "Special" && (
          <toggle key="flip" on={t.flip} call={set("flip")} arg="value" label="flip" />
        )}
        <choice
          key="parity"
          value={s.parity}
          options={s.options.parities}
          call={set("parity")}
          arg="value"
          label="parity"
          mode="row"
        />
        <choice
          key="catalog"
          value={s.catalog}
          options={s.options.catalogs}
          call={set("catalog")}
          arg="value"
          label="catalog"
          mode="row"
        />
        <choice
          key="budget"
          value={String(s.budget)}
          options={strings(s.options.budgets)}
          call={set("budget")}
          arg="value"
          label="budget"
          mode="row"
        />
      </Section>
      <Section keyName="paint" label="paint">
        <choice
          key="edition"
          value={paint?.edition ?? ""}
          options={s.options.editions}
          call={set("edition")}
          arg="value"
          label="edition"
        />
        <choice
          key="scheme"
          value={paint?.scheme ?? ""}
          options={s.options.schemes}
          call={set("scheme")}
          arg="value"
          label="scheme"
        />
        <choice
          key="target"
          value={paint?.target ?? ""}
          options={s.options.targets}
          call={set("target")}
          arg="value"
          label="target"
          mode="row"
        />
        <choice
          key="primary"
          value={paint?.primary ?? ""}
          options={s.options.primaries}
          call={set("primary")}
          arg="value"
          label="primary"
          mode="row"
        />
        <button key="repaint" call={call("tile.paint")}>{paint === null ? "paint" : "repaint"}</button>
        {paint !== null && <button key="strip" call={call("tile.strip")}>strip</button>}
      </Section>
      <card key="actions">
        <button key="roll" call={call("tile.roll")}>roll</button>
        <button key="reset" call={call("tile.reset")}>reset</button>
        <Shot />
      </card>
    </stack>
  )
}
