import { rng } from '../../lib/scene.js';
import { KEYS, SONG, bus, tempo } from '../../lib/space/tempo.js';

const SALT = 0x5f356495;
const SCALES = ['minor', 'dorian', 'phrygian'];

export const BAR = tempo(SONG).bar;

export const LANDING = 8 * BAR;

export function song(seed) {
  const rand = rng(((seed >>> 0) ^ SALT) >>> 0);
  return { ...SONG, seed: seed >>> 0, key: KEYS[Math.floor(rand() * KEYS.length)], scale: SCALES[Math.floor(rand() * SCALES.length)] };
}

export const beat = (t) => Math.round(t / BAR) * BAR;

export function exit(open, hold, least) {
  let out = beat(open + hold);
  while (out < open + least) out += BAR;
  return out;
}

export const levels = (value) => ({ sound: value.sound ? 1 : 0, music: value.music ? 1 : 0 });

export function mixer(get) {
  const line = bus();
  return { ...line, cue: (name, t, args) => (get().sound ? line.cue(name, t, args) : undefined) };
}
