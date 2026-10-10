import { tidy } from '../../lib/knobs.js';
import { NAMES, SPEED, canon, name, pickCode, route, teach } from '../../lib/space/bang.js';
import { look as aim } from '../../lib/space/camera.js';
import { blank, context } from '../../lib/space/gl2.js';
import { hud } from '../../lib/space/hud.js';
import { hyper } from '../../lib/space/hyper.js';
import { GRADES, march, tone } from '../../lib/space/march.js';
import { post } from '../../lib/space/post.js';
import { trails } from '../../lib/space/trails.js';
import { add } from '../../lib/space/vec.js';

export const SPEC = [
  { key: 'code', label: 'Code', kind: 'slider', def: 23, min: -1, max: 255, step: 1, group: 'Shape' },
  { key: 'n', label: 'N', kind: 'segment', def: 3, options: [[3, '3'], [5, '5']], group: 'Shape' },
  { key: 'pick', label: 'Pick', kind: 'segment', def: 'all', options: [['all', 'All'], ['flyable', 'Flyable'], ['named', 'Named'], ['family', 'Family']], group: 'Shape' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.25, max: 2, step: 0.25, group: 'Flight' },
  { key: 'path', label: 'Path', kind: 'toggle', def: 1, group: 'Flight' },
  { key: 'pilot', label: 'Pilot', kind: 'segment', def: 'steady', options: [['steady', 'Steady'], ['fighter', 'Fighter'], ['coaster', 'Coaster']], group: 'Flight' },
  { key: 'look', label: 'Look', kind: 'segment', def: 'studio', options: [['graphic', 'Graphic'], ['studio', 'Studio'], ['haze', 'Haze']], group: 'Look' },
  { key: 'palette', label: 'Palette', kind: 'pick', def: '', options: [['', 'Seeded'], ['accent', 'Accent'], ['fire', 'Fire'], ['ice', 'Ice'], ['mono', 'Mono']], group: 'Look' },
  { key: 'quality', label: 'Quality', kind: 'segment', def: '', options: [['', 'Auto'], ['low', 'Low'], ['high', 'High']], group: 'Look' },
  { key: 'hyper', label: 'Hyper', kind: 'toggle', def: 0, group: 'Look' },
];

export const PAGE = { spec: SPEC, record: true };

const FOV = 1.25;
const OPEN = -4000;
const SKY = { density: 0.8, dust: 0.4 };
const SPAN = 8;
const DROP = 0.2;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const { n } = value;
  const code = pickCode(value, seed);
  const design = { code, n };
  const info = teach(code, n);
  const path = info.kept ? route(design, seed, { pilot: value.pilot }) : null;
  const made = [];
  let film = null;
  let trip = null;
  let world = null;
  let lines = null;
  let face;
  try {
    face = hud(gl, view);
    made.push(face);
    if (path) {
      world = march(gl, view, design, { look: value.look, quality: value.quality, toFrame: path.toFrame });
      made.push(world);
      if (value.path) {
        lines = trails(gl, view);
        made.push(lines);
      }
      if (value.hyper) {
        trip = hyper(gl, view, { seed, sky: SKY, tint: '', look: 'cloud', idle: 0, mode: null, floor: 'cloud' });
        made.push({ drop: trip.stop });
      } else {
        film = post(gl, view);
        made.push(film);
      }
    }
  } catch (error) {
    for (const one of made) one.drop();
    console.error(error);
    return blank(canvas);
  }
  if (trip) {
    const now = view.t;
    view.t = OPEN;
    trip.trigger();
    view.t = now;
    trip.hold();
  }
  const grade = GRADES[value.look];
  const title = `${NAMES.some(([one]) => canon(one) === canon(code)) ? name(code) : 'bang'} ${code}`;
  let tint = null;
  let paints = null;
  let rail = [];
  let said = [];
  const words = (state) => [title, `n ${n}  level ${state.level}`, `bits ${info.bits}  fill ${info.kept}/${n ** 3}`, `dim ${info.dimension.toFixed(2)}  class ${info.class}`, state.mode];
  const empty = () => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    said = ['empty'];
    face.lines(said, 'tl');
    face.draw();
  };
  const draw = () => {
    if (!path) return empty();
    const accent = view.look().accent;
    if (accent !== tint) {
      tint = accent;
      paints = tone(value.palette, seed, accent);
    }
    const tau = (view.t / 1000) * value.speed;
    const state = path.at(tau);
    const cam = aim(state.pos, add(state.pos, state.fwd), state.up, state.roll);
    cam.fov = FOV;
    const points = lines ? path.path(tau, SPAN, DROP) : [];
    rail = points.length ? [{ points, born: 0, step: (SPEED * SPAN) / (points.length - 1) }] : [];
    world.draw(cam, state, paints);
    const over = () => {
      world.layer(1);
      lines?.draw(rail, cam, 1, world.depth());
    };
    if (trip) {
      const ahead = aim([0, 0, 0], [0, 0, 1], [0, 1, 0], state.roll);
      ahead.fov = FOV;
      trip.draw(ahead, null, over, world.scale(), grade);
    } else {
      film.begin(world.scale());
      over();
      film.end(grade);
    }
    said = words(state);
    face.lines(said, 'tl');
    face.draw();
  };
  const stop = () => {
    for (const one of made) one.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  return { draw, stop, code, said: () => said, trails: () => rail };
}
