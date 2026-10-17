import { tidy } from '../../lib/app/spec.js';
import { TINTS } from '../../lib/color.js';
import { RAMPS, colorizer, paint } from '../../lib/moire/engine.js';
import { fit, shell } from '../../lib/moire/sheet.js';
import { CARPET, LEAST, MOST, SIZES, cutter, field, law, more } from './engine.js';

export const SPEC = [
  { key: 'layers', label: 'Layers L', kind: 'slider', def: 28, min: LEAST, max: MOST, step: 2, group: 'Stack' },
  { key: 'size', label: 'Size', kind: 'segment', def: 384, options: SIZES.map((n) => [n, String(n)]), group: 'Stack' },
  { key: 'ramp', label: 'Ramp', kind: 'pick', def: 'fire', options: RAMPS, group: 'Look' },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 24, min: 2, max: 64, step: 1, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
  { key: 'half', label: 'Band W', kind: 'slider', def: 0, min: 0, max: 16, step: 1, unit: 'cells', group: 'Readings' },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  name: (value) => `star-${value.layers}`,
  about: [
    'Stack the hexagonal cuts of the carpet cube, one for each odd side, and a six-armed star stands at the centre. It is not there in the limit: it is the gap between an arm that carries no 1/(2n) term and a background that does, and it fades like (ln L)/L. Tap the picture to add a cut.',
    'The picture resamples every layer onto one square, the ideal frame; the numbers never resample and read each layer on the cube\'s own cells, the cell frame. The same star decays at -0.1807 in the ideal frame, -0.1252 in the lattice frame and exactly -1/4 on the arm in the cell frame.',
    'L steps by two and stays even, since odd L shifts the constant by 1/8. The slope is read from L/2 to L, and only when L is divisible by four. Widening the band W walks the coefficient onto the width law -(K + b)/(4(2K + 1)) at K = floor(W/2), so an odd W reads the same band as the even one under it.',
    'The proofs and the frames the star was measured in are on [the hexagon page](/research/hexagon/).',
  ],
  actions: { reroll: (scene) => scene.random?.() },
};

const read = (opts) => ({ ...tidy(SPEC, opts), seed: opts.seed >>> 0 });

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  const star = new math.six.star.Star(CARPET);
  const cuts = cutter(math);
  let value = read(opts);
  let held = null;
  let reading = null;
  let error = '';
  let failed = '';
  let tone = 0;
  let dirty = true;
  let told = '';
  let gone = false;
  const done = { field: '', paint: '', law: '' };
  const fieldKey = () => `${value.layers} ${value.size}`;
  const paintKey = () => [fieldKey(), value.ramp, value.levels, value.invert, value.tint, tone].join(' ');
  const lawKey = () => `${value.layers} ${value.half}`;
  const tell = () => {
    if (gone || !opts.onFacts) return;
    const now = error ? { error } : { ...reading, size: held.size, low: held.low, high: held.high };
    const key = JSON.stringify(now);
    if (key === told) return;
    told = key;
    opts.onFacts(now);
  };
  const sync = () => {
    const want = `${paintKey()} ${lawKey()}`;
    if (want === failed) return;
    try {
      if (done.field !== fieldKey()) {
        held?.field.free();
        held = null;
        held = field(math, cuts, value.layers, value.size);
        done.field = fieldKey();
      }
      if (done.paint !== paintKey()) {
        pad.put(paint(math, held, colorizer(value.ramp, value.levels, view.look()), value.levels, value.invert), held.size);
        done.paint = paintKey();
        dirty = true;
      }
      if (done.law !== lawKey()) {
        reading = law(math, star, value.layers, value.half);
        done.law = lawKey();
      }
      error = '';
      failed = '';
    } catch (fault) {
      error = String(fault?.message ?? fault);
      failed = want;
      dirty = true;
    }
    tell();
  };
  const draw = () => {
    sync();
    if (!dirty) return;
    pad.clear();
    if (!error && held) pad.blit(held.size, fit(view.w, view.h), true);
    dirty = false;
  };
  const set = (next) => {
    value = read(next);
    dirty = true;
  };
  const touch = (u, v, start) => (start ? { layers: more(value.layers) } : null);
  const theme = () => {
    tone += 1;
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const stop = () => {
    gone = true;
    held?.field.free();
    held = null;
    cuts.free();
    star.free();
  };
  return { draw, set, touch, theme, size, stop, still: true };
}
