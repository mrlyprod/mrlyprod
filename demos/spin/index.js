import { ready, $, ink, blit, paint, say, out } from '../lib/mrly.js';
import { query } from '../lib/query.js';
import { sources, roll } from '../lib/select.js';
import { ramp } from '../lib/ramp.js';
import { board, line, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const STEPS = 512;
const SIZE = 512;
const src = sources(m, $('sources'), build, (seed) => roll(seed, ['rpm']));
const tone = ramp($('ramp-row'), { levels: 64, on: build });
query(['rpm']);
for (const [word, rpm] of [['33', 33], ['45', 45], ['78', 78], ['899', 899], ['900', 900]]) {
  const button = document.createElement('button');
  button.textContent = word;
  button.onclick = () => {
    $('rpm').value = rpm;
    out('rpm', rpm);
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

const chart = keep((profile, stats) => {
  const b = board($('bars'), 200);
  const peak = Math.max(stats.peak, 1e-9);
  const low = Math.min(0, ...profile);
  const last = profile.length - 1;
  line(b, Array.from(profile, (v, k) => [k / last, (v - low) / (peak - low)]), ink.blue, { fill: 0.25 });
  const mark = (r, color, label, dx) => {
    const at = b.x(r / stats.reach);
    b.ctx.strokeStyle = color;
    b.ctx.lineWidth = 1;
    b.ctx.setLineDash([3, 3]);
    b.ctx.beginPath();
    b.ctx.moveTo(at, b.roof - 4);
    b.ctx.lineTo(at, b.floor);
    b.ctx.stroke();
    b.ctx.setLineDash([]);
    tag(b, label, color, 'left', at + dx);
  };
  if (stats.disc > 0) mark(stats.disc, ink.pink, `dark disc ${stats.disc.toFixed(2)}`, 4);
  mark(stats.inner, ink.gold, `edge ${stats.inner.toFixed(1)}`, -60);
  axis(b, [[0, '0'], [1, `radius in cells, corner ${stats.reach.toFixed(1)}`]]);
  tag(b, `circle mean, peak ${stats.peak.toFixed(3)}`, ink.blue);
});

function build() {
  say('note');
  try {
    const look = tone.read();
    const { grid, field, size, name, fills } = src.read();
    const profile = m.profile(field, size, STEPS);
    $('name').textContent = name;
    $('fills').textContent = fills;
    const side = size;
    if (grid) paint(raw, grid, ink.blue, ink.deep);
    else blit(raw, m.sheet(field, size, look.ramp, look.levels, look.invert));
    const dctx = disc.getContext('2d');
    dctx.imageSmoothingEnabled = false;
    dctx.drawImage(raw, 0, 0, SIZE, SIZE);
    blit($('wheel'), m.wheel(profile, SIZE, look.ramp, look.levels, look.invert));
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
  out('rpm', rpm);
  const glow = +$('glow').value;
  out('glow', glow);
  if (playing) angle = (angle + rpm * 6 * dt) % 360;
  $('strobe').textContent = `${fps.toFixed(0)} fps, ${m.frame_step(rpm, fps).toFixed(1)}° per frame`;
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

$('play').onclick = () => {
  playing = !playing;
  $('play').textContent = playing ? 'Stop' : 'Spin';
};
build();
requestAnimationFrame(frame);
