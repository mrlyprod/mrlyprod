import { TILT, eye, frustum } from '../designs/mesh.js';

const TAU = Math.PI * 2;
export const ORBIT = 40000;
const FIT = 0.86;
const FOV = 38;
const FAR = 80;
const BACK = 8;
const LEAN = 1.3;
const NEAR = [0.25, 12];
const DAMP = 0.12;
const FRAME = 1000 / 60;
const STILL = 1e-5;
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
export const START = [Math.atan2(2.8, 2.5), Math.atan2(1.9, Math.hypot(2.5, 2.8))];
export const VIEWS = { corner: [Math.PI / 4, TILT], edge: [Math.PI / 4, 0], face: [0, 0] };

export function detail(count, links) {
  const fits = ({ ball: [w, h], rod, open }) => count * w * (2 * h - 2) + links * rod * (open ? 2 : 4) <= TRIANGLES;
  return DETAIL.find(fits) ?? DETAIL[DETAIL.length - 1];
}

const lean = (phi) => Math.min(LEAN, Math.max(-LEAN, phi));

const clamp = (x, [lo, hi]) => Math.min(hi, Math.max(lo, x));

export function axes(swing, tilt) {
  const right = [-Math.sin(swing), Math.cos(swing), 0];
  const up = [-Math.cos(swing) * Math.sin(tilt), -Math.sin(swing) * Math.sin(tilt), Math.cos(tilt)];
  return { right, up };
}

export function space(three, canvas, view) {
  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setClearColor(0x000000, 0);
  const world = new three.Scene();
  const iso = new three.OrthographicCamera(-1, 1, 1, -1, 0.1, FAR);
  const persp = new three.PerspectiveCamera(FOV, 1, 0.05, FAR);
  iso.up.set(0, 0, 1);
  persp.up.set(0, 0, 1);
  const material = new three.MeshLambertMaterial();
  const sun = new three.DirectionalLight(0xffffff, KEY * Math.PI);
  sun.position.set(...SUN);
  const lamp = new three.DirectionalLight(0xffffff, FILL * Math.PI);
  lamp.position.set(...LAMP);
  world.add(new three.AmbientLight(0xffffff, GLOW * Math.PI), sun, lamp);
  const body = new three.Object3D();
  const up = new three.Vector3(0, 1, 0);
  const along = new three.Vector3();
  const paint = new three.Color();
  const probe = new three.Vector3();
  let mesh = null;
  let mode = 'eye';
  let [swing, tilt] = START;
  let near = 1;
  let spin = 0;
  let held = false;
  let fling = [0, 0];
  let moved = [0, 0];
  let last = null;
  let top = 1;
  let aim = [0, 0, 0];
  const camera = () => (mode === 'iso' ? iso : persp);
  const drop = () => {
    if (!mesh) return;
    world.remove(mesh.balls, mesh.rods);
    mesh.ball.dispose();
    mesh.rod.dispose();
    mesh.balls.dispose();
    mesh.rods.dispose();
    mesh = null;
  };
  const load = (plan) => {
    drop();
    const count = plan.count;
    const links = plan.pairs.length / 2;
    const fine = detail(count, links);
    const ball = new three.SphereGeometry(1, fine.ball[0], fine.ball[1]);
    const rod = new three.CylinderGeometry(1, 1, 1, fine.rod, 1, fine.open);
    const balls = new three.InstancedMesh(ball, material, Math.max(1, count));
    const rods = new three.InstancedMesh(rod, material, Math.max(1, links));
    balls.frustumCulled = false;
    rods.frustumCulled = false;
    balls.count = count;
    rods.count = links;
    world.add(balls, rods);
    mesh = { plan, ball, rod, balls, rods, count, links };
  };
  const place = (at, dot, line) => {
    if (!mesh) return;
    const { plan, balls, rods, count, links } = mesh;
    body.quaternion.identity();
    body.scale.set(dot, dot, dot);
    for (let i = 0; i < count; i++) {
      body.position.set(at[3 * i], at[3 * i + 1], at[3 * i + 2]);
      body.updateMatrix();
      balls.setMatrixAt(i, body.matrix);
    }
    for (let j = 0; j < links; j++) {
      const a = plan.pairs[2 * j] * 3;
      const c = plan.pairs[2 * j + 1] * 3;
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
    if (!mesh) return;
    const { plan, balls, rods, count, links } = mesh;
    for (let i = 0; i < count; i++) balls.setColorAt(i, paint.set(shades[plan.shade.nodes[i]]));
    for (let j = 0; j < links; j++) rods.setColorAt(j, paint.set(shades[plan.shade.branches[j]]));
    if (balls.instanceColor) balls.instanceColor.needsUpdate = true;
    if (rods.instanceColor) rods.instanceColor.needsUpdate = true;
  };
  const size = () => {
    renderer.setSize(view.w, view.h, false);
    const frame = frustum(view.w / view.h, FIT);
    top = frame.top;
    Object.assign(iso, frame);
    iso.updateProjectionMatrix();
    persp.aspect = view.w / view.h;
    persp.updateProjectionMatrix();
  };
  const reach = () => top / Math.tan((FOV * Math.PI) / 360);
  const pose = () => {
    const lens = camera();
    const [x, y, z] = eye(swing, tilt, mode === 'iso' ? BACK : reach() / near);
    lens.position.set(aim[0] + x, aim[1] + y, aim[2] + z);
    lens.lookAt(aim[0], aim[1], aim[2]);
    if (mode !== 'iso') return;
    iso.zoom = near;
    iso.updateProjectionMatrix();
  };
  const render = () => {
    pose();
    renderer.render(world, camera());
  };
  const rotate = (dx, dy) => {
    swing -= dx * TAU;
    tilt = lean(tilt + dy * Math.PI);
  };
  const tick = () => {
    const dt = last === null ? 0 : Math.max(0, view.t - last);
    last = view.t;
    if (!dt) return false;
    let go = false;
    if (held) {
      fling = [(moved[0] * FRAME) / dt, (moved[1] * FRAME) / dt];
      moved = [0, 0];
    } else if (Math.abs(fling[0]) + Math.abs(fling[1]) > STILL) {
      const k = dt / FRAME;
      rotate(fling[0] * k, fling[1] * k);
      fling = fling.map((v) => v * (1 - DAMP) ** k);
      go = true;
    }
    if (spin && !held) {
      swing -= (dt / ORBIT) * TAU;
      go = true;
    }
    return go;
  };
  const draw = (dirty) => {
    if (tick() || dirty || view.fixed) render();
  };
  const turn = (dx, dy) => {
    rotate(dx, dy);
    moved = [moved[0] + dx, moved[1] + dy];
    render();
  };
  const grab = () => {
    held = true;
    fling = [0, 0];
    moved = [0, 0];
  };
  const release = () => {
    held = false;
    if (view.still) fling = [0, 0];
  };
  const zoom = (by) => {
    near = clamp(near * by, NEAR);
    render();
  };
  const pan = (dx, dy) => {
    const tall = (2 * top) / near;
    const wide = tall * (view.w / view.h);
    const { right, up: rise } = axes(swing, tilt);
    aim = aim.map((p, a) => p - right[a] * dx * wide + rise[a] * dy * tall);
    render();
  };
  const snap = (name) => {
    if (!VIEWS[name]) return;
    [swing, tilt] = VIEWS[name];
    near = 1;
    fling = [0, 0];
    aim = [0, 0, 0];
    render();
  };
  const lens = (next) => {
    mode = next === 'iso' ? 'iso' : 'eye';
  };
  const spinning = (on) => {
    spin = Number(on) === 1 ? 1 : 0;
  };
  const unit = (h) => h / (2 * top);
  const project = (at, w, h) => {
    pose();
    const lens = camera();
    lens.updateMatrixWorld();
    const n = at.length / 3;
    const out = new Float64Array(n * 3);
    for (let i = 0; i < n; i++) {
      probe.set(at[3 * i], at[3 * i + 1], at[3 * i + 2]).project(lens);
      out[3 * i] = ((probe.x + 1) / 2) * w;
      out[3 * i + 1] = ((1 - probe.y) / 2) * h;
      out[3 * i + 2] = probe.z;
    }
    return out;
  };
  const stop = () => {
    drop();
    material.dispose();
    renderer.dispose();
    renderer.forceContextLoss();
  };
  size();
  return { draw, load, place, tone, size, turn, grab, release, zoom, pan, snap, lens, spinning, unit, project, stop };
}
