import { ready, $, ink, say, fit, paint } from './mrly.js';

const m = await ready();
const query = new URLSearchParams(location.search);
for (const key of ['object', 'code', 'number', 'side', 'level', 'operator', 'window']) {
  if (query.has(key)) $(key).value = query.get(key);
}

const serve = 1100;
let plot = null;

function capOf(code, number) {
  let level = 1;
  while (level < 9) {
    let next = 0;
    try {
      next = +m.fills(code, number, 2, level + 1, 2);
    } catch {
      break;
    }
    if (!(next > 0) || next > serve) break;
    level += 1;
  }
  return level;
}

function plan() {
  const object = $('object').value;
  for (const [id, live] of [['code', object === 'code'], ['number', object === 'code'],
    ['side', object === 'solid'], ['level', object !== 'solid']]) {
    $(id).disabled = !live;
  }
  const side = +$('side').value;
  const number = 2 * side - 1;
  $('side-out').textContent = `n ${number}`;
  if (object === 'solid') return { kind: 'slice', code: '255', number, cap: 1 };
  if (object === 'carpet') return { kind: 'slice', code: '23', number: 3, cap: 2 };
  if (object === 'triangle') return { kind: 'flat', code: '7', number: 2, cap: capOf('7', 2) };
  const code = $('code').value.trim();
  const tile = +$('number').value;
  return { kind: 'flat', code, number: tile, cap: capOf(code, tile) };
}

function chart(data) {
  plot = data;
  const canvas = $('idos');
  const [ctx, w, h] = fit(canvas, 300);
  ctx.clearRect(0, 0, w, h);
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  const steps = data.stair;
  if (steps.length < 2) return;
  const left = 52, right = 14, roof = 14, floor = h - 26;
  const xs = steps.map((p) => Math.log(p[0]));
  const ys = steps.map((p) => Math.log(p[1]));
  const x0 = xs[0], x1 = xs[xs.length - 1];
  const y0 = ys[0], y1 = 0;
  const px = (x) => left + (w - left - right) * (x1 === x0 ? 0.5 : (x - x0) / (x1 - x0));
  const py = (y) => floor - (floor - roof) * (y1 === y0 ? 0.5 : (y - y0) / (y1 - y0));
  if (data.fitted) {
    ctx.fillStyle = '#151b22';
    ctx.fillRect(px(x0), roof, px(xs[data.fitted - 1]) - px(x0), floor - roof);
  }
  ctx.strokeStyle = ink.line;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(left, roof);
  ctx.lineTo(left, floor);
  ctx.lineTo(w - right, floor);
  ctx.stroke();
  ctx.strokeStyle = ink.blue;
  ctx.lineWidth = 1.6;
  ctx.beginPath();
  ctx.moveTo(px(xs[0]), py(ys[0]));
  for (let i = 1; i < steps.length; i++) {
    ctx.lineTo(px(xs[i]), py(ys[i - 1]));
    ctx.lineTo(px(xs[i]), py(ys[i]));
  }
  ctx.stroke();
  if (data.fit) {
    const [intercept, slope] = data.fit;
    const line = (a, b, dash) => {
      ctx.setLineDash(dash);
      ctx.beginPath();
      ctx.moveTo(px(a), py(intercept + slope * a));
      ctx.lineTo(px(b), py(intercept + slope * b));
      ctx.stroke();
    };
    ctx.strokeStyle = ink.gold;
    ctx.lineWidth = 1.2;
    line(x0, x1, [3, 4]);
    ctx.lineWidth = 2.2;
    line(x0, xs[data.fitted - 1], []);
    ctx.setLineDash([]);
  }
  ctx.font = `11px ${mono}`;
  ctx.fillStyle = ink.dim;
  ctx.textAlign = 'right';
  ctx.fillText('1', left - 6, py(0) + 4);
  ctx.fillText(Math.exp(y0).toFixed(4), left - 6, py(y0) + 4);
  ctx.textAlign = 'left';
  ctx.fillText(steps[0][0].toExponential(2), left, h - 8);
  ctx.textAlign = 'right';
  ctx.fillText(steps[steps.length - 1][0].toFixed(4), w - right, h - 8);
  ctx.textAlign = 'left';
}

function build() {
  const spec = plan();
  const level = Math.min(Math.max(1, +$('level').value), spec.cap);
  $('level').max = spec.cap;
  $('level').value = level;
  $('level-out').textContent = `${level} of ${spec.cap}`;
  const percent = +$('window').value;
  const share = percent / 100;
  $('window-out').textContent = `${percent}%`;
  const normalised = $('operator').value === '1';
  say('note');
  try {
    const data = JSON.parse(m.spectrum(spec.kind, spec.code, spec.number, level, normalised, share));
    for (const key of ['nodes', 'edges', 'components', 'distinct', 'classes', 'repeated', 'one']) {
      $(key).textContent = data[key];
    }
    $('pair').textContent = data.pair.join(' and ');
    $('exponent').textContent = data.exponent === null ? 'none' : data.exponent.toFixed(4);
    $('top').textContent = data.top
      .map(([value, size]) => `${value.toFixed(10).padStart(14)}   x${size}`)
      .join('\n');
    $('axes').textContent = `log rank fraction against log eigenvalue, ${normalised ? 'normalised' : 'combinatorial'}`;
    $('legend').innerHTML = `<span>staircase <b style="color:${ink.blue}">${data.distinct} distinct</b></span>`
      + `<span>shaded low window <b>${percent}%</b></span>`
      + `<span>fitted slope <b style="color:${ink.gold}">${data.fit ? data.fit[1].toFixed(4) : 'none'}</b></span>`
      + `<span>d_s = 2 x slope <b>${data.exponent === null ? 'none' : data.exponent.toFixed(3)}</b></span>`;
    $('what').textContent = `${m.name_of(spec.code, spec.kind === 'flat' ? 2 : 3, 2)}, level ${level}`;
    const slice = spec.kind === 'slice';
    $('art').style.display = slice ? 'none' : 'block';
    $('hex').style.display = slice ? 'block' : 'none';
    if (slice) {
      const side = Math.pow(spec.number, level);
      $('hex').innerHTML = m.hex_svg(spec.code, spec.number, level, 2, 'cut', Math.max(2, Math.round(300 / side)));
    } else {
      $('hex').innerHTML = '';
      paint($('art'), m.two_grid(spec.code, spec.number, level, 0, 2), ink.blue);
    }
    chart(data);
  } catch (error) {
    $('hex').innerHTML = '';
    $('top').textContent = '';
    $('legend').innerHTML = '';
    say('note', error);
  }
}

for (const id of ['object', 'code', 'number', 'side', 'level', 'operator', 'window']) {
  $(id).oninput = build;
}
addEventListener('resize', () => {
  if (plot) chart(plot);
});
build();
