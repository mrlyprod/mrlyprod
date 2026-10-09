import { tidy } from '../../lib/knobs.js';
import { look as aim } from '../../lib/space/camera.js';
import { blank, context } from '../../lib/space/gl2.js';
import { film, hyper } from '../../lib/space/hyper.js';
import { tints } from '../../ui/hues.js';

export const SPEC = [
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.5, max: 2, step: 0.1, group: 'Jump' },
  { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005, group: 'Jump' },
  { key: 'stars', label: 'Density', kind: 'slider', def: 1, min: 0.25, max: 3, step: 0.25, group: 'Sky' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: tints('Film'), group: 'Sky' },
  { key: 'look', label: 'Look', kind: 'segment', def: 'cloud', options: [['cloud', 'Cloud'], ['classic', 'Classic']], group: 'Sky' },
  { key: 'hyper', label: 'Hyperspace', kind: 'segment', def: 'mix', options: [['no', 'No'], ['yes', 'Yes'], ['mix', 'Mix']], group: 'Saver' },
  { key: 'totem', label: 'Totem', kind: 'slider', def: -1, min: -1, max: 255, step: 1, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  keys: [{ key: 'Enter', label: 'Jump', does: 'Again in hyperspace to exit', act: 'jump', button: true }],
  actions: { jump: (scene) => (scene.phase() === 'cruise' ? scene.trigger() : scene.exit()) },
  record: true,
};

const AHEAD = aim([0, 0, 0], [0, 0, 1]);

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const { speed, idle, stars, tint, look, hyper: mode, totem } = tidy(SPEC, opts);
  const seed = Number.isFinite(opts?.seed) ? opts.seed >>> 0 : Math.floor(view.rand() * 4294967296);
  let trip;
  try {
    trip = hyper(gl, view, {
      seed,
      sky: { density: stars, next: true },
      tint: tint ? view.look().accent : '',
      look,
      speed,
      idle,
      film: film(opts),
      audio: opts.audio ?? null,
      mode,
    });
  } catch {
    return blank(canvas);
  }
  let gone = false;
  let idol = null;
  let over = null;
  const draw = () => trip.draw(AHEAD, null, over);
  const carry = ({ totem: build, ride }) => {
    if (gone) return;
    idol = build(gl, view, { code: totem, n: 3, seed });
    if (!idol) return;
    over = (cam) => idol.draw(cam, ...ride(trip, cam));
    if (view.still) draw();
  };
  const ready = totem < 0 ? Promise.resolve() : import('../../lib/space/bang.js').then(carry).catch(() => {});
  const still = (call) => (...args) => {
    call(...args);
    if (view.still && over && !gone) draw();
  };
  const stop = () => {
    gone = true;
    idol?.drop();
    trip.stop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, trigger: still(trip.trigger), exit: still(trip.exit), hold: trip.hold, windDown: still(trip.windDown), phase: trip.phase, stage: trip.stage, ready, stop };
}
