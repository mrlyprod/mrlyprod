export type Spin = { yaw: number; pitch: number; vy: number; held: boolean };

const TAU = 1.6;
const PITCH = 1.1;
const FLING = 5;

const clamp = (v: number, lim: number) => Math.min(lim, Math.max(-lim, v));

export function drag(canvas: HTMLElement, spin: Spin, onMove: () => void): () => void {
  let id = -1;
  let x = 0;
  let y = 0;
  let at = 0;
  canvas.style.touchAction = "pan-y";
  canvas.style.cursor = "grab";
  const down = (e: PointerEvent) => {
    if (id >= 0 || e.button) return;
    id = e.pointerId;
    canvas.setPointerCapture(id);
    canvas.style.cursor = "grabbing";
    spin.held = true;
    spin.vy = 0;
    x = e.clientX;
    y = e.clientY;
    at = e.timeStamp;
  };
  const move = (e: PointerEvent) => {
    if (e.pointerId !== id) return;
    const k = Math.PI / Math.max(1, canvas.clientWidth);
    const dx = (e.clientX - x) * k;
    const dy = (e.clientY - y) * k;
    const dt = (e.timeStamp - at) / 1000;
    spin.yaw += dx;
    spin.pitch = clamp(spin.pitch + dy * 0.7, PITCH);
    if (dt > 0) spin.vy = clamp(spin.vy * 0.5 + (dx / dt) * 0.5, FLING);
    x = e.clientX;
    y = e.clientY;
    at = e.timeStamp;
    onMove();
  };
  const up = (e: PointerEvent) => {
    if (e.pointerId !== id) return;
    id = -1;
    spin.held = false;
    canvas.style.cursor = "grab";
    if (e.timeStamp - at > 80) spin.vy = 0;
  };
  canvas.addEventListener("pointerdown", down);
  canvas.addEventListener("pointermove", move);
  canvas.addEventListener("pointerup", up);
  canvas.addEventListener("pointercancel", up);
  return () => {
    canvas.removeEventListener("pointerdown", down);
    canvas.removeEventListener("pointermove", move);
    canvas.removeEventListener("pointerup", up);
    canvas.removeEventListener("pointercancel", up);
  };
}

export function settle(spin: Spin, dt: number, auto: number): void {
  if (spin.held) return;
  spin.vy += (auto - spin.vy) * (1 - Math.exp(-dt / TAU));
  spin.yaw += spin.vy * dt;
}
