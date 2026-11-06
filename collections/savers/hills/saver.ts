import type { Look, Play } from "../play.ts";
import { hash, inks, play, rng } from "../play.ts";

const TAU = Math.PI * 2;

const LAST = 3;

const PACE = 0.02;

const DRIFT = 0.0015;

const REACH = 0.3;

const STEP = 3;

const RIDGE = { top: 0.59, vary: 0.07, foot: 0.93, sill: 0.97, knee: 0.04, deep: 1.2, tone: 1.4, day: 0.14, dusk: 0.2 };

const SWELL = { span: 0.42, tight: 0.16, base: 0.2, rise: 0.14, fore: 0.26, hang: 0.3, shift: 8, warp: 0.45 };

const DISC = { small: 0.07, grow: 0.06, near: 0.2, far: 0.12, edge: 2.2 };

const SKY = { high: 0.15, fall: 0.06, floor: 0.4, set: 0.33, sunk: 0.1 };

const BITE = { x: 0.45, y: 0.15, r: 0.9 };

const CLOUD = { most: 3, top: 0.09, low: 0.3, span: 0.18, grow: 0.18, puff: 0.16, day: 0.36, dusk: 0.28 };

const KNOT = { low: 0.22, swing: 0.56, vary: 0.3 };

const PUFF = { cuts: 32, round: 0.7, body: 0.3, waves: 2.4, shift: 2, stray: 0.6 };

const TUFT = { gap: 0.09, skip: 0.2, stray: 0.5, few: 3, more: 3, jog: 0.5, fan: 0.16, taper: 0.2 };

const BLADE = { low: 0.05, high: 0.08, hair: 0.003, girth: 0.05, pitch: 1.2, sunk: 2, waist: 0.55, neck: 0.6 };

const BEND = { ease: 0.4, bow: 0.07, whip: 0.1 };

const GUST = { wave: 3.1, speed: 0.41, warp: 1.4, slow: 0.42, calm: 0.13, blow: 0.24, flutter: 0.05, rate: 0.5 };

export default function hills(canvas: HTMLCanvasElement, look: Look): Play {
  const ctx = canvas.getContext("2d")!;
  const { css } = inks(look);
  const rand = rng(look.seed);
  const night = look.theme === "dark";
  const haze = night ? RIDGE.dusk : RIDGE.day;
  const clouds = Math.floor((CLOUD.most + 1) * rand());
  const side = rand() < 0.5 ? -1 : 1;
  const steep = rand();
  const wind = rand() < 0.5 ? -1 : 1;
  const span = SWELL.span - SWELL.tight * steep;
  const relief = SWELL.base + SWELL.rise * steep;
  const horizon = RIDGE.top + RIDGE.vary * rand();
  const disc = { x: rand(), y: rand(), set: rand() < SKY.set, r: rand() };
  const dir = rand() < 0.5 ? -1 : 1;
  const noise = (lane: number, i: number) => hash(look.seed, lane, i);
  const knot = (lane: number, i: number) => KNOT.low + (i & 1) * KNOT.swing + KNOT.vary * (noise(lane, i) - 0.5);
  const curve = (lane: number, u: number) => {
    const i = Math.floor(u);
    const f = u - i;
    const g = 1 - f;
    const a = g * g * g * knot(lane, i - 1) + (3 * f * f * f - 6 * f * f + 4) * knot(lane, i);
    return (a + (3 * g * g * g - 6 * g * g + 4) * knot(lane, i + 1) + f * f * f * knot(lane, i + 2)) / 6;
  };
  return play(canvas, (t, px) => {
    const { width: w, height: h } = canvas;
    const s = Math.min(w, h);
    const half = w / 2 / s;
    const crest = h * horizon;
    const lift = (s + h) / 2;
    const land = (n: number, x: number) => {
      const q = n / LAST;
      const u = ((x - w / 2) / s - dir * PACE * q * q * t) / span + SWELL.shift * noise(30, n);
      const tall = lift * (relief + (SWELL.fore - relief) * q * q);
      const wave = curve(20 + n, u + SWELL.warp * Math.sin(u));
      const y = crest + (h * RIDGE.foot - crest) * q ** RIDGE.deep + tall * (SWELL.hang * (1 - q) + 0.5 - wave);
      const d = h * RIDGE.sill - y;
      return y + (d - Math.sqrt(d * d + (s * RIDGE.knee) ** 2)) / 2;
    };
    const blob = (x: number, y: number, r: number, tone: number) => {
      ctx.fillStyle = css(tone);
      ctx.beginPath();
      ctx.arc(x, y, r, 0, TAU);
      ctx.fill();
    };
    const dr = s * (DISC.small + DISC.grow * disc.r);
    const dx = w / 2 + side * Math.min(w * (DISC.near + DISC.far * disc.x), w / 2 - DISC.edge * dr);
    const sky = Math.min(h * (SKY.high + SKY.fall * (s / h) * disc.y) + dr, h * SKY.floor - dr);
    const dy = disc.set ? land(0, dx) + SKY.sunk * dr : sky;
    ctx.fillStyle = css(0);
    ctx.fillRect(0, 0, w, h);
    blob(dx, dy, dr, 1);
    if (night && !disc.set) blob(dx + side * dr * BITE.x, dy - dr * BITE.y, dr * BITE.r, 0);
    ctx.fillStyle = css(night ? CLOUD.dusk : CLOUD.day);
    ctx.beginPath();
    const lap = 2 * (half + REACH);
    for (let j = 0; j < clouds; j++) {
      const long = s * (CLOUD.span + CLOUD.grow * noise(41, j));
      const cap = long * CLOUD.puff;
      const top = h * CLOUD.top + cap;
      const y = Math.round(top + (h * CLOUD.low - top) * noise(42, j));
      const at = ((j + PUFF.stray * noise(43, j)) / clouds) * lap + dir * DRIFT * t;
      const x = w / 2 + s * (at - lap * Math.floor(at / lap) - lap / 2) - long / 2;
      ctx.moveTo(x, y);
      for (let i = 1; i < PUFF.cuts; i++) {
        const u = (1 - Math.cos((Math.PI * i) / PUFF.cuts)) / 2;
        const bulk = PUFF.body + (1 - PUFF.body) * curve(44 + j, PUFF.waves * u + PUFF.shift * noise(47, j));
        ctx.lineTo(x + long * u, y - cap * Math.sin(Math.PI * u) ** PUFF.round * bulk);
      }
      ctx.lineTo(x + long, y);
    }
    ctx.fill();
    const step = Math.max(1, STEP * px);
    for (let n = 0; n <= LAST; n++) {
      ctx.fillStyle = css(haze + (1 - haze) * (n / LAST) ** RIDGE.tone);
      ctx.beginPath();
      ctx.moveTo(0, h);
      for (let x = 0; x < w + step; x += step) ctx.lineTo(x, land(n, x));
      ctx.lineTo(w, h);
      ctx.fill();
    }
    ctx.fillStyle = css(1);
    ctx.beginPath();
    const near = -dir * PACE * t;
    for (let m = Math.floor((near - half - REACH) / TUFT.gap); m * TUFT.gap - near < half + REACH; m++) {
      if (noise(60, m) < TUFT.skip) continue;
      const root = w / 2 + s * ((m + TUFT.stray * noise(61, m)) * TUFT.gap - near);
      const v = (root / s) * GUST.wave - wind * t * GUST.speed;
      const gust = 0.5 + 0.5 * Math.sin(v + GUST.warp * Math.sin(v * GUST.slow));
      const flutter = GUST.flutter * Math.sin(t * GUST.rate + TAU * noise(62, m));
      const lean = wind * (GUST.calm + GUST.blow * gust * gust) + flutter;
      const count = TUFT.few + Math.floor(TUFT.more * noise(63, m));
      for (let k = 0; k < count; k++) {
        const a = k - (count - 1) / 2 + TUFT.jog * (noise(64 + k, m) - 0.5);
        const tall = s * (BLADE.low + BLADE.high * noise(70 + k, m) * (1 - TUFT.taper * Math.abs(a)));
        const wide = s * BLADE.hair + tall * BLADE.girth;
        const x = root + a * wide * BLADE.pitch;
        const y = land(LAST, x) + wide * BLADE.sunk;
        const tip = a * TUFT.fan + lean;
        const mid = tip - lean * BEND.ease - wind * (BEND.bow + BEND.whip * noise(76 + k, m));
        const bx = x + BLADE.waist * tall * Math.sin(mid);
        const by = y - BLADE.waist * tall * Math.cos(mid);
        ctx.moveTo(x - wide, y);
        ctx.quadraticCurveTo(bx - wide * BLADE.neck, by, x + tall * Math.sin(tip), y - tall * Math.cos(tip));
        ctx.quadraticCurveTo(bx + wide * BLADE.neck, by, x + wide, y);
      }
    }
    ctx.fill();
  });
}
