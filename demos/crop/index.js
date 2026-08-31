import * as THREE from 'three';
import { ready, $, ink, paint, say, bind, out } from '../lib/mrly.js';
import { stage, faces } from '../lib/stage.js';
import { query, stamp } from '../lib/query.js';
import { picker, cap } from '../lib/select.js';
import { board, bars, line, axis, tag, keep, seek } from '../lib/chart.js';

const m = await ready();
const st = stage($('stage'));
st.renderer.localClippingEnabled = true;

const RDEN = 120;
const RMAX = 108;
const SIGNS = [];
for (let bits = 0; bits < 8; bits++) SIGNS.push([1 - 2 * (bits & 1), 1 - 2 * ((bits >> 1) & 1), 1 - 2 * ((bits >> 2) & 1)]);
const WALLS = {
  box: [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]],
  diamond: SIGNS,
  octahedron: SIGNS,
  tetrahedron: [[1, 1, 1], [-1, -1, 1], [-1, 1, -1], [1, -1, -1]],
  pyramid: [[1, 0, 0], [-1, -2, 0], [-1, 2, 0], [-1, 0, -2], [-1, 0, 2]],
};

const pad = 14;
const dim = () => +$('dim').value;
let src = null;
let cache = { key: '', rows: null };

function planes(shape, r) {
  return WALLS[shape].map((n) => {
    const size = Math.hypot(...n);
    return new THREE.Plane(new THREE.Vector3(-n[0], -n[1], -n[2]).divideScalar(size), (2 * r) / size);
  });
}

function tune() {
  const d = dim();
  const shapes = $('shape'), was = shapes.value;
  shapes.innerHTML = '';
  for (const name of JSON.parse(m.crop_shapes(d))) shapes.append(new Option(name));
  shapes.value = [...shapes.options].some((o) => o.value === was) ? was : 'ball';
  const policy = $('policy'), had = policy.value;
  const list = d === 2 ? ['inside', 'touching', 'refined1', 'refined2'] : ['inside', 'touching'];
  policy.innerHTML = '';
  for (const name of list) policy.append(new Option(name));
  policy.value = list.includes(had) ? had : 'touching';
  $('solids').hidden = d === 2;
}

function mount() {
  src = picker({ host: $('picker'), m, dimension: dim(), base: [2, 3], code: dim() === 2 ? '7' : '23', build, extra: ['number', 'level'] });
}

const radiusChart = keep((rows, frac) => {
  const b = board($('rbars'), 170, { pad, top: 16, bottom: 20 });
  const peak = Math.max(...rows.map((r) => Math.max(r.filled_in, r.filled_cut)), 1);
  line(b, rows.map((r) => [r.x, r.filled_in / peak]), ink.gold);
  line(b, rows.map((r) => [r.x, r.filled_cut / peak]), ink.blue);
  axis(b, [[0, '0'], [1, 'radius 1']]);
  b.ctx.strokeStyle = ink.pink;
  b.ctx.beginPath();
  b.ctx.moveTo(b.x(frac), b.roof);
  b.ctx.lineTo(b.x(frac), b.floor);
  b.ctx.stroke();
  const edge = tag(b, 'in', ink.gold);
  tag(b, 'cut', ink.blue, 'left', edge + 12);
});

const levelChart = keep((rows, level) => {
  const b = board($('lbars'), 170, { pad, top: 16, bottom: 20 });
  const logs = rows.map((r) => Math.log10(1 + r.filled_in));
  bars(b, logs, { color: (i) => (i === level ? ink.pink : ink.gold), inset: 2 });
  axis(b, [[0, 'level 0'], [1, String(rows.length - 1)]]);
});

function build() {
  say('note');
  try {
    const d = dim();
    const { code, base, name } = src.read();
    const number = +$('number').value;
    const level = cap('level', number, 1, d === 2 ? 243 : 81);
    const rnum = +$('radius').value;
    const shape = $('shape').value;
    const anti = $('mode').value === 'anti';
    const policy = $('policy').value;
    out('radius', `${rnum}/${RDEN}`);
    stamp({ dim: d, shape, radius: rnum, mode: $('mode').value, policy });
    const census = JSON.parse(m.crop_census(code, number, level, base, d, shape, rnum, RDEN, anti));
    $('name').textContent = name;
    $('side').textContent = m.grid_total(number, 1, level);
    $('f-in').textContent = census.filled_in;
    $('f-cut').textContent = census.filled_cut;
    $('f-out').textContent = census.filled_out;
    $('e-before').textContent = census.exposed_before;
    $('e-after').textContent = census.exposed_after;
    const crisp = $('crisp').checked;
    const solid = d === 3;
    const cut = solid && crisp && !anti && WALLS[shape];
    $('flat').hidden = solid || crisp;
    $('art').hidden = solid || !crisp;
    $('stage').hidden = !solid;
    if (!solid) {
      st.clear();
      if (crisp) {
        const side = Number(m.grid_total(number, 1, level));
        const art = m.crop_svg(code, number, level, base, shape, rnum, RDEN, anti, Math.max(2, Math.round(512 / side)));
        if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
        $('art').innerHTML = art;
        $('view-note').textContent = 'touching cells under the exact outline';
      } else {
        $('art').innerHTML = '';
        paint($('flat'), m.crop_grid(code, number, level, base, shape, rnum, RDEN, anti, policy), ink.gold);
        $('view-note').textContent = policy;
      }
    } else {
      $('art').innerHTML = '';
      const load = Number(cut ? census.exposed_before : census.exposed_after);
      if (load > 400000) throw new Error(`${load} faces is more than this page draws; lower the level.`);
      const mesh = cut
        ? faces(m.three_faces(code, number, level, base), ink.blue)
        : faces(m.crop_faces(code, number, level, base, shape, rnum, RDEN, anti, policy), ink.blue);
      if (cut) mesh.material.clippingPlanes = planes(shape, rnum / RDEN);
      $('spin').disabled = !!cut;
      if (cut) {
        $('spin').checked = false;
        st.spin = 0;
      }
      st.show(mesh);
      $('view-note').textContent = cut ? 'the exact walls clip the full mesh' : policy;
    }
    const steps = d === 2 ? 36 : 24;
    const key = `${code}:${base}:${number}:${level}:${d}:${shape}:${anti}`;
    if (cache.key !== key) {
      cache = { key, rows: JSON.parse(m.crop_series(code, number, level, base, d, shape, rnum, RDEN, anti, 'radius', steps)) };
    }
    const total = Number(census.filled_in) + Number(census.filled_cut) + Number(census.filled_out);
    radiusChart([{ x: 0, filled_in: anti ? total : 0, filled_cut: 0 }, ...cache.rows], rnum / RDEN);
    levelChart(JSON.parse(m.crop_series(code, number, level, base, d, shape, rnum, RDEN, anti, 'level', +$('level').max)), level);
  } catch (error) {
    st.clear();
    $('art').innerHTML = '';
    say('note', error);
  }
}

query(['dim']);
tune();
query(['shape', 'radius', 'mode', 'policy']);
mount();
$('dim').onchange = () => {
  stamp({ code: null, seed: null });
  tune();
  mount();
  build();
};
seek($('rbars'), (frac) => {
  $('radius').value = Math.min(RMAX, Math.max(0, Math.round(frac * RDEN)));
  build();
}, pad);
bind(['number', 'level', 'shape', 'radius', 'mode', 'policy', 'crisp'], build);
$('spin').onchange = () => { st.spin = $('spin').checked ? 0.004 : 0; };
build();
