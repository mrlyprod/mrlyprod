import { ready, $, ink, fit, say, out } from '../lib/mrly.js';
import { query, stamp } from '../lib/query.js';
import { seeds, roll } from '../lib/select.js';
import { board, bars, line, axis, tag, keep } from '../lib/chart.js';

const m = await ready();
const s = seeds();
const IDS = ['n', 'limit', 'top', 'detect'];
const steps = +(query(IDS).get('steps') ?? 0);
if (s.get()) roll(s.get(), IDS);
let sieve, current = 0, playing = false;

const sheet = keep(() => {
  const types = sieve.types(), limit = types.length - 1, mark = sieve.rank() + 1;
  const canvas = $('sheet');
  const cols = limit > 100 ? 20 : 10, rows = Math.ceil(limit / cols);
  const cell = canvas.clientWidth / cols;
  const [ctx, w, h] = fit(canvas, Math.ceil(rows * cell));
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  ctx.fillStyle = ink.deep;
  ctx.fillRect(0, 0, w, h);
  ctx.font = `${Math.min(13, cell * 0.42)}px ${mono}`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  for (let n = 1; n <= limit; n++) {
    const t = types[n], x = ((n - 1) % cols) * cell, y = Math.floor((n - 1) / cols) * cell;
    const lit = n === current || t === mark;
    ctx.fillStyle = n === current ? ink.blue : t === mark ? ink.orange : t === 1 ? ink.gold : t ? ink.line : ink.panel;
    ctx.fillRect(x + 1, y + 1, cell - 2, cell - 2);
    if (cell >= 15) {
      ctx.fillStyle = lit || t === 1 ? ink.bg : ink.dim;
      ctx.fillText(n, x + cell / 2, y + cell / 2 + 1);
    }
  }
  const done = sieve.done();
  $('p').textContent = current || (done ? 'none left' : 'none yet');
  $('struck').textContent = sieve.struck();
  $('found').textContent = sieve.count();
  $('sieve-note').textContent = done ? `done, ${sieve.count()} primes in gold` : current ? `${current} strikes its multiples in orange` : 'blue is the prime in hand';
});

const stones = keep((pile) => {
  const canvas = $('rects'), rects = pile.rectangles, n = pile.n;
  const width = canvas.clientWidth, few = n <= 60;
  const stone = few ? (width - 80) / n : 0;
  const rise = few ? rects.reduce((sum, [a]) => sum + a * stone + 8, 0) : 240;
  const [ctx, w, h] = fit(canvas, Math.max(Math.ceil(rise), 60));
  ctx.fillStyle = ink.deep;
  ctx.fillRect(0, 0, w, h);
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  ctx.font = `11px ${mono}`;
  if (few) {
    let y = 4;
    for (const [a, b] of rects) {
      ctx.fillStyle = rects.length === 1 ? ink.gold : ink.blue;
      for (let i = 0; i < a; i++) {
        for (let j = 0; j < b; j++) {
          ctx.beginPath();
          ctx.arc((j + 0.5) * stone, y + (i + 0.5) * stone, stone * 0.36, 0, Math.PI * 2);
          ctx.fill();
        }
      }
      ctx.fillStyle = ink.fg;
      ctx.textAlign = 'right';
      ctx.fillText(`${a} by ${b}`, w - 4, y + a * stone / 2 + 4);
      y += a * stone + 8;
    }
    return;
  }
  const span = Math.log(n);
  const px = (v) => 8 + (w - 16) * Math.log(v) / span;
  const py = (v) => h - 20 - (h - 36) * Math.log(v) / span;
  rects.forEach(([a, b], k) => {
    const x = px(b), y = Math.min(py(a), h - 24);
    ctx.fillStyle = rects.length === 1 ? ink.gold : ink.blue;
    ctx.globalAlpha = 0.18;
    ctx.fillRect(8, y, x - 8, h - 20 - y);
    ctx.globalAlpha = 1;
    ctx.strokeStyle = ctx.fillStyle;
    ctx.strokeRect(8.5, y + 0.5, x - 8, h - 20 - y);
    if (k === rects.length - 1 || k === 0) {
      ctx.fillStyle = ink.fg;
      ctx.textAlign = k ? 'right' : 'left';
      ctx.fillText(`${a} by ${b}`, k ? x - 4 : 12, y - 5);
    }
  });
  ctx.fillStyle = ink.dim;
  ctx.textAlign = 'left';
  ctx.fillText('1', 8, h - 6);
  ctx.textAlign = 'right';
  ctx.fillText(`${n} stones, sides on a log scale`, w - 8, h - 6);
});

const chart = keep((data) => {
  const b = board($('bars'), 220);
  const top = data.x.at(-1), last = data.x.length - 1;
  const peak = Math.max(data.li.at(-1), data.pi.at(-1), data.ratio.at(-1));
  const trace = (column) => column.map((v, k) => [data.x[k] / top, Math.max(0, v) / peak]);
  line(b, trace(data.ratio), ink.pink, { dash: [4, 4] });
  line(b, trace(data.li), ink.blue);
  line(b, trace(data.pi), ink.gold, { width: 2 });
  axis(b, [[0, '0'], [1, String(top)]]);
  let x = tag(b, `pi(x) ${data.pi[last]}`, ink.gold);
  x = tag(b, `x / ln x ${data.ratio[last].toFixed(1)}`, ink.pink, 'left', x + 14);
  tag(b, `li(x) ${data.li[last].toFixed(1)}`, ink.blue, 'left', x + 14);
});

const witness = keep((trial) => {
  const b = board($('witness'), 220);
  const count = trial.scales.length;
  bars(b, trial.row, { color: ink.gold });
  const every = Math.max(1, Math.round(count / 8));
  axis(b, trial.scales.map((scale, k) => [(k + 0.5) / count, scale]).filter((_, k) => k % every === 0));
  if (trial.prime) tag(b, `${trial.n}: every bar is exactly zero, prime`, ink.green);
  else tag(b, `${trial.n}: largest ${trial.max.toFixed(4)} at scale ${trial.at}`, ink.gold);
});

function reset() {
  const limit = +$('limit').value;
  out('limit', limit);
  playing = false;
  $('play').textContent = 'Play';
  current = 0;
  try {
    sieve = new m.Sieve(limit);
    sheet();
    say('note');
  } catch (error) {
    say('note', error);
  }
}

function step() {
  if (!sieve || sieve.done()) return;
  current = sieve.step();
  sheet();
}

function tick() {
  if (!playing) return;
  step();
  if (sieve.done()) {
    playing = false;
    $('play').textContent = 'Play';
    return;
  }
  setTimeout(() => requestAnimationFrame(tick), 600);
}

function build() {
  const n = $('n').value.trim(), top = +$('top').value, detect = +$('detect').value;
  out('detect', detect);
  stamp({ n, top, detect, limit: $('limit').value });
  say('note');
  try {
    const pile = JSON.parse(m.factor(n));
    stones(pile);
    $('factors').textContent = pile.factors.length ? pile.factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : p)).join(' · ') : 'none';
    $('verdict').textContent = pile.prime ? `${pile.n} is prime, one row only` : `${pile.n} makes ${pile.rectangles.length} rectangles`;
    $('stones-note').textContent = pile.rectangles.slice(0, 8).map(([a, b]) => `${a}×${b}`).join(' ') + (pile.rectangles.length > 8 ? ' …' : '');
    const data = JSON.parse(m.prime_chart(top, 400));
    chart(data);
    $('pi').textContent = data.pi.at(-1);
    $('guess').textContent = data.ratio.at(-1).toFixed(1);
    $('li').textContent = data.li.at(-1).toFixed(1);
    const trial = JSON.parse(m.carpet_witness(detect));
    witness(trial);
    $('max-corr').textContent = trial.max.toFixed(7);
    $('at').textContent = trial.at || 'nowhere';
  } catch (error) {
    say('note', error);
  }
}

$('limit').oninput = reset;
$('step').onclick = () => {
  playing = false;
  $('play').textContent = 'Play';
  step();
};
$('play').onclick = () => {
  if (sieve.done()) reset();
  playing = !playing;
  $('play').textContent = playing ? 'Pause' : 'Play';
  tick();
};
$('reset').onclick = reset;
for (const id of ['n', 'top', 'detect']) $(id).oninput = build;
$('random').onclick = () => {
  roll(s.next(), IDS);
  reset();
  build();
};
reset();
for (let k = 0; k < steps; k++) step();
build();
