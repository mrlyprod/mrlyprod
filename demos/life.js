import { ready, $, ink, paint, say } from './mrly.js';

const m = await ready();
const W = 96, H = 96, BUDGET = 8;
let grid, generation = 0, running = false, seed = 1;

for (const id of ['birth-seq', 'survive-seq']) {
  const select = $(id);
  select.append(new Option('by hand', ''));
  for (const name of m.life_sequences()) select.append(new Option(name, name));
}

const parse = (text) => Uint32Array.from(text.replace(/\D/g, ''), (d) => +d);

function rule() {
  const birth = parse($('birth').value), survive = parse($('survive').value);
  $('rule').textContent = `B${Array.from(birth).join('')}/S${Array.from(survive).join('')}`;
  return [birth, survive, $('wrap').checked];
}

function draw() {
  paint($('sheet'), { width: W, height: H, types: grid }, ink.green);
  $('gen').textContent = `generation ${generation}`;
}

function reseed() {
  const choice = $('seed').value;
  generation = 0;
  $('verdict').textContent = '';
  if (choice === 'noise') {
    grid = m.life_noise(W, H, +$('density').value / 100, seed);
  } else if (choice === 'blank') {
    grid = new Uint8Array(W * H);
  } else {
    const cell = m.two_grid(choice, 3, 3, 0, 2);
    grid = new Uint8Array(W * H);
    const off = Math.floor((W - cell.width) / 2);
    for (let y = 0; y < cell.height; y++) {
      grid.set(cell.types.subarray(y * cell.width, (y + 1) * cell.width), (y + off) * W + off);
    }
  }
  draw();
}

function step() {
  try {
    const [birth, survive, wrap] = rule();
    grid = m.life_next(grid, W, H, birth, survive, wrap);
    generation += 1;
    draw();
    say('note');
  } catch (error) {
    running = false;
    say('note', error);
  }
}

function frame() {
  if (!running) return;
  step();
  setTimeout(() => requestAnimationFrame(frame), 40);
}

$('go').onclick = () => {
  running = !running;
  $('go').textContent = running ? 'Pause' : 'Play';
  if (running) frame();
};
$('step').onclick = step;
$('fate').onclick = () => {
  try {
    const [birth, survive, wrap] = rule();
    const run = JSON.parse(m.life_run(grid, W, H, birth, survive, wrap, 512));
    $('verdict').innerHTML = `fate <b>${run.fate}</b> after <b>${run.count}</b> generations` + (run.loop ? ` in a loop of <b>${run.loop}</b>` : '');
  } catch (error) {
    say('note', error);
  }
};
$('reseed').onclick = () => { seed += 1; reseed(); };
$('seed').onchange = reseed;
$('density').oninput = () => { $('density-out').textContent = (+$('density').value / 100).toFixed(2); reseed(); };
$('sheet').onclick = (event) => {
  const box = $('sheet').getBoundingClientRect();
  const x = Math.floor((event.clientX - box.left) / box.width * W), y = Math.floor((event.clientY - box.top) / box.height * H);
  grid[y * W + x] ^= 1;
  draw();
};
for (const side of ['birth', 'survive']) {
  $(`${side}-seq`).onchange = () => {
    const name = $(`${side}-seq`).value;
    if (name) $(side).value = Array.from(m.life_sequence(name, BUDGET)).join('');
    rule();
  };
  $(side).oninput = () => { $(`${side}-seq`).value = ''; rule(); };
}
$('wrap').onchange = rule;
rule();
reseed();
