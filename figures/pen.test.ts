import { expect, test } from "bun:test";
import sharp from "sharp";
import * as core from "mrlyjs/core";
import { field, ink, raster, svg, type Pen, type Pixels } from "mrlyjs/view";
import { dark, light } from "../site/kit/theme/theme.js";

const SIZE = 256;
const LEVELS = 2;
const EDGE = 64;
const SHARE = 0.04;
const theme = ink(dark);

const png = (p: Pixels) => sharp(p.colors, { raw: { width: p.shape[1], height: p.shape[0], channels: 4 } }).png().toBuffer();
const wave = Array.from({ length: 200 }, (_, i): [number, number] => [10 + i * 1.18, 128 + 80 * Math.sin(i / 20)]);
const star = [0, 1, 2, 3, 4].map((i): [number, number] => [128 + 110 * Math.cos(-Math.PI / 2 + (i * 4 * Math.PI) / 5), 128 + 110 * Math.sin(-Math.PI / 2 + (i * 4 * Math.PI) / 5)]);
const spots = Array.from({ length: 64 }, (_, i) => Math.sin(i * 0.7) + Math.cos(i * 0.3));

const scenes: Record<string, (pen: Pen) => void> = {
  rect: (p) => p.rect(20.3, 30.7, 150.4, 90.2, theme.fg),
  round_rect: (p) => p.round_rect(20.3, 30.7, 200, 150, 30, theme.fg),
  disc: (p) => p.disc(128.3, 127.6, 90, theme.fg),
  ring: (p) => p.ring(128.3, 127.6, 80, 6, theme.fg),
  segment: (p) => p.segment([20.5, 30.25], [230.1, 200.7], 7, theme.fg),
  polyline: (p) => p.polyline(wave, 5, theme.fg),
  triangle: (p) => p.triangle([30, 220], [128.4, 25.3], [230.2, 210.1], theme.fg),
  polygon: (p) => p.polygon(star, theme.fg),
  arc: (p) => p.arc([128.3, 127.6], 80, [0.3, 4.1], 9, theme.fg),
  image: (p) => field.draw(p, p.area(0.125), 8, 8, spots, theme.Ramp.heat()),
  cover: (p) => {
    const patch = field.patch(40, 50, 120, 100);
    for (let py = 50; py < 150; py++) for (let px = 40; px < 160; px++) patch.blend(px, py, theme.orange, ((px - 40) / 120) * ((py - 50) / 100));
    patch.paint(p);
  },
  mixed: (p) => {
    p.rect(10, 10, 236, 236, theme.panel);
    field.draw(p, p.area(0.3), 8, 8, spots, theme.Ramp.fire());
    p.disc(90, 100, 60, theme.blue);
    p.ring(160, 150, 50, 8, theme.fade(theme.orange, 0.6));
    p.polyline(wave, 3, theme.yellow);
    p.triangle([20, 240], [120, 140], [240, 236], theme.fade(theme.green, 0.5));
    p.arc([128, 128], 100, [-1, 2], 5, theme.pink);
    p.round_rect(150, 20, 80, 60, 12, theme.indigo);
    p.segment([20, 20], [100, 60], 4, theme.fg);
    p.polygon(star.map(([x, y]) => [x * 0.4 + 140, y * 0.4 + 120]), theme.fade(theme.fg, 0.4));
  },
};

async function compare(scene: (pen: Pen) => void) {
  const ours = raster(SIZE, SIZE, theme.ground);
  scene(ours);
  const pen = svg(SIZE, SIZE, theme.ground);
  scene(pen);
  const theirs = await sharp(Buffer.from(await pen.text({ png }))).ensureAlpha().raw().toBuffer();
  const a = ours.pixels().colors;
  const row = { max: 0, over: 0, edges: 0, inside: 0, edge: 0 };
  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      const i = (y * SIZE + x) * 4;
      let d = 0;
      for (let k = 0; k < 4; k++) d = Math.max(d, Math.abs(a[i + k] - theirs[i + k]));
      let flat = true;
      for (let j = Math.max(y - 1, 0); j <= Math.min(y + 1, SIZE - 1); j++) {
        for (let k = Math.max(x - 1, 0); k <= Math.min(x + 1, SIZE - 1); k++) {
          const n = (j * SIZE + k) * 4;
          if (a[n] !== a[i] || a[n + 1] !== a[i + 1] || a[n + 2] !== a[i + 2] || a[n + 3] !== a[i + 3]) flat = false;
        }
      }
      row.max = Math.max(row.max, d);
      if (d > LEVELS) row.over++;
      if (flat) row.inside = Math.max(row.inside, d);
      else {
        row.edges++;
        row.edge = Math.max(row.edge, d);
      }
    }
  }
  return row;
}

// PARITY

for (const [name, scene] of Object.entries(scenes)) {
  test(`${name} through svg and sharp meets raster within ${LEVELS} levels off its edges`, async () => {
    const row = await compare(scene);
    const share = row.over / (SIZE * SIZE);
    console.log(`${name.padEnd(10)} max ${String(row.max).padStart(3)}  over ${LEVELS} ${(share * 100).toFixed(3).padStart(6)}%  inside max ${row.inside}  edge max ${String(row.edge).padStart(3)} over ${row.edges} edge pixels`);
    expect(row.inside).toBeLessThanOrEqual(LEVELS);
    expect(row.edge).toBeLessThanOrEqual(EDGE);
    expect(share).toBeLessThanOrEqual(SHARE);
  });
}

// INK

test("ink from the Rust theme equals ink from the kit theme", async () => {
  const bytes = await Bun.file(new URL(import.meta.resolve("mrlyjs/pkg/core/mrlyjs_core_bg.wasm"))).arrayBuffer();
  core.initSync({ module: bytes });
  for (const [rust, kit] of [[core.colors.DARK(), dark], [core.colors.LIGHT(), light]]) {
    const ours = ink(rust);
    const theirs = ink(kit);
    for (const key of Object.keys(theirs)) if (Array.isArray(theirs[key as keyof typeof theirs])) expect([key, ours[key as keyof typeof ours]]).toEqual([key, theirs[key as keyof typeof theirs]]);
  }
});
