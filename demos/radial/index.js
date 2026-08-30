import { ready, $, ink, blit, say, bind, out } from '../lib/mrly.js';
import { query } from '../lib/query.js';
import { sources, roll } from '../lib/select.js';
import { ramp } from '../lib/ramp.js';
import { board, bars, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const OUT = 512;
const ORDERS = 48;
const RINGS = 160;
const src = sources(m, $('sources'), build, (seed) => {
  roll(seed, ['copies', 'blend'], { blend: ['meet'] });
  $('full').checked = true;
});
const tone = ramp($('ramp-row'), { levels: 64, on: build });
const params = query(['copies', 'step', 'blend']);
if (params.has('step')) $('full').checked = false;

const chart = keep((power, copies, full, order, share) => {
  const b = board($('bars'), 300);
  const lives = (k) => (full ? k % copies === 0 : k === 0);
  bars(b, power, { color: (k) => (k === 0 ? ink.gold : lives(k) ? ink.blue : ink.line) });
  axis(b, power.map((_, k) => [(k + 0.5) / power.length, k]).filter(([, k]) => k % 4 === 0));
  tag(b, `order 0 carries ${share.toFixed(1)}% of the power`, ink.gold);
  tag(b, order ? `live orders are multiples of ${order}` : 'no live order', ink.pink, 'right');
});

function build() {
  const copies = +$('copies').value;
  out('copies', copies);
  const full = $('full').checked;
  $('step').disabled = full;
  if (full) $('step').value = +m.full_turn(copies).toFixed(3);
  const step = +$('step').value;
  say('note');
  try {
    const look = tone.read();
    const { field, size, name } = src.read();
    const stack = m.radial(field, size, OUT, copies, step, $('blend').value, +$('samples').value);
    blit($('stack'), m.sheet(stack, OUT, look.ramp, look.levels, look.invert));
    const power = m.harmonics(field, size, RINGS, ORDERS);
    const order = m.turns(power);
    $('name').textContent = name;
    $('side').textContent = size;
    $('copies-stat').textContent = copies;
    $('step-stat').textContent = `${step}°`;
    $('order').textContent = order || 'round';
    $('petals').textContent = full ? (order ? m.petals(copies, order) : 'none') : 'partial orbit';
    $('stack-note').textContent = `${copies} ${copies === 1 ? 'copy' : 'copies'}, ${$('blend').value}`;
    chart(Array.from(power), copies, full, order, m.radial_share(power));
  } catch (error) {
    say('note', error);
  }
}

bind(['copies', 'step', 'full', 'blend', 'samples'], build);
build();
