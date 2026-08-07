import { useCallback, useEffect, useState } from "react"
import "./sink.css"
import {
  Alert,
  applyPrefs,
  Autocomplete,
  Badge,
  Banner,
  Board,
  Box,
  Brand,
  Button,
  Canvas,
  Card,
  Cell,
  Checkbox,
  Chip,
  Choice,
  Chrome,
  Cluster,
  ColorPicker,
  Crumbs,
  DatePicker,
  Drawer,
  Dropdown,
  Emoji,
  Field,
  Fold,
  Footer,
  Frame,
  GraphemeInput,
  Grid,
  HEX,
  Header,
  Icon,
  Image,
  Input,
  isoDate,
  Label,
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
  Skeleton,
  Slider,
  sound,
  Spinner,
  Splash,
  Stack,
  Start,
  StepSlider,
  Stepper,
  Symbol,
  Tabs,
  Text,
  Textarea,
  THEME_ICONS,
  Title,
  Toast,
  toast,
  Toaster,
  Toggle,
  Tooltip,
  Tree,
  useAttract,
  useFont,
  usePanes,
  usePrefs,
  useScrolled,
  useTheme,
  write,
} from "mrlyui"
import type { ColorName, Fill, Fills, Mode, Steer, Swatch, TreeNode, Variant } from "mrlyui"

const VARIANTS: Variant[] = ["info", "success", "warn", "danger"]

const FRUIT = ["apple", "banana", "cherry", "grape", "lemon", "mango", "melon", "peach", "pear", "plum"]

const STEPS = [0, 2, 4, 5, 7, 9, 11]

const WHITE_KEYS = ["C", "D", "E", "F", "G", "A", "B"]

const MARKS = ["github", "discord", "x", "instagram", "reddit", "tiktok", "youtube"] as const

const NODES: TreeNode[] = [
  {
    name: "src",
    href: "#src",
    nodes: [
      { name: "Tree.tsx", href: "#src/Tree.tsx" },
      { name: "seti.ts", href: "#src/seti.ts" },
      { name: "index.ts", href: "#src/index.ts" },
      {
        name: "gen",
        href: "#src/gen",
        nodes: [{ name: "mark.json", href: "#src/gen/mark.json" }],
      },
    ],
  },
  {
    name: "styles",
    href: "#styles",
    nodes: [
      { name: "mrly.css", href: "#styles/mrly.css" },
      { name: "seti.css", href: "#styles/seti.css" },
    ],
  },
  { name: "boot.js", href: "#boot.js" },
  { name: "README.md", href: "#README.md" },
  { name: "LICENSE", href: "#LICENSE" },
]

const CONTENTS = ["plates", "boxes", "words", "glyphs"]

const SIDE = 16

const SHOT = `data:image/svg+xml;utf8,${encodeURIComponent(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 4 4" shape-rendering="crispEdges"><rect width="4" height="4" fill="#1ec9f3"/><rect x="1" y="1" width="2" height="2" fill="#ffd100"/><rect y="3" width="1" height="1" fill="#ff325a"/><rect x="3" width="1" height="1" fill="#32cc58"/></svg>',
)}`

const NOTES = `data:text/plain;charset=utf-8,${encodeURIComponent("one unit, many boxes")}`

const MODES: { label: string; value: Mode }[] = [
  { label: "row", value: "row" },
  { label: "stack", value: "stack" },
  { label: "icon", value: "icon" },
  { label: "text", value: "text" },
]

const PACES: { label: string; value: "slow" | "steady" | "fast" }[] = [
  { label: "slow", value: "slow" },
  { label: "steady", value: "steady" },
  { label: "fast", value: "fast" },
]

const PENS: Swatch<string>[] = [
  { name: "ink", color: "#0d1117" },
  { name: "sky", color: "#1ec9f3" },
  { name: "moss", color: "#32cc58" },
  { name: "sun", color: "#ffd100" },
  { name: "rose", color: "#ff325a" },
  { name: "plum", color: "#d332e9" },
]

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
      <Box>
        <Frame>
          <Box>frame holds prose to the reading measure</Box>
        </Frame>
        <Frame wide>
          <Box>frame wide holds code and tables to the wider measure</Box>
        </Frame>
      </Box>
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
      <Text className="caption">cells in flow</Text>
      <Grid cols={cols}>
        {Array.from({ length: 6 }, (_, i) => (
          <Cell key={i} on={on === i} bg={on === i ? undefined : HEX[POOL[i] ?? "gray"]} onClick={() => setOn(i)}>
            <Text>{i + 1}</Text>
          </Cell>
        ))}
      </Grid>
      <Text className="caption">grid snap</Text>
      <Grid cols={3} snap>
        {Array.from({ length: 9 }, (_, i) => (
          <Card key={i}>
            <Text>{i + 1}</Text>
          </Card>
        ))}
      </Grid>
    </Stack>
  )
}

function Eyes() {
  const [ink, setInk] = useState<ColorName>("blue")
  const [cells, setCells] = useState<string[]>(() => Array<string>(SIDE * SIDE).fill(""))
  const [crisp, setCrisp] = useState(false)
  const [mode, setMode] = useState<Mode>("row")
  const [turn, setTurn] = useState(0)
  const [dist, setDist] = useState(0)
  const [steer, setSteer] = useState<Steer | null>(null)

  const put = (points: [number, number][]) => {
    setCells(held => {
      const next = [...held]
      for (const [x, y] of points) next[y * SIDE + x] = HEX[ink]
      return next
    })
  }

  const paint = useCallback(
    (surface: HTMLCanvasElement) => {
      if (surface.width !== SIDE) surface.width = SIDE
      if (surface.height !== SIDE) surface.height = SIDE
      const ctx = surface.getContext("2d")
      if (ctx === null) return
      ctx.clearRect(0, 0, SIDE, SIDE)
      cells.forEach((color, i) => {
        if (color === "") return
        ctx.fillStyle = color
        ctx.fillRect(i % SIDE, Math.floor(i / SIDE), 1, 1)
      })
    },
    [cells],
  )

  const yaw = turn + (steer?.yaw ?? 0) * 40
  const reach = dist + (steer?.dist ?? 0) * 4

  return (
    <Stack>
      <Text className="caption">tap or drag to paint, {SIDE}x{SIDE}</Text>
      <Canvas grid={[SIDE, SIDE]} crisp={crisp} paint={paint} onTap={(x, y) => put([[x, y]])} onDrag={put} />
      <Setting label="crisp" hint="pixelated when off">
        <Toggle value={crisp} onChange={setCrisp} />
      </Setting>
      <ColorPicker value={ink} onChange={color => setInk(color ?? "blue")} />
      <Button wide onClick={() => setCells(Array<string>(SIDE * SIDE).fill(""))}>
        clear
      </Button>
      <Text className="caption">drag to turn, wheel or pinch to zoom</Text>
      <Canvas
        sig={`dial:${Math.round(yaw)}:${Math.round(reach)}`}
        paint={surface => {
          const rect = surface.getBoundingClientRect()
          const side = Math.max(1, Math.round(rect.width))
          if (surface.width !== side) surface.width = side
          if (surface.height !== side) surface.height = side
          const ctx = surface.getContext("2d")
          if (ctx === null) return
          ctx.clearRect(0, 0, side, side)
          ctx.strokeStyle = getComputedStyle(surface).color
          ctx.lineWidth = 2
          const r = Math.max(8, side / 2 - 12 - reach * 3)
          ctx.beginPath()
          ctx.arc(side / 2, side / 2, r, 0, Math.PI * 2)
          ctx.stroke()
          ctx.beginPath()
          ctx.moveTo(side / 2, side / 2)
          ctx.lineTo(side / 2 + Math.cos(yaw / 20) * r, side / 2 + Math.sin(yaw / 20) * r)
          ctx.stroke()
        }}
        onTurn={dyaw => setTurn(held => held + dyaw)}
        onZoom={(dir, n) => setDist(held => held + (dir === "in" ? -n : n))}
        onSteer={setSteer}
      />
      <Text className="caption">
        yaw {Math.round(turn)} dist {Math.round(dist)}
      </Text>
      <Image src={SHOT} alt="four squares" />
      <Choice options={MODES} value={mode} onChange={setMode} mode="row" label="label" />
      <Label mode={mode} symbol={{ as: "emoji", value: "🍎" }} text="apple" note="a fruit" />
      <Label mode={mode} symbol={{ as: "icon", value: "settings" }} text="settings" note="the knobs" />
      <Label mode={mode} symbol={{ as: "glyph", value: "mrly" }} text="mrly" note="the mark" />
      <Label mode={mode} symbol={{ as: "icon", value: "download" }} text="notes.txt" note="saves the file" href={NOTES} />
      <Label mode={mode} symbol={{ as: "icon", value: "star" }} text="press me" note="fires onClick" onClick={() => toast("label pressed")} />
    </Stack>
  )
}

function Attract() {
  const [splashed, setSplashed] = useState(false)
  const [armed, setArmed] = useState(false)
  const [beats, setBeats] = useState(0)
  const { idle, wake } = useAttract(armed, () => setBeats(n => n + 1), 500)
  const rouse = () => {
    setArmed(false)
    wake()
  }
  return (
    <Stack>
      <Setting label="attract" hint="ticks while idle">
        <Toggle value={armed} onChange={setArmed} />
      </Setting>
      <Text>{idle ? `idle, ${beats} beats` : "awake"}</Text>
      <Button wide onClick={() => setSplashed(true)}>
        splash
      </Button>
      {idle && <Start onClick={rouse} />}
      {splashed && (
        <Splash onDismiss={() => setSplashed(false)}>
          <Letters text="mrly" />
          <Text className="caption">tap to enter</Text>
        </Splash>
      )}
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
      <Text className="lead">The lead opens a page.</Text>
      <Text className="out">The standfirst carries it on.</Text>
      <Text className="fine">The fine print closes it out.</Text>
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
      <Cluster>
        {MARKS.map(name => (
          <Brand key={name} name={name} />
        ))}
      </Cluster>
      <Box>
        <Mark />
      </Box>
    </Stack>
  )
}

function Buttons() {
  const [armed, setArmed] = useState(false)
  const [held, setHeld] = useState(0)
  const [down, setDown] = useState(false)
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
      <Button
        wide
        active={down}
        onPress={() => {
          setDown(true)
          setHeld(n => n + 1)
        }}
        onLift={() => setDown(false)}
      >
        {down ? "holding" : `press and lift, ${held} presses`}
      </Button>
      <Grid cols={6}>
        {POOL.slice(0, 6).map(name => (
          <Button key={name} big bg={HEX[name]} onClick={() => toast(name)}>
            {" "}
          </Button>
        ))}
      </Grid>
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
      <Box>
        <Stack>
          <Choice options={PACES} value={pace} onChange={setPace} mode="row" label="row" />
          <Choice options={PACES} value={pace} onChange={setPace} mode="cycle" label="cycle" />
          <Choice options={PACES} value={pace} onChange={setPace} mode="select" label="select" />
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
  const [pens, setPens] = useState<string[]>(["sky", "sun"])
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
      <Field label="palette" hint={pens.length === 0 ? "none picked" : pens.join(", ")}>
        <ColorPicker
          big
          swatches={PENS}
          value={pens}
          onChange={name => {
            if (name === null) return
            setPens(held => (held.includes(name) ? held.filter(pen => pen !== name) : [...held, name]))
          }}
        />
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
  const [toasted, setToasted] = useState(false)
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
        <Button onClick={() => setToasted(true)}>toast</Button>
        <Button onClick={() => toast("popped from anywhere", "info")}>pop</Button>
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
      <Skeleton head lines={3} block />
      <Toast open={toasted} onClose={() => setToasted(false)} variant="success">
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

function Explorer() {
  return (
    <Stack>
      <Box>
        <Tree nodes={NODES} current="#src/Tree.tsx" root={{ name: "mrlyui", href: "#" }} />
      </Box>
      <Box>
        <Stack tight>
          <Fold icon="toc" label="contents" open>
            <ol className="toc">
              {CONTENTS.map(name => (
                <li key={name}>
                  <a href={`#${name}`}>{name}</a>
                </li>
              ))}
            </ol>
          </Fold>
          <Fold icon="explore" label="places" grid>
            <a href="#home">
              <Brand name="home" /> home
            </a>
            <a href="#mail">
              <Brand name="mail" /> mail
            </a>
          </Fold>
          <Fold icon="groups" label="socials" grid>
            {MARKS.map(name => (
              <a key={name} href={`#${name}`}>
                <Brand name={name} /> {name}
              </a>
            ))}
          </Fold>
        </Stack>
      </Box>
      <code className="clone">curl -fsSL https://cdn.mrly.net/mrlyprod.tar.gz | tar xz</code>
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
  useScrolled()

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
      <Toaster />
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
          <Section label="eyes">
            <Eyes />
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
          <Section label="explorer">
            <Explorer />
          </Section>
          <Section label="overlays">
            <Overlays />
          </Section>
          <Section label="attract">
            <Attract />
          </Section>
        </Stack>
        <Footer>
          <Letters text="mrlyui" scramble={false} />
        </Footer>
        </Frame>
      </Panes>
    </Chrome>
  )
}
