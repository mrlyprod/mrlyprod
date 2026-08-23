import { useMemo, useState } from "react"
import { Button, Chip, Choice, Cluster, Row, Setting, Stack, Symbol, Text, Toggle } from "mrlyui"
import { saveOBJ, Stage } from "mrlyui/three"
import type { ColorName } from "mrlyui"

const SIDE = 3 ** 3

const FILLS: (ColorName | undefined)[] = [undefined, "cyan", "indigo", "pink", "yellow"]

const CAMERAS: { label: string; value: "ortho" | "persp" }[] = [
  { label: "ortho", value: "ortho" },
  { label: "persp", value: "persp" },
]

function solid(x: number, y: number, z: number): boolean {
  for (let s = 1; s < SIDE; s *= 3) {
    let middles = 0
    if (Math.floor(x / s) % 3 === 1) middles += 1
    if (Math.floor(y / s) % 3 === 1) middles += 1
    if (Math.floor(z / s) % 3 === 1) middles += 1
    if (middles >= 2) return false
  }
  return true
}

function sponge(): number[][][] {
  return Array.from({ length: SIDE }, (_, z) =>
    Array.from({ length: SIDE }, (_, y) =>
      Array.from({ length: SIDE }, (_, x) => (solid(x, y, z) ? 1 : 0)),
    ),
  )
}

export default function Stages() {
  const [camera, setCamera] = useState<"ortho" | "persp">("ortho")
  const [edges, setEdges] = useState(true)
  const [translucent, setTranslucent] = useState(false)
  const [fill, setFill] = useState(0)
  const grid = useMemo(() => sponge(), [])
  return (
    <Stack>
      <Stage grid={grid} fill={FILLS[fill]} camera={camera} edges={edges} translucent={translucent} />
      <Row>
        <Choice options={CAMERAS} value={camera} onChange={setCamera} mode="row" label="camera" />
        <Setting label="edges">
          <Toggle value={edges} onChange={setEdges} />
        </Setting>
        <Setting label="translucent">
          <Toggle value={translucent} onChange={setTranslucent} />
        </Setting>
      </Row>
      <Row>
        <Cluster>
          {FILLS.map((name, i) => (
            <Chip key={name ?? "accent"} active={fill === i} onClick={() => setFill(i)}>
              {name ?? "accent"}
            </Chip>
          ))}
        </Cluster>
        <Button onClick={() => saveOBJ(grid)}>
          <Symbol name="download" /> obj
        </Button>
      </Row>
      <Text className="caption">a level 3 menger sponge, {SIDE} cells a side</Text>
    </Stack>
  )
}
