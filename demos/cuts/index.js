import { ready, $, ink, say, bind, out } from '../lib/mrly.js';
import { picker, cap } from '../lib/select.js';
import { board, bars, axis, keep, seek } from '../lib/chart.js';

const m = await ready();
const src = picker({ host: $('picker'), m, dimension: 3, code: '126', build, extra: ['level'] });

const pad = 12;
let span = [0, 0];
let stamp = '';

const chart = keep((counts, low, marks) => {
  const b = board($('bars'), 180, { pad, top: 12, bottom: 20 });
  bars(b, counts, { color: (i) => (marks.includes(low + i) ? ink.gold : ink.blue) });
  axis(b, [[0, low], [1, low + counts.length - 1]]);
});

function build() {
  const view = $('view').value;
  const level = cap('level', 2, 1, view === 'section' ? 64 : 512);
  say('note');
  try {
    const { code, name } = src.read();
    const cut = JSON.parse(m.diagonal_profile(code, 2, level, 2));
    const [low, high] = cut.support;
    span = [low, high];
    const slider = $('height');
    const mark = `${code}/${level}`;
    slider.min = low;
    slider.max = high;
    if (mark !== stamp) {
      stamp = mark;
      slider.value = cut.central[0];
    }
    slider.value = Math.min(high, Math.max(low, +slider.value));
    const both = $('both').checked;
    slider.disabled = both;
    const heights = both ? cut.central : [+slider.value];
    out('height', heights.join(' and '));
    $('name').textContent = name;
    $('side').textContent = cut.side;
    $('support').textContent = `[${low}, ${high}]`;
    $('live').textContent = `${cut.nonempty} of ${cut.heights}`;
    $('here').textContent = heights.map((s) => m.diagonal_count(code, 2, level, 2, s)).join(' and ');
    $('least').textContent = cut.min;
    $('most').textContent = cut.max;
    $('flat').textContent = cut.constant ? 'yes' : 'no';
    $('digits').textContent = heights.map((s) => m.diagonal_digits(code, 2, level, 2, s)).join(' and ');
    chart(cut.counts.map(Number), low, heights);
    const art = view === 'section'
      ? m.hex_svg(code, 2, level, 2, 'cut', Math.max(1, Math.round(256 / 2 ** level)))
      : m.diagonal_svg(code, 2, level, 2, heights, Math.max(1, Math.round(512 / 2 ** level)));
    if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    $('art').innerHTML = art;
    $('drawn').textContent = view === 'section' ? '-' : m.diagonal_total(code, 2, level, 2, heights);
  } catch (error) {
    $('art').innerHTML = '';
    $('drawn').textContent = '';
    say('note', error);
  }
}

seek($('bars'), (frac) => {
  if ($('both').checked) return;
  const [low, high] = span;
  $('height').value = Math.min(high, Math.max(low, Math.round(low + frac * (high - low))));
  build();
}, pad);
bind(['level', 'height', 'view'], build);
$('both').onchange = build;
build();
