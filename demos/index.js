import { ready, $, ink, blit, paint, fit } from './mrly.js';

const m = await ready();

paint($('t-race'), m.two_grid('127', 3, 4, 0, 3), ink.blue);
$('t-sponge').innerHTML = m.hex_svg('23', 3, 2, 2, 'iso', 3);
paint($('t-universe'), m.two_grid('9', 3, 3, 0, 2), ink.gold);
paint($('t-life'), { width: 48, height: 48, types: m.life_noise(48, 48, 0.4, 3) }, ink.green);
blit($('t-moire'), m.moire('weave', 11, 120, 'fire', 2, false));

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
