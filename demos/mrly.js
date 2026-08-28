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
  const camera = new THREE.PerspectiveCamera(38, 1, 0.01, 100);
  camera.position.set(2.5, 1.9, 2.8);
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
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  };
  new ResizeObserver(resize).observe(canvas);
  resize();
  const st = { scene, camera, renderer, controls, group, spin: 0 };
  st.clear = () => {
    for (const child of [...group.children]) {
      group.remove(child);
      child.geometry?.dispose();
      child.material?.dispose();
    }
  };
  st.show = (object) => {
    st.clear();
    group.add(object);
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

export function faces(buffer, color = ink.blue) {
  const data = buffer.subarray(2, 2 + buffer[0]);
  const interleaved = new THREE.InterleavedBuffer(data, 6);
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.InterleavedBufferAttribute(interleaved, 3, 0));
  geometry.setAttribute('normal', new THREE.InterleavedBufferAttribute(interleaved, 3, 3));
  const material = new THREE.MeshStandardMaterial({ color, roughness: 0.6, metalness: 0.05 });
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
