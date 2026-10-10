import { rng } from '../../lib/scene.js';
import { KEYS, SONG, bus } from '../../lib/space/tempo.js';

const SALT = 0x5f356495;
const GOLD = 0x9e3779b1;
const SCALES = ['minor', 'dorian', 'phrygian'];
const BPM = [122, 130];
const FIRE = 9900;
const OPEN = 12000;
const APPROACH = 2000;
const BEFORE = 9700;
const LEAVE = 1700;
const BLACK = 1500;
const RISE = 3600;
const HATS = 6400;
const CUT = 450;
const BREAK = 1900;
const CLIMB = 10000;
const WHITE = 400;
const RISER = 2400;
const SILENT = 800;
const KICK = 2600;
const HAT = 3500;
const ARP = 6000;
const EARLY = 0.4;
const BOOST = [0.5, 0.5];
const DRIVE = [0.55, 0.45];
const LOW = 0.2;
const TAIL = 0.3;

/* CLOCK */

export function clock(length) {
  const out = length - BEFORE;
  return { fire: FIRE, open: OPEN, enter: OPEN + APPROACH, out, arrive: out + LEAVE, end: length, next: length + BLACK };
}

/* ARRANGEMENT */

export function arrange(length) {
  const { fire, open, out, arrive, end, next } = clock(length);
  const middle = (open + out) / 2;
  return (t) => {
    if (t < RISE) return { name: 'intro', swell: (EARLY * t) / fire };
    if (t < HATS) return { name: 'rise', swell: (EARLY * t) / fire };
    if (t < fire) return { name: 'hats', swell: (EARLY * t) / fire };
    if (t < open) return { name: 'boost', swell: BOOST[0] + (BOOST[1] * (t - fire)) / (open - fire) };
    if (t < open + CUT) return { name: 'cut' };
    if (Math.abs(t - middle) < BREAK / 2) return { name: 'breakdown', swell: LOW };
    if (t < out + WHITE) return { name: 'hyper', swell: DRIVE[0] + DRIVE[1] * Math.min(1, (t - open) / CLIMB) };
    if (t < out + SILENT) return { name: 'cut' };
    if (t < end) return { name: 'outro', kick: t < arrive + KICK, hat: t < arrive + HAT, arp: t < arrive + ARP, swell: TAIL };
    if (t < next) return { name: 'cut' };
    return { name: 'intro', swell: 0 };
  };
}

export const cues = ({ open, out }) => [[open - RISER, 'riser', { dur: RISER / 1000 }], [open, 'hit', {}], [out, 'riser', { dur: WHITE / 1000 }], [out + WHITE, 'hit', {}]];

/* SONG */

export function song(seed, loop = 0) {
  const rand = rng(((seed >>> 0) ^ SALT ^ Math.imul(loop + 1, GOLD)) >>> 0);
  return { ...SONG, seed: Math.floor(rand() * 4294967296), key: KEYS[Math.floor(rand() * KEYS.length)], scale: SCALES[Math.floor(rand() * SCALES.length)], bpm: BPM[0] + Math.floor(rand() * (BPM[1] - BPM[0] + 1)) };
}

/* MIX */

export const levels = (value) => ({ sound: value.sound ? 1 : 0, music: value.music ? 1 : 0 });

export function mixer(get) {
  const line = bus();
  return { ...line, cue: (name, t, args) => (get().sound ? line.cue(name, t, args) : undefined) };
}
