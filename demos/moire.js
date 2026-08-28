import { ready, $, blit, say } from './mrly.js';

const m = await ready();
for (const name of m.moire_names()) $('preset').append(new Option(name, name));
let playing = false;

function render() {
  const limit = +$('limit').value, size = +$('size').value;
  $('limit-out').textContent = limit;
  $('levels-out').textContent = $('levels').value;
  try {
    blit($('sheet'), m.moire($('preset').value, limit, size, $('ramp').value, +$('levels').value, $('invert').checked));
    const scales = [];
    for (let n = 1; n <= limit; n += 2) scales.push(n);
    $('scales').textContent = scales.join(' ');
    $('pixels').textContent = `${size} by ${size}`;
    say('note');
  } catch (error) {
    say('note', error);
  }
}

function play() {
  if (!playing) return;
  const next = +$('limit').value + 2;
  $('limit').value = next > +$('limit').max ? 1 : next;
  render();
  setTimeout(play, 350);
}

for (const id of ['preset', 'limit', 'size', 'ramp', 'levels', 'invert']) $(id).oninput = render;
$('play').onclick = () => {
  playing = !playing;
  $('play').textContent = playing ? 'Stop' : 'Play the scales';
  play();
};
render();
