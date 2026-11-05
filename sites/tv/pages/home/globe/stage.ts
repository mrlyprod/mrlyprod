import { Group, PerspectiveCamera, Scene, WebGLRenderer } from "three";

export type Stage = {
  canvas: HTMLCanvasElement;
  root: Group;
  spin: Group;
  still: boolean;
  frame: (dt: number, t: number) => void;
  sized: (width: number, height: number, dpr: number, distance: number) => void;
  invalidate: () => void;
  dispose: () => void;
};

const FOV = 34;
const REACH = 1.34;
const NARROW = 1.14;
const DPR = 2;
const STILL = "(prefers-reduced-motion: reduce)";
const START = 20;
const WIDE = 1.25;
const SHIFT = 0.2;

export function stage(el: HTMLElement): Stage {
  const renderer = new WebGLRenderer({ alpha: true, antialias: true });
  renderer.setClearColor(0x000000, 0);
  const canvas = renderer.domElement;
  const scene = new Scene();
  const camera = new PerspectiveCamera(FOV, 1, 0.1, 50);
  const root = new Group();
  const spin = new Group();
  root.add(spin);
  scene.add(root);
  const still = matchMedia(STILL).matches;
  const out: Stage = {
    canvas,
    root,
    spin,
    still,
    frame: () => {},
    sized: () => {},
    invalidate,
    dispose,
  };
  let raf = 0;
  let last = 0;
  let t = START;
  let seen = true;
  let shown = !document.hidden;
  const live = () => !still && seen && shown;

  function draw(dt: number) {
    t += dt;
    out.frame(dt, t);
    renderer.render(scene, camera);
  }

  function tick(now: number) {
    raf = 0;
    const dt = Math.min(0.1, (now - last) / 1000);
    last = now;
    draw(dt);
    if (live()) raf = requestAnimationFrame(tick);
  }

  function wake() {
    if (raf || !live()) return;
    last = performance.now();
    raf = requestAnimationFrame(tick);
  }

  function invalidate() {
    if (!live()) draw(0);
  }

  function resize() {
    const width = Math.max(1, el.clientWidth);
    const height = Math.max(1, el.clientHeight);
    const dpr = Math.min(devicePixelRatio || 1, DPR);
    renderer.setPixelRatio(dpr);
    renderer.setSize(width, height, false);
    camera.aspect = width / height;
    const tan = Math.tan((FOV / 2) * (Math.PI / 180));
    const reach = camera.aspect < 1 ? NARROW : REACH;
    const distance = reach / (tan * Math.min(1, camera.aspect));
    camera.position.set(0, 0, distance);
    if (camera.aspect > WIDE) camera.setViewOffset(width, height, -width * SHIFT, 0, width, height);
    else camera.clearViewOffset();
    camera.updateProjectionMatrix();
    out.sized(width, height, dpr, distance);
    invalidate();
  }

  const onVisible = () => {
    shown = !document.hidden;
    wake();
  };
  const eye = new IntersectionObserver(([entry]) => {
    seen = entry.isIntersecting;
    wake();
  });
  const ruler = new ResizeObserver(resize);
  document.addEventListener("visibilitychange", onVisible);
  eye.observe(el);
  ruler.observe(el);
  resize();
  wake();

  function dispose() {
    if (raf) cancelAnimationFrame(raf);
    raf = 0;
    document.removeEventListener("visibilitychange", onVisible);
    eye.disconnect();
    ruler.disconnect();
    renderer.dispose();
    renderer.forceContextLoss();
  }

  return out;
}
