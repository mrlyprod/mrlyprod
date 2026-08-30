import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { ink, blit, role } from './mrly.js';

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

export function web(nodes, branches, roles, radius) {
  const n = nodes.length / 3;
  const tint = new THREE.Color();
  const balls = new THREE.InstancedMesh(new THREE.SphereGeometry(radius, 10, 7), new THREE.MeshStandardMaterial({ roughness: 0.5 }), n);
  for (let i = 0; i < n; i++) balls.setColorAt(i, tint.set(role[roles ? roles[i] : 2]));
  const colors = new Float32Array(branches.length * 3);
  for (let k = 0; k < branches.length; k += 2) {
    tint.set(roles ? role[Math.min(roles[branches[k]], roles[branches[k + 1]])] : ink.line);
    colors.set([tint.r, tint.g, tint.b, tint.r, tint.g, tint.b], k * 3);
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(new Float32Array(branches.length * 3), 3));
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
  const lines = new THREE.LineSegments(geometry, new THREE.LineBasicMaterial({ vertexColors: true, transparent: true, opacity: 0.85 }));
  const m = new THREE.Matrix4();
  const place = (nodes) => {
    const low = [Infinity, Infinity, Infinity], high = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < n; i++) {
      for (let a = 0; a < 3; a++) {
        low[a] = Math.min(low[a], nodes[3 * i + a]);
        high[a] = Math.max(high[a], nodes[3 * i + a]);
      }
    }
    const span = Math.max(high[0] - low[0], high[1] - low[1], high[2] - low[2]) || 1;
    const scale = 2 / span;
    const at = (i, a) => (nodes[3 * i + a] - (low[a] + high[a]) / 2) * scale;
    for (let i = 0; i < n; i++) {
      m.makeTranslation(at(i, 0), at(i, 1), at(i, 2));
      balls.setMatrixAt(i, m);
    }
    balls.instanceMatrix.needsUpdate = true;
    const pos = geometry.attributes.position.array;
    for (let k = 0; k < branches.length; k++) {
      for (let a = 0; a < 3; a++) pos[3 * k + a] = at(branches[k], a);
    }
    geometry.attributes.position.needsUpdate = true;
    geometry.computeBoundingSphere();
  };
  place(nodes);
  return { parts: [balls, lines], place };
}
