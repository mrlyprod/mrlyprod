import { ready, $, ink, blit, say, bind, out } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, bars, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const SIZE = 768;
const NORMS = 60;
const s = seeds();
const IDS = ['ring', 'radius', 'colour'];
const params = query(IDS);
for (const id of ['units', 'composites']) if (params.has(id)) $(id).checked = params.get(id) !== '0';
let look, pixels, picked = null;

function shuffle(seed) {
  const range = $('radius');
  [range.min, range.max] = [20, 120];
  roll(seed, ['ring', 'radius', 'colour']);
  [range.min, range.max] = [5, 200];
}

function read() {
  return {
    ring: $('ring').value,
    radius: +$('radius').value,
    colour: $('colour').value,
    units: $('units').checked,
    faint: $('composites').checked,
  };
}

function name(a, b) {
  const unit = look.ring === 'gaussian' ? 'i' : 'ω';
  const size = Math.abs(b) === 1 ? '' : Math.abs(b);
  if (b === 0) return `${a}`;
  if (a === 0) return `${b < 0 ? '-' : ''}${size}${unit}`;
  return `${a} ${b < 0 ? '-' : '+'} ${size}${unit}`;
}

const chart = keep((weights, fates, [at, peak]) => {
  const b = board($('bars'), 220);
  const values = Array.from(weights).slice(1);
  const colour = (k) => [ink.dim, ink.blue, ink.orange, ink.pink][fates[k + 1]];
  bars(b, values, { color: colour });
  values.forEach((v, k) => {
    if (v || fates[k + 1] !== 2) return;
    b.ctx.fillStyle = ink.orange;
    b.ctx.fillRect(b.x(k / values.length) + 1, b.floor - 3, Math.max(1, b.wide / values.length - 2), 3);
  });
  axis(b, values.map((_, k) => [(k + 0.5) / values.length, k + 1]).filter(([, n]) => n % 10 === 0));
  tag(b, `peak r(${at}) = ${peak}`, ink.fg);
  tag(b, 'blue split · orange inert · pink ramified', ink.dim, 'right');
});

function draw() {
  const canvas = $('sheet');
  blit(canvas, pixels);
  if (!picked) return;
  const ctx = canvas.getContext('2d');
  const ring = (x, y, color, width, dash = []) => {
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.setLineDash(dash);
    ctx.beginPath();
    ctx.arc(x, y, picked.span / 2 + 3, 0, Math.PI * 2);
    ctx.stroke();
    ctx.setLineDash([]);
  };
  if (look.units && picked.norm > 1) {
    for (const [, , x, y] of picked.associates.slice(1)) ring(x, y, ink.fg, 1.5);
    const [, , x, y] = picked.conjugate;
    ring(x, y, ink.pink, 1.5, [4, 3]);
  }
  ring(picked.px, picked.py, ink.fg, 3);
}

function verdict(p) {
  const norm = `norm ${p.norm}`;
  const shown = p.factors.map(([q, e]) => (e > 1 ? `${q}^${e}` : q)).join(' · ');
  const [ca, cb] = p.conjugate;
  if (p.class === 'split') return `prime: ${p.norm} splits as (${name(p.a, p.b)})(${name(ca, cb)})`;
  if (p.class === 'inert') return `prime: ${p.factors[0][0]} stays prime in the plane`;
  if (p.class === 'ramified') return `prime: ${p.norm} ramifies, a unit times a square`;
  if (p.class === 'unit') return 'a unit, norm 1';
  if (p.class === 'zero') return 'the origin';
  return `composite, ${norm} = ${shown}`;
}

function pick(x, y) {
  if (!pixels) return;
  say('note');
  try {
    picked = JSON.parse(m.ring_at(look.ring, look.radius, x, y, SIZE));
    $('point').textContent = `${name(picked.a, picked.b)} at ${picked.a}, ${picked.b}`;
    $('norm').textContent = picked.norm;
    $('class').textContent = picked.class;
    $('verdict').textContent = verdict(picked);
    draw();
  } catch (error) {
    say('note', error);
  }
}

function build() {
  look = read();
  out('radius', look.radius);
  stamp({ ...Object.fromEntries(IDS.map((id) => [id, $(id).value])), units: look.units ? null : 0, composites: look.faint ? null : 0 });
  say('note');
  picked = null;
  for (const id of ['point', 'norm', 'class', 'verdict']) $(id).textContent = '';
  try {
    pixels = m.ring_pixels(look.ring, look.radius, look.colour, look.faint, SIZE);
    draw();
    const census = JSON.parse(m.ring_census(look.ring, look.radius));
    $('points').textContent = census.points;
    $('primes').textContent = census.primes;
    $('split').textContent = census.split;
    $('inert').textContent = census.inert;
    $('ramified').textContent = census.ramified;
    $('density').textContent = `${(census.density * 100).toFixed(2)}%`;
    $('symmetry').textContent = `${census.symmetry}-fold`;
    $('sheet-note').textContent = `${look.ring}, reach ${look.radius}, norms to ${census.top}, ${census.units} units`;
    $('bars-note').textContent = `norms 1 to ${NORMS}`;
    chart(m.ring_weights(look.ring, NORMS), m.ring_fates(look.ring, NORMS), m.ring_peak(look.ring, NORMS));
  } catch (error) {
    pixels = null;
    say('note', error);
  }
}

$('sheet').onclick = (event) => {
  const box = event.currentTarget.getBoundingClientRect();
  pick((event.clientX - box.left) * SIZE / box.width, (event.clientY - box.top) * SIZE / box.height);
};
bind(IDS.concat('units', 'composites'), build);
$('random').onclick = () => {
  shuffle(s.next());
  build();
};
if (s.get()) shuffle(s.get());
build();
if (params.has('click')) pick(...params.get('click').split(',').map(Number));
