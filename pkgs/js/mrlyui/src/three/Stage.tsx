import { useCallback, useEffect, useRef, useState } from "react"
import * as THREE from "three"
import { OrbitControls } from "three/addons/controls/OrbitControls.js"
import { cx } from "../lib"
import { voxelGeometry } from "./voxel"
import type { ColorName } from "../colors"

// FRAME

const FOV = 45

const NEAR = 0.1

const FAR = 1000

const FRUSTUM = 20

const PAD = 2.5

const PERSP_STEP = 1.5

const ORTHO_STEP = 2

const ALPHA = 0.25

const STILL = 45

type Lens = THREE.PerspectiveCamera | THREE.OrthographicCamera

// PAINT

type Shades = { paper: string; ink: string; fill: string }

function solid(value: string): string {
  const held = /^color\(srgb ([\d.]+) ([\d.]+) ([\d.]+)/.exec(value)
  if (held === null) return value
  const r = Math.round(Number(held[1] ?? "0") * 255)
  const g = Math.round(Number(held[2] ?? "0") * 255)
  const b = Math.round(Number(held[3] ?? "0") * 255)
  return `rgb(${r}, ${g}, ${b})`
}

function shadesOf(node: HTMLElement, fill: ColorName | undefined): Shades {
  const style = getComputedStyle(node)
  const paper = solid(style.backgroundColor)
  const ink = solid(style.color)
  const accent = solid(style.borderTopColor)
  if (fill === undefined) return { paper, ink, fill: accent }
  const named = style.getPropertyValue(`--c-${fill}`).trim()
  return { paper, ink, fill: named === "" ? accent : solid(named) }
}

// STAGE

export function Stage({ grid, fill, camera = "ortho", edges = true, translucent = false, className }: {
  grid: number[][][]
  fill?: ColorName
  camera?: "ortho" | "persp"
  edges?: boolean
  translucent?: boolean
  className?: string
}) {
  const boxRef = useRef<HTMLDivElement | null>(null)
  const canvasRef = useRef<HTMLCanvasElement | null>(null)
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null)
  const sceneRef = useRef<THREE.Scene | null>(null)
  const perspRef = useRef<THREE.PerspectiveCamera | null>(null)
  const orthoRef = useRef<THREE.OrthographicCamera | null>(null)
  const lensRef = useRef<Lens | null>(null)
  const controlsRef = useRef<OrbitControls<Lens> | null>(null)
  const meshRef = useRef<THREE.Mesh<THREE.BufferGeometry, THREE.MeshBasicMaterial> | null>(null)
  const lineRef = useRef<THREE.LineSegments<THREE.BufferGeometry, THREE.LineBasicMaterial> | null>(null)
  const spanRef = useRef(FRUSTUM)
  const frameRef = useRef<number | null>(null)
  const stillRef = useRef(0)
  const paintRef = useRef("")
  const [tick, setTick] = useState(0)

  const draw = useCallback((): void => {
    const renderer = rendererRef.current
    const scene = sceneRef.current
    const lens = lensRef.current
    if (renderer === null || scene === null || lens === null) return
    renderer.render(scene, lens)
  }, [])

  const fit = useCallback((): void => {
    const box = boxRef.current
    const renderer = rendererRef.current
    if (box === null || renderer === null) return
    const width = box.clientWidth
    const height = box.clientHeight
    if (width === 0 || height === 0) return
    renderer.setSize(width, height, false)
    const aspect = width / height
    const persp = perspRef.current
    if (persp !== null) {
      persp.aspect = aspect
      persp.updateProjectionMatrix()
    }
    const ortho = orthoRef.current
    if (ortho !== null) {
      const tall = spanRef.current
      const wide = tall * aspect
      ortho.left = wide / -2
      ortho.right = wide / 2
      ortho.top = tall / 2
      ortho.bottom = tall / -2
      ortho.updateProjectionMatrix()
    }
    draw()
  }, [draw])

  const kick = useCallback((): void => {
    stillRef.current = 0
    if (frameRef.current !== null) return
    const step = (): void => {
      const controls = controlsRef.current
      if (controls === null) {
        frameRef.current = null
        return
      }
      const moving = controls.update()
      stillRef.current = moving ? 0 : stillRef.current + 1
      if (stillRef.current >= STILL) {
        const held = frameRef.current
        if (held !== null) cancelAnimationFrame(held)
        frameRef.current = null
        return
      }
      frameRef.current = requestAnimationFrame(step)
    }
    frameRef.current = requestAnimationFrame(step)
  }, [])

  useEffect(() => {
    const box = boxRef.current
    const canvas = canvasRef.current
    if (box === null || canvas === null) return
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, preserveDrawingBuffer: true })
    renderer.outputColorSpace = THREE.SRGBColorSpace
    renderer.setPixelRatio(window.devicePixelRatio)
    rendererRef.current = renderer
    const scene = new THREE.Scene()
    sceneRef.current = scene
    const width = box.clientWidth
    const height = box.clientHeight
    const aspect = width === 0 || height === 0 ? 1 : width / height
    perspRef.current = new THREE.PerspectiveCamera(FOV, aspect, NEAR, FAR)
    orthoRef.current = new THREE.OrthographicCamera(
      (FRUSTUM * aspect) / -2,
      (FRUSTUM * aspect) / 2,
      FRUSTUM / 2,
      FRUSTUM / -2,
      NEAR,
      FAR,
    )
    const sizer = new ResizeObserver(() => {
      fit()
    })
    sizer.observe(box)
    const bump = (): void => {
      setTick(held => held + 1)
    }
    const watcher = new MutationObserver(bump)
    watcher.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme", "style"] })
    const media = window.matchMedia("(prefers-color-scheme: dark)")
    media.addEventListener("change", bump)
    return () => {
      const held = frameRef.current
      if (held !== null) cancelAnimationFrame(held)
      frameRef.current = null
      sizer.disconnect()
      watcher.disconnect()
      media.removeEventListener("change", bump)
      const mesh = meshRef.current
      if (mesh !== null) {
        scene.remove(mesh)
        mesh.geometry.dispose()
        mesh.material.dispose()
      }
      meshRef.current = null
      const line = lineRef.current
      if (line !== null) {
        scene.remove(line)
        line.geometry.dispose()
        line.material.dispose()
      }
      lineRef.current = null
      renderer.dispose()
      rendererRef.current = null
      sceneRef.current = null
      perspRef.current = null
      orthoRef.current = null
      lensRef.current = null
    }
  }, [fit])

  useEffect(() => {
    const canvas = canvasRef.current
    const lens: Lens | null = camera === "persp" ? perspRef.current : orthoRef.current
    if (canvas === null || lens === null) return
    lensRef.current = lens
    const controls = new OrbitControls<Lens>(lens, canvas)
    controls.enableDamping = true
    controls.addEventListener("start", kick)
    controls.addEventListener("change", () => {
      draw()
      kick()
    })
    controlsRef.current = controls
    return () => {
      controls.dispose()
      if (controlsRef.current === controls) controlsRef.current = null
    }
  }, [camera, draw, kick])

  useEffect(() => {
    const box = boxRef.current
    const scene = sceneRef.current
    if (box === null || scene === null) return
    const shades = shadesOf(box, fill)
    const geometry = voxelGeometry(grid)
    geometry.computeBoundingBox()
    const bounds = geometry.boundingBox
    if (bounds !== null) {
      const middle = bounds.getCenter(new THREE.Vector3())
      geometry.translate(-middle.x, -middle.y, -middle.z)
    }
    const wires = new THREE.EdgesGeometry(geometry)
    const mesh = meshRef.current
    const line = lineRef.current
    if (mesh === null || line === null) {
      const material = new THREE.MeshBasicMaterial({
        color: new THREE.Color().setStyle(shades.fill),
        transparent: translucent,
        opacity: translucent ? ALPHA : 1,
        depthWrite: !translucent,
      })
      const next = new THREE.Mesh(geometry, material)
      scene.add(next)
      meshRef.current = next
      const strokes = new THREE.LineBasicMaterial({ color: new THREE.Color().setStyle(shades.ink) })
      const rim: THREE.LineSegments<THREE.BufferGeometry, THREE.LineBasicMaterial> = new THREE.LineSegments(wires, strokes)
      rim.visible = edges
      scene.add(rim)
      lineRef.current = rim
    } else {
      mesh.geometry.dispose()
      mesh.geometry = geometry
      line.geometry.dispose()
      line.geometry = wires
    }
    draw()
  }, [grid, draw])

  useEffect(() => {
    const box = boxRef.current
    const scene = sceneRef.current
    if (box === null || scene === null) return
    const shades = shadesOf(box, fill)
    const paint = `${shades.paper}|${shades.ink}|${shades.fill}|${String(translucent)}|${String(edges)}`
    if (paint === paintRef.current) return
    paintRef.current = paint
    scene.background = new THREE.Color().setStyle(shades.paper)
    const mesh = meshRef.current
    if (mesh !== null) {
      mesh.material.color.setStyle(shades.fill)
      mesh.material.transparent = translucent
      mesh.material.opacity = translucent ? ALPHA : 1
      mesh.material.depthWrite = !translucent
      mesh.material.needsUpdate = true
    }
    const line = lineRef.current
    if (line !== null) {
      line.visible = edges
      line.material.color.setStyle(shades.ink)
    }
    draw()
  }, [fill, translucent, edges, tick, draw])

  useEffect(() => {
    const mesh = meshRef.current
    const controls = controlsRef.current
    if (mesh === null || controls === null) return
    const bounds = mesh.geometry.boundingBox
    if (bounds === null) return
    const span = bounds.getSize(new THREE.Vector3())
    const size = Math.max(span.x, span.y, span.z)
    if (size === 0) return
    if (camera === "persp") {
      const persp = perspRef.current
      if (persp === null) return
      const away = size * PERSP_STEP
      persp.position.set(away, away, away)
      persp.lookAt(0, 0, 0)
    } else {
      const ortho = orthoRef.current
      if (ortho === null) return
      spanRef.current = size * PAD
      const away = size * ORTHO_STEP
      ortho.position.set(away, away, away)
      ortho.lookAt(0, 0, 0)
    }
    fit()
    controls.target.set(0, 0, 0)
    controls.update()
    draw()
  }, [grid, camera, fit, draw])

  return (
    <div ref={boxRef} className={cx("stage", className)}>
      <canvas ref={canvasRef} />
    </div>
  )
}
