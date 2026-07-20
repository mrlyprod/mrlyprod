import { call, setter, pickOpen } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const MODES = ["grid", "list"]

const FONTS = ["mono", "sans", "serif", "display", "mrly"]

const EMOJIS = ["system", "noto"]

const MATERIALS = ["solid", "glass"]

const WALLPAPERS = ["color", "pattern"]

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
      <Section keyName="system" label="system">
        <choice key="launchpad" value={s.launchpad} options={MODES} call={turn("launchpad")} arg="value" label="launchpad" mode="row" />
        <choice key="render" value={s.render} options={RENDERS} call={turn("render")} arg="value" label="render" mode="row" />
      </Section>
      <Section keyName="paint" label="paint">
        <toggle key="darkmode" on={s.darkmode} call={turn("darkmode")} arg="value" label="dark mode" />
        <button key="color" call={pickOpen("colors", "settings", "color", s.color)}>{`accent · ${s.color}`}</button>
        <button key="fill" call={pickOpen("colors", "settings", "fill", s.fill)}>{`fill · ${s.fill}`}</button>
        <button key="fill-random" call={call("settings.set", { key: "fill", value: "random" })}>random fill</button>
        <button key="background" call={pickOpen("colors", "settings", "background", s.background)}>{`background · ${s.background}`}</button>
      </Section>
      <Section keyName="stage" label="stage">
        <choice key="material" value={s.material} options={MATERIALS} call={turn("material")} arg="value" label="material" mode="row" />
        <choice key="wallpaper" value={s.wallpaper} options={WALLPAPERS} call={turn("wallpaper")} arg="value" label="wallpaper" mode="row" />
        <range key="seed" value={s.seed} min={0} max={999} call={turn("seed")} arg="value" step={1} label="seed" />
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
        <button key="export" call={call("journal.export")}>export</button>
        <button key="import" call={call("journal.import")}>import</button>
        <button key="reset" call={call("journal.reset")}>reset</button>
      </Section>
    </stack>
  )
}
