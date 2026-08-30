import { ready, $, ink, blit, say, bind, out } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';

const m = await ready();
const SIZE = 768;
const LABELS = 1000;
const s = seeds();
const IDS = ['lattice', 'side', 'mark', 'a', 'b', 'c'];
const params = query(IDS);
if (params.has('faint')) $('faint').checked = params.get('faint') !== '0';
let look, pixels, centres = null, picked = null;

function shuffle(seed) {
  roll(seed, ['lattice', 'side', 'a', 'b', 'c']);
  $('c').value = m.prime_from(+$('c').value);
}

function read() {
  return {
    lattice: $('lattice').value,
    side: +$('side').value,
    mark: $('mark').value,
    a: +$('a').value,
    b: +$('b').value,
    c: +$('c').value,
    faint: $('faint').checked,
  };
}

function draw() {
  const canvas = $('sheet');
  blit(canvas, pixels);
  const ctx = canvas.getContext('2d');
  if (centres) {
    const mono = getComputedStyle(document.body).getPropertyValue('--mono');
    const px = Math.min(14, SIZE / look.side * 0.42);
    ctx.font = `${px}px ${mono}`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillStyle = ink.bg;
    for (let n = 1; 2 * n <= centres.length; n++) {
      const [x, y] = [centres[2 * n - 2], centres[2 * n - 1]];
      const light = pixels.rgba[(Math.floor(y) * SIZE + Math.floor(x)) * 4 + 1] > 100;
      ctx.fillStyle = light ? ink.bg : ink.dim;
      ctx.fillText(n, x, y + 1);
    }
  }
  if (picked) {
    ctx.strokeStyle = ink.fg;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(picked.px, picked.py, picked.span / 2 + 3, 0, Math.PI * 2);
    ctx.stroke();
  }
}

function pick(x, y) {
  if (!pixels) return;
  say('note');
  try {
    picked = JSON.parse(m.spiral_at(look.lattice, look.side, x, y, SIZE));
    const { n, ring, prime, factors } = picked;
    $('n').textContent = `${n} on ring ${ring} at ${picked.x}, ${picked.y}`;
    $('verdict').textContent = n === 1 ? 'one, neither prime nor composite' : prime ? 'prime' : 'composite';
    $('factors').textContent = factors.length ? factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : p)).join(' · ') : 'none';
    draw();
  } catch (error) {
    say('note', error);
  }
}

function build() {
  look = read();
  out('side', look.side);
  stamp({ ...Object.fromEntries(IDS.map((id) => [id, $(id).value])), faint: look.faint ? null : 0 });
  say('note');
  picked = null;
  for (const id of ['n', 'verdict', 'factors']) $(id).textContent = '';
  try {
    pixels = m.spiral_pixels(look.lattice, look.side, look.a, look.b, look.c, look.mark, look.faint, SIZE);
    const line = JSON.parse(m.spiral_polynomial(look.lattice, look.side, look.a, look.b, look.c));
    centres = line.top <= LABELS ? m.spiral_centers(look.lattice, look.side, SIZE) : null;
    draw();
    $('count').textContent = line.top;
    $('primes').textContent = line.primes;
    $('density').textContent = `${(line.density * 100).toFixed(2)}%`;
    $('hits').textContent = `${line.hits} of ${line.count}`;
    $('share').textContent = `${(line.share * 100).toFixed(1)}%`;
    $('streak').textContent = line.count === 0 ? 'off the sheet' : line.streak === line.count ? `all ${line.count} prime` : `${line.streak} primes, then ${line.values[line.streak]}`;
    const shown = line.values.slice(0, 8).join(' ') + (line.values.length > 8 ? ' …' : '');
    $('sheet-note').textContent = `${look.a} k² ${look.b < 0 ? '-' : '+'} ${Math.abs(look.b)} k ${look.c < 0 ? '-' : '+'} ${Math.abs(look.c)}: ${shown}`;
  } catch (error) {
    pixels = null;
    say('note', error);
  }
}

$('sheet').onclick = (event) => {
  const box = event.currentTarget.getBoundingClientRect();
  pick((event.clientX - box.left) * SIZE / box.width, (event.clientY - box.top) * SIZE / box.height);
};
bind(IDS.concat('faint'), build);
$('random').onclick = () => {
  shuffle(s.next());
  build();
};
if (s.get()) shuffle(s.get());
build();
if (params.has('click')) pick(...params.get('click').split(',').map(Number));
