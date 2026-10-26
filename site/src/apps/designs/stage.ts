import type { Three } from "../../lib/app/hooks.ts";
import type { View } from "../../lib/app/run.ts";
import { rgb } from "../../lib/color.ts";
import type { mesh } from "../../lib/designs/mesh.ts";
import { eye, frustum, SWING, THETA, TILT } from "../../lib/designs/mesh.ts";

/* TYPES */

export type Built = ReturnType<typeof mesh>;

export type Seen = Pick<View, "w" | "h" | "t">;

export type Triple = [number, number, number];

const TAU = Math.PI * 2;
const ORBIT = 48000;
const FIT = 0.86;
const REACH = 4;
const LEAN = 1.3;
const GLOW = 0.5;
const SHINE = 0.6;
const SUN: Triple = [1, 0.6, 1.4];

const lean = (phi: number) => Math.min(LEAN, Math.max(-LEAN, phi));

function shape(three: Three, built: Built) {
  const geometry = new three.BufferGeometry();
  geometry.setAttribute("position", new three.BufferAttribute(built.position, 3));
  geometry.setAttribute("normal", new three.BufferAttribute(built.normal, 3));
  geometry.setIndex(new three.BufferAttribute(built.index, 1));
  return geometry;
}

export function space(
  three: Three,
  canvas: HTMLCanvasElement,
  view: Seen,
  built: Built,
  colour: () => string,
  spin = () => false,
) {
  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setClearColor(0x000000, 0);
  const world = new three.Scene();
  const camera = new three.OrthographicCamera(-1, 1, 1, -1, 0.1, REACH * 2);
  camera.up.set(0, 0, 1);
  let geometry = shape(three, built);
  const material = new three.MeshLambertMaterial();
  const body = new three.Mesh(geometry, material);
  const sun = new three.DirectionalLight(0xffffff, SHINE * Math.PI);
  sun.position.set(...SUN);
  world.add(body, new three.AmbientLight(0xffffff, GLOW * Math.PI), sun);
  let swing = 0;
  let tilt = 0;
  let dirty = true;
  let last = view.t;
  const draw = () => {
    if (spin() && view.t !== last) {
      swing -= ((view.t - last) / ORBIT) * TAU;
      dirty = true;
    }
    last = view.t;
    if (!dirty) return;
    dirty = false;
    camera.position.set(...eye(SWING + swing, lean(TILT + tilt), REACH));
    camera.lookAt(0, 0, 0);
    renderer.render(world, camera);
  };
  const fit = () => {
    renderer.setSize(view.w, view.h, false);
    Object.assign(camera, frustum(view.w / view.h, FIT));
    camera.updateProjectionMatrix();
  };
  const paint = () => {
    const [r, g, b] = rgb(colour());
    material.color.setRGB(r / 255, g / 255, b / 255, three.SRGBColorSpace);
  };
  const size = () => {
    fit();
    dirty = true;
    draw();
  };
  const theme = () => {
    paint();
    dirty = true;
    draw();
  };
  const swap = (next: Built) => {
    const old = geometry;
    geometry = shape(three, next);
    body.geometry = geometry;
    old.dispose();
    dirty = true;
  };
  const turn = (dx: number, dy: number) => {
    swing -= dx * TAU;
    tilt = lean(TILT + tilt + dy * Math.PI) - TILT;
    dirty = true;
    draw();
  };
  const pose = (): [number, number] => [THETA - swing, lean(TILT + tilt)];
  const stop = () => {
    geometry.dispose();
    material.dispose();
    renderer.dispose();
    renderer.forceContextLoss();
  };
  paint();
  fit();
  return { draw, size, theme, swap, turn, pose, stop };
}
