import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { deep } from '../../lib/space/deep.js';
import { blank, context } from '../../lib/space/gl2.js';

export const SPEC = [
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.25, max: 2, step: 0.25, group: 'Flight' },
  { key: 'path', label: 'Path', kind: 'toggle', def: 0, group: 'Flight' },
  { key: 'palette', label: 'Palette', kind: 'pick', def: '', options: [['', 'Classic'], ['seeded', 'Seeded'], ['accent', 'Accent'], ['fire', 'Fire'], ['ice', 'Ice'], ['mono', 'Mono']], group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = { spec: SPEC, record: true };

const HOME = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const { speed, palette, path } = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  let space;
  try {
    space = deep(gl, view, { kind: 'set', seed, palette: palette === 'seeded' ? '' : palette || 'classic', speed, path, home: HOME });
  } catch (error) {
    console.error(error);
    return blank(canvas);
  }
  const stop = () => {
    space.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw: () => space.draw(), theme: space.theme, stop };
}
