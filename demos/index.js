import { ready, $, ink, blit, paint, fit } from './lib/mrly.js';
import { web, board, bars } from './lib/chart.js';

const m = await ready();

{
  const canvas = $('t-tour');
  const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
  ctx.imageSmoothingEnabled = false;
  const codes = ['1', '3', '7', '9', '11', '15'];
  const slot = (w - 12) / codes.length, size = Math.min(slot - 6, h - 12);
  codes.forEach((code, i) => {
    const tile = document.createElement('canvas');
    paint(tile, m.two_grid(code, 5, 1, 0, 2), ink.gold);
    ctx.drawImage(tile, 6 + i * slot + (slot - size) / 2, (h - size) / 2, size, size);
  });
}

paint($('t-race'), m.two_grid('127', 3, 4, 0, 3), ink.blue);
$('t-sponge').innerHTML = m.hex_svg('23', 3, 2, 2, 'iso', 3);
$('t-cuts').innerHTML = m.diagonal_svg('126', 2, 5, 2, JSON.parse(m.diagonal_profile('126', 2, 5, 2)).central, 6);
$('t-slices').innerHTML = m.hex_svg('23', 7, 1, 2, 'cut', 8);
paint($('t-spectra'), m.two_grid('7', 2, 5, 0, 2), ink.pink);
paint($('t-universe'), m.two_grid('9', 3, 3, 0, 2), ink.gold);
paint($('t-life'), { width: 48, height: 48, types: m.life_noise(48, 48, 0.4, 3) }, ink.green);
blit($('t-moire'), m.moire('weave', 11, 120, 'fire', 2, false));
const carpet = m.two_grid('495', 3, 3, 0, 3);
blit($('t-spin'), m.wheel(m.profile(Float32Array.from(m.two_grid('495', 3, 4, 0, 3).types), 81, 256), 180, 'fire', 64, false));
const solid = m.volume('23', 2, 7, 'sum', 1, 48);
const range = JSON.parse(m.volume_stats(solid, 48));
blit($('t-volume'), m.paint_span(m.plane_field(solid, 48, [1, 1, 1], 0.5, 180), 180, range.min, range.max, 'fire', 16, false));
blit($('t-radial'), m.sheet(m.radial(Float32Array.from(carpet.types), 27, 180, 5, 72, 'mean', 2), 180, 'fire', 64, false));
const sieve = new m.Sieve(150);
sieve.finish();
paint($('t-primes'), sieve.grid(15), ink.gold);
blit($('t-ulam'), m.spiral_pixels('square', 61, 4, -2, 41, 'prime', false, 180));
blit($('t-gaussian'), m.ring_pixels('gaussian', 24, 'class', false, 180));
web($('t-graphs'), $('t-graphs').clientWidth / 1.5, m.graph_nodes('flat', '495', 3, 2, 3, 'core').subarray(2), m.graph_branches('flat', '495', 3, 2, 3, 'core'), null, 3);

const canvas = $('t-farey');
const [ctx, w, h] = fit(canvas, canvas.clientWidth / 1.5);
ctx.strokeStyle = ink.pink;
for (const [num, den, bright] of JSON.parse(m.farey(24))) {
  const x = 6 + (w - 12) * num / den;
  ctx.globalAlpha = 0.25 + 0.75 * bright / 24;
  ctx.beginPath();
  ctx.moveTo(x, h - 6);
  ctx.lineTo(x, h - 6 - (h - 12) * bright / 24);
  ctx.stroke();
}

{
  const canvas = $('t-sequences');
  const b = board(canvas, canvas.clientWidth / 1.5, { top: 8, bottom: 8 });
  bars(b, m.ledger_terms('7', 2, 2, 'fills', 'level', 8, '500000').map((t) => Math.log10(Number(t))), { color: ink.gold, inset: 3 });
}

{
  m.census_walk(JSON.parse(m.census_window()).tiers[0].keys);
  const counts = m.census_counts();
  paint($('t-integers'), { width: 40, height: counts.length / 40, types: Uint8Array.from(counts, (rows) => (rows ? 1 : 0)) }, ink.gold);
}

const zeta = $('t-zeta');
{
  const [ctx, w, h] = fit(zeta, zeta.clientWidth / 1.5);
  const path = m.zeta_line(0, 50, 600);
  let reach = 1;
  for (let k = 0; k < path.length; k += 4) reach = Math.max(reach, Math.abs(path[k + 1]), Math.abs(path[k + 2]));
  const scale = (h / 2 - 6) / reach;
  ctx.strokeStyle = ink.blue;
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  for (let k = 0; k < path.length; k += 4) {
    const x = w / 2 + path[k + 1] * scale, y = h / 2 - path[k + 2] * scale;
    if (k) ctx.lineTo(x, y);
    else ctx.moveTo(x, y);
  }
  ctx.stroke();
  ctx.strokeStyle = ink.gold;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(w / 2, h / 2, 4, 0, Math.PI * 2);
  ctx.stroke();
}
