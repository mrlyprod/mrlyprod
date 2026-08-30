import { ready, $, ink, say, paint, bind, out } from '../lib/mrly.js';
import { query } from '../lib/query.js';
import { picker } from '../lib/select.js';
import { board, line, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
query(['object', 'side', 'operator', 'window']);
const src = picker({
  host: $('picker'), m, dimension: 2, code: '7', extra: ['number', 'level'],
  build: () => {
    $('object').value = 'code';
    build();
  },
  more: () => {
    $('object').value = 'code';
  },
});
const serve = 1100;

function plan() {
  const object = $('object').value;
  for (const [id, live] of [['pick', object === 'code'], ['code', object === 'code'], ['number', object === 'code'],
    ['side', object === 'solid'], ['level', object !== 'solid']]) {
    $(id).disabled = !live;
  }
  const side = +$('side').value;
  const number = JSON.parse(m.slice_series('255', side))[side - 1].n;
  out('side', `n ${number}`);
  if (object === 'solid') return { kind: 'slice', code: '255', number, cap: 1 };
  if (object === 'carpet') return { kind: 'slice', code: '23', number: 3, cap: 2 };
  if (object === 'triangle') return { kind: 'flat', code: '7', number: 2, cap: m.fill_cap('7', 2, 2, 2, serve) };
  const { code } = src.read();
  const tile = +$('number').value;
  return { kind: 'flat', code, number: tile, cap: m.fill_cap(code, tile, 2, 2, serve) };
}

const chart = keep((data) => {
  const b = board($('idos'), 300, { left: 52, right: 14, top: 14, bottom: 26 });
  const steps = data.stair;
  if (steps.length < 2) return;
  const xs = steps.map((p) => Math.log(p[0]));
  const ys = steps.map((p) => Math.log(p[1]));
  const x0 = xs[0], x1 = xs.at(-1), y0 = ys[0];
  const fx = (x) => (x1 === x0 ? 0.5 : (x - x0) / (x1 - x0));
  const fy = (y) => (y0 === 0 ? 0.5 : (y - y0) / -y0);
  if (data.fitted) {
    b.ctx.fillStyle = '#151b22';
    b.ctx.fillRect(b.x(0), b.roof, b.x(fx(xs[data.fitted - 1])) - b.x(0), b.tall);
  }
  axis(b, [[0, steps[0][0].toExponential(2)], [1, steps.at(-1)[0].toFixed(4)]], { wall: true });
  const stair = [[0, fy(ys[0])]];
  for (let i = 1; i < steps.length; i++) stair.push([fx(xs[i]), fy(ys[i - 1])], [fx(xs[i]), fy(ys[i])]);
  line(b, stair, ink.blue, { width: 1.6 });
  if (data.fit) {
    const [intercept, slope] = data.fit;
    const seg = (a, c) => [[fx(a), fy(intercept + slope * a)], [fx(c), fy(intercept + slope * c)]];
    line(b, seg(x0, x1), ink.gold, { width: 1.2, dash: [3, 4] });
    line(b, seg(x0, xs[data.fitted - 1]), ink.gold, { width: 2.2 });
  }
  tag(b, '1', ink.dim, 'right', b.x(0) - 6, b.y(fy(0)) + 4);
  tag(b, steps[0][1].toFixed(4), ink.dim, 'right', b.x(0) - 6, b.y(fy(y0)) + 4);
});

function build() {
  const percent = +$('window').value;
  const share = percent / 100;
  out('window', `${percent}%`);
  const normalised = $('operator').value === '1';
  say('note');
  try {
    const spec = plan();
    const level = Math.min(Math.max(1, +$('level').value), spec.cap);
    $('level').max = spec.cap;
    $('level').value = level;
    out('level', `${level} of ${spec.cap}`);
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
      $('hex').innerHTML = m.hex_svg(spec.code, spec.number, level, 2, 'cut', Math.max(2, Math.round(300 / spec.number ** level)));
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

bind(['object', 'number', 'side', 'level', 'operator', 'window'], build);
build();
