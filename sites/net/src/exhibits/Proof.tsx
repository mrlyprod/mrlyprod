import { useCallback, useMemo } from "react"
import { Button, Cluster, Plot, Stack } from "mrlyui"
import { Tex } from "mrlyui/math"
import { Stage, saveOBJ } from "mrlyui/three"
import type { PlotMark } from "mrlyui"

// SPONGE

function solid(x: number, y: number, z: number, n: number): boolean {
  for (let s = 1; s < n; s *= 3) {
    let mid = 0
    if (Math.floor(x / s) % 3 === 1) mid += 1
    if (Math.floor(y / s) % 3 === 1) mid += 1
    if (Math.floor(z / s) % 3 === 1) mid += 1
    if (mid >= 2) return false
  }
  return true
}

function sponge(level: number): number[][][] {
  const n = 3 ** level
  const grid: number[][][] = []
  for (let x = 0; x < n; x += 1) {
    const plane: number[][] = []
    for (let y = 0; y < n; y += 1) {
      const row: number[] = []
      for (let z = 0; z < n; z += 1) row.push(solid(x, y, z, n) ? 1 : 0)
      plane.push(row)
    }
    grid.push(plane)
  }
  return grid
}

// SQUARES

const SQUARES = Array.from({ length: 12 }, (blank, at) => (at + 1) * (at + 1))

const MARKS: PlotMark[] = [
  { kind: "curve", f: at => at * at },
  { kind: "seq", terms: SQUARES, from: 1 },
]

// PROOF

export default function Proof() {
  const grid = useMemo(() => sponge(3), [])
  const save = useCallback(() => {
    saveOBJ(grid)
  }, [grid])
  return (
    <Stack tight>
      <Plot
        marks={MARKS}
        x={{ label: <Tex src="n" />, min: 0, max: 12 }}
        y={{ label: <Tex src="n^2" /> }}
      />
      <Stage grid={grid} />
      <Cluster>
        <Button onClick={save}>Save OBJ</Button>
      </Cluster>
    </Stack>
  )
}
