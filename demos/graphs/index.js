import { ready, $, say, bind, out } from '../lib/mrly.js';
import { stage, web as solid } from '../lib/stage.js';
import { web, keep } from '../lib/chart.js';
import { query, stamp } from '../lib/query.js';
import { picker, seeds, roll } from '../lib/select.js';

const m = await ready();
const st = stage($('stage'));
const s = seeds();
const KINDS = { flat: ['core', 'edge', 'tunnel'], cube: ['core', 'edge', 'tunnel'], hex: ['core', 'dual', 'edge'] };
const BUDGET = { lattice: 20000, force: 2000 };
const EXTRA = ['number', 'level', 'graph', 'layout', 'dots'];
const params = query(['space', 'camera']);
const more = (seed) => roll(seed, ['level', 'graph', 'layout']);
const flat = picker({ host: $('flat-pick'), m, dimension: 2, base: [3, 2], code: '495', build, extra: EXTRA, seeds: s, more });
const cube = picker({ host: $('cube-pick'), m, dimension: 3, base: [2, 3], code: '23', build, key: 'c', extra: EXTRA, seeds: s, more });
let lattice = null, layout = null, body = null, playing = false, ticks = 2;

const sheet = keep((nodes, radius) => web($('sheet'), Math.min($('sheet').clientWidth, 620), nodes, lattice.branches, lattice.roles, radius));

function kind(space) {
  const select = $('graph');
  for (const option of select.options) option.disabled = !KINDS[space].includes(option.value);
  if (select.selectedOptions[0]?.disabled) select.value = 'core';
  return select.value;
}

function current() {
  return layout ? layout.positions() : lattice.nodes.subarray(2);
}

function draw() {
  if (!lattice) return;
  const radius = +$('dots').value;
  out('dots', radius);
  const nodes = current();
  if (lattice.dim === 2) {
    sheet(nodes, radius);
    return;
  }
  if (body) {
    body.place(nodes);
    return;
  }
  body = solid(nodes, lattice.branches, lattice.roles, radius / 250);
  st.clear();
  for (const part of body.parts) st.add(part);
}

function pulse() {
  $('ticks').textContent = layout ? layout.ticks() : '';
  $('energy').textContent = layout && layout.ticks() ? layout.energy().toExponential(2) : '';
  $('moved').textContent = layout && layout.ticks() ? layout.moved().toExponential(2) : '';
  $('heat').textContent = layout ? layout.temperature().toExponential(2) : '';
}

function relax() {
  if (!playing) return;
  const t0 = performance.now();
  layout.step(ticks);
  const dt = performance.now() - t0;
  if (dt < 8 && ticks < 8) ticks += 1;
  else if (dt > 14 && ticks > 1) ticks -= 1;
  draw();
  pulse();
  requestAnimationFrame(relax);
}

function stop() {
  playing = false;
  $('play').textContent = 'Relax';
}

function reset() {
  stop();
  if (!lattice) return;
  layout = $('layout').value === 'force' ? new m.Layout(lattice.nodes.subarray(2), lattice.branches, lattice.dim, s.get()) : null;
  $('play').disabled = !layout;
  $('reset').disabled = !layout;
  body = null;
  draw();
  pulse();
  if (layout) play();
}

function play() {
  if (!layout) return;
  playing = !playing;
  $('play').textContent = playing ? 'Pause' : 'Relax';
  if (playing) relax();
}

function build() {
  stop();
  const space = $('space').value;
  $('flat-pick').hidden = space !== 'flat';
  $('cube-pick').hidden = space === 'flat';
  $('cube-row').hidden = space !== 'cube';
  $('sheet').hidden = space === 'cube';
  $('stage').hidden = space !== 'cube';
  stamp({ space });
  const which = kind(space);
  const mode = $('layout').value;
  say('note');
  try {
    const { code, base, name } = (space === 'flat' ? flat : cube).read();
    const number = +$('number').value;
    const top = m.graph_cap(space, code, number, base, which, BUDGET[mode]);
    const level = Math.min(+$('level').value, top);
    $('level').max = top;
    $('level').value = level;
    out('level', `${level} of ${top}`);
    const nodes = m.graph_nodes(space, code, number, level, base, which);
    lattice = { dim: nodes[0], nodes, branches: m.graph_branches(space, code, number, level, base, which), roles: m.graph_roles(space, code, number, level, base, which) };
    const tally = JSON.parse(m.graph_census(space, code, number, level, base, which));
    $('name').textContent = name;
    $('nodes').textContent = tally.nodes;
    $('branches').textContent = tally.branches;
    $('tips').textContent = tally.tips;
    $('junctions').textContent = tally.junctions;
    $('pieces').textContent = tally.components;
    $('length').textContent = tally.length.toFixed(2);
    $('box').textContent = tally.box.toFixed(3);
    $('euler').textContent = tally.euler ?? 'none';
    $('net-note').textContent = `${which} graph, level ${level}, ${mode}${space === 'cube' ? `, ${$('camera').value === 'iso' ? 'isometric' : 'perspective'}` : ''}`;
    reset();
  } catch (error) {
    lattice = null;
    layout = null;
    st.clear();
    say('note', error);
  }
}

bind(['space', 'number', 'level', 'graph', 'layout'], build);
bind(['dots'], () => {
  body = null;
  draw();
});
$('play').onclick = play;
$('reset').onclick = reset;
$('camera').onchange = () => {
  st.project($('camera').value);
  build();
};
$('corner').onclick = () => st.view(1, 1, 1);
$('edge').onclick = () => st.view(1, 1, 0);
$('face').onclick = () => st.view(0, 0, 1);
st.project($('camera').value);
if (params.get('camera') === 'iso') st.view(1, 1, 1);
if (s.get()) ($('space').value === 'flat' ? flat : cube).apply();
build();
