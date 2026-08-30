import init, * as wasm from './pkg/mrlyweb.js';
import wasmUrl from './pkg/mrlyweb_bg.wasm';
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

export const mrly = wasm;

export async function ready() {
  await init({ module_or_path: wasmUrl });
  return wasm;
}

export const ink = {
  bg: '#0b0d10', deep: '#07090b', panel: '#12161b', line: '#1f262e', fg: '#e8ecf1', dim: '#7f8a97',
  blue: '#5cc8ff', orange: '#ff8a5c', gold: '#ffd166', green: '#6ee7a8', pink: '#ff7ab6',
};

export const $ = (id) => document.getElementById(id);

export function rgb(hex) {
  const n = parseInt(hex.slice(1), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

export function say(id, error) {
  $(id).textContent = error ? String(error.message ?? error) : '';
}

// FLAT

export function blit(canvas, pixels) {
  canvas.width = pixels.width;
  canvas.height = pixels.height;
  const image = new ImageData(new Uint8ClampedArray(pixels.rgba), pixels.width, pixels.height);
  canvas.getContext('2d').putImageData(image, 0, 0);
}

export function paint(canvas, grid, on = ink.fg, off = ink.deep) {
  const [w, h] = [grid.width, grid.height];
  const a = rgb(on), b = rgb(off);
  const rgba = new Uint8ClampedArray(w * h * 4);
  for (let i = 0; i < w * h; i++) {
    const c = grid.types[i] ? a : b;
    rgba.set(c, i * 4);
    rgba[i * 4 + 3] = 255;
  }
  canvas.width = w;
  canvas.height = h;
  canvas.getContext('2d').putImageData(new ImageData(rgba, w, h), 0, 0);
}

export function fit(canvas, height) {
  const scale = Math.min(devicePixelRatio || 1, 2);
  const w = canvas.clientWidth;
  canvas.width = w * scale;
  canvas.height = height * scale;
  canvas.style.height = height + 'px';
  const ctx = canvas.getContext('2d');
  ctx.setTransform(scale, 0, 0, scale, 0, 0);
  return [ctx, w, height];
}

// STAGE

export function stage(canvas) {
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
  renderer.setPixelRatio(Math.min(devicePixelRatio || 1, 2));
  const scene = new THREE.Scene();
  const eye = new THREE.PerspectiveCamera(38, 1, 0.01, 100);
  const flat = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.01, 100);
  eye.position.set(2.5, 1.9, 2.8);
  flat.position.copy(eye.position);
  let camera = eye;
  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true;
  scene.add(new THREE.HemisphereLight(0xffffff, 0x1a2230, 1.2));
  const key = new THREE.DirectionalLight(0xffffff, 1.8);
  key.position.set(3, 4, 2);
  scene.add(key);
  const rim = new THREE.DirectionalLight(0x5cc8ff, 0.6);
  rim.position.set(-3, -1, -2);
  scene.add(rim);
  const group = new THREE.Group();
  scene.add(group);
  const resize = () => {
    const w = canvas.clientWidth, h = canvas.clientHeight;
    renderer.setSize(w, h, false);
    const aspect = w / h;
    eye.aspect = aspect;
    eye.updateProjectionMatrix();
    const half = 1.7;
    flat.left = -half * aspect;
    flat.right = half * aspect;
    flat.top = half;
    flat.bottom = -half;
    flat.updateProjectionMatrix();
  };
  new ResizeObserver(resize).observe(canvas);
  resize();
  const st = { scene, renderer, controls, group, spin: 0 };
  st.clear = () => {
    for (const child of [...group.children]) {
      group.remove(child);
      child.geometry?.dispose();
      child.material?.map?.dispose();
      child.material?.dispose();
    }
  };
  st.add = (object) => group.add(object);
  st.show = (object) => {
    st.clear();
    group.add(object);
  };
  st.project = (mode) => {
    const next = mode === 'iso' ? flat : eye;
    if (next === camera) return;
    next.position.copy(camera.position);
    next.quaternion.copy(camera.quaternion);
    next.zoom = 1;
    next.updateProjectionMatrix();
    camera = next;
    controls.object = camera;
    controls.update();
  };
  st.view = (x, y, z) => {
    const direction = new THREE.Vector3(x, y, z).normalize();
    camera.position.copy(controls.target).addScaledVector(direction, 4.2);
    camera.zoom = 1;
    camera.updateProjectionMatrix();
    camera.lookAt(controls.target);
    controls.update();
  };
  const frame = () => {
    requestAnimationFrame(frame);
    group.rotation.y += st.spin;
    controls.update();
    renderer.render(scene, camera);
  };
  frame();
  return st;
}

export function plane(pixels, frame, opacity = 1) {
  const canvas = document.createElement('canvas');
  blit(canvas, pixels);
  const texture = new THREE.CanvasTexture(canvas);
  texture.magFilter = THREE.NearestFilter;
  texture.generateMipmaps = false;
  texture.minFilter = THREE.LinearFilter;
  const { centre: c, u, v, width } = frame;
  const h = width / 2;
  const at = (a, b) => [c[0] + a * u[0] + b * v[0], c[1] + a * u[1] + b * v[1], c[2] + a * u[2] + b * v[2]];
  const [p00, p01, p10, p11] = [at(-h, -h), at(h, -h), at(-h, h), at(h, h)];
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute([...p00, ...p10, ...p11, ...p00, ...p11, ...p01], 3));
  geometry.setAttribute('uv', new THREE.Float32BufferAttribute([0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1], 2));
  const material = new THREE.MeshBasicMaterial({ map: texture, transparent: true, opacity, alphaTest: 0.5, side: THREE.DoubleSide });
  return new THREE.Mesh(geometry, material);
}

export function faces(buffer, color = ink.blue, opacity = 1) {
  const data = buffer.subarray(2, 2 + buffer[0]);
  const interleaved = new THREE.InterleavedBuffer(data, 6);
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.InterleavedBufferAttribute(interleaved, 3, 0));
  geometry.setAttribute('normal', new THREE.InterleavedBufferAttribute(interleaved, 3, 3));
  const clear = opacity < 1;
  const material = new THREE.MeshStandardMaterial({ color, roughness: 0.6, metalness: 0.05, transparent: clear, opacity, depthWrite: !clear });
  return new THREE.Mesh(geometry, material);
}

export function cubes(cells, side, color = ink.orange) {
  const count = cells.length / 3;
  const unit = 2 / side;
  const geometry = new THREE.BoxGeometry(unit * 0.9, unit * 0.9, unit * 0.9);
  const material = new THREE.MeshStandardMaterial({ color, roughness: 0.55 });
  const mesh = new THREE.InstancedMesh(geometry, material, count);
  const m = new THREE.Matrix4();
  for (let i = 0; i < count; i++) {
    m.makeTranslation(
      (cells[3 * i] + 0.5) * unit - 1,
      (cells[3 * i + 1] + 0.5) * unit - 1,
      (cells[3 * i + 2] + 0.5) * unit - 1,
    );
    mesh.setMatrixAt(i, m);
  }
  return mesh;
}

// SOURCES

const FLAT = [['carpet', '495', 3], ['runner', '127', 3], ['gasket', '7', 2], ['pinwheel', '9', 2]];
const CUBES = [['carpet', '23'], ['net', '232'], ['tree', '3'], ['antipodal', '129'], ['solid', '255']];

export function sources(m, host, build) {
  host.innerHTML = `
    <label>source <select id="source">
      <option value="flat">flat design</option>
      <option value="moire">moire stack</option>
      <option value="slice">hex slice</option>
    </select></label>
    <span id="flat-row">
      <label>design <select id="pick"></select></label>
      <label>code <input type="text" id="code" value="495"></label>
      <label>base <select id="base"><option>2</option><option selected>3</option></select></label>
      <label>side <select id="number"><option selected>3</option><option>5</option><option>7</option></select></label>
      <label>level <input type="range" id="level" min="1" max="5" value="4"><span class="num" id="level-out">4</span></label>
    </span>
    <span id="moire-row" hidden>
      <label>preset <select id="preset"></select></label>
      <label>scales up to <input type="range" id="limit" min="1" max="41" step="2" value="9"><span class="num" id="limit-out">9</span></label>
    </span>
    <span id="slice-row" hidden>
      <label>cube <select id="spick"></select></label>
      <label>code <input type="text" id="scode" value="23"></label>
      <label>tile <select id="tile"><option selected>3</option><option>5</option></select></label>
      <label>level <input type="range" id="slevel" min="1" max="3" value="2"><span class="num" id="slevel-out">2</span></label>
    </span>`;
  const query = new URLSearchParams(location.search);
  for (const key of ['source', 'code', 'base', 'level', 'preset', 'limit', 'scode', 'tile', 'slevel']) {
    if (query.has(key)) $(key).value = query.get(key);
  }
  const pick = $('pick'), spick = $('spick');
  pick.append(new Option('type a code', ''));
  for (const [word, code, base] of FLAT) pick.append(new Option(`${word} · ${code} · base ${base}`, `${code}:${base}`));
  pick.onchange = () => {
    if (pick.value) {
      const [code, base] = pick.value.split(':');
      $('code').value = code;
      $('base').value = base;
      build();
    }
  };
  spick.append(new Option('type a code', ''));
  for (const [word, code] of CUBES) spick.append(new Option(`${word} · ${code}`, code));
  spick.onchange = () => {
    if (spick.value) {
      $('scode').value = spick.value;
      build();
    }
  };
  for (const name of m.moire_names()) $('preset').append(new Option(name, name));
  for (const id of ['source', 'code', 'base', 'number', 'level', 'preset', 'limit', 'scode', 'tile', 'slevel']) $(id).oninput = build;
  const known = (select, value) => (select.querySelector(`option[value="${value}"]`) ? value : '');
  const read = () => {
    const kind = $('source').value;
    for (const row of ['flat', 'moire', 'slice']) $(`${row}-row`).hidden = row !== kind;
    if (kind === 'flat') {
      const code = $('code').value.trim(), number = +$('number').value, base = +$('base').value;
      const top = Math.max(1, Math.floor(Math.log(243.5) / Math.log(number)));
      $('level').max = top;
      const level = Math.min(+$('level').value, top);
      $('level').value = level;
      $('level-out').textContent = level;
      pick.value = known(pick, `${code}:${base}`);
      const grid = m.two_grid(code, number, level, 0, base);
      const fills = JSON.parse(m.two_census(code, number, level, 0, base)).fills;
      return { kind, grid, field: Float32Array.from(grid.types), size: grid.width, name: m.name_of(code, 2, base), fills };
    }
    if (kind === 'moire') {
      const limit = +$('limit').value, name = $('preset').value;
      $('limit-out').textContent = limit;
      return { kind, grid: null, field: m.moire_field(name, limit, 256), size: 256, name, fills: '' };
    }
    const code = $('scode').value.trim(), tile = +$('tile').value;
    const top = tile === 3 ? 3 : 2;
    $('slevel').max = top;
    const level = Math.min(+$('slevel').value, top);
    $('slevel').value = level;
    $('slevel-out').textContent = level;
    spick.value = known(spick, code);
    const grid = m.slice_grid(code, tile, level, 2, 384);
    const fills = JSON.parse(m.slice_census(code, tile, level, 2)).fills;
    return { kind, grid, field: Float32Array.from(grid.types), size: 384, name: m.name_of(code, 3, 2), fills };
  };
  return { read };
}
