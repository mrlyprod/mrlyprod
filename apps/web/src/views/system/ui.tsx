import {
  Badge,
  Box,
  Button,
  Caption,
  Chip,
  Choice,
  Cluster,
  Emoji,
  Field,
  Grid,
  HEX,
  Image,
  Input,
  Label,
  Letters,
  Modal,
  Progress,
  Search,
  Section,
  Skeleton,
  Slider,
  Spinner,
  Stack,
  Stepper,
  Symbol,
  Text,
  Title,
  Toggle,
} from "mrlyui"
import { set } from "../../builders"
import { opts } from "../../components/options"
import { Bits } from "../../eyes/Bits"
import { useSend } from "../../send"

const PICKS = ["alpha", "beta", "gamma"]

const ICONS = ["search", "check", "close", "play_arrow"] as const

const SWATCH = `data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='96'%20height='96'%3E%3Ccircle%20cx='48'%20cy='48'%20r='40'%20fill='%23${HEX.gray.slice(1)}'/%3E%3C/svg%3E`

type State = {
  sample: string
  overlay: boolean
  toggle: boolean
  pick: string
  span: number
}

const ramp = (): number[][] =>
  Array.from({ length: 24 }, (_, y) => Array.from({ length: 24 }, (_, x) => Math.floor(((x + y) * 255) / 46)))

export function Ui({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = (key: string, value: unknown) => send(set("ui", key, value))
  const write = (value: string) => {
    if (value.trim() !== "") turn("sample", value)
  }
  return (
    <Stack>
      <Section label="text">
        <Title>{s.sample}</Title>
        <Text>{s.sample}</Text>
        <Caption>{s.sample}</Caption>
        <Letters text={s.sample} />
      </Section>
      <Section label="button">
        <Grid cols={2}>
          <Button onClick={() => turn("toggle", !s.toggle)}>{s.toggle ? "on" : "off"}</Button>
          <Button primary onClick={() => turn("overlay", true)}>
            primary
          </Button>
          <Button active={s.toggle} onClick={() => turn("toggle", !s.toggle)}>
            active
          </Button>
          <Button disabled onClick={() => turn("toggle", s.toggle)}>
            disabled
          </Button>
        </Grid>
        <Button wide onClick={() => turn("overlay", true)}>
          open the overlay
        </Button>
      </Section>
      <Section label="input">
        <Field label="field" hint="writes straight to the kernel">
          <Input value={s.sample} onChange={write} />
        </Field>
        <Search value={s.sample} onChange={write} placeholder="search" />
        <Field label="toggle">
          <Toggle value={s.toggle} onChange={v => turn("toggle", v)} />
        </Field>
      </Section>
      <Section label="choice">
        <Choice label="select" options={opts(PICKS)} value={s.pick} onChange={v => turn("pick", v)} />
        <Choice label="row" mode="row" options={opts(PICKS)} value={s.pick} onChange={v => turn("pick", v)} />
        <Choice label="cycle" mode="cycle" options={opts(PICKS)} value={s.pick} onChange={v => turn("pick", v)} />
      </Section>
      <Section label="range">
        <Field label="slider">
          <Slider min={0} max={10} step={1} value={s.span} onChange={v => turn("span", v)} />
        </Field>
        <Field label="stepper">
          <Stepper min={0} max={10} step={1} value={s.span} onChange={v => turn("span", v)} />
        </Field>
        <Progress value={s.span} max={10} />
      </Section>
      <Section label="label">
        <Label mode="row" symbol={{ as: "emoji", value: "🧪" }} text={s.sample} />
        <Label mode="text" text={s.sample} note="a caption" />
        <Label mode="row" symbol={{ as: "icon", value: "download" }} text="an icon label" />
        <Grid cols={3}>
          <Label
            mode="stack"
            symbol={{ as: "emoji", value: "🧪" }}
            text={s.sample}
            note="a tile"
            onClick={() => turn("toggle", !s.toggle)}
          />
          <Label mode="icon" symbol={{ as: "emoji", value: "🧪" }} text="specimen" />
          <Label mode="row" symbol={{ as: "glyph", value: "42" }} text="a glyph" />
        </Grid>
      </Section>
      <Section label="media">
        <Emoji glyph="🧪" label="specimen" size="var(--font-xl)" />
        <Image src={SWATCH} alt="gray circle swatch" />
        <Grid cols={4}>
          {ICONS.map(name => (
            <Symbol key={name} name={name} />
          ))}
        </Grid>
      </Section>
      <Section label="status">
        <Cluster>
          <Badge variant="info">info</Badge>
          <Badge variant="success">success</Badge>
          <Badge variant="warn">warn</Badge>
          <Badge variant="danger">danger</Badge>
        </Cluster>
        <Cluster>
          <Chip active={s.toggle} onClick={() => turn("toggle", !s.toggle)}>
            chip
          </Chip>
          <Chip>plain</Chip>
        </Cluster>
        <Spinner />
        <Skeleton lines={2} head />
      </Section>
      <Section label="canvas">
        <Bits rows={ramp()} handle="specimen" />
      </Section>
      <Section label="grid">
        <Grid cols={3}>
          {Array.from({ length: 6 }, (_, i) => (
            <Text key={i}>{i + 1}</Text>
          ))}
        </Grid>
      </Section>
      <Modal open={s.overlay} onClose={() => turn("overlay", false)} title="overlay">
        <Box>
          <Text>{s.sample}</Text>
          <Button wide onClick={() => turn("overlay", false)}>
            close
          </Button>
        </Box>
      </Modal>
    </Stack>
  )
}
