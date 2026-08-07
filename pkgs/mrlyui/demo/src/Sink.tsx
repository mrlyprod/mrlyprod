import { useEffect, useState } from "react"
import "./sink.css"
import {
  Alert,
  applyPrefs,
  Autocomplete,
  Badge,
  Banner,
  Board,
  Box,
  Button,
  Card,
  Cell,
  Checkbox,
  Chip,
  Chrome,
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
  Grid,
  Header,
  Icon,
  Input,
  isoDate,
  Letters,
  loadPrefs,
  Mark,
  Modal,
  parseDate,
  POOL,
  Pager,
  Panes,
  Popover,
  Progress,
  Radio,
  read,
  Row,
  Search,
  Section,
  Select,
  Setting,
  Sheet,
  Slider,
  sound,
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
  usePanes,
  usePrefs,
  useTheme,
  write,
} from "mrlyui"
import type { ColorName, Fill, Fills, Variant } from "mrlyui"

const VARIANTS: Variant[] = ["info", "success", "warn", "danger"]

const FRUIT = ["apple", "banana", "cherry", "grape", "lemon", "mango", "melon", "peach", "pear", "plum"]

const STEPS = [0, 2, 4, 5, 7, 9, 11]

const WHITE_KEYS = ["C", "D", "E", "F", "G", "A", "B"]

function download(name: string, text: string, type: string) {
  const url = URL.createObjectURL(new Blob([text], { type }))
  const a = document.createElement("a")
  a.href = url
  a.download = name
  a.click()
  URL.revokeObjectURL(url)
}

function FillRow({ label, value, onPick }: {
  label: string
  value: Fill
  onPick: (next: Fill) => void
}) {
  return (
    <Stack>
      <Text className="caption">{label}</Text>
      <Cluster>
        <Chip active={value === ""} onClick={() => onPick("")}>
          mono
        </Chip>
        <Chip active={value === "random"} onClick={() => onPick("random")}>
          random
        </Chip>
      </Cluster>
      <ColorPicker
        value={value === "" || value === "random" ? null : value}
        onChange={c => onPick(c ?? "random")}
      />
    </Stack>
  )
}

function SettingsPane() {
  const [prefs, patch, reset] = usePrefs()
  const [theme, cycle] = useTheme()
  const [mrly, toggleFont] = useFont()
  const paint = (level: number) => (next: Fill) => {
    const fills: Fills = [...prefs.fills]
    fills[level] = next
    patch({ fills })
  }
  return (
    <Stack>
      <Setting label="theme">
        <Button onClick={cycle}>
          <Symbol name={THEME_ICONS[theme]} /> {theme === "" ? "auto" : theme}
        </Button>
      </Setting>
      <Setting label="mrlyfont">
        <Toggle value={mrly} onChange={toggleFont} />
      </Setting>
      <Box>
        <Stack>
          <Text className="caption">measure</Text>
          <Setting label="unit" hint="0 keeps the clamp">
            <Slider min={0} max={8} value={prefs.unit} onChange={v => patch({ unit: v })} />
          </Setting>
          <Setting label="border">
            <Slider min={0} max={5} value={prefs.border} onChange={v => patch({ border: v })} />
          </Setting>
          <Setting label="radius">
            <Slider min={0} max={8} value={prefs.radius} onChange={v => patch({ radius: v })} />
          </Setting>
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">accent</Text>
          <ColorPicker auto value={prefs.accent === "" ? null : prefs.accent} onChange={c => patch({ accent: c ?? "" })} />
        </Stack>
      </Box>
      <Box>
        <FillRow label="plates" value={prefs.fills[2]} onPick={paint(2)} />
      </Box>
      <Box>
        <FillRow label="boxes" value={prefs.fills[1]} onPick={paint(1)} />
      </Box>
      <Box>
        <FillRow label="bricks" value={prefs.fills[0]} onPick={paint(0)} />
      </Box>
      <Box>
        <Stack>
          <Text className="caption">lines</Text>
          <Cluster>
            <Chip active={prefs.line === ""} onClick={() => patch({ line: "" })}>
              ink
            </Chip>
          </Cluster>
          <ColorPicker
            value={prefs.line === "" ? null : prefs.line}
            onChange={c => patch({ line: c ?? "" })}
          />
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">background</Text>
          <Cluster>
            <Chip active={prefs.background === ""} onClick={() => patch({ background: "" })}>
              mono
            </Chip>
          </Cluster>
          <ColorPicker
            value={prefs.background === "" ? null : prefs.background}
            onChange={c => patch({ background: c ?? "" })}
          />
        </Stack>
      </Box>
      <Box>
        <Stack>
          <Text className="caption">sound</Text>
          <Setting label="sound">
            <Toggle value={prefs.sound} onChange={v => patch({ sound: v })} />
          </Setting>
          <Setting label="haptics">
            <Toggle value={prefs.haptics} onChange={v => patch({ haptics: v })} />
          </Setting>
          <Setting label="note">
            <Select
              options={["random", ...sound.NOTES].map(n => ({ label: n, value: n }))}
              value={prefs.note}
              onChange={n => patch({ note: n })}
            />
          </Setting>
          <Field label="wave">
            <Radio
              options={sound.WAVES.map(w => ({ label: w, value: w }))}
              value={prefs.wave}
              onChange={w => patch({ wave: w })}
            />
          </Field>
          <Setting label="duration">
            <Slider min={50} max={1000} step={50} value={prefs.duration} onChange={v => patch({ duration: v })} />
          </Setting>
        </Stack>
      </Box>
      <Button wide onClick={reset}>
        reset
      </Button>
    </Stack>
  )
}

type Iden = {
  name: string
  born: string
  favorite: ColorName | null
  bio: string
}

const IDEN_KEY = "mrly-iden"

function loadIden(): Iden {
  const held = read(IDEN_KEY)
  if (held === "") return { name: "", born: "", favorite: null, bio: "" }
  try {
    return JSON.parse(held) as Iden
  } catch {
    return { name: "", born: "", favorite: null, bio: "" }
  }
}

function IdenPane() {
  const [iden, setIden] = useState(loadIden)

  const patch = (part: Partial<Iden>) => {
    setIden(held => {
      const next = { ...held, ...part }
      write(IDEN_KEY, JSON.stringify(next))
      return next
    })
  }

  const md = () =>
    [
      `# ${iden.name || "someone"}`,
      "",
      `- born: ${iden.born || "unknown"}`,
      `- favorite: ${iden.favorite ?? "unknown"}`,
      "",
      iden.bio,
    ].join("\n")

  return (
    <Stack>
      <Field label="name">
        <Input value={iden.name} onChange={v => patch({ name: v })} placeholder="who are you" />
      </Field>
      <Field label="born">
        <DatePicker
          value={iden.born === "" ? undefined : parseDate(iden.born) ?? undefined}
          onChange={d => patch({ born: isoDate(d) })}
          placeholder="pick a day"
        />
      </Field>
      <Field label="favorite">
        <ColorPicker value={iden.favorite} onChange={c => patch({ favorite: c })} />
      </Field>
      <Field label="bio">
        <Textarea value={iden.bio} onChange={v => patch({ bio: v })} placeholder="a few words" rows={4} />
      </Field>
      <Row>
        <Button onClick={() => download("iden.json", JSON.stringify(iden, null, 2), "application/json")}>
          <Symbol name="download" /> json
        </Button>
        <Button onClick={() => download("iden.md", md(), "text/markdown")}>
          <Symbol name="download" /> md
        </Button>
      </Row>
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
        <Symbol name="warning" />
        <Symbol name="error" />
        <Symbol name="add" />
        <Symbol name="remove" />
        <Symbol name="menu" />
        <Symbol name="star" />
        <Symbol name="light_mode" />
        <Symbol name="dark_mode" />
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
      <Box>
        <Mark />
      </Box>
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
        <DatePicker value={date} onChange={setDate} />
      </Field>
    </Stack>
  )
}

function Sounds() {
  return (
    <Stack>
      <Grid cols={7}>
        {WHITE_KEYS.map((n, i) => (
          <Card key={n} onClick={() => sound.play(sound.freq(60 + (STEPS[i] ?? 0)))}>
            <Text>{n}</Text>
          </Card>
        ))}
      </Grid>
      <Row>
        <Button onClick={() => sound.tap()}>tap</Button>
        <Text className="caption">every tap rolls a note from the major scale</Text>
      </Row>
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
      <Cluster>
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
      </Cluster>
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
  const panes = usePanes()
  const [marked, setMarked] = useState(false)

  useEffect(() => {
    applyPrefs(loadPrefs())
    const arm = () => sound.unlock()
    window.addEventListener("pointerdown", arm, { once: true })
    return () => window.removeEventListener("pointerdown", arm)
  }, [])

  return (
    <Chrome>
      <Header
        menu={panes.left.open}
        iden={panes.right.open}
        onMenu={panes.left.toggle}
        onMark={() => setMarked(true)}
        onIden={panes.right.toggle}
        panes={panes}
      />
      <Toast open={marked} onClose={() => setMarked(false)}>
        mrly
      </Toast>
      <Panes panes={panes} left={<SettingsPane />} right={<IdenPane />} leftTitle="settings" rightTitle="iden">
        <Frame>
        <Stack airy>
          <header className="masthead">
            <Letters text="mrlyui" />
            <Text className="caption">one unit, many boxes</Text>
          </header>
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
          <Section label="sound">
            <Sounds />
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
      </Panes>
    </Chrome>
  )
}
