import * as font from "mrlyjs/font";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { font };

const WORDMARK = "MRLYPROD";

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const grid = font.raster(WORDMARK);
  const rows = grid.length;
  const cols = grid[0].length;
  if (rows !== 5 || cols !== 47) throw new Error(`demo-font: the raster is ${rows} by ${cols}, want 5 by 47`);
  const scale = Math.max(Math.floor(area.w / cols), 1);
  const ox = Math.round((pen.width - cols * scale) / 2);
  const oy = Math.round((pen.height - rows * scale) / 2);
  let lit = 0;
  grid.forEach((line, row) => {
    line.forEach((cell, col) => {
      if (cell !== 1) return;
      lit++;
      pen.rect(ox + col * scale, oy + row * scale, scale, scale, ink.fg);
    });
  });
  if (lit !== 103) throw new Error(`demo-font: ${lit} lit cells, want 103`);
}
