import { call, setter } from "../../builders.ts"
import { hex } from "../../palette.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const PICKS = ["alpha", "beta", "gamma"]

function swatch(): string {
  return `data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='96'%20height='96'%3E%3Ccircle%20cx='48'%20cy='48'%20r='40'%20fill='%23${hex("gray").slice(1)}'/%3E%3C/svg%3E`
}

type State = {
  sample: string
  overlay: boolean
  toggle: boolean
  pick: string
  span: number
}

const set = (key: string, value: unknown) => call("ui.set", { key, value })

const turn = setter("ui")

const ramp = (): number[][] =>
  Array.from({ length: 24 }, (_, y) => Array.from({ length: 24 }, (_, x) => Math.floor(((x + y) * 255) / 46)))

export function ui(state: unknown, _send: Send): Node {
  const s = state as State
  const sample = s.sample
  return (
    <stack key="ui">
      <Section keyName="text" label="text">
        <text key="text-title" role="title">{sample}</text>
        <text key="text-body">{sample}</text>
        <text key="text-note" role="note">{sample}</text>
      </Section>
      <Section keyName="buttons" label="button">
        <button key="overlay-open" call={set("overlay", true)}>overlay</button>
        <button key="toggle-flip" call={set("toggle", !s.toggle)}>{s.toggle ? "on" : "off"}</button>
      </Section>
      <Section keyName="controls" label="controls">
        <field key="sample" value={sample} live={false} call={turn("sample")} arg="value" label="field" hint="sample text" />
        <field key="sample-bare" value={sample} live={false} call={turn("sample")} arg="value" />
        <toggle key="toggle" on={s.toggle} call={turn("toggle")} arg="value" label="toggle" />
        <toggle key="overlay" on={s.overlay} call={turn("overlay")} arg="value" />
        <choice key="pick" value={s.pick} options={PICKS} call={turn("pick")} arg="value" label="choice" />
        <choice key="pick-row" value={s.pick} options={PICKS} call={turn("pick")} arg="value" label="row" mode="row" />
        <choice key="pick-cycle" value={s.pick} options={PICKS} call={turn("pick")} arg="value" label="cycle" mode="cycle" />
        <range key="span" value={s.span} min={0} max={10} call={turn("span")} arg="value" step={1} label="range" />
      </Section>
      <Section keyName="labels" label="label">
        <label key="label-row" mode="row" symbol={{ as: "emoji", value: "🧪" }} text={sample} />
        <label key="label-text" mode="text" text={sample} note="a caption" />
        <grid key="label-tiles" cols={3}>
          <label key="label-stack" mode="stack" symbol={{ as: "emoji", value: "🧪" }} text={sample} note="a tile" call={set("toggle", !s.toggle)} />
          <label key="label-icon" mode="icon" symbol={{ as: "emoji", value: "🧪" }} text="specimen" />
        </grid>
      </Section>
      <Section keyName="media" label="media">
        <symbol key="glyph" as="emoji" value="🧪" />
        <image key="swatch" src={swatch()} alt="gray circle swatch" />
        <grid key="icons" cols={4}>
          <symbol key="icon-search" as="icon" value="search" />
          <symbol key="icon-check" as="icon" value="check" />
          <symbol key="icon-close" as="icon" value="close" />
          <symbol key="icon-play" as="icon" value="play" />
        </grid>
        <label key="label-icon-real" mode="row" symbol={{ as: "icon", value: "download" }} text="an icon label" />
      </Section>
      <Section keyName="canvas" label="canvas">
        <canvas key="ramp" handle="specimen" rows={ramp()} />
      </Section>
      <Section keyName="layout" label="grid">
        <grid key="grid" cols={3}>
          {Array.from({ length: 6 }, (_, i) => (
            <text key={`cell-${i + 1}`}>{i + 1}</text>
          ))}
        </grid>
      </Section>
      {s.overlay && (
        <overlay key="overlay-demo" close={set("overlay", false)}>
          <card key="overlay-card">
            <text key="overlay-text">{sample}</text>
            <button key="overlay-close" call={set("overlay", false)}>close</button>
          </card>
        </overlay>
      )}
    </stack>
  )
}
