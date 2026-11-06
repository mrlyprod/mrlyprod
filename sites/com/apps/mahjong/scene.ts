import { blend, bright, veil } from "@mrlyprod/ui/kit/color.ts";
import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { held, hold } from "@mrlyprod/ui/kit/keep.ts";
import type { Fired } from "@mrlyprod/ui/kit/keys.tsx";
import type { Handle, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import { arp } from "../shared/audio/voices.ts";
import type { Given as Base, Inks, Told as Plain } from "../shared/games/stage.ts";
import { clear, inks, lower, TURNS } from "../shared/games/stage.ts";
import type { Bed } from "./bed.ts";
import { bedOf, roomOf, spotOf } from "./bed.ts";
import type { Act, Face, State } from "./engine.ts";
import { deal, freeTiles, multiplier, newest, step } from "./engine.ts";
import type { Tones } from "./faces.ts";
import { artOf, LABELS } from "./faces.ts";

export type Told = Plain & { hints: number; undos: number; shuffles: number; cleared: boolean; stuck: boolean };

export type Given = Base<Told> & { mrly: Mrly };

export type Mahjong = ReturnType<typeof make>;

type Paper = OffscreenCanvas;

type Pen = OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D;

type Grain = { x: number; y: number; color: string };

type Spark = { x: number; y: number; vx: number; vy: number; born: number; life: number; color: string; size: number };

type Flight = {
  face: number;
  ax: number;
  ay: number;
  bx: number;
  by: number;
  born: number;
  word: string;
  big: string;
};

type Ring = { x: number; y: number; born: number; life: number; color: string; size: number };

type Word = { text: string; x: number; y: number; born: number; color: string; size: number };

type Shift = { from: [number, number]; born: number; face: number; span: number };

type Kept = { at: number; score: number; best: number; top: number };

const KEEP = "mahjong";
const ROUND = 0.13;
const FACE = 0.09;
const SHADE = 0.36;
const FALL = 520;
const STAGGER = 620;
const FLY = 240;
const LIFE = 900;
const RING = 460;
const WORD = 800;
const POINTS = 10;
const SHAKE = 280;
const SWIRL = 620;
const CLEARED = 2600;
const SWEEP = 1200;
const GRAINS = 12;
const MOST = 1400;
const NOTES = [60, 62, 65, 67, 69, 72, 74, 77, 79, 81, 84];
const CALM = 0.04;
const FAINT = 0.05;
const DIMMING = 0.012;
const FIREWORKS = 28;
const PACE = 34;

const deepened = (face?: Face): Face | undefined =>
  face?.suit === "plate"
    ? { ...face, fine: 480 }
    : face?.suit === "carpet"
      ? { ...face, number: 3, level: 5 }
      : face?.suit === "cube" || face?.suit === "star"
        ? { ...face, level: 3 }
        : face;

const fresh = (): Kept => ({ at: 1, score: 0, best: 1, top: 0 });

const eased = (t: number) => 1 - (1 - t) ** 3;

function bounce(t: number) {
  const u = 1 - t;
  return 1 - u * u * Math.abs(Math.cos(t * Math.PI * 1.5));
}

const clamp01 = (v: number) => Math.min(1, Math.max(0, v));

const rounded = (pen: Pen, x: number, y: number, w: number, h: number, r: number) => {
  pen.beginPath();
  pen.roundRect(x, y, w, h, r);
};

const sheet = (w: number, h: number): [Paper, OffscreenCanvasRenderingContext2D] => {
  const paper = new OffscreenCanvas(Math.max(1, Math.ceil(w)), Math.max(1, Math.ceil(h)));
  return [paper, paper.getContext("2d")!];
};

export const units = wasm(...CRATES);

export const PAGE = {
  units,
  gestures: true,
  keys: [
    { key: ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"], label: "Move", act: "arrows" },
    { key: ["Enter", " "], label: "Pick", act: "press" },
    { key: "h", label: "Hint", act: "hint" },
    { key: "u", label: "Undo", act: "undo" },
    { key: "s", label: "Shuffle", act: "shuffle" },
    { key: "n", label: "Next level", act: "next" },
    { key: "m", label: "Sound", act: "sound" },
    { key: "r", label: "New deal", act: "reroll" },
    { key: "?", label: "Keys", act: "keys" },
  ],
  actions: {
    arrows: (scene: Handle<Mahjong>, e: Fired) => scene.nudge(e.key),
    press: (scene: Handle<Mahjong>) => scene.press(),
    hint: (scene: Handle<Mahjong>) => scene.act({ verb: "hint" }),
    undo: (scene: Handle<Mahjong>) => scene.act({ verb: "undo" }),
    shuffle: (scene: Handle<Mahjong>) => scene.act({ verb: "shuffle" }),
    next: (scene: Handle<Mahjong>) => scene.act({ verb: "next" }),
    sound: (scene: Handle<Mahjong>) => scene.mute(),
  },
} satisfies Page<Loaded<Crate>, Mahjong>;

export function make(canvas: HTMLCanvasElement, view: View, opts: Given) {
  const ctx = canvas.getContext("2d")!;
  const quiet = view.still;
  const { mrly } = opts;
  let kept: Kept = { ...fresh(), ...held<Kept>(KEEP, fresh()) };
  const begin = () => {
    const room = roomOf(view);
    return deal({}, { rand: view.rand, w: room.w, h: room.h, level: kept.at, score: kept.score });
  };
  let state: State = begin();
  let free = freeTiles(state);
  let clock = 0;
  let raf = 0;
  let stamp = 0;
  let gone = false;
  let aim = -1;
  let loud = held<boolean>(`${KEEP}-sound`, true);
  let audio: AudioContext | null = null;
  let bed: Bed = { w: 1, h: 1, wall: 1, rise: 1, left: 0, top: 0, line: 1, low: [0, 0] };
  const dressed = (): Inks & Tones => {
    const look = inks(canvas, view);
    const light = bright(look.paper);
    return { ...look, pale: light ? look.paper : look.ink, dark: light ? look.ink : look.paper };
  };
  let tone = dressed();
  let body: Paper | null = null;
  let shade: Paper | null = null;
  let glow: Paper | null = null;
  let aura: Paper | null = null;
  let backdrop: Paper | null = null;
  let pad = 0;
  let font = "";
  let faces: (Paper | null)[] = [];
  let grains: (Grain[] | null)[] = [];
  let dims = new Float32Array(state.tiles.length);
  let births = new Float32Array(state.tiles.length);
  let order: number[] = [];
  let shifts = new Map<number, Shift>();
  const shakes = new Map<number, number>();
  const flights: Flight[] = [];
  const sparks: Spark[] = [];
  const rings: Ring[] = [];
  const words: Word[] = [];
  let fireworks: { at: number; when: number; face: number }[] = [];
  let clearedAt = -1;
  let introAt = -1;
  let swirlAt = -1;

  const tell = () =>
    opts.tell?.({
      rows: [
        ["level", state.level],
        ["score", state.score],
        ["best", `level ${kept.best}, ${kept.top}`],
        ["left", state.tiles.filter((one) => !one.gone).length],
        ["streak", state.streak],
      ],
      over: state.over,
      hints: state.hints,
      undos: state.undos,
      shuffles: state.shuffles,
      cleared: state.cleared,
      stuck: state.stuck,
    });

  const dress = () => {
    tone = dressed();
    font = getComputedStyle(canvas).fontFamily;
  };

  const lay = () => {
    bed = bedOf(view, state.tiles);
  };

  const carve = () => {
    const { w, h, wall } = bed;
    pad = Math.ceil(w * 0.34);
    const fullW = w + 2 * pad;
    const fullH = h + 2 * pad;
    const r = w * ROUND;
    const pw = w - wall;
    const ph = h - wall;
    const [slab, one] = sheet(fullW, fullH);
    one.shadowColor = veil(tone.dark, 0.5);
    one.shadowBlur = wall * 2.2;
    one.shadowOffsetX = wall * 0.5;
    one.shadowOffsetY = wall * 0.7;
    one.fillStyle = blend(tone.accent, tone.dark, 0.25);
    rounded(one, pad + wall, pad + wall, pw, ph, r);
    one.fill();
    one.shadowColor = "transparent";
    const side = one.createLinearGradient(pad, pad, pad + w, pad + h);
    side.addColorStop(0, blend(tone.accent, tone.pale, 0.28));
    side.addColorStop(1, blend(tone.accent, tone.dark, 0.3));
    one.fillStyle = side;
    for (let i = 1; i <= wall; i++) {
      rounded(one, pad + i, pad + i, pw, ph, r);
      one.fill();
    }
    const top = one.createLinearGradient(pad, pad, pad + pw * 0.7, pad + ph);
    top.addColorStop(0, tone.pale);
    top.addColorStop(1, blend(tone.pale, tone.accent, 0.1));
    one.fillStyle = top;
    rounded(one, pad, pad, pw, ph, r);
    one.fill();
    one.strokeStyle = veil(tone.dark, 0.14);
    one.lineWidth = Math.max(1, w * 0.012);
    rounded(one, pad + one.lineWidth / 2, pad + one.lineWidth / 2, pw - one.lineWidth, ph - one.lineWidth, r);
    one.stroke();
    body = slab;
    const [dim, two] = sheet(fullW, fullH);
    two.fillStyle = veil(blend(tone.dark, tone.accent, 0.22), SHADE);
    rounded(two, pad, pad, pw, ph, r);
    two.fill();
    shade = dim;
    const [lit, three] = sheet(fullW, fullH);
    three.shadowColor = tone.accent;
    three.shadowBlur = pad * 0.8;
    three.strokeStyle = tone.accent;
    three.lineWidth = Math.max(2, w * 0.05);
    rounded(three, pad, pad, pw, ph, r);
    three.stroke();
    three.stroke();
    glow = lit;
    const [ring, four] = sheet(fullW, fullH);
    four.strokeStyle = tone.ink;
    four.lineWidth = Math.max(2, w * 0.035);
    rounded(four, pad - four.lineWidth, pad - four.lineWidth, pw + 2 * four.lineWidth, ph + 2 * four.lineWidth, r);
    four.stroke();
    aura = ring;
    faces = state.deck.map(() => null);
    grains = state.deck.map(() => null);
    backdrop = null;
  };

  const faceOf = (at: number): Paper => {
    const cached = faces[at];
    if (cached) return cached;
    const side = Math.max(4, Math.round((bed.w - bed.wall) * (1 - 2 * FACE)));
    const [paper, pen] = sheet(side, side);
    const face = state.deck[at];
    artOf(mrly, face)(pen as never, side, tone, face.hue);
    faces[at] = paper;
    return paper;
  };

  const grainsOf = (at: number): Grain[] => {
    const cached = grains[at];
    if (cached) return cached;
    const paper = faceOf(at);
    const side = paper.width;
    const data = paper.getContext("2d")!.getImageData(0, 0, side, side).data;
    const out: Grain[] = [];
    for (let row = 0; row < GRAINS; row++) {
      for (let col = 0; col < GRAINS; col++) {
        const px = Math.floor(((col + 0.5) / GRAINS) * side);
        const py = Math.floor(((row + 0.5) / GRAINS) * side);
        const i = (py * side + px) * 4;
        if (data[i + 3] < 160) continue;
        out.push({
          x: col / GRAINS - 0.5,
          y: row / GRAINS - 0.5,
          color: `rgb(${data[i]},${data[i + 1]},${data[i + 2]})`,
        });
      }
    }
    grains[at] = out;
    return out;
  };

  const spot = (at: number) => spotOf(bed, state.tiles[at]);

  const sorted = () => {
    order = state.tiles
      .map((_, at) => at)
      .sort((a, b) => {
        const p = state.tiles[a];
        const q = state.tiles[b];
        return p.z - q.z || p.y - q.y || p.x - q.x;
      });
  };

  const entrance = () => {
    const n = state.tiles.length;
    dims = new Float32Array(n);
    births = new Float32Array(n);
    free = freeTiles(state);
    for (const at of state.tiles.keys()) dims[at] = free[at] ? 0 : 1;
    sorted();
    const pace = Math.min(STAGGER / Math.max(1, n), 22);
    if (quiet) return;
    introAt = clock;
    for (const [i, at] of order.entries()) births[at] = clock + i * pace;
  };

  const setup = () => {
    dress();
    lay();
    carve();
    entrance();
  };

  const burst = (face: number, x: number, y: number, side: number, force = 1) => {
    if (quiet) return;
    const list = grainsOf(face);
    const room = Math.max(0, MOST - sparks.length);
    const stride = Math.max(1, Math.ceil(list.length / Math.max(1, room)));
    for (let i = 0; i < list.length; i += stride) {
      const grain = list[i];
      const away = Math.atan2(grain.y, grain.x) + (view.rand() - 0.5) * 0.7;
      const speed = (0.2 + view.rand() * 0.9) * bed.w * 0.0065 * force;
      sparks.push({
        x: x + grain.x * side,
        y: y + grain.y * side,
        vx: Math.cos(away) * speed,
        vy: Math.sin(away) * speed - bed.w * 0.0012,
        born: clock,
        life: LIFE * (0.6 + view.rand() * 0.6),
        color: grain.color,
        size: (side / GRAINS) * (0.55 + view.rand() * 0.5),
      });
    }
  };

  const blip = (step: number) => {
    if (!loud) return;
    try {
      audio ??= new AudioContext();
      if (audio.state === "suspended") audio.resume();
      const note = NOTES[Math.min(step, NOTES.length - 1)];
      arp(audio, audio.destination, audio.currentTime, { note, level: 0.05, cutoff: 2600 });
    } catch {
      loud = false;
    }
  };

  const remember = () => {
    kept = {
      at: state.over ? 1 : state.cleared ? state.level + 1 : state.level,
      score: state.over ? 0 : state.cleared ? state.score : kept.score,
      best: Math.max(kept.best, state.level + (state.cleared ? 1 : 0)),
      top: Math.max(kept.top, state.score),
    };
    hold(KEEP, kept);
  };

  const settle = (before: State) => {
    free = freeTiles(state);
    if (before.past.length < state.past.length && state.past.length) {
      const { a, b } = state.past.at(-1)!;
      const [ax, ay] = spot(a);
      const [bx, by] = spot(b);
      blip(state.streak - 1);
      if (!quiet) {
        const earned = multiplier(state.streak);
        flights.push({
          face: state.tiles[a].face,
          ax,
          ay,
          bx,
          by,
          born: clock,
          word: `+${POINTS * earned}`,
          big: state.streak > 1 && multiplier(state.streak - 1) < earned ? `×${earned}` : "",
        });
      }
      if (aim === a || aim === b) aim = -1;
    }
    if (state.level !== before.level) {
      flights.length = 0;
      fireworks = [];
      lay();
      carve();
      entrance();
      clearedAt = -1;
    } else if (state.cleared && !before.cleared) {
      clearedAt = clock;
      if (!quiet)
        fireworks = Array.from({ length: Math.min(FIREWORKS, state.tiles.length) }, (_, i) => {
          const at = Math.floor(view.rand() * state.tiles.length);
          return { at, when: clock + FLY + i * PACE, face: state.tiles[at].face };
        });
    } else if (before.past.length > state.past.length && !quiet) {
      const undone = before.past.at(-1)!;
      for (const at of [undone.a, undone.b]) {
        const [x, y] = spot(at);
        shifts.set(at, { from: [x, y - bed.h * 0.8], born: clock, face: state.tiles[at].face, span: SWIRL / 2 });
      }
    }
    if (before.shuffles > state.shuffles) {
      const width = bed.w;
      lay();
      if (bed.w !== width) carve();
      sorted();
      if (!quiet) {
        swirlAt = clock;
        shifts = new Map();
        const cx = view.w / 2;
        const cy = view.h / 2;
        for (const at of state.tiles.keys()) {
          if (state.tiles[at].gone) continue;
          const [x, y] = spot(at);
          shifts.set(at, {
            from: [cx + (x - cx) * 0.2, cy + (y - cy) * 0.2],
            born: clock + view.rand() * 90,
            face: before.tiles[at].face,
            span: SWIRL,
          });
        }
      }
    }
    remember();
    tell();
    wake();
  };

  const act = (one: Act) => {
    const before = state;
    state = step(state, one);
    if (state === before) {
      if (one.verb === "pick" && !quiet) shakes.set(one.args.at, clock);
      wake();
      return;
    }
    settle(before);
  };

  const hitAt = (u: number, v: number) => {
    const px = u * view.w;
    const py = v * view.h;
    for (let i = order.length - 1; i >= 0; i--) {
      const at = order[i];
      if (state.tiles[at].gone) continue;
      const [x, y] = spot(at);
      if (px >= x && px < x + bed.w && py >= y && py < y + bed.h) return at;
    }
    return -1;
  };

  const lift = (u: number, v: number) => {
    if (state.cleared) {
      act({ verb: "next" });
      return null;
    }
    aim = -1;
    const at = hitAt(u, v);
    if (at >= 0) act({ verb: "pick", args: { at } });
    else wake();
    return null;
  };

  const nudge = (key?: string) => {
    const open = state.tiles.flatMap((_, at) => (free[at] ? [at] : []));
    if (!open.length) return;
    const place = (at: number) => {
      const [x, y] = spot(at);
      return [x + bed.w / 2, y + bed.h / 2];
    };
    if (aim < 0 || !free[aim]) {
      const middle = [view.w / 2, view.h / 2];
      aim = open.reduce((best, at) => {
        const [x, y] = place(at);
        const [bx, by] = place(best);
        return Math.hypot(x - middle[0], y - middle[1]) < Math.hypot(bx - middle[0], by - middle[1]) ? at : best;
      }, open[0]);
      wake();
      return;
    }
    const dir = TURNS[lower(key)];
    const [dx, dy] = [
      [0, -1],
      [1, 0],
      [0, 1],
      [-1, 0],
    ][dir] ?? [0, 0];
    const [ox, oy] = place(aim);
    let best = aim;
    let score = Infinity;
    for (const at of open) {
      if (at === aim) continue;
      const [x, y] = place(at);
      const ahead = (x - ox) * dx + (y - oy) * dy;
      if (ahead <= 0) continue;
      const across = Math.abs((x - ox) * dy) + Math.abs((y - oy) * dx);
      const cost = ahead + 2 * across;
      if (cost < score) {
        score = cost;
        best = at;
      }
    }
    aim = best;
    wake();
  };

  const press = () => {
    if (state.cleared) act({ verb: "next" });
    else if (aim < 0) nudge();
    else act({ verb: "pick", args: { at: aim } });
  };

  const mute = () => {
    loud = !loud;
    hold(`${KEEP}-sound`, loud);
  };

  const restart = () => {
    kept = { ...kept, at: 1, score: 0 };
    hold(KEEP, kept);
    state = begin();
    flights.length = 0;
    sparks.length = 0;
    rings.length = 0;
    words.length = 0;
    fireworks = [];
    clearedAt = -1;
    setup();
    tell();
    wake();
  };

  const write = (text: string, x: number, y: number, size: number, color: string, edge = "") => {
    ctx.font = `${Math.round(size)}px ${font}`;
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    if (edge) {
      ctx.strokeStyle = edge;
      ctx.lineJoin = "round";
      ctx.lineWidth = size * 0.16;
      ctx.strokeText(text, x, y, view.w);
    }
    ctx.fillStyle = color;
    ctx.fillText(text, x, y, view.w);
  };

  const drawTile = (at: number, x: number, y: number, face: number, scale = 1) => {
    if (scale !== 1) {
      ctx.save();
      ctx.translate(x + bed.w / 2, y + bed.h / 2);
      ctx.scale(scale, scale);
      ctx.translate(-x - bed.w / 2, -y - bed.h / 2);
    }
    ctx.drawImage(body!, x - pad, y - pad);
    const paper = faceOf(face);
    const plateW = bed.w - bed.wall;
    const plateH = bed.h - bed.wall;
    ctx.drawImage(paper, x + (plateW - paper.width) / 2, y + (plateH - paper.height) / 2);
    const dim = at >= 0 ? dims[at] : 0;
    if (dim > 0.01) {
      const was = ctx.globalAlpha;
      ctx.globalAlpha = was * dim;
      ctx.drawImage(shade!, x - pad, y - pad);
      ctx.globalAlpha = was;
    }
    if (scale !== 1) ctx.restore();
  };

  const spawnFireworks = () => {
    while (fireworks.length && fireworks[0].when <= clock) {
      const one = fireworks.shift()!;
      const [x, y] = spot(one.at);
      burst(one.face, x + bed.w / 2, y + bed.h / 2, bed.w, 1.6);
      rings.push({
        x: x + bed.w / 2,
        y: y + bed.h / 2,
        born: clock,
        life: RING * 1.4,
        color: tone.accent,
        size: bed.w * 1.3,
      });
    }
  };

  const fly = () => {
    for (let i = flights.length - 1; i >= 0; i--) {
      const one = flights[i];
      const t = (clock - one.born) / FLY;
      if (t >= 1) {
        const mx = (one.ax + one.bx) / 2 + bed.w / 2;
        const my = (one.ay + one.by) / 2 + bed.h / 2;
        burst(one.face, mx, my, bed.w * 1.5);
        rings.push({ x: mx, y: my, born: clock, life: RING, color: tone.accent, size: bed.w * 1.6 });
        words.push({ text: one.word, x: mx, y: my, born: clock, color: tone.ink, size: bed.w * 0.32 });
        if (one.big)
          words.push({
            text: one.big,
            x: mx,
            y: my + bed.h * 0.5,
            born: clock,
            color: tone.accent,
            size: bed.w * 0.95,
          });
        flights.splice(i, 1);
        continue;
      }
      const k = t * t;
      const mx = (one.ax + one.bx) / 2;
      const my = (one.ay + one.by) / 2;
      const lift = Math.sin(t * Math.PI) * bed.h * 0.35;
      drawTile(-1, one.ax + (mx - one.ax) * k, one.ay + (my - one.ay) * k - lift, one.face, 1 + 0.12 * t);
      drawTile(-1, one.bx + (mx - one.bx) * k, one.by + (my - one.by) * k - lift, one.face, 1 + 0.12 * t);
    }
  };

  const dust = (dt: number) => {
    for (let i = sparks.length - 1; i >= 0; i--) {
      const one = sparks[i];
      const t = (clock - one.born) / one.life;
      if (t >= 1) {
        sparks[i] = sparks[sparks.length - 1];
        sparks.pop();
        continue;
      }
      one.x += one.vx * dt;
      one.y += one.vy * dt;
      one.vy += bed.w * 0.000018 * dt;
      one.vx *= 0.995;
      ctx.globalAlpha = 1 - t * t;
      ctx.fillStyle = one.color;
      const size = one.size * (1 - t * 0.5);
      ctx.fillRect(one.x - size / 2, one.y - size / 2, size, size);
    }
    ctx.globalAlpha = 1;
    for (let i = rings.length - 1; i >= 0; i--) {
      const one = rings[i];
      const t = (clock - one.born) / one.life;
      if (t >= 1) {
        rings.splice(i, 1);
        continue;
      }
      ctx.globalAlpha = (1 - t) * 0.45;
      ctx.strokeStyle = one.color;
      ctx.lineWidth = Math.max(1, bed.w * 0.05 * (1 - t));
      ctx.beginPath();
      ctx.arc(one.x, one.y, eased(t) * one.size, 0, Math.PI * 2);
      ctx.stroke();
    }
    ctx.globalAlpha = 1;
    for (let i = words.length - 1; i >= 0; i--) {
      const one = words[i];
      const t = (clock - one.born) / WORD;
      if (t >= 1) {
        words.splice(i, 1);
        continue;
      }
      ctx.globalAlpha = t < 0.7 ? 1 : (1 - t) / 0.3;
      write(
        one.text,
        one.x,
        one.y - eased(t) * bed.h * 0.9,
        one.size * (1 + 0.5 * Math.sin(Math.min(t * 5, 1) * Math.PI)),
        one.color,
        tone.paper,
      );
    }
    ctx.globalAlpha = 1;
  };

  const sweep = () => {
    if (clearedAt < 0) return;
    const age = clock - clearedAt;
    const t = quiet ? 1 : clamp01(age / SWEEP);
    if (!quiet && t < 1) {
      const spread = Math.hypot(view.w, view.h);
      const at = eased(t) * (spread + view.w * 0.4) - view.w * 0.2;
      const band = ctx.createLinearGradient(at - view.w * 0.2, 0, at + view.w * 0.2, view.h * 0.5);
      band.addColorStop(0, veil(tone.accent, 0));
      band.addColorStop(0.5, veil(tone.accent, 0.3));
      band.addColorStop(1, veil(tone.accent, 0));
      ctx.fillStyle = band;
      ctx.fillRect(0, 0, view.w, view.h);
    }
    const shown = quiet ? 1 : clamp01((age - 180) / 320);
    if (shown <= 0) return;
    const middle = view.h / 2;
    const size = Math.min(view.w, view.h) * 0.12;
    ctx.globalAlpha = shown;
    const hush = ctx.createRadialGradient(view.w / 2, middle, 0, view.w / 2, middle, size * 4.5);
    hush.addColorStop(0, veil(tone.paper, 0.9));
    hush.addColorStop(0.6, veil(tone.paper, 0.72));
    hush.addColorStop(1, veil(tone.paper, 0));
    ctx.fillStyle = hush;
    ctx.fillRect(0, 0, view.w, view.h);
    write(`Level ${state.level} clear`, view.w / 2, middle - size * 0.6, size * (0.85 + 0.15 * eased(shown)), tone.ink);
    write(`${state.score}`, view.w / 2, middle + size * 0.45, size * 0.55, tone.accent);
    write(`next: ${LABELS[newest(state.level + 1)]}`, view.w / 2, middle + size * 1.15, size * 0.32, tone.ink);
    ctx.globalAlpha = 1;
  };

  const heading = () => {
    const left = state.tiles.filter((one) => !one.gone).length;
    const mult = multiplier(state.streak);
    const say = state.over
      ? `Out of moves, level ${state.level}, ${state.score}`
      : state.stuck
        ? `No pairs left: shuffle or undo`
        : `Level ${state.level}, ${LABELS[newest(state.level)]}, ${state.score}${mult > 1 ? `, ×${mult}` : ""}`;
    const y = bed.line * 0.5;
    write(say, view.w / 2, y, bed.line * 0.5, tone.ink);
    const width = Math.min(view.w * 0.5, bed.line * 7);
    const height = Math.max(2, Math.round(bed.line * 0.06));
    const x = (view.w - width) / 2;
    const ready = 1 - left / Math.max(1, state.tiles.length);
    ctx.fillStyle = veil(tone.ink, 0.12);
    ctx.fillRect(x, bed.line * 0.86, width, height);
    ctx.fillStyle = tone.accent;
    ctx.fillRect(x, bed.line * 0.86, width * ready, height);
  };

  const backdropOf = (): Paper | null => {
    if (backdrop) return backdrop;
    const side = Math.round(Math.max(view.w, view.h) * 0.95);
    const face = deepened(state.deck.find((one) => one.suit === newest(state.level)));
    if (!face) return null;
    const [paper, pen] = sheet(side, side);
    artOf(mrly, face)(pen as never, side, { ...tone, hue: () => tone.accent }, face.hue);
    backdrop = paper;
    return paper;
  };

  const paint = (dt = 0) => {
    clear(ctx, view);
    const art = backdropOf();
    if (art) {
      ctx.globalAlpha = FAINT;
      ctx.drawImage(art, (view.w - art.width) / 2, (view.h - art.height) / 2);
      ctx.globalAlpha = 1;
    }
    heading();
    const swirling = swirlAt >= 0 && clock - swirlAt < SWIRL + 100;
    for (const at of order) {
      const one = state.tiles[at];
      if (one.gone) continue;
      const target = free[at] ? 0 : 1;
      dims[at] = quiet ? target : dims[at] + (target - dims[at]) * clamp01(dt * DIMMING);
      const [hx, hy] = spot(at);
      let x = hx;
      let y = hy;
      let face = one.face;
      if (clock < births[at]) continue;
      const age = (clock - births[at]) / FALL;
      if (age < 1 && !quiet) {
        const dropped = bounce(clamp01(age));
        y = hy - (1 - dropped) * (view.h * 0.5 + bed.h);
        ctx.globalAlpha = clamp01(age * 4);
      }
      const moved = shifts.get(at);
      if (moved) {
        const t = (clock - moved.born) / moved.span;
        if (t >= 1) shifts.delete(at);
        else if (t > 0) {
          const k = eased(Math.abs(t * 2 - 1));
          x = moved.from[0] + (hx - moved.from[0]) * k;
          y = moved.from[1] + (hy - moved.from[1]) * k;
          if (t < 0.5) face = moved.face;
        } else if (swirling) face = moved.face;
      }
      const shook = shakes.get(at);
      if (shook !== undefined) {
        const t = (clock - shook) / SHAKE;
        if (t >= 1) shakes.delete(at);
        else x += Math.sin(t * 22) * (1 - t) * bed.w * 0.06;
      }
      const picked = state.picked === at;
      const hinted = state.hint.includes(at);
      if (picked) y -= bed.rise * 0.8;
      drawTile(at, x, y, face);
      ctx.globalAlpha = 1;
      if (picked || hinted) {
        const pulse = quiet ? 1 : 0.65 + 0.35 * Math.sin(clock * 0.008);
        ctx.globalAlpha = hinted && !picked ? pulse : 0.85 + 0.15 * pulse;
        ctx.drawImage(glow!, x - pad, y - pad);
        ctx.globalAlpha = 1;
      }
      if (aim === at) ctx.drawImage(aura!, x - pad, y - pad);
    }
    spawnFireworks();
    fly();
    dust(dt);
    sweep();
    view.drew();
  };

  const busy = () =>
    state.picked >= 0 ||
    state.hint.length > 0 ||
    flights.length > 0 ||
    sparks.length > 0 ||
    rings.length > 0 ||
    words.length > 0 ||
    fireworks.length > 0 ||
    shifts.size > 0 ||
    shakes.size > 0 ||
    (clearedAt >= 0 && clock - clearedAt < SWEEP + 400) ||
    (introAt >= 0 && clock - introAt < STAGGER + FALL + 200) ||
    state.tiles.some((one, at) => !one.gone && Math.abs(dims[at] - (free[at] ? 0 : 1)) > CALM);

  const tick = (now: number) => {
    raf = 0;
    if (gone) return;
    const dt = stamp ? Math.min(now - stamp, 80) : 16;
    stamp = now;
    clock += dt;
    if (clearedAt >= 0 && clock - clearedAt > CLEARED) {
      act({ verb: "next" });
    }
    paint(dt);
    if (busy() || clearedAt >= 0) wake();
    else stamp = 0;
  };

  const wake = () => {
    if (gone) return;
    if (quiet) {
      paint();
      return;
    }
    if (!raf) raf = requestAnimationFrame(tick);
  };

  setup();
  tell();
  wake();

  return {
    draw: () => {
      paint();
      wake();
    },
    still: true,
    size: () => {
      lay();
      carve();
      paint();
    },
    theme: () => {
      dress();
      carve();
      paint();
    },
    restart,
    lift,
    nudge,
    press,
    act,
    mute,
    state: () => state,
    stop: () => {
      gone = true;
      if (raf) cancelAnimationFrame(raf);
      void audio?.close();
    },
  };
}
