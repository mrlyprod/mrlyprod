import { ready, $, ink, say, stage, faces, cubes } from './mrly.js';

const m = await ready();
const st = stage($('stage'));
const query = new URLSearchParams(location.search);
for (const key of ['code', 'number', 'base']) {
  if (query.has(key)) $(key).value = query.get(key);
}

const pick = $('pick');
pick.append(new Option('type a code', ''));
for (const design of JSON.parse(m.universe(3)).designs) {
  pick.append(new Option(`${design.code} · ${design.anf}`, design.code));
}
pick.onchange = () => {
  if (pick.value) {
    $('code').value = pick.value;
    $('base').value = 2;
    build();
  }
};

function build() {
  const code = $('code').value.trim(), number = +$('number').value, base = +$('base').value;
  const top = Math.max(1, Math.floor(Math.log(128) / Math.log(number)));
  $('level').max = top;
  const level = Math.min(+$('level').value, top);
  $('level').value = level;
  $('level-out').textContent = level;
  $('opacity-out').textContent = $('opacity').value;
  const side = number ** level;
  say('note');
  try {
    $('name').textContent = m.name_of(code, 3, base);
    $('side').textContent = side;
    const fills = m.fills(code, number, 3, level, base);
    const surface = m.three_surface(code, number, level, base);
    $('fills').textContent = fills;
    $('voids').textContent = m.voids(code, number, 3, level, base);
    $('surface').textContent = surface;
    $('ratio').textContent = m.ratio(code, number, 3, level, base).toFixed(4);
    $('dimension').textContent = m.dimension(code, number, 3, base).toFixed(4);
    $('topology').textContent = '';
    if (side ** 3 <= 30000) {
      const tally = JSON.parse(m.three_census(code, number, level, base));
      $('topology').innerHTML = `vertices <b>${tally.vertices}</b> edges <b>${tally.edges}</b> faces <b>${tally.faces}</b> euler <b>${tally.euler}</b>`;
    }
    if ($('view').value === 'shell') {
      if (Number(surface) > 400000) throw new Error(`${surface} faces is more than this page draws; lower the level.`);
      st.show(faces(m.three_faces(code, number, level, base), ink.blue, +$('opacity').value));
    } else {
      if (Number(fills) > 250000) throw new Error(`${fills} cubes is more than this page draws; lower the level.`);
      st.show(cubes(m.three_cells(code, number, level, base), side, ink.orange));
    }
  } catch (error) {
    st.clear();
    say('note', error);
  }
}

for (const id of ['code', 'number', 'base', 'level', 'view', 'opacity']) {
  $(id).oninput = build;
}
$('spin').onchange = () => { st.spin = $('spin').checked ? 0.004 : 0; };
st.spin = 0.004;
build();
