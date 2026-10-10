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
const DETAIL = [
  { ball: [12, 8], rod: 8, open: false },
  { ball: [8, 6], rod: 6, open: false },
  { ball: [6, 4], rod: 5, open: true },
  { ball: [4, 3], rod: 4, open: true },
];
export const TRIANGLES = 400000;

export function detail(count, links) {
  const fits = ({ ball: [w, h], rod, open }) => count * w * (2 * h - 2) + links * rod * (open ? 2 : 4) <= TRIANGLES;
  return DETAIL.find(fits) ?? DETAIL[DETAIL.length - 1];
}

const lean = (phi) => Math.min(LEAN, Math.max(-LEAN, phi));

export function angle(start, swing, t, spin) {
  return start + swing + (spin ? (t / ORBIT) * TAU : 0);
}

export function space(three, canvas, view, plan, { start = 0, spin = 1 } = {}) {
  const { count, pairs, order, shade } = plan;
  const links = pairs.length / 2;
  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true });
  renderer.setPixelRatio(1);
  renderer.setClearColor(0x000000, 0);
  const world = new three.Scene();
  const camera = new three.OrthographicCamera(-1, 1, 1, -1, 0.1, REACH * 2);
  camera.up.set(0, 0, 1);
  const fine = detail(count, links);
  const ball = new three.SphereGeometry(1, fine.ball[0], fine.ball[1]);
  const rod = new three.CylinderGeometry(1, 1, 1, fine.rod, 1, fine.open);
  const material = new three.MeshLambertMaterial();
  const balls = new three.InstancedMesh(ball, material, Math.max(1, count));
  const rods = new three.InstancedMesh(rod, material, Math.max(1, links));
  balls.frustumCulled = false;
  rods.frustumCulled = false;
  balls.count = 0;
  rods.count = 0;
  const sun = new three.DirectionalLight(0xffffff, KEY * Math.PI);
  sun.position.set(...SUN);
  const lamp = new three.DirectionalLight(0xffffff, FILL * Math.PI);
  lamp.position.set(...LAMP);
  world.add(balls, rods, new three.AmbientLight(0xffffff, GLOW * Math.PI), sun, lamp);
  const body = new three.Object3D();
  const up = new three.Vector3(0, 1, 0);
  const along = new three.Vector3();
  const paint = new three.Color();
  const probe = new three.Vector3();
  let swing = 0;
  let tilt = 0;
  const place = (at, dot, line) => {
    body.quaternion.identity();
    body.scale.set(dot, dot, dot);
    for (let i = 0; i < count; i++) {
      const n = order.nodes[i] * 3;
      body.position.set(at[n], at[n + 1], at[n + 2]);
      body.updateMatrix();
      balls.setMatrixAt(i, body.matrix);
    }
    for (let j = 0; j < links; j++) {
      const b = order.branches[j] * 2;
      const a = pairs[b] * 3;
      const c = pairs[b + 1] * 3;
      const dx = at[c] - at[a];
      const dy = at[c + 1] - at[a + 1];
      const dz = at[c + 2] - at[a + 2];
      const len = Math.hypot(dx, dy, dz) || 1;
      body.position.set((at[a] + at[c]) / 2, (at[a + 1] + at[c + 1]) / 2, (at[a + 2] + at[c + 2]) / 2);
      along.set(dx / len, dy / len, dz / len);
      body.quaternion.setFromUnitVectors(up, along);
      body.scale.set(line, len, line);
      body.updateMatrix();
      rods.setMatrixAt(j, body.matrix);
    }
    balls.instanceMatrix.needsUpdate = true;
    rods.instanceMatrix.needsUpdate = true;
  };
  const tone = (shades) => {
    for (let i = 0; i < count; i++) balls.setColorAt(i, paint.set(shades[shade.nodes[i]]));
    for (let j = 0; j < links; j++) rods.setColorAt(j, paint.set(shades[shade.branches[j]]));
    if (balls.instanceColor) balls.instanceColor.needsUpdate = true;
    if (rods.instanceColor) rods.instanceColor.needsUpdate = true;
  };
  const show = (k, m) => {
    balls.count = k;
    rods.count = m;
  };
  const size = () => {
    renderer.setSize(view.w, view.h, false);
    Object.assign(camera, frustum(view.w / view.h, FIT));
    camera.updateProjectionMatrix();
  };
  const pose = () => {
    camera.position.set(...eye(angle(start, swing, view.t, spin), lean(TILT + tilt), REACH));
    camera.lookAt(0, 0, 0);
  };
  const draw = () => {
    pose();
    renderer.render(world, camera);
  };
  const turn = (dx, dy) => {
    swing -= dx * TAU;
    tilt = lean(TILT + tilt + dy * Math.PI) - TILT;
    draw();
  };
  const unit = (h) => h / (2 * frustum(view.w / view.h, FIT).top);
  const project = (at, w, h) => {
    pose();
    camera.updateMatrixWorld();
    const out = new Float64Array(count * 3);
    for (let i = 0; i < count; i++) {
      probe.set(at[3 * i], at[3 * i + 1], at[3 * i + 2]).project(camera);
      out[3 * i] = ((probe.x + 1) / 2) * w;
      out[3 * i + 1] = ((1 - probe.y) / 2) * h;
      out[3 * i + 2] = probe.z;
    }
    return out;
  };
  const stop = () => {
    ball.dispose();
    rod.dispose();
    material.dispose();
    balls.dispose();
    rods.dispose();
    renderer.dispose();
    renderer.forceContextLoss();
  };
  size();
  return { draw, place, tone, show, turn, size, unit, project, stop };
}
