import { ready, $, blit, say, bind, out } from '../lib/mrly.js';
import { ramp } from '../lib/ramp.js';
import { seeds, roll } from '../lib/select.js';

const m = await ready();
for (const name of m.moire_names()) $('preset').append(new Option(name, name));
const tone = ramp($('ramp-row'), { levels: 16, on: render });
const s = seeds();
let playing = false;

function render() {
  const limit = +$('limit').value, size = +$('size').value;
  out('limit', limit);
  const look = tone.read();
  try {
    blit($('sheet'), m.moire($('preset').value, limit, size, look.ramp, look.levels, look.invert));
    $('scales').textContent = Array.from(m.odd_scales(limit)).join(' ');
    $('pixels').textContent = `${size} by ${size}`;
    say('note');
  } catch (error) {
    say('note', error);
  }
}

function play() {
  if (!playing) return;
  const limit = $('limit');
  if (limit.value === limit.max) limit.value = limit.min;
  else limit.stepUp();
  render();
  setTimeout(play, 350);
}

bind(['preset', 'limit', 'size'], render);
$('random').onclick = () => {
  roll(s.next(), ['preset', 'limit']);
  render();
};
if (s.get()) roll(s.get(), ['preset', 'limit']);
$('play').onclick = () => {
  playing = !playing;
  $('play').textContent = playing ? 'Stop' : 'Play the scales';
  play();
};
render();
