import { rng } from '../../lib/scene.js';
import { codes } from '../../lib/space/bang.js';
import { world } from '../../lib/space/planet.js';

const TAU = Math.PI * 2;
const SYLLABLES = ['ka', 'lo', 've', 'ri', 'sol', 'tau', 'mi', 'zen', 'or', 'an', 'el', 'dra', 'nu', 'qui', 'ser', 'tal', 'vo', 'xen', 'ya', 'bel', 'cor', 'phi', 'ru', 'sa'];
const LETTERS = 'BCDEFGHIJ';
const TYPES = ['terran', 'desert', 'ice', 'lava', 'gas'];
const WEIGHTS = [0.42, 0.16, 0.14, 0.1, 0.18];
const COLD = 2;
const GAS = 3;
const FIRST = 0.35;
const RATIO = 1.75;
const SPREAD = 0.15;
const INCL = (3 * Math.PI) / 180;
const COOL = 3500;
const HOT = 9000;
const SALT = 0x6c8e9cf5;
const BANG = 0x3b9f4c27;
const MOST = 4;
const FIVE = 0.25;
const TILT = [(20 * Math.PI) / 180, (60 * Math.PI) / 180];
const JITTER = 0.6;
const FAR = 1000;

const KINDS = { terran: 'terran', desert: 'desert', ice: 'ice world', lava: 'lava world', gas: 'gas giant' };

export const NAMES = { 23: 'carpet', 232: 'net', 22: 'star', 129: 'void', 17: 'xtree', 5: 'ytree', 3: 'ztree' };

export const kind = (body) => (body.kind === 'bang' ? `n ${body.n}` : KINDS[body.type]);

const clamp = (v) => Math.min(Math.max(v, 0), 255) / 255;

function kelvin(temp) {
  const t = temp / 100;
  const r = t <= 66 ? 255 : 329.698727446 * (t - 60) ** -0.1332047592;
  const g = t <= 66 ? 99.4708025861 * Math.log(t) - 161.1195681661 : 288.1221695283 * (t - 60) ** -0.0755148492;
  const b = t >= 66 ? 255 : t <= 19 ? 0 : 138.5177312231 * Math.log(t - 10) - 305.0447927307;
  return [clamp(r), clamp(g), clamp(b)];
}

function roll(rand, a) {
  const weights = WEIGHTS.map((w, i) => (a > COLD && TYPES[i] === 'gas' ? w * GAS : w));
  let r = rand() * weights.reduce((sum, w) => sum + w, 0);
  for (let i = 0; i < TYPES.length - 1; i++) {
    if (r < weights[i]) return TYPES[i];
    r -= weights[i];
  }
  return TYPES[TYPES.length - 1];
}

function seedOf(rand, type) {
  const base = Math.floor(rand() * 4294967296);
  for (let i = 0; ; i++) {
    const seed = (base + i) >>> 0;
    if (world('exo', seed).kind === type) return seed;
  }
}

function named(rand) {
  const count = rand() < 0.5 ? 2 : 3;
  return Array.from({ length: count }, () => SYLLABLES[Math.floor(rand() * SYLLABLES.length)]).join('').toUpperCase();
}

function bangs(seed, count, first) {
  const rand = rng(((seed >>> 0) ^ BANG) >>> 0);
  const used = new Set();
  const turn = rand() * TAU;
  return Array.from({ length: count }, (_, k) => {
    const n = rand() < FIVE ? 5 : 3;
    const pool = codes(n).filter((code) => !used.has(code));
    const code = pool[Math.floor(rand() * pool.length)];
    used.add(code);
    const lon = turn + ((k + (1 - JITTER) / 2 + JITTER * rand()) / count) * TAU;
    const lat = (TILT[0] + rand() * (TILT[1] - TILT[0])) * (rand() < 0.5 ? -1 : 1);
    const dir = [Math.cos(lat) * Math.cos(lon), Math.sin(lat), Math.cos(lat) * Math.sin(lon)];
    const name = NAMES[code] ? `${NAMES[code]} ${code}` : `bang ${code}`;
    return { id: first + k, name: name.toUpperCase(), kind: 'bang', type: 'bang', code, n, dir, a: FAR, pos: dir.map((v) => v * FAR) };
  });
}

export function system(seed, { worlds = 5, bangs: many = 0 } = {}) {
  const rand = rng(((seed >>> 0) ^ SALT) >>> 0);
  const temp = COOL + rand() * (HOT - COOL);
  const star = { name: named(rand), temp, color: kelvin(temp) };
  const count = Math.min(Math.max(Math.round(worlds) || 0, 1), LETTERS.length);
  const bodies = Array.from({ length: count }, (_, k) => {
    const a = FIRST * RATIO ** k * (1 + SPREAD * (2 * rand() - 1));
    const angle = rand() * TAU;
    const incl = (2 * rand() - 1) * INCL;
    const type = roll(rand, a);
    const pos = [a * Math.cos(angle) * Math.cos(incl), a * Math.sin(incl), a * Math.sin(angle) * Math.cos(incl)];
    return { id: k, name: `${star.name} ${LETTERS[k]}`, kind: 'planet', type, a, pos, world: seedOf(rand, type) };
  });
  const extra = Math.min(Math.max(Math.round(many) || 0, 0), MOST);
  return { star, bodies: [...bodies, ...bangs(seed, extra, bodies.length)] };
}
