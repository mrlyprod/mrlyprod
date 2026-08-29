import { ready, $, ink, say, fit } from './mrly.js';

const m = await ready();
const query = new URLSearchParams(location.search);
for (const key of ['code', 'k', 'level']) {
  if (query.has(key)) $(key).value = query.get(key);
}

const pick = $('pick');
pick.append(new Option('type a code', ''));
for (const [word, code] of [['carpet', '23'], ['net', '232'], ['tree', '3'], ['antipodal', '129'], ['solid', '255']]) {
  pick.append(new Option(`${word} · ${code}`, code));
}
const known = (code) => (pick.querySelector(`option[value="${code}"]`) ? code : '');
pick.value = known($('code').value.trim());
pick.onchange = () => {
  if (pick.value) {
    $('code').value = pick.value;
    build();
  }
};

const pad = 14;
const top = 16;
let plot = null;

function chart(rows, here) {
  plot = [rows, here];
  const canvas = $('bars');
  const [ctx, w, h] = fit(canvas, 220);
  ctx.clearRect(0, 0, w, h);
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  const floor = h - 22, roof = 26;
  const wide = w - 2 * pad, step = wide / rows.length;
  const peak = rows.reduce((a, r) => Math.max(a, r.fills), 1);
  const crest = rows.reduce((a, r) => Math.max(a, r.components, r.holes), 1);
  for (let i = 0; i < rows.length; i++) {
    const tall = (floor - roof) * rows[i].fills / peak;
    ctx.fillStyle = rows[i].k === here ? ink.gold : ink.blue;
    ctx.fillRect(pad + i * step + 1, floor - tall, Math.max(1, step - 2), tall);
  }
  const at = (i, value) => [pad + (i + 0.5) * step, floor - (floor - roof) * value / crest];
  for (const [key, color] of [['components', ink.green], ['holes', ink.pink]]) {
    ctx.strokeStyle = color;
    ctx.fillStyle = color;
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    rows.forEach((row, i) => {
      const [x, y] = at(i, row[key]);
      if (i) ctx.lineTo(x, y); else ctx.moveTo(x, y);
    });
    ctx.stroke();
    rows.forEach((row, i) => {
      const [x, y] = at(i, row[key]);
      ctx.beginPath();
      ctx.arc(x, y, 2.5, 0, Math.PI * 2);
      ctx.fill();
    });
  }
  ctx.strokeStyle = ink.line;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(pad, floor);
  ctx.lineTo(w - pad, floor);
  ctx.stroke();
  ctx.font = `11px ${mono}`;
  ctx.fillStyle = ink.dim;
  rows.forEach((row, i) => ctx.fillText(row.k, pad + i * step + step / 2 - 3, h - 6));
  ctx.fillStyle = ink.blue;
  ctx.fillText(`filled triangles, peak ${peak}`, pad, 14);
  ctx.fillStyle = ink.green;
  ctx.fillText('pieces', pad + 170, 14);
  ctx.fillStyle = ink.pink;
  ctx.fillText(`holes, peak ${crest}`, pad + 224, 14);
}

function build() {
  const code = $('code').value.trim();
  const fractal = +$('fractal').value;
  const cap = fractal === 0 ? 1 : fractal === 3 ? 4 : 2;
  $('level').max = cap;
  $('level').disabled = fractal === 0;
  $('k').disabled = fractal !== 0;
  const level = Math.min(+$('level').value, cap);
  $('level').value = level;
  $('level-out').textContent = level;
  const k = Math.min(top, Math.max(1, +$('k').value));
  const number = fractal === 0 ? 2 * k - 1 : fractal;
  $('k-out').textContent = fractal === 0 ? `k ${k}, n ${number}` : `tile ${number}`;
  pick.value = known(code);
  say('note');
  try {
    const tally = JSON.parse(m.slice_census(code, number, level, 2));
    $('name').textContent = m.name_of(code, 3, 2);
    for (const key of ['side', 'triangles', 'boundary', 'edges', 'interior', 'vertices', 'euler', 'fills', 'voids', 'components', 'holes', 'giant']) {
      $(key).textContent = tally[key];
    }
    $('closed').textContent = tally.closed.triangles;
    if (level === 1) {
      const carpet = JSON.parse(m.slice_census('23', number, 1, 2)).fills;
      const net = JSON.parse(m.slice_census('232', number, 1, 2)).fills;
      const whole = carpet + net === tally.triangles;
      $('split').innerHTML = `<span>carpet <b>${carpet}</b></span><span>net <b>${net}</b></span>`
        + `<span>together <b>${carpet + net}</b></span><span>hexagon <b>${tally.triangles}</b></span>`
        + `<span>partition <b>${whole ? 'exact' : 'broken'}</b></span>`;
    } else {
      $('split').innerHTML = '';
    }
    chart(JSON.parse(m.slice_series(code, top)), fractal === 0 ? k : 0);
    const art = m.hex_svg(code, number, level, 2, 'cut', Math.max(1, Math.round(360 / tally.side)));
    if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    $('art').innerHTML = art;
  } catch (error) {
    $('art').innerHTML = '';
    $('split').innerHTML = '';
    say('note', error);
  }
}

function seek(event) {
  const box = $('bars').getBoundingClientRect();
  const frac = (event.clientX - box.left - pad) / (box.width - 2 * pad);
  $('fractal').value = '0';
  $('k').value = Math.min(top, Math.max(1, Math.floor(frac * top) + 1));
  build();
}

$('bars').onpointerdown = (event) => {
  $('bars').setPointerCapture(event.pointerId);
  seek(event);
};
$('bars').onpointermove = (event) => {
  if (event.buttons) seek(event);
};
for (const id of ['code', 'k', 'level', 'fractal']) {
  $(id).oninput = build;
}
addEventListener('resize', () => {
  if (plot) chart(...plot);
});
build();
