import { ready, $, ink, blit, say, stage, faces, plane } from './mrly.js';

const m = await ready();
const st = stage($('stage'));
const query = new URLSearchParams(location.search);
for (const key of ['code', 'base', 'limit', 'combine', 'level', 'size', 'camera', 'opacity']) {
  if (query.has(key)) $(key).value = query.get(key);
}

const pick = $('pick');
pick.append(new Option('type a code', ''));
for (const [word, code] of [['low corner', '1'], ['sponge', '23'], ['net', '232'], ['octa', '126'], ['antipodal', '129']]) {
  pick.append(new Option(`${word} · ${code}`, code));
}
pick.onchange = () => {
  if (pick.value) {
    $('code').value = pick.value;
    $('base').value = 2;
    sample();
  }
};

const NORMALS = { x: [1, 0, 0], y: [0, 1, 0], z: [0, 0, 1], d: [1, 1, 1] };
let data = null, size = 0, stats = null;

function draw() {
  say('note');
  if (!data) return;
  try {
    st.clear();
    const ramp = $('ramp').value, levels = +$('levels').value, invert = $('invert').checked;
    const threshold = +$('threshold').value, opacity = +$('opacity').value;
    $('threshold-out').textContent = threshold;
    $('opacity-out').textContent = opacity;
    $('levels-out').textContent = levels;
    $('count').textContent = m.volume_count(data, size, threshold);
    $('faces').textContent = '';
    if ($('mesh').checked) {
      const buffer = m.volume_faces(data, size, threshold);
      const quads = buffer[0] / 36;
      if (quads > 400000) throw new Error(`${quads} faces is more than this page draws; raise the threshold or lower the size.`);
      $('faces').textContent = quads;
      st.add(faces(buffer, ink.blue, opacity));
    }
    $('cut-note').textContent = '';
    for (const key of ['x', 'y', 'z', 'd']) {
      if (!$(`${key}-on`).checked) continue;
      const normal = NORMALS[key];
      const offset = +$(`${key}-at`).value;
      const frame = JSON.parse(m.plane_frame(normal, offset));
      const out = key === 'd' ? 384 : 256;
      const pixels = m.paint_span(m.plane_field(data, size, normal, offset, out), out, stats.min, stats.max, ramp, levels, invert);
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
  const code = $('code').value.trim(), base = +$('base').value, limit = +$('limit').value;
  const level = +$('level').value;
  $('limit-out').textContent = limit;
  $('level-out').textContent = level;
  pick.value = pick.querySelector(`option[value="${code}"]`) && base === 2 ? code : '';
  size = +$('size').value;
  try {
    data = m.volume(code, base, limit, $('combine').value, level, size);
    stats = JSON.parse(m.volume_stats(data, size));
    const layers = Math.ceil(limit / 2);
    $('name').textContent = m.name_of(code, 3, base);
    $('layers').textContent = layers;
    $('voxels').textContent = size ** 3;
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

for (const id of ['code', 'base', 'limit', 'combine', 'level', 'size']) $(id).oninput = sample;
for (const id of ['mesh', 'threshold', 'opacity', 'x-on', 'x-at', 'y-on', 'y-at', 'z-on', 'z-at', 'd-on', 'd-at', 'ramp', 'levels', 'invert']) $(id).oninput = draw;
$('camera').onchange = () => {
  st.project($('camera').value);
  draw();
};
$('corner').onclick = () => st.view(1, 1, 1);
$('edge').onclick = () => st.view(1, 1, 0);
$('face').onclick = () => st.view(0, 0, 1);
$('spin').onchange = () => { st.spin = $('spin').checked ? 0.004 : 0; };
st.project($('camera').value);
if (query.get('camera') === 'iso') st.view(1, 1, 1);
sample();
