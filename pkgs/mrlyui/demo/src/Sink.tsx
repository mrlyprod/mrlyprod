import { useState } from "react"
import "./sink.css"
import {
  Alert,
  Autocomplete,
  Badge,
  Banner,
  Board,
  Box,
  Button,
  Calendar,
  Card,
  Cell,
  Checkbox,
  Chip,
  Cluster,
  ColorPicker,
  Crumbs,
  DatePicker,
  Drawer,
  Dropdown,
  Emoji,
  Field,
  Frame,
  GraphemeInput,
  Header,
  Grid,
  Icon,
  Input,
  Letters,
  Modal,
  POOL,
  Pager,
  Popover,
  Progress,
  Radio,
  Row,
  Search,
  Section,
  Select,
  Sheet,
  Slider,
  Spinner,
  Stack,
  StepSlider,
  Stepper,
  Symbol,
  Tabs,
  Text,
  Textarea,
  THEME_ICONS,
  Title,
  Toast,
  Toggle,
  Tooltip,
  useFont,
  useTheme,
} from "mrlyui"
import type { ColorName, Variant } from "mrlyui"

const VARIANTS: Variant[] = ["info", "success", "warn", "danger"]

const FRUIT = ["apple", "banana", "cherry", "grape", "lemon", "mango", "melon", "peach", "pear", "plum"]

function set(name: string, value: string) {
  if (value === "") document.documentElement.style.removeProperty(name)
  else document.documentElement.style.setProperty(name, value)
}

function Knobs() {
  const [theme, cycle] = useTheme()
  const [mrly, toggleFont] = useFont()
  const [unit, setUnit] = useState(0)
  const [border, setBorder] = useState(1)
  const [radius, setRadius] = useState(2)
  const [accent, setAccent] = useState<ColorName | null>(null)

  const tune = (next: number) => {
    setUnit(next)
    set("--unit", next === 0 ? "" : `${next}px`)
  }

  const fence = (next: number) => {
    setBorder(next)
    set("--border-width", `${next}px`)
  }

  const round = (next: number) => {
    setRadius(next)
    set("--radius", next === 0 ? "0px" : `calc(var(--unit) * ${next})`)
  }

  const paint = (next: ColorName | null) => {
    setAccent(next)
    set("--accent-color", next === null ? "" : `var(--c-${next})`)
  }

  const reset = () => {
    tune(0)
    fence(1)
    round(2)
    paint(null)
  }

  return (
    <Stack>
      <Row>
        <Button onClick={cycle}>
          <Symbol name={THEME_ICONS[theme]} /> theme
        </Button>
        <Button active={mrly} onClick={toggleFont}>
          mrlyfont
        </Button>
        <Button onClick={reset}>reset</Button>
      </Row>
      <Field label="unit" hint="0 keeps the fluid clamp">
        <Slider min={0} max={8} value={unit} onChange={tune} />
      </Field>
      <Field label="border">
        <Slider min={0} max={5} value={border} onChange={fence} />
      </Field>
      <Field label="radius">
        <Slider min={0} max={8} value={radius} onChange={round} />
      </Field>
      <Field label="accent">
        <ColorPicker auto value={accent} onChange={paint} />
      </Field>
    </Stack>
  )
}

function Plates() {
  const [pick, setPick] = useState<ColorName | null>(null)
  return (
    <Stack>
      <Cluster>
        {POOL.map(name => (
          <Chip key={name} active={pick === name} onClick={() => setPick(name)}>
            {name}
          </Chip>
        ))}
      </Cluster>
      <Grid cols={4}>
        <Card plate={pick ?? "auto"}>
          <Emoji glyph="🧱" />
        </Card>
        <Card plate="auto">
          <Emoji glyph="🎲" />
        </Card>
        <Card plate="auto">
          <Emoji glyph="🕹️" />
        </Card>
        <Card plate="auto">
          <Emoji glyph="🎨" />
        </Card>
      </Grid>
    </Stack>
  )
}

function Boxes() {
  const [cols, setCols] = useState(3)
  const [on, setOn] = useState(2)
  return (
    <Stack>
      <Field label="grid">
        <Slider min={2} max={6} value={cols} onChange={setCols} />
      </Field>
      <Grid cols={cols}>
        {Array.from({ length: 6 }, (_, i) => (
          <Card key={i} active={on === i} onClick={() => setOn(i)}>
            <Text>{i + 1}</Text>
          </Card>
        ))}
      </Grid>
      <Row>
        <Box>row</Box>
        <Box>of</Box>
        <Box>boxes</Box>
      </Row>
      <Board cols={8} rows={4}>
        <Cell x={1} y={1}>
          <Emoji glyph="🐍" />
        </Cell>
        <Cell x={2} y={1} />
        <Cell x={3} y={1} />
        <Cell x={6} y={3}>
          <Emoji glyph="🍎" />
        </Cell>
      </Board>
    </Stack>
  )
}

function Words() {
  return (
    <Stack>
      <Letters text="mrlyui" />
      <Title>The title row</Title>
      <Text>Body text carries the reading voice of the system.</Text>
      <Text className="caption">A caption whispers under it.</Text>
      <div className="doc">
        <h2>Prose</h2>
        <p>
          The <code>.doc</code> scope renders markdown output: paragraphs, <strong>bold</strong>, <em>emphasis</em>, and lists.
        </p>
        <ul>
          <li>One unit.</li>
          <li>Many boxes.</li>
        </ul>
      </div>
    </Stack>
  )
}

function Glyphs() {
  const [app, setApp] = useState("snake")
  return (
    <Stack>
      <Cluster>
        <Symbol name="search" />
        <Symbol name="close" />
        <Symbol name="check" />
        <Symbol name="info" />
        <Symbol name="light_mode" />
        <Symbol name="dark_mode" />
        <Symbol name="chevron_left" />
        <Symbol name="chevron_right" />
      </Cluster>
      <Row>
        <Icon emoji="🐍" label="snake" active={app === "snake"} onClick={() => setApp("snake")} />
        <Icon emoji="♟️" label="chess" active={app === "chess"} onClick={() => setApp("chess")} />
        <Icon emoji="🧠" label="memory" active={app === "memory"} onClick={() => setApp("memory")} />
        <Icon emoji="⏱️" label="timer" active={app === "timer"} onClick={() => setApp("timer")} />
      </Row>
      <Row>
        <Emoji glyph="🧱" size="2rem" />
        <Emoji glyph="🧱" size="3rem" />
        <Emoji glyph="🧱" size="4rem" />
      </Row>
    </Stack>
  )
}

function Buttons() {
  const [armed, setArmed] = useState(false)
  return (
    <Stack>
      <Row>
        <Button>plain</Button>
        <Button primary>primary</Button>
        <Button active={armed} onClick={() => setArmed(!armed)}>
          {armed ? "armed" : "arm"}
        </Button>
        <Button disabled>disabled</Button>
      </Row>
      <Button wide>wide</Button>
    </Stack>
  )
}

function Controls() {
  const [name, setName] = useState("")
  const [notes, setNotes] = useState("")
  const [query, setQuery] = useState("")
  const [glyph, setGlyph] = useState("m")
  const [on, setOn] = useState(true)
  const [checked, setChecked] = useState(false)
  const [mode, setMode] = useState<"fill" | "carve" | "shade">("fill")
  const [count, setCount] = useState(4)
  const [level, setLevel] = useState(40)
  const [pace, setPace] = useState<"slow" | "steady" | "fast">("steady")
  const [ink, setInk] = useState<ColorName | null>("blue")
  return (
    <Stack>
      <Box>
        <Stack>
          <Text className="caption">words</Text>
          <Field label="name" hint="a controlled input">
            <Input value={name} onChange={setName} placeholder="type here" />
          </Field>
          <Field label="notes">
            <Textarea value={notes} onChange={setNotes} placeholder="a few lines" rows={3} />
          </Field>
          <Field label="search">
            <Search value={query} onChange={setQuery} onClear={() => setQuery("")} placeholder="find" />
          </Field>
          <Field label="glyph" hint="keeps the last grapheme">
            <GraphemeInput value={glyph} onChange={setGlyph} />
          </Field>
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">switches</Text>
          <Row>
            <Toggle value={on} onChange={setOn} />
            <Checkbox checked={checked} onChange={setChecked} label="agreed" />
          </Row>
          <Field label="mode">
            <Radio
              options={[
                { label: "fill", value: "fill" },
                { label: "carve", value: "carve" },
                { label: "shade", value: "shade" },
              ]}
              value={mode}
              onChange={setMode}
            />
          </Field>
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">amounts</Text>
          <Field label="count">
            <Stepper value={count} onChange={setCount} min={1} max={9} />
          </Field>
          <Field label="level" error={level > 80 ? "too hot" : undefined}>
            <Slider min={0} max={100} value={level} onChange={setLevel} />
          </Field>
          <Field label="pace">
            <StepSlider
              steps={[
                { label: "slow", value: "slow" },
                { label: "steady", value: "steady" },
                { label: "fast", value: "fast" },
              ]}
              value={pace}
              onChange={setPace}
            />
          </Field>
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">ink</Text>
          <ColorPicker value={ink} onChange={setInk} />
        </Stack>
      </Box>
    </Stack>
  )
}

function Pickers() {
  const [fruit, setFruit] = useState<string>()
  const [query, setQuery] = useState("")
  const [picked, setPicked] = useState("")
  const [date, setDate] = useState<Date>()
  const [day, setDay] = useState<Date>()
  return (
    <Stack>
      <Field label="select">
        <Select
          options={FRUIT.map(f => ({ label: f, value: f }))}
          value={fruit}
          onChange={setFruit}
          placeholder="pick a fruit"
        />
      </Field>
      <Field label="menu">
        <Dropdown
          label="actions"
          items={[
            { label: "copy", value: "copy" },
            { label: "export", value: "export" },
            { label: "delete", value: "delete", danger: true },
          ]}
          onSelect={() => {}}
        />
      </Field>
      <Field label="autocomplete" hint={picked === "" ? undefined : `picked ${picked}`}>
        <Autocomplete
          value={query}
          onChange={setQuery}
          items={FRUIT.filter(f => query !== "" && f.includes(query.toLowerCase()))}
          itemKey={f => f}
          renderItem={f => f}
          onSelect={f => {
            setPicked(f)
            setQuery(f)
          }}
          placeholder="fruit again"
          emptyLabel="no fruit"
        />
      </Field>
      <Field label="date">
        <DatePicker value={date} onChange={setDate} placeholder="pick a day" />
      </Field>
      <Calendar value={day} onSelect={setDay} />
    </Stack>
  )
}

function Feedback() {
  const [toast, setToast] = useState(false)
  const [banner, setBanner] = useState(true)
  const [progress, setProgress] = useState(64)
  return (
    <Stack>
      {banner && (
        <Banner variant="info" onClose={() => setBanner(false)}>
          A banner across the page.
        </Banner>
      )}
      {VARIANTS.map(v => (
        <Alert key={v} variant={v} title={v}>
          An alert in the {v} voice.
        </Alert>
      ))}
      <Row>
        <Button onClick={() => setToast(true)}>toast</Button>
        <Badge variant="info">3</Badge>
        <Badge variant="danger">9+</Badge>
        <Badge dot variant="success" />
        <Spinner />
      </Row>
      <Cluster>
        <Chip>plain</Chip>
        <Chip active>active</Chip>
        <Chip onRemove={() => {}}>removable</Chip>
      </Cluster>
      <Field label="progress">
        <Slider min={0} max={100} value={progress} onChange={setProgress} />
      </Field>
      <Progress value={progress} />
      <Progress />
      <Toast open={toast} onClose={() => setToast(false)} variant="success">
        Toasted.
      </Toast>
    </Stack>
  )
}

function Navigation() {
  const [tab, setTab] = useState<"one" | "two" | "three">("one")
  const [page, setPage] = useState(1)
  return (
    <Stack>
      <Tabs
        tabs={[
          { label: "one", value: "one" },
          { label: "two", value: "two" },
          { label: "three", value: "three" },
        ]}
        value={tab}
        onChange={setTab}
      />
      <Text>the {tab} pane</Text>
      <Crumbs root="mrlyui" route="demo/sink" />
      <Pager current={page} total={5} onPrev={() => setPage(page - 1)} onNext={() => setPage(page + 1)} />
    </Stack>
  )
}

function Overlays() {
  const [modal, setModal] = useState(false)
  const [drawer, setDrawer] = useState(false)
  const [sheet, setSheet] = useState(false)
  return (
    <Stack>
      <Row>
        <Button onClick={() => setModal(true)}>modal</Button>
        <Button onClick={() => setDrawer(true)}>drawer</Button>
        <Button onClick={() => setSheet(true)}>sheet</Button>
        <Popover trigger={({ toggle }) => <Button onClick={toggle}>popover</Button>}>
          <Box>
            <Text>Anchored to the trigger.</Text>
          </Box>
        </Popover>
        <Tooltip label="a hint">
          <Button>hover me</Button>
        </Tooltip>
      </Row>
      <Modal open={modal} onClose={() => setModal(false)} title="A modal">
        <Text>Esc, backdrop, or the close button.</Text>
      </Modal>
      <Drawer open={drawer} onClose={() => setDrawer(false)} title="A drawer">
        <Text>Slides in from the right.</Text>
      </Drawer>
      <Sheet open={sheet} onClose={() => setSheet(false)}>
        <Text>The bottom sheet.</Text>
      </Sheet>
    </Stack>
  )
}

export function Sink() {
  const [pane, setPane] = useState<"" | "menu" | "iden">("")
  const [marked, setMarked] = useState(false)
  return (
    <Frame>
      <Stack airy>
        <Header
          open={pane}
          onMenu={() => setPane(pane === "menu" ? "" : "menu")}
          onMark={() => setMarked(true)}
          onIden={() => setPane(pane === "iden" ? "" : "iden")}
        />
        <Drawer open={pane === "menu"} onClose={() => setPane("")} side="left" title="menu">
          <Text>The menu drawer, opened by the plus.</Text>
        </Drawer>
        <Modal open={pane === "iden"} onClose={() => setPane("")} title="iden">
          <Text>The identity pane, opened by the O.</Text>
        </Modal>
        <Toast open={marked} onClose={() => setMarked(false)}>
          mrly
        </Toast>
        <header className="masthead">
          <Letters text="mrlyui" />
          <Text className="caption">one unit, many boxes</Text>
        </header>
        <Section label="theme">
          <Knobs />
        </Section>
        <Section label="plates">
          <Plates />
        </Section>
        <Section label="boxes">
          <Boxes />
        </Section>
        <Section label="words">
          <Words />
        </Section>
        <Section label="glyphs">
          <Glyphs />
        </Section>
        <Section label="buttons">
          <Buttons />
        </Section>
        <Section label="controls">
          <Controls />
        </Section>
        <Section label="pickers">
          <Pickers />
        </Section>
        <Section label="feedback">
          <Feedback />
        </Section>
        <Section label="navigation">
          <Navigation />
        </Section>
        <Section label="overlays">
          <Overlays />
        </Section>
      </Stack>
    </Frame>
  )
}
