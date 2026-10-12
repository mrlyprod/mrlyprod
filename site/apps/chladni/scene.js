import { tidy } from '../../lib/knobs.js';
import { veil } from '../../lib/scene.js';
import { counted, maskName, maskOf, population, windowsOf } from '../life/engine.js';
import { PANE, runs, slug, worn } from '../life/scene.js';

const NUMBERS = [3, 5, 7, 9].map((n) => [n, String(n)]);
const LEVELS = [1, 2, 3, 4, 5];

export const SPEC = [
  { key: 'born', label: 'Born', kind: 'text', def: '0.283-0.375', group: 'Rule', when: PANE },
  { key: 'stay', label: 'Stay', kind: 'text', def: '0.283-0.483', group: 'Rule', when: PANE },
  { key: 'code', label: 'Code', kind: 'text', def: '7', group: 'Mask', when: PANE },
  { key: 'number', label: 'Side', kind: 'pick', def: 3, options: NUMBERS, group: 'Mask', when: PANE },
  { key: 'size', label: 'Size', kind: 'segment', def: 256, options: [[128, '128'], [256, '256']], group: 'Run', when: PANE },
  { key: 'level', label: 'Level', kind: 'pick', def: 3, options: (value) => LEVELS.filter((n) => value.number ** n <= value.size).map((n) => [n, String(n)]), group: 'Mask', when: PANE },
  { key: 'steps', label: 'Steps', kind: 'slider', def: 64, min: 1, max: 128, step: 1, group: 'Run', when: PANE },
  { key: 'density', label: 'Density', kind: 'slider', def: 0.45, min: 0.05, max: 0.95, step: 0.01, group: 'Run', when: PANE },
];

export const PRESETS = [
  ['Bugs', { born: '0.283-0.375', stay: '0.283-0.483' }],
  ['Conway', { code: '7', number: 3, level: 1, born: '0.375', stay: '0.25-0.375' }],
];

export const PAGE = {
  spec: SPEC,
  about: [
    'A soup run under a Larger-than-Life rule on a big design mask settles into a still with a grain of its own.',
    "The rule is two windows on the share of the mask that is alive: a dead cell is born inside the birth window, a live cell stays inside the survive window. Bugs is Evans' rule, birth 34 to 45 and survive 34 to 58 of 120, kept as fractions so it moves to any mask. [Life](/life/) runs the same masks on whole counts.",
    "The status reads the grain: the ring where the still's spectrum peaks is the wavelength the rule prefers, beside the mask's own. Ring k on a torus of side N is N / k cells.",
    'Every change runs the soup to the still at once. Step or a tap adds one generation; Run goes back to the still. The family is written up in [the automata note](/research/automata/).',
  ],
  name: (value) => ['chladni', slug(worn(PRESETS, value) || `b${value.born} s${value.stay}`), value.seed].join('-'),
  keys: [
    { key: 'Enter', label: 'Step', act: 'step', button: true },
    { key: ['0', 'Home'], label: 'Run', act: 'run' },
  ],
  actions: { step: (scene) => scene.step(), run: (scene) => scene.rerun() },
};

const FIT = 0.96;
const GROUND = 0.07;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const { life, math } = opts;
  let value = { ...tidy(SPEC, opts), seed: opts.seed };
  let kept = '';
  let shape = null;
  let maskRing = null;
  let rule = null;
  let field = null;
  let ring = null;
  let extra = 0;
  let error = '';
  let skin = null;
  let dirty = true;
  let gone = false;
  const cell = () => ({ shape: [value.size, value.size], types: field });
  const mould = () => {
    const key = [value.code, value.number, value.level, value.size].join('|');
    if (key === kept) return;
    shape = maskOf(life, value, value.size);
    shape.named = maskName(math, shape.code);
    maskRing = life.chladni.profile(life.chladni.kernel(shape.mask, value.size));
    kept = key;
  };
  const read = () => {
    ring = life.chladni.profile(cell());
  };
  const settle = () => {
    try {
      mould();
      rule = windowsOf(life, value, shape.cells);
      field = life.chladni.run(shape.mask, rule.birth, rule.survive, value.size, value.steps, value.density, Number(value.seed) || 0).types;
      extra = 0;
      read();
      error = '';
    } catch (thrown) {
      error = String(thrown?.message ?? thrown);
    }
  };
  const tell = () => {
    if (gone) return;
    if (!field) {
      opts.onStat?.({ error });
      return;
    }
    const live = population(field);
    opts.onStat?.({
      gen: value.steps + extra,
      live,
      share: live / field.length,
      peak: ring.peak_ring,
      wave: ring.wavelength,
      maskPeak: maskRing.peak_ring,
      maskWave: maskRing.wavelength,
      span: shape.span,
      cells: shape.cells,
      birth: counted(rule.birth, shape.cells),
      survive: counted(rule.survive, shape.cells),
      name: rule.name,
      label: `${shape.named || `code ${shape.code}`}, side ${shape.number}, level ${shape.level}`,
      mask: shape.mask,
      error,
    });
  };
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.clearRect(0, 0, view.w, view.h);
    dirty = false;
    if (!field) return;
    const side = Math.min(view.w, view.h) * FIT;
    const x = (view.w - side) / 2;
    const y = (view.h - side) / 2;
    ctx.fillStyle = skin.dim;
    ctx.fillRect(x, y, side, side);
    ctx.fillStyle = skin.ink;
    runs(ctx, field, value.size, value.size, x, y, side / value.size);
  };
  const draw = () => {
    if (dirty) paint();
  };
  const theme = () => {
    dress();
    paint();
  };
  const step = () => {
    if (!field) return;
    field = life.chladni.next(cell(), shape.mask, rule.birth, rule.survive).types;
    extra++;
    read();
    tell();
    paint();
  };
  const rerun = () => {
    settle();
    tell();
    paint();
  };
  const set = (next) => {
    value = { ...tidy(SPEC, next), seed: next.seed };
    rerun();
  };
  const stop = () => {
    gone = true;
  };
  dress();
  settle();
  tell();
  return { still: true, draw, size: paint, theme, set, step, rerun, stop };
}
