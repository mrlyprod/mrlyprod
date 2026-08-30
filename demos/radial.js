import { ready, $, ink, blit, say, fit, sources } from './mrly.js';

const m = await ready();
const OUT = 512;
const ORDERS = 48;
const RINGS = 160;
const src = sources(m, $('sources'), build);
const query = new URLSearchParams(location.search);
for (const key of ['copies', 'step', 'blend']) {
  if (query.has(key)) $(key).value = query.get(key);
}
if (query.has('step')) $('full').checked = false;

let plot = null;

function chart(power, copies, full, order) {
  plot = [power, copies, full, order];
  const canvas = $('bars');
  const [ctx, w, h] = fit(canvas, 300);
  ctx.clearRect(0, 0, w, h);
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  const pad = 14, floor = h - 22, roof = 26;
  const wide = w - 2 * pad, step = wide / power.length;
  const peak = Math.max(...power, 1e-12);
  for (let k = 0; k < power.length; k++) {
    const tall = (floor - roof) * power[k] / peak;
    const lives = full ? k % copies === 0 : k === 0;
    ctx.fillStyle = k === 0 ? ink.gold : lives ? ink.blue : ink.line;
    ctx.fillRect(pad + k * step + 1, floor - tall, Math.max(1, step - 2), tall);
  }
  ctx.strokeStyle = ink.line;
  ctx.beginPath();
  ctx.moveTo(pad, floor);
  ctx.lineTo(w - pad, floor);
  ctx.stroke();
  ctx.font = `11px ${mono}`;
  ctx.fillStyle = ink.dim;
  for (let k = 0; k < power.length; k += 4) ctx.fillText(k, pad + k * step + step / 2 - 3, h - 6);
  ctx.fillStyle = ink.gold;
  ctx.fillText(`order 0 carries ${(power[0] / power.reduce((a, b) => a + b, 0) * 100).toFixed(1)}% of the power`, pad, 14);
  ctx.fillStyle = ink.pink;
  const tag = order ? `live orders are multiples of ${order}` : 'no live order';
  ctx.fillText(tag, w - pad - ctx.measureText(tag).width, 14);
}

function build() {
  const copies = +$('copies').value;
  $('copies-out').textContent = copies;
  $('levels-out').textContent = $('levels').value;
  const full = $('full').checked;
  $('step').disabled = full;
  if (full) $('step').value = +(360 / copies).toFixed(3);
  const step = +$('step').value;
  say('note');
  try {
    const { field, size, name } = src.read();
    const stack = m.radial(field, size, OUT, copies, step, $('blend').value, +$('samples').value);
    blit($('stack'), m.sheet(stack, OUT, $('ramp').value, +$('levels').value, $('invert').checked));
    const power = m.harmonics(field, size, RINGS, ORDERS);
    const order = m.turns(power);
    $('name').textContent = name;
    $('side').textContent = size;
    $('copies-stat').textContent = copies;
    $('step-stat').textContent = `${step}°`;
    $('order').textContent = order || 'round';
    $('petals').textContent = full ? (order ? m.petals(copies, order) : 'none') : 'partial orbit';
    $('stack-note').textContent = `${copies} ${copies === 1 ? 'copy' : 'copies'}, ${$('blend').value}`;
    chart(Array.from(power), copies, full, order);
  } catch (error) {
    say('note', error);
  }
}

for (const id of ['copies', 'step', 'full', 'blend', 'samples', 'ramp', 'levels', 'invert']) {
  $(id).oninput = build;
}
addEventListener('resize', () => {
  if (plot) chart(...plot);
});
build();
