import { ready, $, ink, say, bind, out } from '../lib/mrly.js';
import { picker, cap } from '../lib/select.js';
import { board, bars, line, axis, tag, keep, seek } from '../lib/chart.js';

const m = await ready();
const src = picker({ host: $('picker'), m, dimension: 3, code: '23', build, extra: ['k', 'level'] });
const top = 16;

const chart = keep((rows, here) => {
  const b = board($('bars'), 220);
  const n = rows.length;
  const peak = rows.reduce((a, r) => Math.max(a, r.fills), 1);
  const crest = rows.reduce((a, r) => Math.max(a, r.components, r.holes), 1);
  bars(b, rows.map((r) => r.fills), { peak, color: (i) => (rows[i].k === here ? ink.gold : ink.blue) });
  for (const [key, color] of [['components', ink.green], ['holes', ink.pink]]) {
    line(b, rows.map((r, i) => [(i + 0.5) / n, r[key] / crest]), color, { dots: 2.5 });
  }
  axis(b, rows.map((r, i) => [(i + 0.5) / n, r.k]));
  const next = tag(b, `filled triangles, peak ${peak}`, ink.blue);
  tag(b, `holes, peak ${crest}`, ink.pink, 'left', tag(b, 'pieces', ink.green, 'left', next + 16) + 16);
});

function build() {
  const fractal = +$('fractal').value;
  $('level').disabled = fractal === 0;
  $('k').disabled = fractal !== 0;
  const level = fractal ? cap('level', fractal, 1, 100) : cap('level', 1, 1, 1);
  const k = Math.min(top, Math.max(1, +$('k').value));
  say('note');
  try {
    const { code, name } = src.read();
    const rows = JSON.parse(m.slice_series(code, top));
    const number = fractal || rows[k - 1].n;
    out('k', fractal ? `tile ${number}` : `k ${k}, n ${number}`);
    const tally = JSON.parse(m.slice_census(code, number, level, 2));
    $('name').textContent = name;
    for (const key of ['side', 'triangles', 'boundary', 'edges', 'interior', 'vertices', 'euler', 'fills', 'voids', 'components', 'holes', 'giant']) {
      $(key).textContent = tally[key];
    }
    $('closed').textContent = tally.closed.triangles;
    if (level === 1) {
      const split = JSON.parse(m.slice_partition(number));
      $('split').innerHTML = `<span>carpet <b>${split.carpet}</b></span><span>net <b>${split.net}</b></span>`
        + `<span>together <b>${split.together}</b></span><span>hexagon <b>${split.hexagon}</b></span>`
        + `<span>partition <b>${split.exact ? 'exact' : 'broken'}</b></span>`;
    } else {
      $('split').innerHTML = '';
    }
    chart(rows, fractal ? 0 : k);
    const art = m.hex_svg(code, number, level, 2, 'cut', Math.max(1, Math.round(360 / tally.side)));
    if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    $('art').innerHTML = art;
  } catch (error) {
    $('art').innerHTML = '';
    $('split').innerHTML = '';
    say('note', error);
  }
}

seek($('bars'), (frac) => {
  $('fractal').value = '0';
  $('k').value = Math.min(top, Math.max(1, Math.floor(frac * top) + 1));
  build();
});
bind(['k', 'level', 'fractal'], build);
build();
