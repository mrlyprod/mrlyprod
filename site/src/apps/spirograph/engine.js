import { home } from "../../lib/designs/engine.js";

export const KINDS = [
  ["in", "Inside a circle"],
  ["out", "Outside a circle"],
  ["line", "A line"],
  ["polyin", "Inside a polygon"],
  ["polyout", "Outside a polygon"],
];
export const MODES = [
  ["fill", "Filled cells"],
  ["void", "Empty cells"],
  ["both", "Both"],
  ["corners", "Corners"],
];
export const INKS = [
  ["kind", "By kind"],
  ["wheel", "Six inks"],
  ["one", "One ink"],
];
export const BASES = [
  [3, "3"],
  [2, "2"],
];
export const NUMBERS = [3, 5, 7];
export const SEATS = ["Fill", "Void", "Corner"];
export const KIND = ["blue", "orange", "teal"];
export const SIX = ["blue", "orange", "yellow", "green", "pink", "indigo"];
export const BANDS = 8;
export const PAD = 24;
export const PACE = 0.5;
export const HOLD = 3000;
export const EASE = 2;
export const LEAST = 3000;
export const MOST = 60000;
export const LIVE = 480000;
export const DENSITY = 720;
export const CURVES = 64;
export const RASTER = [512, 256];
export const ORBIT = 720;
export const GLANCE = 90;
export const TRACKS = [
  ["Classic", { kind: "in", ring: 7, wheel: 3, sides: 4, laps: 2 }],
  ["Three lobes", { kind: "in", ring: 3, wheel: 1, sides: 4, laps: 2 }],
  ["Four lobes", { kind: "in", ring: 4, wheel: 1, sides: 4, laps: 2 }],
  ["Star", { kind: "in", ring: 5, wheel: 2, sides: 4, laps: 2 }],
  ["Rose", { kind: "in", ring: 5, wheel: 3, sides: 4, laps: 2 }],
  ["Weave", { kind: "in", ring: 8, wheel: 3, sides: 4, laps: 2 }],
  ["Limacon", { kind: "out", ring: 1, wheel: 1, sides: 4, laps: 2 }],
  ["Two lobes", { kind: "out", ring: 2, wheel: 1, sides: 4, laps: 2 }],
  ["Bloom", { kind: "out", ring: 5, wheel: 3, sides: 4, laps: 2 }],
  ["Cycloid", { kind: "line", ring: 7, wheel: 3, sides: 4, laps: 2 }],
  ["Square", { kind: "polyin", ring: 7, wheel: 3, sides: 4, laps: 2 }],
  ["Hexagon", { kind: "polyout", ring: 5, wheel: 2, sides: 6, laps: 2 }],
];
export const ROUND = "the fill needs a circle track: the wall of a line or a polygon roulette need not close";
const FEW = 3;
const INSET = 1e-9;
const TAU = Math.PI * 2;

export const message = (error) => String(error?.message ?? error);

/* DESIGN */

export function roll(math, base, rand) {
  return String(math.bang.random_code(2, base, new math.Rng(Math.floor(rand() * 2 ** 32)), FEW));
}

export function named(math, code, base) {
  return math.name.of(String(code), 2, base) ?? null;
}

export function resolve(math, value) {
  const text = String(value.code ?? "").trim();
  return text === "" ? home(math, 2, value.base) : text;
}

export function cap(math, number) {
  return math.bang.level_cap(number, 2, math.spirograph.PENCIL_CAP());
}

export function seat(math, value, code) {
  const level = Math.max(1, Math.min(value.level, cap(math, value.number)));
  const cell = math.two.create(code, value.number, level, 0, value.base);
  const [height, width] = cell.shape;
  const pencils = math.spirograph.pencils(
    cell.types,
    width,
    height,
    value.mode,
    value.reach,
    value.jitter,
    value.seed >>> 0,
  );
  return { cell, level, width, height, pencils };
}

/* TRACK */

export function legal(kind, ring, wheel, sides) {
  if (kind === "in") {
    const r = Math.max(ring, 2);
    return [r, Math.min(wheel, r - 1)];
  }
  if (kind !== "polyin") return [ring, wheel];
  const cos = Math.cos(Math.PI / sides);
  const r = Math.max(ring, Math.ceil((1 + INSET) / cos));
  return [r, Math.min(wheel, Math.floor(r * cos - INSET))];
}

/* STUDY */

export function bands(pencils) {
  let lo = Infinity;
  let hi = -Infinity;
  const radii = pencils.map((p) => Math.hypot(p.x, p.y));
  for (const r of radii) {
    lo = Math.min(lo, r);
    hi = Math.max(hi, r);
  }
  return radii.map((r) => (hi > lo ? Math.min(BANDS - 1, Math.floor(((r - lo) / (hi - lo)) * BANDS)) : BANDS - 1));
}

export function study(math, value, code) {
  const s = math.spirograph;
  const { cell, level, width, height, pencils } = seat(math, value, code);
  const [ring, wheel] = legal(value.kind, value.ring, value.wheel, value.sides);
  const track = s.track(value.kind, ring, wheel, value.sides, value.laps);
  const exact = value.jitter === 0;
  const n = pencils.length;
  const turns = Math.abs(s.turn(track, track.total)) / TAU;
  const beats = Math.max(turns, track.orbits);
  const samples = n
    ? Math.max(2, Math.min(Math.floor(s.POINT_CAP() / n), Math.floor(LIVE / n), Math.ceil(DENSITY * beats) + 1))
    : 2;
  const trace = n ? s.trace(track, pencils, samples) : new Float32Array(0);
  const frame = Array.from(s.frame(track, pencils));
  const seats = s.seats(pencils);
  const distinct = n ? s.distinct(track, pencils, exact) : 0;
  const nodes = n ? s.nodes(track, pencils, exact) : undefined;
  const round = value.kind === "in" || value.kind === "out";
  const side = s.cell(width, height, value.reach);
  const facts = {
    code,
    name: named(math, code, value.base),
    base: value.base,
    level,
    width,
    height,
    pencils: n,
    fills: seats.fills,
    voids: seats.voids,
    corners: seats.corners,
    distinct,
    nodes: nodes === undefined ? null : Number(nodes),
    ratio: [...track.ratio],
    orbits: track.orbits,
    fold: track.fold,
    turns,
    total: track.total,
    samples,
    side,
    reach: value.reach,
    round,
    kind: value.kind,
  };
  return {
    code,
    level,
    cell,
    track,
    pencils,
    samples,
    trace,
    frame,
    side,
    turns,
    beats,
    exact,
    round,
    bands: bands(pencils),
    kinds: pencils.map((p) => Math.max(0, SEATS.indexOf(p.kind))),
    facts,
  };
}

export function cover(math, plan) {
  if (!plan.round) return { fault: ROUND };
  if (!plan.pencils.length) return { fault: "no pencil draws a wall" };
  const s = math.spirograph;
  const raster = RASTER[plan.facts.distinct > CURVES ? 1 : 0];
  try {
    return {
      ...s.cover(plan.track, plan.pencils, plan.exact, ORBIT * plan.track.orbits, raster),
      disc: s.disc(plan.track, plan.pencils),
    };
  } catch (error) {
    return { fault: message(error) };
  }
}

/* TIME */

export function span(beats) {
  return Math.min(MOST, Math.max(LEAST, (beats / PACE) * 1000));
}

export function played(from, t, length) {
  return Math.min(1, from + Math.max(0, t) / length);
}

export function share(t, length, still) {
  if (still) return 1;
  const loop = length + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, phase / length);
}

export function index(part, samples) {
  return Math.round(part * (samples - 1));
}

/* GLANCE */

export function glance(math, value, code, patch) {
  const s = math.spirograph;
  const { pencils } = seat(math, { ...value, level: 1 }, code);
  const [ring, wheel] = legal(patch.kind, patch.ring, patch.wheel, patch.sides);
  const track = s.track(patch.kind, ring, wheel, patch.sides, patch.laps);
  const beats = Math.max(Math.abs(s.turn(track, track.total)) / TAU, track.orbits);
  const samples = Math.max(2, Math.ceil(GLANCE * beats) + 1);
  return {
    count: pencils.length,
    samples,
    trace: s.trace(track, pencils, samples),
    frame: Array.from(s.frame(track, pencils)),
  };
}

/* FIT */

export function fit(frame, w, h, pad) {
  const [x0, y0, x1, y1] = frame;
  const gap = Math.min(pad, Math.min(w, h) / 4);
  const k = Math.min((w - 2 * gap) / (x1 - x0 || 1), (h - 2 * gap) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}
