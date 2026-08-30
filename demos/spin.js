import { ready, $, ink, blit, paint, say, fit, sources } from './mrly.js';

const m = await ready();
const STEPS = 512;
const SIZE = 512;
const src = sources(m, $('sources'), build);
if (new URLSearchParams(location.search).has('rpm')) $('rpm').value = new URLSearchParams(location.search).get('rpm');
for (const [word, rpm] of [['33', 33], ['45', 45], ['78', 78], ['899', 899], ['900', 900]]) {
  const button = document.createElement('button');
  button.textContent = word;
  button.onclick = () => {
    $('rpm').value = rpm;
    $('rpm-out').textContent = rpm;
  };
  $('needles').append(button);
}

const disc = document.createElement('canvas');
disc.width = SIZE;
disc.height = SIZE;
const raw = document.createElement('canvas');
const table = $('table');
table.width = SIZE;
table.height = SIZE;
const tctx = table.getContext('2d');
let plot = null;

function chart(profile, stats) {
  plot = [profile, stats];
  const canvas = $('bars');
  const [ctx, w, h] = fit(canvas, 200);
  ctx.clearRect(0, 0, w, h);
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  const pad = 14, floor = h - 22, roof = 26;
  const wide = w - 2 * pad;
  const peak = Math.max(stats.peak, 1e-9);
  const low = Math.min(0, ...profile);
  const x = (k) => pad + wide * k / (profile.length - 1);
  const y = (v) => floor - (floor - roof) * (v - low) / (peak - low);
  ctx.fillStyle = ink.blue;
  ctx.globalAlpha = 0.25;
  ctx.beginPath();
  ctx.moveTo(x(0), y(low));
  profile.forEach((v, k) => ctx.lineTo(x(k), y(v)));
  ctx.lineTo(x(profile.length - 1), y(low));
  ctx.fill();
  ctx.globalAlpha = 1;
  ctx.strokeStyle = ink.blue;
  ctx.lineWidth = 1.5;
  ctx.beginPath();
  profile.forEach((v, k) => (k ? ctx.lineTo(x(k), y(v)) : ctx.moveTo(x(k), y(v))));
  ctx.stroke();
  const mark = (r, color, label, dx) => {
    const at = pad + wide * r / stats.reach;
    ctx.strokeStyle = color;
    ctx.lineWidth = 1;
    ctx.setLineDash([3, 3]);
    ctx.beginPath();
    ctx.moveTo(at, roof - 4);
    ctx.lineTo(at, floor);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = color;
    ctx.fillText(label, at + dx, 14);
  };
  ctx.font = `11px ${mono}`;
  if (stats.disc > 0) mark(stats.disc, ink.pink, `dark disc ${stats.disc.toFixed(2)}`, 4);
  mark(stats.inner, ink.gold, `edge ${stats.inner.toFixed(1)}`, -60);
  ctx.strokeStyle = ink.line;
  ctx.beginPath();
  ctx.moveTo(pad, floor);
  ctx.lineTo(w - pad, floor);
  ctx.stroke();
  ctx.fillStyle = ink.dim;
  ctx.fillText('0', pad, h - 6);
  ctx.fillText(`radius in cells, corner ${stats.reach.toFixed(1)}`, w - pad - 190, h - 6);
  ctx.fillStyle = ink.blue;
  ctx.fillText(`circle mean, peak ${stats.peak.toFixed(3)}`, pad, 14);
}

function build() {
  $('levels-out').textContent = $('levels').value;
  say('note');
  try {
    const ramp = $('ramp').value, levels = +$('levels').value, invert = $('invert').checked;
    const { grid, field, size, name, fills } = src.read();
    const profile = m.profile(field, size, STEPS);
    $('name').textContent = name;
    $('fills').textContent = fills;
    const side = size;
    if (grid) paint(raw, grid, ink.blue, ink.deep);
    else blit(raw, m.sheet(field, size, ramp, levels, invert));
    const dctx = disc.getContext('2d');
    dctx.imageSmoothingEnabled = false;
    dctx.drawImage(raw, 0, 0, SIZE, SIZE);
    blit($('wheel'), m.wheel(profile, SIZE, ramp, levels, invert));
    const stats = JSON.parse(m.spin_stats(profile, side));
    $('side').textContent = side;
    $('mass').textContent = stats.mass.toFixed(1);
    $('disc').textContent = stats.disc.toFixed(2);
    $('peak').textContent = stats.peak.toFixed(3);
    $('reach').textContent = stats.reach.toFixed(1);
    chart(profile, stats);
    tctx.globalAlpha = 1;
    tctx.fillStyle = ink.deep;
    tctx.fillRect(0, 0, SIZE, SIZE);
  } catch (error) {
    say('note', error);
  }
}

let angle = 0;
let last = 0;
let fps = 60;
let playing = true;

function frame(now) {
  requestAnimationFrame(frame);
  const dt = last ? Math.min(0.05, (now - last) / 1000) : 0;
  last = now;
  if (dt > 0) fps = fps * 0.95 + 0.05 / dt;
  const rpm = +$('rpm').value;
  $('rpm-out').textContent = rpm;
  const glow = +$('glow').value;
  $('glow-out').textContent = glow;
  if (playing) angle = (angle + rpm * 6 * dt) % 360;
  $('strobe').textContent = `${fps.toFixed(0)} fps, ${(rpm * 6 / fps).toFixed(1)}° per frame`;
  tctx.globalAlpha = 1 / glow;
  tctx.fillStyle = ink.deep;
  tctx.fillRect(0, 0, SIZE, SIZE);
  tctx.save();
  tctx.translate(SIZE / 2, SIZE / 2);
  tctx.rotate(angle * Math.PI / 180);
  const s = SIZE / Math.SQRT2;
  tctx.drawImage(disc, -s / 2, -s / 2, s, s);
  tctx.restore();
}

for (const id of ['ramp', 'levels', 'invert']) $(id).oninput = build;
$('play').onclick = () => {
  playing = !playing;
  $('play').textContent = playing ? 'Stop' : 'Spin';
};
addEventListener('resize', () => {
  if (plot) chart(...plot);
});
build();
requestAnimationFrame(frame);
