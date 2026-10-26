import { rng } from "../rand.ts";
import type { Air, Kind, Ring, World } from "./exo.ts";

const TAU = Math.PI * 2;
const GAIN = 1.12;
const GAMMA = 1 / 2.2;
const FILL = 0.92;
const BLOBS = 9;
const CLOUDS = 5;
const SQUEEZE = 2.1;
const FLAT = 0.3;
const SLANT = -0.32;
const BANDS = 4;
const RIM = 0.14;
const SUN = 0.45;

const tone = (rgb: readonly number[], a = 1) =>
  `rgba(${rgb.map((c) => Math.round(Math.min(1, c ** GAMMA * GAIN) * 255)).join(",")},${a})`;

const halo = (air: Air, kind: Kind) => {
  const rgb = kind === "terran" || kind === "ice" ? air.ray : air.mie;
  const top = Math.max(...rgb) || 1;
  return rgb.map((c) => Math.round(255 * (c / top) ** 0.5));
};

export function globe(ctx: CanvasRenderingContext2D, w: number, h: number, params: World) {
  const mid = w / 2;
  const ring = params.ring;
  const r = (FILL * mid) / (ring ? SQUEEZE : 1);
  const rand = rng(params.noise);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  const band = (ring: Ring, front: boolean) => {
    const [a, b] = [ring.inner * r, ring.outer * r];
    const step = (b - a) / BANDS;
    ctx.save();
    ctx.translate(mid, mid);
    ctx.rotate(SLANT);
    ctx.lineWidth = step * FLAT * 2;
    for (let i = 0; i < BANDS; i++) {
      const k = i / (BANDS - 1);
      ctx.strokeStyle = tone(
        ring.dark.map((c, j) => c + (ring.bright[j] - c) * k),
        0.85,
      );
      ctx.beginPath();
      ctx.ellipse(
        0,
        0,
        a + step * (i + 0.5),
        (a + step * (i + 0.5)) * FLAT,
        0,
        front ? 0 : Math.PI,
        front ? Math.PI : TAU,
      );
      ctx.stroke();
    }
    ctx.restore();
  };
  if (ring) band(ring, false);
  ctx.save();
  ctx.beginPath();
  ctx.arc(mid, mid, r, 0, TAU);
  ctx.clip();
  if (params.bands) {
    const sky = ctx.createLinearGradient(0, mid - r, 0, mid + r);
    for (const [y, ...c] of params.bands) sky.addColorStop((y + 1) / 2, tone(c));
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, w, h);
  } else {
    ctx.fillStyle = tone(params.colors![0]);
    ctx.fillRect(0, 0, w, h);
    for (let i = 0; i < BLOBS; i++) {
      ctx.fillStyle = tone(params.colors![1 + (i % 4)]);
      ctx.beginPath();
      ctx.arc(mid + (rand() - 0.5) * 1.6 * r, mid + (rand() - 0.5) * 1.6 * r, r * (0.12 + rand() * 0.3), 0, TAU);
      ctx.fill();
    }
    if (params.cloud && params.cover > 0.2) {
      ctx.strokeStyle = tone(params.cloud, 0.7);
      ctx.lineCap = "round";
      ctx.lineWidth = r * 0.1;
      for (let i = 0; i < CLOUDS; i++) {
        const a = rand() * TAU;
        ctx.beginPath();
        ctx.arc(mid + (rand() - 0.5) * r, mid + (rand() - 0.5) * r, r * (0.25 + rand() * 0.4), a, a + 1 + rand());
        ctx.stroke();
      }
    }
  }
  const lit = ctx.createRadialGradient(mid - r * SUN, mid - r * SUN, r * 0.1, mid - r * 0.15, mid - r * 0.15, r * 1.5);
  lit.addColorStop(0, "rgba(0,0,0,0)");
  lit.addColorStop(0.55, "rgba(0,0,0,0.15)");
  lit.addColorStop(1, "rgba(0,0,0,0.9)");
  ctx.fillStyle = lit;
  ctx.fillRect(0, 0, w, h);
  ctx.restore();
  if (params.air) {
    const [x, y, z] = halo(params.air, params.kind);
    const haze = ctx.createRadialGradient(mid, mid, r * (1 - RIM / 2), mid, mid, r * (1 + RIM));
    haze.addColorStop(0, `rgba(${x},${y},${z},0)`);
    haze.addColorStop(0.3, `rgba(${x},${y},${z},0.45)`);
    haze.addColorStop(1, `rgba(${x},${y},${z},0)`);
    ctx.fillStyle = haze;
    ctx.fillRect(0, 0, w, h);
  }
  if (ring) band(ring, true);
}
