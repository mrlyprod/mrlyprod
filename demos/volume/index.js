import { ready, $, ink, blit, say, bind, out } from '../lib/mrly.js';
import { stage, faces, plane } from '../lib/stage.js';
import { query } from '../lib/query.js';
import { picker, roll } from '../lib/select.js';
import { ramp } from '../lib/ramp.js';
import { cropper } from '../lib/crop.js';

const m = await ready();
const st = stage($('stage'));
const params = query(['camera', 'opacity']);
const tone = ramp($('ramp-row'), { levels: 16, on: draw });
const crop = cropper($('crop-row'), { dimension: 3, on: draw });
const src = picker({
  host: $('picker'), m, dimension: 3, base: [2, 3], code: '23', build: sample,
  extra: ['limit', 'combine', 'level', 'size'], more: (seed) => roll(seed, ['combine'], { combine: ['and'] }),
});

const NORMALS = { x: [1, 0, 0], y: [0, 1, 0], z: [0, 0, 1], d: [1, 1, 1] };
let data = null, size = 0, stats = null;

function draw() {
  say('note');
  if (!data) return;
  try {
    st.clear();
    const look = tone.read();
    const c = crop.read();
    const shown = c.active ? m.field_crop(data, size, 3, c.shape, c.rnum, c.rden, c.anti) : data;
    const threshold = +$('threshold').value, opacity = +$('opacity').value;
    out('threshold', threshold);
    out('opacity', opacity);
    $('count').textContent = m.volume_count(shown, size, threshold);
    $('faces').textContent = '';
    if ($('mesh').checked) {
      const quads = m.volume_surface(shown, size, threshold);
      if (quads > 400000) throw new Error(`${quads} faces is more than this page draws; raise the threshold or lower the size.`);
      $('faces').textContent = quads;
      st.add(faces(m.volume_faces(shown, size, threshold), ink.blue, opacity));
    }
    $('cut-note').textContent = '';
    for (const key of ['x', 'y', 'z', 'd']) {
      if (!$(`${key}-on`).checked) continue;
      const normal = NORMALS[key];
      const offset = +$(`${key}-at`).value;
      const frame = JSON.parse(m.plane_frame(normal, offset));
      const wide = key === 'd' ? 384 : 256;
      const pixels = m.paint_span(m.plane_field(shown, size, normal, offset, wide), wide, stats.min, stats.max, look.ramp, look.levels, look.invert);
      st.add(plane(pixels, frame));
      if (key === 'd') {
        blit($('cut'), pixels);
        $('cut-note').textContent = `diagonal at ${offset}`;
      }
    }
    $('solid-note').textContent = `${$('combine').value}, ${$('camera').value === 'iso' ? 'isometric' : 'perspective'}`;
  } catch (error) {
    st.clear();
    say('note', error);
  }
}

function sample() {
  say('note');
  const limit = +$('limit').value, level = +$('level').value;
  out('limit', limit);
  out('level', level);
  size = +$('size').value;
  try {
    const { code, base, name } = src.read();
    data = m.volume(code, base, limit, $('combine').value, level, size);
    stats = JSON.parse(m.volume_stats(data, size));
    const shape = JSON.parse(m.volume_shape(limit, size));
    $('name').textContent = name;
    $('layers').textContent = shape.layers;
    $('voxels').textContent = shape.voxels;
    $('range').textContent = `${stats.min} to ${stats.max}`;
    const was = +$('threshold').value === +$('threshold').max;
    $('threshold').max = stats.max;
    if (was || +$('threshold').value > stats.max) $('threshold').value = stats.max;
    draw();
  } catch (error) {
    data = null;
    st.clear();
    say('note', error);
  }
}

bind(['limit', 'combine', 'level', 'size'], sample);
bind(['mesh', 'threshold', 'opacity', 'x-on', 'x-at', 'y-on', 'y-at', 'z-on', 'z-at', 'd-on', 'd-at'], draw);
$('camera').onchange = () => {
  st.project($('camera').value);
  draw();
};
$('corner').onclick = () => st.view(1, 1, 1);
$('edge').onclick = () => st.view(1, 1, 0);
$('face').onclick = () => st.view(0, 0, 1);
$('spin').onchange = () => { st.spin = $('spin').checked ? 0.004 : 0; };
st.project($('camera').value);
if (params.get('camera') === 'iso') st.view(1, 1, 1);
sample();
