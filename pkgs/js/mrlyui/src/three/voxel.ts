import * as THREE from "three"

// GEOMETRY

type Corner = [number, number, number]

export function voxelGeometry(grid: number[][][]): THREE.BufferGeometry {
  const points: number[] = []
  const quad = (a: Corner, b: Corner, c: Corner, d: Corner): void => {
    points.push(...a, ...d, ...c)
    points.push(...a, ...c, ...b)
  }
  const depth = grid.length
  const first = grid[0]
  const height = first?.length ?? 0
  const width = first?.[0]?.length ?? 0
  for (let z = 0; z < depth; z++) {
    const layer = grid[z]
    if (layer === undefined) continue
    const back = grid[z - 1]
    const front = grid[z + 1]
    for (let y = 0; y < height; y++) {
      const row = layer[y]
      if (row === undefined) continue
      const under = layer[y - 1]
      const over = layer[y + 1]
      for (let x = 0; x < width; x++) {
        if ((row[x] ?? 0) !== 1) continue
        if (x === 0 || (row[x - 1] ?? 0) !== 1) {
          quad([x, y, z], [x, y + 1, z], [x, y + 1, z + 1], [x, y, z + 1])
        }
        if (x === width - 1 || (row[x + 1] ?? 0) !== 1) {
          quad([x + 1, y, z + 1], [x + 1, y + 1, z + 1], [x + 1, y + 1, z], [x + 1, y, z])
        }
        if (y === 0 || (under?.[x] ?? 0) !== 1) {
          quad([x, y, z], [x, y, z + 1], [x + 1, y, z + 1], [x + 1, y, z])
        }
        if (y === height - 1 || (over?.[x] ?? 0) !== 1) {
          quad([x, y + 1, z], [x + 1, y + 1, z], [x + 1, y + 1, z + 1], [x, y + 1, z + 1])
        }
        if (z === 0 || (back?.[y]?.[x] ?? 0) !== 1) {
          quad([x, y, z], [x + 1, y, z], [x + 1, y + 1, z], [x, y + 1, z])
        }
        if (z === depth - 1 || (front?.[y]?.[x] ?? 0) !== 1) {
          quad([x, y, z + 1], [x, y + 1, z + 1], [x + 1, y + 1, z + 1], [x + 1, y, z + 1])
        }
      }
    }
  }
  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(points, 3))
  geometry.computeVertexNormals()
  return geometry
}

// OBJ

export function toOBJ(grid: number[][][]): string {
  const corners: Corner[] = []
  const quads: Array<[number, number, number, number]> = []
  const index = new Map<string, number>()
  const at = (x: number, y: number, z: number): number => {
    const key = `${x},${y},${z}`
    const seen = index.get(key)
    if (seen !== undefined) return seen
    const next = corners.length + 1
    index.set(key, next)
    corners.push([x, y, z])
    return next
  }
  const depth = grid.length
  const first = grid[0]
  const height = first?.length ?? 0
  const width = first?.[0]?.length ?? 0
  for (let z = 0; z < depth; z++) {
    const layer = grid[z]
    if (layer === undefined) continue
    const back = grid[z - 1]
    const front = grid[z + 1]
    for (let y = 0; y < height; y++) {
      const row = layer[y]
      if (row === undefined) continue
      const under = layer[y - 1]
      const over = layer[y + 1]
      for (let x = 0; x < width; x++) {
        if ((row[x] ?? 0) !== 1) continue
        if (x === 0 || (row[x - 1] ?? 0) !== 1) {
          quads.push([at(x, y, z), at(x, y, z + 1), at(x, y + 1, z + 1), at(x, y + 1, z)])
        }
        if (x === width - 1 || (row[x + 1] ?? 0) !== 1) {
          quads.push([at(x + 1, y, z + 1), at(x + 1, y, z), at(x + 1, y + 1, z), at(x + 1, y + 1, z + 1)])
        }
        if (y === 0 || (under?.[x] ?? 0) !== 1) {
          quads.push([at(x, y, z), at(x + 1, y, z), at(x + 1, y, z + 1), at(x, y, z + 1)])
        }
        if (y === height - 1 || (over?.[x] ?? 0) !== 1) {
          quads.push([at(x, y + 1, z), at(x, y + 1, z + 1), at(x + 1, y + 1, z + 1), at(x + 1, y + 1, z)])
        }
        if (z === 0 || (back?.[y]?.[x] ?? 0) !== 1) {
          quads.push([at(x, y, z), at(x, y + 1, z), at(x + 1, y + 1, z), at(x + 1, y, z)])
        }
        if (z === depth - 1 || (front?.[y]?.[x] ?? 0) !== 1) {
          quads.push([at(x, y, z + 1), at(x + 1, y, z + 1), at(x + 1, y + 1, z + 1), at(x, y + 1, z + 1)])
        }
      }
    }
  }
  const lines: string[] = []
  for (const corner of corners) lines.push(`v ${corner[0]} ${corner[1]} ${corner[2]}`)
  for (const quad of quads) lines.push(`f ${quad[0]} ${quad[1]} ${quad[2]} ${quad[3]}`)
  return lines.join("\n")
}

// SAVE

export function saveOBJ(grid: number[][][]): void {
  const blob = new Blob([toOBJ(grid)], { type: "model/obj" })
  const url = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = url
  link.download = `mrly-${Date.now()}.obj`
  link.click()
  URL.revokeObjectURL(url)
}
