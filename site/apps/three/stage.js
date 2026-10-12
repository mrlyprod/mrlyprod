const TAU = Math.PI * 2;
export const ORBIT = 26000;
export const POSE = [0.84, 0.47];
export const LOOKS = { corner: [Math.PI / 4, Math.atan(Math.SQRT1_2)], edge: [Math.PI / 4, 0], face: [0, 0] };
const FOV = 38;
const REACH = 4.2;
const HALF = 1.7;
const LEAN = 1.5;
const GRIP = 0.9;
const REST = 1e-4;
const ZOOM = [0.3, 8];
const CUBE = 0.9;
const SKY = 1.2;
const KEY = 1.8;
const RIM = 0.6;
const LAMPS = { key: [2, 3, 2], rim: [-2, -1, -3] };
const ROUGH = 0.6;
const METAL = 0.05;
const ALPHA = 0.5;

const lean = (phi) => Math.min(LEAN, Math.max(-LEAN, phi));

const clamp = (v, [lo, hi]) => Math.min(hi, Math.max(lo, v));

export function angle(swing, t, spin) {
  return swing + (spin ? (t / ORBIT) * TAU : 0);
}

export function eye(theta, phi, reach) {
  const flat = Math.cos(phi) * reach;
  return [Math.cos(theta) * flat, Math.sin(theta) * flat, Math.sin(phi) * reach];
}

export function pack(quads) {
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

export function spots(cell) {
  const [a, b, c] = cell.shape;
  const unit = 2 / Math.max(a, b, c);
  const out = [];
  for (let flat = 0; flat < cell.types.length; flat++) {
    if (!cell.types[flat]) continue;
    const z = flat % c;
    const y = Math.floor(flat / c) % b;
    const x = Math.floor(flat / (b * c));
    out.push((x + 0.5) * unit - 1, (y + 0.5) * unit - 1, (z + 0.5) * unit - 1);
  }
  return { centres: Float32Array.from(out), unit };
}

const corner = (frame, a, b) => frame.centre.map((c, i) => c + a * frame.u[i] + b * frame.v[i]);

export function sheet(frame) {
  const h = frame.width / 2;
  const [p00, p10, p11, p01] = [corner(frame, -h, -h), corner(frame, -h, h), corner(frame, h, h), corner(frame, h, -h)];
  return { position: Float32Array.from([...p00, ...p10, ...p11, ...p00, ...p11, ...p01]), uv: Float32Array.from([0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0]) };
}

export function space(three, canvas, view) {
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
  let camera = near;
  let body = null;
  let [swing, tilt] = POSE;
  let zoom = 1;
  let pan = [0, 0];
  let fling = [0, 0];
  let spin = 0;
  let dirty = true;
  let aspect = 1;
  const fit = () => {
    renderer.setSize(view.w, view.h, false);
    aspect = view.w / view.h;
    near.aspect = aspect;
    const half = HALF / Math.min(1, aspect);
    Object.assign(flat, { left: -half * aspect, right: half * aspect, top: half, bottom: -half });
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
    const reach = (REACH / Math.min(1, aspect)) / (camera === near ? zoom : 1);
    camera.position.set(...eye(angle(swing, view.t, spin), tilt, reach));
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
  const held = new Map([[solid, []], [cuts, []]]);
  const hold = (group, one, ...parts) => {
    group.add(one);
    held.get(group).push([one, parts]);
  };
  const drop = (group) => {
    for (const [one, parts] of held.get(group)) {
      group.remove(one);
      for (const part of parts) part.dispose();
    }
    held.set(group, []);
  };
  const show = (next) => {
    if (next === body) return;
    body = next;
    drop(solid);
    if (next?.quads?.length) {
      const built = pack(next.quads);
      const geometry = new three.BufferGeometry();
      geometry.setAttribute('position', new three.BufferAttribute(built.position, 3));
      geometry.setAttribute('normal', new three.BufferAttribute(built.normal, 3));
      geometry.setIndex(new three.BufferAttribute(built.index, 1));
      hold(solid, new three.Mesh(geometry, material), geometry);
    } else if (next?.cell && next.cubes) {
      const { centres, unit } = spots(next.cell);
      const count = centres.length / 3;
      if (count) {
        const box = new three.BoxGeometry(unit * CUBE, unit * CUBE, unit * CUBE);
        const many = new three.InstancedMesh(box, material, count);
        const place = new three.Matrix4();
        for (let i = 0; i < count; i++) many.setMatrixAt(i, place.makeTranslation(centres[i * 3], centres[i * 3 + 1], centres[i * 3 + 2]));
        many.instanceMatrix.needsUpdate = true;
        hold(solid, many, box);
      }
    }
    dirty = true;
  };
  const paint = ({ color, opacity = 1, clip = null }) => {
    const [r, g, b] = color;
    material.color.setRGB(r / 255, g / 255, b / 255, three.SRGBColorSpace);
    const clear = opacity < 1;
    material.transparent = clear;
    material.opacity = opacity;
    material.depthWrite = !clear;
    material.clippingPlanes = clip ? clip.map(({ normal, constant }) => new three.Plane(new three.Vector3(...normal), constant)) : null;
    material.side = clip ? three.DoubleSide : three.FrontSide;
    material.needsUpdate = true;
    dirty = true;
  };
  const planes = (list) => {
    drop(cuts);
    for (const { frame: at, pixels } of list) {
      const [h, w] = pixels.shape;
      const texture = new three.DataTexture(pixels.colors, w, h, three.RGBAFormat);
      texture.magFilter = three.NearestFilter;
      texture.colorSpace = three.SRGBColorSpace;
      texture.needsUpdate = true;
      const quad = sheet(at);
      const shape = new three.BufferGeometry();
      shape.setAttribute('position', new three.BufferAttribute(quad.position, 3));
      shape.setAttribute('uv', new three.BufferAttribute(quad.uv, 2));
      const skin = new three.MeshBasicMaterial({ map: texture, transparent: true, alphaTest: ALPHA, side: three.DoubleSide });
      hold(cuts, new three.Mesh(shape, skin), shape, skin, texture);
    }
    dirty = true;
  };
  const project = (kind) => {
    const next = kind === 'iso' ? flat : near;
    if (next === camera) return;
    camera = next;
    dirty = true;
  };
  const spinning = (on) => {
    const was = spin;
    spin = on ? 1 : 0;
    swing = angle(swing, view.t, was) - (spin ? (view.t / ORBIT) * TAU : 0);
    dirty = true;
  };
  const look = (name) => {
    const [theta, phi] = LOOKS[name] ?? POSE;
    swing = theta - (spin ? (view.t / ORBIT) * TAU : 0);
    tilt = phi;
    fling = [0, 0];
    zoom = 1;
    pan = [0, 0];
    frame();
    touch();
  };
  const turn = (dx, dy) => {
    fling = [0, 0];
    swing -= dx * TAU;
    tilt = lean(tilt + dy * Math.PI);
    touch();
  };
  const hurl = (dx, dy) => {
    fling = [-dx * TAU, dy * Math.PI];
    dirty = true;
  };
  const scale = (by) => {
    zoom = clamp(zoom * by, ZOOM);
    frame();
    touch();
  };
  const shift = (dx, dy) => {
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
