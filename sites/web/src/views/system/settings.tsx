import { Button, Choice, Grid, Section, Setting, Slider, Stack, Toggle } from "mrlyui"
import { call, set } from "../../builders"
import { Library } from "../../components/Library"
import { opts } from "../../components/options"
import { useSend } from "../../send"

const MODES = ["grid", "list"]

const FONTS = ["mono", "sans", "serif", "display", "mrly"]

const EMOJIS = ["system", "noto"]

const MATERIALS = ["solid", "glass"]

const WALLPAPERS = ["color", "pattern"]

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
  detail: number
  sound: boolean
  haptics: boolean
  note: string
  wave: string
  duration: number
}

const px = (n: number) => `${String(n)}px`

const ms = (n: number) => `${String(n)}ms`

export function Settings({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = (key: string, value: unknown) => send(set("settings", key, value))
  return (
    <Stack>
      <Section label="mode">
        <Grid cols={2}>
          <Button active={!s.darkmode} onClick={() => turn("darkmode", false)}>
            light
          </Button>
          <Button active={s.darkmode} onClick={() => turn("darkmode", true)}>
            dark
          </Button>
        </Grid>
        <Setting label="material" hint="the kernel keeps it, nothing paints it">
          <Choice mode="row" options={opts(MATERIALS)} value={s.material} onChange={v => turn("material", v)} />
        </Setting>
        <Setting label="launchpad">
          <Choice mode="row" options={opts(MODES)} value={s.launchpad} onChange={v => turn("launchpad", v)} />
        </Setting>
      </Section>
      <Section label="accent">
        <Library kind="colors" host="settings" name="color" current={s.color} />
      </Section>
      <Section label="background">
        <Library kind="colors" host="settings" name="background" current={s.background} />
      </Section>
      <Section label="fill">
        <Library kind="colors" host="settings" name="fill" current={s.fill} />
        <Button wide active={s.fill === "random"} onClick={() => turn("fill", "random")}>
          random
        </Button>
      </Section>
      <Section label="type">
        <Setting label="font">
          <Choice options={opts(FONTS)} value={s.font} onChange={v => turn("font", v)} />
        </Setting>
        <Setting label="emoji">
          <Choice mode="row" options={opts(EMOJIS)} value={s.emoji} onChange={v => turn("emoji", v)} />
        </Setting>
      </Section>
      <Section label="measure">
        <Setting label="scale">
          <Slider min={3} max={6} step={1} value={s.scale} onChange={v => turn("scale", v)} format={px} />
        </Setting>
        <Setting label="radius">
          <Slider min={0} max={4} step={1} value={s.radius} onChange={v => turn("radius", v)} />
        </Setting>
        <Setting label="width">
          <Slider min={500} max={1500} step={250} value={s.width} onChange={v => turn("width", v)} format={px} />
        </Setting>
        <Setting label="pace">
          <Slider min={0} max={400} step={50} value={s.pace} onChange={v => turn("pace", v)} format={ms} />
        </Setting>
        <Setting label="detail">
          <Slider min={32} max={160} step={1} value={s.detail} onChange={v => turn("detail", v)} />
        </Setting>
      </Section>
      <Section label="wallpaper">
        <Setting label="source">
          <Choice mode="row" options={opts(WALLPAPERS)} value={s.wallpaper} onChange={v => turn("wallpaper", v)} />
        </Setting>
        <Setting label="seed">
          <Slider min={0} max={999} step={1} value={s.seed} onChange={v => turn("seed", v)} />
        </Setting>
      </Section>
      <Section label="sound">
        <Setting label="sound">
          <Toggle value={s.sound} onChange={v => turn("sound", v)} />
        </Setting>
        <Setting label="haptics">
          <Toggle value={s.haptics} onChange={v => turn("haptics", v)} />
        </Setting>
        <Setting label="note">
          <Choice options={opts(NOTES)} value={s.note} onChange={v => turn("note", v)} />
        </Setting>
        <Setting label="wave">
          <Choice mode="row" options={opts(WAVES)} value={s.wave} onChange={v => turn("wave", v)} />
        </Setting>
        <Setting label="duration">
          <Slider min={50} max={1000} step={50} value={s.duration} onChange={v => turn("duration", v)} format={ms} />
        </Setting>
      </Section>
      <Section label="session">
        <Grid cols={3}>
          <Button onClick={() => send(call("journal.export"))}>export</Button>
          <Button onClick={() => send(call("journal.import"))}>import</Button>
          <Button onClick={() => send(call("journal.reset"))}>reset</Button>
        </Grid>
        <Button wide onClick={() => send(call("device.install"))}>
          install
        </Button>
      </Section>
    </Stack>
  )
}
