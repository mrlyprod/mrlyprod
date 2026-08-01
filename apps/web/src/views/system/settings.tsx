import { call, setter } from "../../builders.ts"
import { library } from "../../components/library.tsx"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const MODES = ["grid", "list"]

const FONTS = ["mono", "sans", "serif", "display", "mrly"]

const EMOJIS = ["system", "noto"]

const MATERIALS = ["solid", "glass"]

const RENDERS = ["cpu", "gpu"]

const NOTES = ["random", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

const WAVES = ["sine", "triangle", "square", "sawtooth"]

type State = {
  launchpad: string
  darkmode: boolean
  color: string
  fill: string
  font: string
  emoji: string
  scale: number
  radius: number
  pace: number
  background: string
  width: number
  material: string
  wallpaper: string
  seed: number
  render: string
  detail: number
  sound: boolean
  haptics: boolean
  note: string
  wave: string
  duration: number
}

const turn = setter("settings")

export function settings(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="settings">
      <Section keyName="launchpad" label="launchpad">
        <choice key="launchpad" value={s.launchpad} options={MODES} call={turn("launchpad")} arg="value" mode="row" />
      </Section>
      <Section keyName="render" label="render">
        <choice key="render" value={s.render} options={RENDERS} call={turn("render")} arg="value" mode="row" />
      </Section>
      <Section keyName="detail" label="detail">
        <range key="detail" value={s.detail} min={32} max={160} step={1} call={turn("detail")} arg="value" label="detail" />
      </Section>
      <Section keyName="mode" label="mode">
        <grid key="mode-options" cols={2}>
          <button key="light" active={!s.darkmode} call={call("settings.set", { key: "darkmode", value: false })}>light</button>
          <button key="dark" active={s.darkmode} call={call("settings.set", { key: "darkmode", value: true })}>dark</button>
        </grid>
      </Section>
      <Section keyName="accent" label="accent">
        {library("colors", "settings", "color", s.color)}
      </Section>
      <Section keyName="fill" label="fill">
        {library("colors", "settings", "fill", s.fill)}
        <button key="fill-random" call={call("settings.set", { key: "fill", value: "random" })}>random fill</button>
      </Section>
      <Section keyName="background" label="background">
        {library("colors", "settings", "background", s.background)}
      </Section>
      <Section keyName="material" label="material">
        <choice key="material" value={s.material} options={MATERIALS} call={turn("material")} arg="value" mode="row" />
      </Section>
      <Section keyName="pattern" label="pattern">
        <grid key="pattern-options" cols={4}>
          <button key="none">none</button>
          <button key="emoji">emoji</button>
          <button key="tile">tile</button>
          <button key="glyph">glyph</button>
        </grid>
      </Section>
      <Section keyName="fonts" label="fonts">
        <choice key="font" value={s.font} options={FONTS} call={turn("font")} arg="value" mode="row" />
      </Section>
      <Section keyName="emojis" label="emojis">
        <choice key="emoji" value={s.emoji} options={EMOJIS} call={turn("emoji")} arg="value" mode="row" />
      </Section>
      <Section keyName="measure" label="measure">
        <range key="scale" value={s.scale} min={3} max={6} call={turn("scale")} arg="value" step={1} label="scale" />
        <range key="radius" value={s.radius} min={0} max={4} call={turn("radius")} arg="value" step={1} label="radius" />
        <range key="width" value={s.width} min={500} max={1500} call={turn("width")} arg="value" step={250} label="width" />
        <range key="pace" value={s.pace} min={0} max={400} call={turn("pace")} arg="value" step={50} label="pace" />
      </Section>
      <Section keyName="sound" label="sound">
        <toggle key="sound" on={s.sound} call={turn("sound")} arg="value" label="sound" />
        <choice key="note" value={s.note} options={NOTES} call={turn("note")} arg="value" label="note" />
        <choice key="wave" value={s.wave} options={WAVES} call={turn("wave")} arg="value" mode="row" />
        <range key="duration" value={s.duration} min={50} max={1000} call={turn("duration")} arg="value" step={50} label="duration" />
        <toggle key="haptics" on={s.haptics} call={turn("haptics")} arg="value" label="haptics" />
      </Section>
      <Section keyName="session" label="session">
        <grid key="session-tabs" cols={3}>
          <button key="export" call={call("journal.export")}>export</button>
          <button key="import" call={call("journal.import")}>import</button>
          <button key="reset" call={call("journal.reset")}>reset</button>
        </grid>
      </Section>
    </stack>
  )
}
