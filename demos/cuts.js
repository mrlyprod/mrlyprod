import { ready, $, ink, say, fit } from './mrly.js';

const m = await ready();
const query = new URLSearchParams(location.search);
for (const key of ['code', 'level']) {
  if (query.has(key)) $(key).value = query.get(key);
}

const pick = $('pick');
pick.append(new Option('type a code', ''));
for (const code of ['63', '105', '111', '126', '127']) {
  pick.append(new Option(m.name_of(code, 3, 2), code));
}
pick.value = pick.querySelector(`option[value="${$('code').value.trim()}"]`) ? $('code').value.trim() : '';
pick.onchange = () => {
  if (pick.value) {
    $('code').value = pick.value;
    build();
  }
};

const pad = 12;
let span = [0, 0];
let stamp = '';
let plot = null;

function chart(counts, low, marks) {
  plot = [counts, low, marks];
  const canvas = $('bars');
  const [ctx, w, h] = fit(canvas, 180);
  ctx.clearRect(0, 0, w, h);
  const peak = counts.reduce((a, b) => (b > a ? b : a), 0);
  const floor = h - 20, wide = w - 2 * pad;
  const step = wide / counts.length;
  for (let i = 0; i < counts.length; i++) {
    const tall = peak ? (floor - 12) * counts[i] / peak : 0;
    ctx.fillStyle = marks.includes(low + i) ? ink.gold : ink.blue;
    ctx.fillRect(pad + i * step, floor - tall, Math.max(1, step - 1), tall);
  }
  ctx.strokeStyle = ink.line;
  ctx.beginPath();
  ctx.moveTo(pad, floor);
  ctx.lineTo(w - pad, floor);
  ctx.stroke();
  ctx.fillStyle = ink.dim;
  ctx.font = `11px ${getComputedStyle(document.body).getPropertyValue('--mono')}`;
  ctx.fillText(low, pad, h - 5);
  const last = String(low + counts.length - 1);
  ctx.fillText(last, w - pad - 6 * last.length, h - 5);
}

function build() {
  const code = $('code').value.trim();
  const view = $('view').value;
  const top = view === 'section' ? 6 : 9;
  $('level').max = top;
  const level = Math.min(+$('level').value, top);
  $('level').value = level;
  $('level-out').textContent = level;
  say('note');
  try {
    const cut = JSON.parse(m.diagonal_profile(code, 2, level, 2));
    const [low, high] = cut.support;
    span = [low, high];
    const mid = Math.floor((low + high) / 2);
    const slider = $('height');
    const mark = `${code}/${level}`;
    slider.min = low;
    slider.max = high;
    if (mark !== stamp) {
      stamp = mark;
      slider.value = mid;
    }
    slider.value = Math.min(high, Math.max(low, +slider.value));
    const both = $('both').checked;
    slider.disabled = both;
    const heights = both ? [mid, Math.min(high, mid + 1)] : [+slider.value];
    $('height-out').textContent = heights.join(' and ');
    $('name').textContent = m.name_of(code, 3, 2);
    $('side').textContent = cut.side;
    $('support').textContent = `[${low}, ${high}]`;
    $('live').textContent = `${cut.nonempty} of ${cut.heights}`;
    $('here').textContent = heights.map((s) => m.diagonal_count(code, 2, level, 2, s)).join(' and ');
    $('least').textContent = cut.min;
    $('most').textContent = cut.max;
    $('flat').textContent = cut.constant ? 'yes' : 'no';
    $('digits').textContent = heights.map((s) => (s - low).toString(2)).join(' and ');
    chart(cut.counts.map(Number), low, heights);
    const art = view === 'section'
      ? m.hex_svg(code, 2, level, 2, 'cut', Math.max(1, Math.round(256 / 2 ** level)))
      : m.diagonal_svg(code, 2, level, 2, heights, Math.max(1, Math.round(512 / 2 ** level)));
    if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    $('art').innerHTML = art;
    $('drawn').textContent = view === 'section' ? '-' : art.split('<circle').length - 1;
  } catch (error) {
    $('art').innerHTML = '';
    $('drawn').textContent = '';
    say('note', error);
  }
}

function seek(event) {
  if ($('both').checked) return;
  const box = $('bars').getBoundingClientRect();
  const frac = (event.clientX - box.left - pad) / (box.width - 2 * pad);
  const [low, high] = span;
  $('height').value = Math.min(high, Math.max(low, Math.round(low + frac * (high - low))));
  build();
}

$('bars').onpointerdown = (event) => {
  $('bars').setPointerCapture(event.pointerId);
  seek(event);
};
$('bars').onpointermove = (event) => {
  if (event.buttons) seek(event);
};
for (const id of ['code', 'level', 'height', 'view']) {
  $(id).oninput = build;
}
$('both').onchange = build;
addEventListener('resize', () => {
  if (plot) chart(...plot);
});
build();
