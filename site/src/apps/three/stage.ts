import type { Cell, moire, three } from "@mrlyprod/mrly/math";
import type { Group, Object3D, OrthographicCamera, PerspectiveCamera } from "three";
import type { Three } from "../../lib/app/hooks.ts";
import type { View } from "../../lib/app/run.ts";
import type { Plane, Wall } from "../../lib/three/engine.ts";
import type { Angles, Pose } from "../../lib/three/thumbs.ts";

/* TYPES */

export type Triple = [number, number, number];

export type Seen = Pick<View, "w" | "h" | "dpr" | "t" | "still">;

export type Body = { key: string; cell: Cell; cubes: boolean; quads: three.Quad[] | null };

export type Coat = { color: readonly number[]; opacity?: number; clip?: Wall[] | null };

export type Packed = { position: Float32Array; normal: Float32Array; index: Uint32Array; faces: number };

export type Spots = { centres: Float32Array; unit: number };

export type Sheet = { position: Float32Array; uv: Float32Array };

export type Stage = ReturnType<typeof space>;

type Part = { dispose: () => void };

type Held = [Object3D, Part[]];

const TAU = Math.PI * 2;
export const ORBIT = 26000;
export const POSE: Angles = [0.84, 0.47];
export const LOOKS: Partial<Record<string, Angles>> & Record<Pose, Angles> = {
  corner: [Math.PI / 4, Math.atan(Math.SQRT1_2)],
  edge: [Math.PI / 4, 0],
  face: [0, 0],
};
const FOV = 38;
const EDGE = 16;
const LEAST = 0.5;
const BALL = Math.sqrt(3);
const TAN = Math.tan((FOV / 2) * (Math.PI / 180));
const LEAN = 1.5;
const GRIP = 0.9;
const REST = 1e-4;
const ZOOM = [0.3, 8];
const CUBE = 0.9;
const SKY = 1.2;
const KEY = 1.8;
const RIM = 0.6;
const LAMPS: Record<"key" | "rim", Triple> = { key: [2, 3, 2], rim: [-2, -1, -3] };
const ROUGH = 0.6;
const METAL = 0.05;
const ALPHA = 0.5;

const lean = (phi: number) => Math.min(LEAN, Math.max(-LEAN, phi));

const clamp = (v: number, [lo, hi]: number[]) => Math.min(hi, Math.max(lo, v));

export function room(w: number, h: number, dpr = 1): number {
  const tall = Math.max(LEAST, 1 - (2 * EDGE * dpr) / h);
  const wide = Math.max(LEAST, 1 - (2 * EDGE * dpr) / w);
  return Math.min(tall, (wide * w) / h);
}

export function stand(free: number): number {
  const k = TAN * free;
  return (BALL * Math.sqrt(1 + k * k)) / k;
}

export const span = (free: number) => BALL / free;

export function angle(swing: number, t: number, spin: number): number {
  return swing + (spin ? (t / ORBIT) * TAU : 0);
}

export function eye(theta: number, phi: number, reach: number): Triple {
  const flat = Math.cos(phi) * reach;
  return [Math.cos(theta) * flat, Math.sin(theta) * flat, Math.sin(phi) * reach];
}

export function pack(quads: three.Quad[]): Packed {
  const faces = quads.length;
  const position = new Float32Array(faces * 12);
  const normal = new Float32Array(faces * 12);
  const index = new Uint32Array(faces * 6);
  quads.forEach(({ normal: n, verts }, face) => {
    for (let k = 0; k < 4; k++) {
      const at = face * 12 + k * 3;
      position.set([verts[k].x, verts[k].y, verts[k].z], at);
      normal.set([n.x, n.y, n.z], at);
    }
    const corner = face * 4;
    index.set([corner, corner + 1, corner + 2, corner, corner + 2, corner + 3], face * 6);
  });
  return { position, normal, index, faces };
}

export function spots(cell: Cell): Spots {
  const [a, b, c] = cell.shape;
  const unit = 2 / Math.max(a, b, c);
  const out: number[] = [];
  for (let flat = 0; flat < cell.types.length; flat++) {
    if (!cell.types[flat]) continue;
    const z = flat % c;
    const y = Math.floor(flat / c) % b;
    const x = Math.floor(flat / (b * c));
    out.push((x + 0.5) * unit - 1, (y + 0.5) * unit - 1, (z + 0.5) * unit - 1);
  }
  return { centres: Float32Array.from(out), unit };
}

const corner = (frame: moire.Frame, a: number, b: number) =>
  frame.centre.map((c, i) => c + a * frame.u[i] + b * frame.v[i]);

export function sheet(frame: moire.Frame): Sheet {
  const h = frame.width / 2;
  const [p00, p10, p11, p01] = [corner(frame, -h, -h), corner(frame, -h, h), corner(frame, h, h), corner(frame, h, -h)];
  return {
    position: Float32Array.from([...p00, ...p10, ...p11, ...p00, ...p11, ...p01]),
    uv: Float32Array.from([0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0]),
  };
}

export function space(three: Three, canvas: HTMLCanvasElement, view: Seen) {
  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setClearColor(0x000000, 0);
  renderer.localClippingEnabled = true;
  const world = new three.Scene();
  const near = new three.PerspectiveCamera(FOV, 1, 0.01, 100);
  const flat = new three.OrthographicCamera(-1, 1, 1, -1, 0.01, 100);
  for (const one of [near, flat]) one.up.set(0, 0, 1);
  const rig = new three.Group();
  const key = new three.DirectionalLight(0xffffff, KEY);
  key.position.set(...LAMPS.key);
  const rim = new three.DirectionalLight(0x008cff, RIM);
  rim.position.set(...LAMPS.rim);
  rig.add(key, rim);
  const sky = new three.HemisphereLight(0xffffff, 0x444444, SKY);
  sky.position.set(0, 0, 1);
  const solid = new three.Group();
  const cuts = new three.Group();
  world.add(sky, rig, solid, cuts);
  const material = new three.MeshStandardMaterial({ roughness: ROUGH, metalness: METAL });
  let camera: PerspectiveCamera | OrthographicCamera = near;
  let body: Body | null = null;
  let [swing, tilt] = POSE;
  let zoom = 1;
  let pan = [0, 0];
  let fling = [0, 0];
  let spin = 0;
  let dirty = true;
  let away = 1;
  const fit = () => {
    renderer.setSize(view.w, view.h, false);
    const aspect = view.w / view.h;
    const free = room(view.w, view.h, view.dpr);
    const high = span(free);
    away = stand(free);
    near.aspect = aspect;
    Object.assign(flat, { left: -high * aspect, right: high * aspect, top: high, bottom: -high });
    frame();
  };
  const frame = () => {
    flat.zoom = zoom;
    for (const one of [near, flat]) {
      one.setViewOffset(view.w, view.h, -pan[0] * view.w, -pan[1] * view.h, view.w, view.h);
      one.updateProjectionMatrix();
    }
  };
  const pose = () => {
    camera.position.set(...eye(angle(swing, view.t, spin), tilt, away / (camera === near ? zoom : 1)));
    camera.lookAt(0, 0, 0);
    rig.quaternion.copy(camera.quaternion);
  };
  const draw = () => {
    const moving = Math.abs(fling[0]) + Math.abs(fling[1]) > REST;
    if (moving) {
      swing += fling[0];
      tilt = lean(tilt + fling[1]);
      fling = fling.map((v) => v * GRIP);
    }
    if (!dirty && !moving && !(spin && !view.still)) return;
    dirty = false;
    pose();
    renderer.render(world, camera);
  };
  const mark = () => {
    dirty = true;
  };
  const touch = () => {
    mark();
    draw();
  };
  const held = new Map<Group, Held[]>([
    [solid, []],
    [cuts, []],
  ]);
  const hold = (group: Group, one: Object3D, ...parts: Part[]) => {
    group.add(one);
    held.get(group)!.push([one, parts]);
  };
  const drop = (group: Group) => {
    for (const [one, parts] of held.get(group)!) {
      group.remove(one);
      for (const part of parts) part.dispose();
    }
    held.set(group, []);
  };
  const show = (next: Body | null) => {
    if (next === body) return;
    body = next;
    drop(solid);
    if (next?.quads?.length) {
      const built = pack(next.quads);
      const geometry = new three.BufferGeometry();
      geometry.setAttribute("position", new three.BufferAttribute(built.position, 3));
      geometry.setAttribute("normal", new three.BufferAttribute(built.normal, 3));
      geometry.setIndex(new three.BufferAttribute(built.index, 1));
      hold(solid, new three.Mesh(geometry, material), geometry);
    } else if (next?.cell && next.cubes) {
      const { centres, unit } = spots(next.cell);
      const count = centres.length / 3;
      if (count) {
        const box = new three.BoxGeometry(unit * CUBE, unit * CUBE, unit * CUBE);
        const many = new three.InstancedMesh(box, material, count);
        const place = new three.Matrix4();
        for (let i = 0; i < count; i++)
          many.setMatrixAt(i, place.makeTranslation(centres[i * 3], centres[i * 3 + 1], centres[i * 3 + 2]));
        many.instanceMatrix.needsUpdate = true;
        hold(solid, many, box);
      }
    }
    dirty = true;
  };
  const paint = ({ color, opacity = 1, clip = null }: Coat) => {
    const [r, g, b] = color;
    material.color.setRGB(r / 255, g / 255, b / 255, three.SRGBColorSpace);
    const clear = opacity < 1;
    material.transparent = clear;
    material.opacity = opacity;
    material.depthWrite = !clear;
    material.clippingPlanes = clip
      ? clip.map(({ normal, constant }) => new three.Plane(new three.Vector3(...(normal as Triple)), constant))
      : null;
    material.side = clip ? three.DoubleSide : three.FrontSide;
    material.needsUpdate = true;
    dirty = true;
  };
  const planes = (list: Plane[]) => {
    drop(cuts);
    for (const { frame: at, pixels } of list) {
      const [h, w] = pixels.shape;
      const texture = new three.DataTexture(pixels.colors, w, h, three.RGBAFormat);
      texture.magFilter = three.NearestFilter;
      texture.colorSpace = three.SRGBColorSpace;
      texture.needsUpdate = true;
      const quad = sheet(at);
      const shape = new three.BufferGeometry();
      shape.setAttribute("position", new three.BufferAttribute(quad.position, 3));
      shape.setAttribute("uv", new three.BufferAttribute(quad.uv, 2));
      const skin = new three.MeshBasicMaterial({
        map: texture,
        transparent: true,
        alphaTest: ALPHA,
        side: three.DoubleSide,
      });
      hold(cuts, new three.Mesh(shape, skin), shape, skin, texture);
    }
    dirty = true;
  };
  const project = (kind: string) => {
    const next = kind === "iso" ? flat : near;
    if (next === camera) return;
    camera = next;
    dirty = true;
  };
  const spinning = (on: boolean) => {
    const was = spin;
    spin = on ? 1 : 0;
    swing = angle(swing, view.t, was) - (spin ? (view.t / ORBIT) * TAU : 0);
    dirty = true;
  };
  const look = (name: string) => {
    const [theta, phi] = LOOKS[name] ?? POSE;
    swing = theta - (spin ? (view.t / ORBIT) * TAU : 0);
    tilt = phi;
    fling = [0, 0];
    zoom = 1;
    pan = [0, 0];
    frame();
    touch();
  };
  const turn = (dx: number, dy: number) => {
    fling = [0, 0];
    swing -= dx * TAU;
    tilt = lean(tilt + dy * Math.PI);
    touch();
  };
  const hurl = (dx: number, dy: number) => {
    fling = [-dx * TAU, dy * Math.PI];
    dirty = true;
  };
  const scale = (by: number) => {
    zoom = clamp(zoom * by, ZOOM);
    frame();
    touch();
  };
  const shift = (dx: number, dy: number) => {
    pan = [pan[0] + dx, pan[1] + dy];
    frame();
    touch();
  };
  const stop = () => {
    drop(solid);
    drop(cuts);
    material.dispose();
    renderer.dispose();
    renderer.forceContextLoss();
  };
  fit();
  return {
    draw,
    size: () => {
      fit();
      touch();
    },
    mark,
    show,
    paint,
    planes,
    project,
    spinning,
    look,
    turn,
    fling: hurl,
    zoom: scale,
    pan: shift,
    stop,
  };
}
