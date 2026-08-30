import { ready, $, ink, rgb, out } from '../lib/mrly.js';
import { seeds } from '../lib/select.js';

const m = await ready();
const NUMBER = 3, LEVEL = 4, BASE = 3, WALKERS = 300, TICKS = 12, SCALE = 6;
const s = seeds();
let target = 20;
const SITE = [46, 54, 64], HOLE = rgb(ink.deep);

class Side {
  constructor(code, id, color) {
    this.code = code;
    this.id = id;
    this.color = rgb(color);
    this.canvas = $(`sheet-${id}`);
    $(`name-${id}`).textContent = m.name_of(code, 2, BASE);
    $(`fill-${id}`).textContent = `${m.fills(code, NUMBER, 2, LEVEL, BASE)} of ${m.grid_total(NUMBER, 2, LEVEL)}`;
  }
  reset(seed) {
    this.race = new m.Race(this.code, NUMBER, LEVEL, BASE, WALKERS, seed);
    this.side = this.race.side();
    this.types = this.race.types();
    this.canvas.width = this.canvas.height = this.side * SCALE;
    this.off = document.createElement('canvas');
    this.off.width = this.off.height = this.side;
    this.image = new ImageData(this.side, this.side);
    this.far = 0;
  }
  tick() {
    this.far = this.race.step(1);
  }
  draw() {
    const n = this.side, px = this.image.data, trail = this.race.trail(), [r, g, b] = this.color;
    for (let i = 0; i < n * n; i++) {
      const base = this.types[i] ? SITE : HOLE;
      const heat = Math.min(1, trail[i] / 12) * 0.55;
      px[i * 4] = base[0] + (r - base[0]) * heat;
      px[i * 4 + 1] = base[1] + (g - base[1]) * heat;
      px[i * 4 + 2] = base[2] + (b - base[2]) * heat;
      px[i * 4 + 3] = 255;
    }
    for (const p of this.race.positions()) {
      px.set([r, g, b, 255], p * 4);
    }
    this.off.getContext('2d').putImageData(this.image, 0, 0);
    const ctx = this.canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(this.off, 0, 0, n * SCALE, n * SCALE);
    const home = this.race.home();
    ctx.strokeStyle = ink.fg;
    ctx.lineWidth = 2;
    ctx.strokeRect((home % n) * SCALE - 2, Math.floor(home / n) * SCALE - 2, SCALE + 4, SCALE + 4);
    $(`far-${this.id}`).textContent = this.far.toFixed(1) + ' cells';
    $(`bar-${this.id}`).style.width = Math.min(100, this.far / target * 100) + '%';
  }
}

$('dim').textContent = m.dimension('127', NUMBER, 2, BASE).toFixed(3);
const A = new Side('127', 'a', ink.blue);
const B = new Side('239', 'b', ink.orange);
let running = false, done = false;

function resetAll() {
  running = false;
  done = false;
  target = +$('target').value;
  out('target', target);
  A.reset(s.get() || 1);
  B.reset((s.get() || 1) + 777);
  A.draw();
  B.draw();
  $('steps').textContent = '';
  $('banner').textContent = '';
  $('go').textContent = 'Start the race';
}

function frame() {
  if (!running) return;
  for (let k = 0; k < TICKS && !done; k++) {
    A.tick();
    B.tick();
    if (A.far >= target || B.far >= target) {
      done = true;
      running = false;
      const [winner, loser] = A.far >= target ? [A, B] : [B, A];
      $('banner').textContent = `${$(`name-${winner.id}`).textContent} wins at step ${winner.race.steps()}; the other team is at ${loser.far.toFixed(1)} cells. Same mass. Different music.`;
      $('go').textContent = 'Race again';
    }
  }
  A.draw();
  B.draw();
  $('steps').textContent = A.race.steps() + ' steps';
  if (running) requestAnimationFrame(frame);
}

$('go').onclick = () => {
  if (done) {
    s.next();
    resetAll();
  }
  running = !running;
  $('go').textContent = running ? 'Pause' : 'Resume';
  if (running) requestAnimationFrame(frame);
};
$('random').onclick = () => {
  s.next();
  resetAll();
  running = false;
  $('go').onclick();
};
$('target').onchange = resetAll;
resetAll();
