import { rng, run } from '../../lib/scene.js';
import { record } from '../../lib/space/record.js';

export const SIZE = [1280, 720];

const TILE = [320, 180];

const COLS = 6;

export const sheet = (frames) => [COLS * TILE[0], Math.ceil(frames / COLS) * TILE[1]];

export const tile = (i) => [(i % COLS) * TILE[0], Math.floor(i / COLS) * TILE[1]];

function play(scene, view, due, t) {
  while (due.length && due[0][0] <= t) {
    const [at, name] = due.shift();
    if (typeof scene[name] !== 'function') throw new Error(`film: the scene has no ${name}()`);
    view.t = at;
    scene[name]();
  }
  view.t = t;
}

const order = (cues) => [...cues].sort((a, b) => a[0] - b[0]);

export async function roll(make, canvas, { seed, opts = {}, cues = [], from = 0, step, frames, look }, each) {
  [canvas.width, canvas.height] = SIZE;
  const view = { rand: rng(seed), look: () => look, still: false, w: SIZE[0], h: SIZE[1], dpr: 1, t: from, fixed: true };
  const scene = make(canvas, view, { ...opts, seed });
  if (scene.font && globalThis.document?.fonts) await document.fonts.load(`1em ${JSON.stringify(scene.font)}`);
  await scene.ready;
  const due = order(cues);
  for (let i = 0; i < frames; i++) {
    play(scene, view, due, from + i * step);
    scene.draw();
    each(i);
    await new Promise((r) => setTimeout(r, 0));
  }
}

function tape(make, canvas, { name, seed, opts = {}, cues = [], from = 0, fps, frames, look }) {
  const step = 1000 / fps;
  const due = order(cues);
  canvas.style.setProperty('--art', look.paper);
  canvas.style.setProperty('--accent', look.accent);
  const staged = (at, view, given) => {
    view.t = from - step;
    const scene = make(at, view, given);
    return { ...scene, draw: () => (play(scene, view, due, view.t), scene.draw()) };
  };
  const stop = run(canvas, staged, { ...opts, seed, step, frame: SIZE });
  let drawn = 0;
  let take = null;
  const fix = (held) =>
    stop.fix(
      held && {
        ...held,
        after: (info) => {
          if (drawn >= frames) return take.stop();
          drawn += 1;
          held.after(info);
        },
      },
    );
  take = record({ canvas, fix }, { ratio: '16:9', fps, name, size: SIZE });
  return take.done.finally(stop).then((file) => new Promise((r) => setTimeout(() => r(file), 2000)));
}

export async function film(make) {
  const config = JSON.parse(document.getElementById('film').textContent);
  const canvas = document.createElement('canvas');
  document.body.setAttribute('aria-busy', 'true');
  try {
    if (config.video) {
      document.body.append(canvas);
      return await tape(make, canvas, config);
    }
    const out = document.createElement('canvas');
    [out.width, out.height] = sheet(config.frames);
    const pen = out.getContext('2d');
    pen.imageSmoothingQuality = 'high';
    document.body.append(out);
    await roll(make, canvas, config, (i) => pen.drawImage(canvas, ...tile(i), ...TILE));
  } finally {
    document.body.removeAttribute('aria-busy');
  }
}
