import { ready, $, blit, say, bind, out } from '../lib/mrly.js';
import { ramp } from '../lib/ramp.js';
import { cropper } from '../lib/crop.js';
import { seeds, roll } from '../lib/select.js';

const m = await ready();
for (const name of m.moire_names()) $('preset').append(new Option(name, name));
const tone = ramp($('ramp-row'), { levels: 16, on: render });
const crop = cropper($('crop-row'), { dimension: 2, on: render });
const s = seeds();
let playing = false;

function render() {
  const limit = +$('limit').value, size = +$('size').value;
  out('limit', limit);
  const look = tone.read();
  try {
    const c = crop.read();
    if (c.active) {
      const field = m.field_crop(m.moire_field($('preset').value, limit, size), size, 2, c.shape, c.rnum, c.rden, c.anti);
      let lo = Infinity, hi = -Infinity;
      for (const v of field) if (!Number.isNaN(v)) { lo = Math.min(lo, v); hi = Math.max(hi, v); }
      blit($('sheet'), m.paint_span(field, size, lo, hi, look.ramp, look.levels, look.invert));
    } else {
      blit($('sheet'), m.moire($('preset').value, limit, size, look.ramp, look.levels, look.invert));
    }
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
