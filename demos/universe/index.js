import { ready, $, ink, paint, say } from '../lib/mrly.js';
import { query, share } from '../lib/query.js';
import { seeds, cap } from '../lib/select.js';

const m = await ready();
$('counts').textContent = m.counting_sequence(4).join(', ');
$('base3').textContent = m.baseq_sequence(3, 2).join(', ');
const colors = [ink.blue, ink.orange, ink.gold, ink.green, ink.pink];
let dimension = query([]).get('d') === '3' ? 3 : 2, chosen = null;
const s = seeds();

function thumb(design) {
  if (dimension === 2) {
    const canvas = document.createElement('canvas');
    paint(canvas, m.two_grid(design.code, 3, 2, 0, 2), colors[design.degree % colors.length]);
    return canvas;
  }
  const box = document.createElement('div');
  try {
    box.innerHTML = m.hex_svg(design.code, 3, 1, 2, 'iso', 6);
  } catch {
    box.innerHTML = '<svg viewBox="0 0 1 1"></svg>';
  }
  return box;
}

function show() {
  const universe = JSON.parse(m.universe(dimension));
  $('summary').textContent = `${universe.distinct} designs from ${universe.total} codes`;
  const cards = $('cards');
  cards.replaceChildren();
  for (const design of universe.designs) {
    const card = document.createElement('div');
    card.className = 'card';
    card.append(thumb(design));
    card.insertAdjacentHTML('beforeend', `<div class="code">${design.code}</div><div class="name">${design.anf}</div>`);
    card.onclick = () => {
      for (const other of cards.children) other.classList.remove('on');
      card.classList.add('on');
      chosen = design;
      grow();
    };
    cards.append(card);
    if (design.code === (dimension === 2 ? '7' : '23')) card.onclick();
  }
}

function grow() {
  const design = chosen;
  const level = cap('level', 3, dimension, 60000);
  $('grow').style.display = '';
  $('grow-name').textContent = design.name;
  $('open').style.display = dimension === 3 ? '' : 'none';
  $('open').href = `../sponge${share({ code: design.code, number: 3, base: 2 })}`;
  $('orbit').textContent = design.orbit;
  $('degree').textContent = design.degree;
  $('anf').textContent = design.anf;
  $('fills').textContent = `${m.fills(design.code, 3, dimension, level, 2)} of ${m.grid_total(3, dimension, level)}`;
  $('ratio').textContent = m.ratio(design.code, 3, dimension, level, 2).toFixed(4);
  $('dimension').textContent = m.dimension(design.code, 3, dimension, 2).toFixed(4);
  $('grown').style.display = dimension === 2 ? '' : 'none';
  $('grown-svg').replaceChildren();
  if (dimension === 2) {
    paint($('grown'), m.two_grid(design.code, 3, level, 0, 2), colors[design.degree % colors.length]);
  } else {
    try {
      $('grown-svg').innerHTML = m.hex_svg(design.code, 3, level, 2, 'iso', level === 3 ? 2 : 8);
    } catch (error) {
      say('grown-svg', error);
    }
  }
}

for (const d of [2, 3]) {
  $(`d${d}`).onclick = () => {
    dimension = d;
    $('d2').classList.toggle('on', d === 2);
    $('d3').classList.toggle('on', d === 3);
    show();
  };
}
function roll(seed) {
  const cards = $('cards').children;
  cards[m.random_between(seed, [0], [cards.length - 1])[0]].onclick();
}

$('level').oninput = () => { if (chosen) grow(); };
$('random').onclick = () => roll(s.next());
$(`d${dimension}`).onclick();
if (s.get()) roll(s.get());
