import { ready, $, ink, fit, say, out } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, line, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const s = seeds();
const IDS = ['t', 'speed', 'zeros', 'x'];
const DENSITY = 30;
const LISTED = 8;
query(IDS);
const REACH = +$('t').max;
const zeros = m.zeta_zeros(+$('zeros').max);
const [join, seam] = m.zeta_seam(REACH, 1000);
let path = new Float64Array(0);
let head = 0;
let playing = false;
let last = 0;

function narrow(seed, spans) {
  const saved = Object.entries(spans).map(([id, [lo, hi]]) => {
    const input = $(id), was = [input.min, input.max];
    [input.min, input.max] = [lo, hi];
    return [input, was];
  });
  roll(seed, Object.keys(spans));
  for (const [input, [lo, hi]] of saved) [input.min, input.max] = [lo, hi];
}
if (s.get()) narrow(s.get(), { t: [10, 150], zeros: [1, 60], x: [50, 500] });

const walk = keep((t, count) => {
  const canvas = $('path');
  const [ctx, w, h] = fit(canvas, canvas.clientWidth);
  ctx.fillStyle = ink.deep;
  ctx.fillRect(0, 0, w, h);
  let reach = 2;
  for (let k = 0; k < path.length; k += 4) reach = Math.max(reach, Math.abs(path[k + 1]), Math.abs(path[k + 2]));
  const scale = (Math.min(w, h) / 2 - 14) / reach;
  const px = (re) => w / 2 + re * scale, py = (im) => h / 2 - im * scale;
  ctx.strokeStyle = ink.line;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(0, py(0));
  ctx.lineTo(w, py(0));
  ctx.moveTo(px(0), 0);
  ctx.lineTo(px(0), h);
  ctx.stroke();
  ctx.setLineDash([3, 5]);
  ctx.beginPath();
  ctx.arc(px(0), py(0), scale, 0, Math.PI * 2);
  ctx.stroke();
  ctx.setLineDash([]);
  const trace = (from, color, width) => {
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.beginPath();
    for (let k = from; k < path.length; k += 4) {
      if (k === from) ctx.moveTo(px(path[k + 1]), py(path[k + 2]));
      else ctx.lineTo(px(path[k + 1]), py(path[k + 2]));
    }
    ctx.stroke();
  };
  if (path.length) {
    trace(0, ink.blue, 1.2);
    trace(Math.max(0, path.length - 4 * 2 * DENSITY), ink.fg, 2);
  }
  ctx.strokeStyle = ink.gold;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(px(0), py(0), 5, 0, Math.PI * 2);
  ctx.stroke();
  if (path.length) {
    ctx.fillStyle = ink.orange;
    ctx.beginPath();
    ctx.arc(px(path.at(-3)), py(path.at(-2)), 4, 0, Math.PI * 2);
    ctx.fill();
  }
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  ctx.font = `11px ${mono}`;
  ctx.fillStyle = ink.dim;
  ctx.fillText('1', px(1) + 4, py(0) - 5);
  ctx.fillText('i', px(0) + 5, py(1) - 4);
  $('walk-note').textContent = `t = ${t.toFixed(2)}, ${count} ${count === 1 ? 'pass' : 'passes'} through the origin`;
});

const chart = keep((t, count) => {
  const b = board($('bars'), 220);
  const span = Math.max(t, 1);
  let peak = 1e-9;
  for (let k = 3; k < path.length; k += 4) peak = Math.max(peak, Math.abs(path[k]));
  b.ctx.strokeStyle = ink.line;
  b.ctx.beginPath();
  b.ctx.moveTo(b.x(0), b.y(0.5));
  b.ctx.lineTo(b.x(1), b.y(0.5));
  b.ctx.stroke();
  b.ctx.strokeStyle = ink.gold;
  for (const zero of zeros) {
    if (zero > t) break;
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(zero / span), b.floor);
    b.ctx.lineTo(b.x(zero / span), b.floor - 10);
    b.ctx.stroke();
  }
  const points = [];
  for (let k = 0; k < path.length; k += 4) points.push([path[k] / span, 0.5 + 0.5 * path[k + 3] / peak]);
  if (points.length > 1) line(b, points, ink.blue);
  axis(b, [[0, '0'], [1, `t = ${t.toFixed(2)}`]]);
  tag(b, 'Z(t), real on the line', ink.blue);
  tag(b, `${count} ${count === 1 ? 'zero' : 'zeros'}`, ink.gold, 'right');
});

const stairs = keep((x, k) => {
  const b = board($('stair'), 260);
  const stair = m.psi_stair(x);
  const some = m.psi_formula(x, zeros.subarray(0, k), 500);
  const none = m.psi_formula(x, zeros.subarray(0, 0), 500);
  let peak = stair.at(-1);
  for (let i = 1; i < some.length; i += 2) peak = Math.max(peak, some[i]);
  peak *= 1.04;
  const fx = (u) => (u - 1) / (x - 1);
  const steps = [];
  for (let n = 1; n <= x; n++) {
    steps.push([fx(n), stair[n - 1] / peak]);
    if (n < x) steps.push([fx(n + 1), stair[n - 1] / peak]);
  }
  const curve = (flat) => {
    const pts = [];
    for (let i = 0; i < flat.length; i += 2) pts.push([fx(flat[i]), flat[i + 1] / peak]);
    return pts;
  };
  b.ctx.save();
  b.ctx.beginPath();
  b.ctx.rect(b.left, b.roof - 4, b.wide, b.tall + 4);
  b.ctx.clip();
  line(b, curve(none), ink.pink, { dash: [4, 4], width: 1 });
  line(b, curve(some), ink.blue);
  line(b, steps, ink.gold, { width: 2 });
  b.ctx.restore();
  axis(b, [[0, '1'], [1, `x = ${x}`]]);
  let at = tag(b, 'psi(x)', ink.gold);
  at = tag(b, `formula with ${k} ${k === 1 ? 'zero' : 'zeros'}`, ink.blue, 'left', at + 14);
  tag(b, 'no zeros', ink.pink, 'left', at + 14);
  $('psi').textContent = stair.at(-1).toFixed(4);
  $('formula').textContent = some.at(-1).toFixed(4);
  $('gap').textContent = m.psi_gap(x, zeros.subarray(0, k)).toFixed(4);
  $('stair-note').textContent = `${k} of ${zeros.length} zeros folded in`;
});

function show(t) {
  const [re, im, z, theta] = m.zeta_at(t);
  const count = m.zeta_count(t);
  $('re').textContent = re.toFixed(4);
  $('im').textContent = im.toFixed(4);
  $('z').textContent = z.toFixed(4);
  $('theta').textContent = theta.toFixed(4);
  $('count').textContent = count;
  const next = zeros.find((zero) => zero > t);
  $('next').textContent = next === undefined ? 'past the list' : next.toFixed(6);
  $('list').textContent = Array.from(zeros.subarray(0, LISTED), (zero, i) => `${String(i + 1).padStart(2)}  ${zero.toFixed(6)}${zero <= t ? '  passed' : ''}`).join('\n');
  walk(t, count);
  chart(t, count);
}

function trace() {
  head = +$('t').value;
  out('t', head.toFixed(2));
  say('note');
  try {
    path = m.zeta_line(0, head, Math.max(1, Math.ceil(head * DENSITY)));
    show(head);
  } catch (error) {
    say('note', error);
  }
}

function fold() {
  const k = +$('zeros').value, x = +$('x').value;
  out('zeros', k);
  out('x', x);
  say('note');
  try {
    stairs(x, k);
  } catch (error) {
    say('note', error);
  }
}

function settle() {
  stamp({ t: head.toFixed(2), speed: $('speed').value, zeros: $('zeros').value, x: $('x').value });
}

function stop() {
  playing = false;
  $('play').textContent = 'Play';
  settle();
}

function frame(now) {
  if (!playing) return;
  requestAnimationFrame(frame);
  const dt = last ? Math.min(0.1, (now - last) / 1000) : 0;
  last = now;
  if (!dt) return;
  const to = Math.min(REACH, head + +$('speed').value * dt);
  try {
    const grown = m.zeta_line(head, to, Math.max(1, Math.ceil((to - head) * DENSITY)));
    const longer = new Float64Array(path.length + grown.length - 4);
    longer.set(path);
    longer.set(grown.subarray(4), path.length);
    path = longer;
    head = to;
    $('t').value = to;
    out('t', to.toFixed(2));
    show(to);
  } catch (error) {
    say('note', error);
    stop();
  }
  if (to >= REACH) stop();
}

$('join').textContent = join;
$('seam').textContent = seam.toExponential(1);
$('t').oninput = () => {
  trace();
  settle();
};
$('speed').oninput = () => {
  out('speed', $('speed').value);
  settle();
};
$('zeros').oninput = () => {
  fold();
  settle();
};
$('x').oninput = () => {
  fold();
  settle();
};
$('play').onclick = () => {
  if (playing) {
    stop();
    return;
  }
  if (head >= REACH) {
    $('t').value = 0;
    trace();
  }
  playing = true;
  last = 0;
  $('play').textContent = 'Pause';
  requestAnimationFrame(frame);
};
$('random').onclick = () => {
  stop();
  narrow(s.next(), { t: [10, 150], zeros: [1, 60], x: [50, 500] });
  trace();
  fold();
  settle();
};
out('speed', $('speed').value);
trace();
fold();
settle();
