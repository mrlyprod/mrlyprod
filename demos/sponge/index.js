import { ready, $, ink, say, bind, out } from '../lib/mrly.js';
import { stage, faces, cubes } from '../lib/stage.js';
import { picker, cap } from '../lib/select.js';
import { cropper } from '../lib/crop.js';

const m = await ready();
const st = stage($('stage'));
const src = picker({ host: $('picker'), m, dimension: 3, base: [2, 3], code: '23', build, extra: ['number', 'level'] });
const crop = cropper($('crop-row'), { dimension: 3, on: build });

function build() {
  const number = +$('number').value;
  const level = cap('level', number, 1, 128);
  out('opacity', $('opacity').value);
  say('note');
  try {
    const { code, base, name } = src.read();
    const c = crop.read();
    const side = m.grid_total(number, 1, level);
    $('name').textContent = name;
    $('side').textContent = side;
    const fills = m.fills(code, number, 3, level, base);
    const surface = m.three_surface(code, number, level, base);
    $('fills').textContent = fills;
    $('voids').textContent = m.voids(code, number, 3, level, base);
    $('surface').textContent = surface;
    $('ratio').textContent = m.ratio(code, number, 3, level, base).toFixed(4);
    $('dimension').textContent = m.dimension(code, number, 3, base).toFixed(4);
    $('topology').textContent = '';
    if (Number(m.grid_total(number, 3, level)) <= 30000) {
      const tally = JSON.parse(m.three_census(code, number, level, base));
      $('topology').innerHTML = `vertices <b>${tally.vertices}</b> edges <b>${tally.edges}</b> faces <b>${tally.faces}</b> euler <b>${tally.euler}</b>`;
    }
    if ($('view').value === 'shell') {
      if (Number(surface) > 400000) throw new Error(`${surface} faces is more than this page draws; lower the level.`);
      const mesh = c.active ? m.crop_faces(code, number, level, base, c.shape, c.rnum, c.rden, c.anti, 'touching') : m.three_faces(code, number, level, base);
      if (c.active && mesh[0] / 36 > 400000) throw new Error(`${mesh[0] / 36} faces is more than this page draws; lower the level.`);
      st.show(faces(mesh, ink.blue, +$('opacity').value));
    } else {
      if (Number(fills) > 250000) throw new Error(`${fills} cubes is more than this page draws; lower the level.`);
      const cells = c.active ? m.crop_cells(code, number, level, base, c.shape, c.rnum, c.rden, c.anti, 'touching') : m.three_cells(code, number, level, base);
      st.show(cubes(cells, Number(side), ink.orange));
    }
  } catch (error) {
    st.clear();
    say('note', error);
  }
}

bind(['number', 'level', 'view', 'opacity'], build);
$('spin').onchange = () => { st.spin = $('spin').checked ? 0.004 : 0; };
st.spin = 0.004;
build();
