import { rgb } from '../../lib/scene.js';
import { TILT, eye, frustum } from '../designs/mesh.js';

const TAU = Math.PI * 2;
export const ORBIT = 40000;
const FIT = 0.86;
const REACH = 4;
const LEAN = 1.3;
const GLOW = 0.4;
const KEY = 0.65;
const FILL = 0.25;
const SUN = [1, 0.6, 1.4];
const LAMP = [-1.2, -0.5, 0.3];

const lean = (phi) => Math.min(LEAN, Math.max(-LEAN, phi));

export function angle(start, swing, t, spin) {
  return start + swing + (spin ? (t / ORBIT) * TAU : 0);
}

export function space(three, canvas, view, built, { start = 0, spin = 1 } = {}) {
  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true });
  renderer.setPixelRatio(1);
  renderer.setClearColor(0x000000, 0);
  const world = new three.Scene();
  const camera = new three.OrthographicCamera(-1, 1, 1, -1, 0.1, REACH * 2);
  camera.up.set(0, 0, 1);
  const geometry = new three.BufferGeometry();
  geometry.setAttribute('position', new three.BufferAttribute(built.position, 3));
  geometry.setAttribute('normal', new three.BufferAttribute(built.normal, 3));
  geometry.setIndex(new three.BufferAttribute(built.index, 1));
  const material = new three.MeshLambertMaterial();
  const sun = new three.DirectionalLight(0xffffff, KEY * Math.PI);
  sun.position.set(...SUN);
  const lamp = new three.DirectionalLight(0xffffff, FILL * Math.PI);
  lamp.position.set(...LAMP);
  world.add(new three.AmbientLight(0xffffff, GLOW * Math.PI), sun, lamp);
  if (built.faces) world.add(new three.Mesh(geometry, material));
  let swing = 0;
  let tilt = 0;
  const fit = () => {
    renderer.setSize(view.w, view.h, false);
    Object.assign(camera, frustum(view.w / view.h, FIT));
    camera.updateProjectionMatrix();
  };
  const tone = () => {
    const [r, g, b] = rgb(view.look().accent);
    material.color.setRGB(r / 255, g / 255, b / 255, three.SRGBColorSpace);
  };
  const draw = () => {
    camera.position.set(...eye(angle(start, swing, view.t, spin), lean(TILT + tilt), REACH));
    camera.lookAt(0, 0, 0);
    renderer.render(world, camera);
  };
  const turn = (dx, dy) => {
    swing -= dx * TAU;
    tilt = lean(TILT + tilt + dy * Math.PI) - TILT;
    draw();
  };
  const size = () => {
    fit();
    draw();
  };
  const theme = () => {
    tone();
    draw();
  };
  const stop = () => {
    geometry.dispose();
    material.dispose();
    renderer.dispose();
    renderer.forceContextLoss();
  };
  tone();
  fit();
  return { draw, size, theme, turn, stop };
}
