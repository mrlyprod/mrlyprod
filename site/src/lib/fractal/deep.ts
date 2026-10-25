import type { Program, Quads, View as Screen, Target, Uniforms } from "../gl/gl2.ts";
import { blend, fill, program, quads, target, texture, tier } from "../gl/gl2.ts";
import { smooth } from "../gl/vec.ts";
import { rng } from "../rand.ts";
import { FLOOR, MIX, TILE, WIDE } from "./consts.ts";
import type { Orbit, Pair, Point } from "./orbit.ts";
import { apart, ball, exact, fixed, float, LIMIT, nucleus, orbit, shift, size, slice, WORD } from "./orbit.ts";
import type { Tile } from "./probe.ts";
import { edge, prefer, score, spread } from "./probe.ts";
import { COPY, INK, LINE, shader, tones, unit } from "./shader.ts";
import type { View } from "./view.ts";

/* TYPES */

export type Box = { xMin: number; xMax: number; yMin: number; yMax: number };

export type Look = { paper: string; accent: string };

export type Stage = Screen & { look: () => Look };

export type Options = {
  kind?: "set" | "julia";
  seed?: number;
  palette?: string;
  speed?: number;
  presets?: Pair[];
  home?: Box | null;
  path?: number;
  bottom?: number;
  still?: View | null;
};

export type Deep = {
  draw: () => void;
  theme: () => void;
  drop: () => void;
  fly: (from?: View | null) => void;
  halt: () => View;
  look: (v: View, c?: Pair | null) => void;
  flying: () => boolean;
  at: () => View;
};

type Ref = { at: Point; o: Orbit; tex: WebGLTexture | null; keep: boolean };

type Crit = { j: number; cf: Pair; c: bigint[]; o: Orbit };

type Prep = { j: number; c: bigint[] | null; cf: Pair | null; crit: Orbit | null; base: Ref };

type Stop = { at: Point; v: Pair | null };

type Found = { at: Point; n: Pair; p: number };

type Hit = { at: Point; s: Pair; span: number; p: number };

type Best = Hit & { far: number };

type Spot = Tile & { at: Point; hit?: Hit | null };

type Scan = { k: number; to: Point; run: Generator<undefined, Hit | null, unknown> };

type Pending = { k: number; v: View; sync: WebGLSync };

type Arrive = {
  t: number;
  la: number;
  le: number;
  go: number;
  cruise: number;
  end: number;
  angle: number;
  p: number;
  to: Point;
  gap: Pair;
  push: number[];
};

type Dive = Prep & {
  t0: number;
  opening: number;
  tilt: number;
  phase: "dive" | "arrive" | "fade" | "rest";
  stops: Stop[];
  k: number;
  pending: Pending | null;
  scan: Scan | null;
  goal: Hit | null;
  land: number;
  arrive: Arrive | null;
  rim: Point | null;
  floor: number;
  frozen: View | null;
  heir: Prep | null;
  fade: number;
};

type Ring = [Pair, number[], number];

const RATE = 0.5;
const GLIDE = 1500;
const HOLD = 2000;
export const FADE = 800;
const FRAMED = 0.4;
const EXTENT = 2.5;
export const SPAN = EXTENT / FRAMED;
const BASE = 256;
const MORE = 96;
const LEASH = 2 ** 12;
const NEAR = 2;
const REACH = 64;
const SOON = 24;
const CLOSE = 5;
const WINDOW = 12;
const LATE = 30;
const HEAVY = { set: 1 / 6, julia: 1 / 3 };
const DRIFT = 1 / 64000;
const FRAME = 1000 / 60;
const TURN = 1 / 2048;
const CYCLE = 1 / 256;
const WIDEN = 2;
const REACHED = 0.08;
const CAP = 0.12;
const FIT = 0.64;
const STEP = 1;
const LEAD = 2;
const SEEK = 1 / 8;
const TRAIL = 96;
const PEN = 3;
const HALO = 2;
const HUB = [5, 1.5];
const MARK = [8, 2];
const SHADE = 0.5;
const DOTS = 8;
const SPIN = 0x1f83d9ab;
const PACK = 0x88eb;
const SYNC = 0x9117;
const DONE = [0x911a, 0x911c];
const LOST = 0x911d;
const BUDGET = 3;
const GOLD = 0x9e3779b1;
const TIERS = {
  phone: { probe: 48, picks: 3, cap: 4096, budget: 800, pixels: 0.5e6 },
  desk: { probe: 64, picks: 4, cap: 8192, budget: 2000, pixels: 1.2e6 },
};
const HOME = { set: [-0.75, 0], julia: [0, 0] };

/* DEEP */

function across(box: Box, w: number, h: number) {
  const long = (box.xMax - box.xMin) * WIDEN;
  const tall = (box.yMax - box.yMin) * WIDEN;
  const shape = w / h;
  const [fw, fh] = shape > long / tall ? [tall * shape, tall] : [long, long / shape];
  return Math.min(fw, fh);
}

const hermite = (s: number) => [2 * s ** 3 - 3 * s * s + 1, s ** 3 - 2 * s * s + s, s ** 3 - s * s];

const slope = (s: number) => [6 * s * s - 6 * s, 3 * s * s - 4 * s + 1, 3 * s * s - 2 * s];

const tangent = (a: readonly number[], b: readonly number[]): Pair => {
  const la = Math.hypot(a[0], a[1]);
  const lb = Math.hypot(b[0], b[1]);
  return la + lb > 0 ? [(lb * a[0] + la * b[0]) / (la + lb), (lb * a[1] + la * b[1]) / (la + lb)] : [0, 0];
};

export function deep(
  gl: WebGL2RenderingContext,
  view: Stage,
  {
    kind = "set",
    seed = 0,
    palette = "",
    speed = 1,
    presets,
    home = null,
    path = 0,
    bottom = FLOOR,
    still = null,
  }: Options = {},
): Deep {
  const julia = kind === "julia";
  if (julia && !presets?.length) throw new Error("deep: a julia dive needs presets");
  const classic = palette === "classic";
  const plan = TIERS[tier(view) === "phone" ? "phone" : "desk"];
  const rate = (RATE * speed) / 1000;
  const deepest = Math.log2(SPAN / bottom);
  const side = plan.probe;
  const pace = STEP / rate;
  const ease = STEP * Math.LN2;
  const gain = (pace * (1 - 2 ** -STEP)) / ease;
  const reach = (FIT * CAP * speed * gain) / 1000;
  const ahead = LEAD * STEP;
  const paint = program(gl, shader(false, classic));
  const sense = program(gl, shader(true, false));
  const sight = target(gl, side, side);
  const pixels = new Uint8Array(side * side * 4);
  const fence = gl.fenceSync(SYNC, 0);
  const lazy = Boolean(fence);
  if (fence) gl.deleteSync(fence);
  const buffer = lazy ? gl.createBuffer() : null;
  if (lazy) {
    gl.bindBuffer(PACK, buffer);
    gl.bufferData(PACK, pixels.length, gl.STREAM_READ);
    gl.bindBuffer(PACK, null);
  }
  const now = () => performance.now();
  let by = Infinity;
  let copy: Program | null = null;
  let bin: Target | null = null;
  let pen: Quads | null = null;
  let nib: Program | null = null;
  const spin = (t: number) => (way * TURN * t) / FRAME;
  const tone = tones(palette, seed);
  const rest = HOME[julia ? "julia" : "set"].map((v) => fixed(v)) as Point;
  const origin: Point = home ? [fixed((home.xMin + home.xMax) / 2), fixed((home.yMin + home.yMax) / 2)] : rest;
  const opening = () => (home ? across(home, view.w, view.h) : SPAN);
  const way = rng((seed ^ SPIN) >>> 0)() < 0.5 ? 1 : -1;
  let dive: Dive | null = null;
  let cur: Ref | null = null;
  let next: Ref | null = null;
  let upcoming: Crit | null = null;
  let ink: Look | null = null;
  let shades: Uniforms | null = null;
  const upload = (ref: Ref) => {
    if (ref.tex) return ref;
    const a = ref.o.data;
    const data = julia ? new Float32Array(a.length + ref.o.crit.data.length) : a;
    if (julia) {
      data.set(a);
      data.set(ref.o.crit.data, a.length);
    }
    ref.tex = texture(gl, { w: WIDE, h: julia ? ref.o.rows * 2 : ref.o.rows, format: "rg32f", data });
    return ref;
  };
  const release = (ref: Ref | null | undefined) => {
    if (ref?.tex && !ref.keep) gl.deleteTexture(ref.tex);
    if (ref && !ref.keep) ref.tex = null;
  };
  const reference = (at: Point, dv: Prep): Ref => {
    const o = julia ? orbit({ c: dv.c!, z0: at, max: plan.cap }) : orbit({ c: at, max: plan.cap });
    if (julia) o.crit = dv.crit!;
    return { at, o, tex: null, keep: false };
  };
  const crit = (j: number): Crit => {
    const cf = presets![(seed + j) % presets!.length];
    const c = cf.map((v) => fixed(v));
    return { j, cf, c, o: orbit({ c, z0: [0n, 0n], max: plan.cap }) };
  };
  const settled: Ref | null = julia ? null : { at: rest, o: orbit({ c: rest, max: plan.cap }), tex: null, keep: true };
  settled?.o.step();
  if (settled) upload(settled);
  const prepare = (j: number): Prep => {
    if (!julia) return { j, c: null, cf: null, crit: null, base: settled! };
    const made = upcoming?.j === j ? upcoming : crit(j);
    made.o.step();
    return {
      j,
      c: made.c,
      cf: made.cf,
      crit: made.o,
      base: upload({ at: [0n, 0n], o: made.o, tex: null, keep: false }),
    };
  };
  const begin = (prep: Prep, t0: number, from: View | null = null, ref = prep.base) => {
    for (const one of [cur, next, dive?.base]) if (one !== prep.base && one !== ref) release(one);
    const span = from ? from.span : opening();
    dive = {
      ...prep,
      t0,
      opening: span,
      tilt: from ? from.angle - spin(t0) : 0,
      phase: "dive",
      stops: [{ at: from ? from.centre : origin, v: [0, 0] }],
      k: 0,
      pending: null,
      scan: null,
      goal: null,
      land: Infinity,
      arrive: null,
      rim: null,
      floor: t0 + Math.max(0, Math.log2(span / bottom)) / rate,
      frozen: null,
      heir: null,
      fade: 0,
    };
    cur = ref;
    next = null;
    upcoming = julia && !still ? crit(prep.j + 1) : null;
  };
  const knot = (dv: Dive, k: number) => dv.t0 + k * pace;
  const width = (dv: Dive, k: number) => dv.opening * 2 ** (-STEP * k);
  const segment = (dv: Dive, t: number) => Math.floor(Math.max(0, t - dv.t0) / pace);
  const chord = (dv: Dive, k: number) => apart(dv.stops[k + 1].at, dv.stops[k].at).map((d) => d / width(dv, k) / gain);
  const known = (dv: Dive) => {
    let n = dv.stops.length - 1;
    while (n > 0 && !dv.stops[n].v) n -= 1;
    return n;
  };
  const glideAt = (dv: Dive, t: number) => {
    const i = Math.min(segment(dv, t), known(dv) - 1);
    if (i < 0) return { at: dv.stops[0].at, vel: [0, 0] };
    const a = dv.stops[i];
    const b = dv.stops[i + 1];
    const s = Math.min(1, (t - knot(dv, i)) / pace);
    const w = width(dv, i);
    const [dx, dy] = apart(b.at, a.at);
    const q0 = [-dx / w, -dy / w];
    const m0 = [pace * a.v![0] + ease * q0[0], pace * a.v![1] + ease * q0[1]];
    const m1 = [pace * b.v![0], pace * b.v![1]];
    const [h0, h1, h2] = hermite(s);
    const [g0, g1, g2] = slope(s);
    const q = [0, 1].map((n) => h0 * q0[n] + h1 * m0[n] + h2 * m1[n]);
    const dq = [0, 1].map((n) => g0 * q0[n] + g1 * m0[n] + g2 * m1[n]);
    const scale = w * 2 ** (-STEP * s);
    return { at: shift(b.at, scale * q[0], scale * q[1]), vel: [0, 1].map((n) => (dq[n] - ease * q[n]) / pace) };
  };
  const centreAt = (dv: Dive, t: number) => {
    const a = dv.arrive;
    if (!a || t <= a.t) return glideAt(dv, t).at;
    const [h0, h1] = hermite(Math.min(1, (t - a.t) / GLIDE));
    return shift(a.to, h0 * a.gap[0] + h1 * a.push[0], h0 * a.gap[1] + h1 * a.push[1]);
  };
  const spanAt = (dv: Dive, t: number) => {
    const a = dv.arrive;
    if (!a || t <= a.t) return dv.opening * 2 ** (-rate * (Math.min(t, dv.floor) - dv.t0));
    const s = Math.min(t, a.end) - a.t;
    let l: number;
    if (a.go <= 0) l = a.la + (a.le - a.la) * smooth(s / GLIDE);
    else if (s <= a.cruise) l = a.la - rate * s;
    else {
      const u = s - a.cruise;
      const left = a.go - rate * a.cruise;
      l = a.la - rate * a.cruise - rate * u + ((rate * rate) / (4 * left)) * u * u;
    }
    return 2 ** l;
  };
  const angleAt = (dv: Dive, t: number) => {
    const a = dv.arrive;
    if (!a || t <= a.t) return 0;
    return a.angle * smooth((t - a.t) / (a.end - a.t));
  };
  const viewAt = (dv: Dive, t: number): View =>
    dv.phase === "fade" ? dv.frozen! : { centre: centreAt(dv, t), span: spanAt(dv, t), angle: angleAt(dv, t) };
  const homeView = (): View => ({ centre: origin, span: opening(), angle: 0 });
  const iterations = (v: View) => Math.min(plan.cap, Math.round(BASE + MORE * Math.max(0, Math.log2(SPAN / v.span))));
  const spans = (v: View) => {
    const short = Math.min(view.w, view.h);
    return [(v.span * view.w) / short, (v.span * view.h) / short];
  };
  const polish = (at: Point, p: number, dv: Prep): Point => {
    let [nr, ni] = at;
    for (let round = 0; round < 2; round++) {
      let zr = julia ? nr : 0n;
      let zi = julia ? ni : 0n;
      const [cr, ci] = julia ? dv.c! : [nr, ni];
      let gr = julia ? 1 : 0;
      let gi = 0;
      for (let n = 0; n < p; n++) {
        const fr = float(zr);
        const fi = float(zi);
        const t = 2 * (fr * gr - fi * gi) + (julia ? 0 : 1);
        gi = 2 * (fr * gi + fi * gr);
        gr = t;
        const rr = (zr * zr) >> WORD;
        const ii = (zi * zi) >> WORD;
        if (rr + ii > LIMIT) return [nr, ni];
        const ri = (zr * zi) >> (WORD - 1n);
        zr = rr - ii + cr;
        zi = ri + ci;
      }
      const fr = float(zr);
      const fi = float(zi);
      const q = gr * gr + gi * gi;
      if (!(q > 0)) break;
      nr -= fixed((fr * gr + fi * gi) / q);
      ni -= fixed((fi * gr - fr * gi) / q);
    }
    return [nr, ni];
  };
  const settle = (at: Point, p: number, dv: Prep, reach = Infinity, fine = true): Found | null => {
    const dc = apart(at, cur!.at);
    const n = p ? nucleus(cur!.o, dc, p) : null;
    if (!n || Math.hypot(n[0] - dc[0], n[1] - dc[1]) > reach) return null;
    const q = exact(cur!.o, n, p) || p;
    const rough = shift(cur!.at, n[0], n[1]);
    return { at: fine ? polish(rough, q, dv) : rough, n, p: q };
  };
  const anchor = (at: Point, radius: number, dv: Prep, fine = true) =>
    settle(at, ball(cur!.o, apart(at, cur!.at), radius, plan.cap), dv, REACH * radius, fine);
  const renew = (at: Point, radius: number, dv: Prep) => {
    const found = anchor(at, radius, dv);
    if (found) next = reference(found.at, dv);
  };
  const framing = (v: View, found: Found): Hit | null => {
    const s = size(cur!.o, found.n, found.p);
    const span = Math.hypot(s[0], s[1]) * SPAN;
    return span >= bottom && span < 0.7 * v.span ? { at: found.at, s, span, p: found.p } : null;
  };
  const swap = () => {
    upload(next!);
    release(cur);
    cur = next;
    next = null;
  };
  const ensure = (v: View) => {
    if (next?.o.done) return swap();
    const [dx, dy] = apart(v.centre, cur!.at);
    if (Math.hypot(dx, dy) / v.span < LEASH) return;
    next ??= reference(anchor(v.centre, v.span, dive!)?.at ?? v.centre, dive!);
    next.o.step();
    swap();
  };
  const shade = (prog: Program, ref: Ref, v: View, w: number, h: number, extra: Uniforms) => {
    const [sx, sy] = spans(v);
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    prog.set({
      uOrbit: ref.tex,
      uLen: ref.o.length,
      uCrit: julia ? ref.o.rows * WIDE : 0,
      uCritLen: ref.o.crit.length,
      uMax: iterations(v),
      uJulia: julia,
      uRes: [w, h],
      uOff: apart(v.centre, ref.at),
      uU: [(cos * sx) / w, (sin * sx) / w],
      uV: [(-sin * sy) / h, (cos * sy) / h],
      uPx: sx / w,
      uFall: 1 / (REACHED * Math.min(w, h)),
      ...extra,
    });
    fill(gl);
  };
  const locate = (v: View, x: number, y: number) => {
    const [sx, sy] = spans(v);
    const u = (x / side - 0.5) * sx;
    const w = (y / side - 0.5) * sy;
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    return shift(v.centre, u * cos - w * sin, u * sin + w * cos);
  };
  const toward = (from: Point, to: Point, w: number) => {
    const [dx, dy] = apart(to, from);
    const far = Math.hypot(dx, dy) / w;
    return far <= reach ? to : shift(from, (dx * reach) / far, (dy * reach) / far);
  };
  const see = (v: View, urgent: boolean) => {
    ensure(v);
    gl.bindFramebuffer(gl.FRAMEBUFFER, sight.fb);
    gl.viewport(0, 0, side, side);
    blend(gl, null);
    shade(sense, upload(cur!), v, side, side, {});
    if (!lazy || view.fixed || urgent) {
      gl.readPixels(0, 0, side, side, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
      return { pixels };
    }
    gl.bindBuffer(PACK, buffer);
    gl.readPixels(0, 0, side, side, gl.RGBA, gl.UNSIGNED_BYTE, 0);
    gl.bindBuffer(PACK, null);
    const sync = gl.fenceSync(SYNC, 0);
    gl.flush();
    return { sync };
  };
  const forget = (dv: Dive) => {
    if (dv.pending) gl.deleteSync(dv.pending.sync);
    dv.pending = null;
  };
  const land = (dv: Dive, urgent: boolean) => {
    const one = dv.pending!;
    const state = gl.clientWaitSync(one.sync, 0, 0);
    const ready = DONE.includes(state);
    if (!ready && !urgent && state !== LOST) return false;
    if (ready) {
      gl.bindBuffer(PACK, buffer);
      gl.getBufferSubData(PACK, 0, pixels);
      gl.bindBuffer(PACK, null);
    } else {
      gl.bindFramebuffer(gl.FRAMEBUFFER, sight.fb);
      gl.readPixels(0, 0, side, side, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
    }
    forget(dv);
    judge(dv, one.k, one.v, pixels);
    return true;
  };
  const resolve = (v: View, one: Spot) => {
    if (one.hit !== undefined) return one.hit;
    const px = spans(v)[0] / side;
    const near = (found: Found | null) => found && Math.hypot(...apart(found.at, one.at)) <= NEAR * TILE * px;
    let found = settle(one.at, one.period, dive!, Infinity, false);
    if (!near(found)) found = anchor(one.at, px * Math.max(0.5, Math.sqrt(one.n / Math.PI)), dive!, false);
    one.hit = near(found) ? framing(v, found!) : null;
    return one.hit;
  };
  function* aim(
    dv: Dive,
    v: View,
    to: Point,
    picks: Spot[],
    late: boolean,
  ): Generator<undefined, Best | null, unknown> {
    let best: Best | null = null;
    for (const at of [v.centre, to, ...picks.map((one) => one.at)]) {
      const found = anchor(at, v.span * reach * SEEK, dv, false);
      if (found && found.p <= plan.cap * HEAVY[julia ? "julia" : "set"]) {
        const [dx, dy] = apart(found.at, v.centre);
        const far = Math.hypot(dx, dy) / v.span;
        const hit = far <= reach ? framing(v, found) : null;
        if (hit && (hit.span <= bottom * 2 ** WINDOW || late) && (!best || far < best.far)) best = { ...hit, far };
      }
      yield;
    }
    if (best) best.at = polish(best.at, best.p, dv);
    return best;
  }
  const arrive = (dv: Dive, t: number, hit: Hit) => {
    const [hr, hi] = HOME[julia ? "julia" : "set"];
    const [sr, si] = hit.s;
    const to = shift(hit.at, sr * hr - si * hi, sr * hi + si * hr);
    const span = spanAt(dv, t);
    const la = Math.log2(span);
    const le = Math.log2(hit.span);
    const go = la - le;
    const cruise = go > 1 ? (go - 1) / rate : 0;
    const end = t + (go <= 0 ? GLIDE : cruise + (2 * Math.min(go, 1)) / rate);
    const { at, vel } = glideAt(dv, t);
    dv.arrive = {
      t,
      la,
      le,
      go,
      cruise,
      end,
      angle: Math.atan2(si, sr),
      p: hit.p,
      to,
      gap: apart(at, to),
      push: vel.map((one) => one * span * GLIDE),
    };
    next = reference(hit.at, dv);
    dv.scan = null;
    forget(dv);
    dv.phase = "arrive";
  };
  const put = (dv: Dive, k: number, at: Point, exact = false) => {
    dv.stops.length = k + 1;
    dv.stops.push({ at, v: null });
    dv.stops[k].v ??= tangent(chord(dv, k - 1), chord(dv, k));
    if (exact) next = reference(at, dv);
    else if (at !== dv.stops[k].at) renew(at, width(dv, k + 1) / 4, dv);
  };
  const aimed = (dv: Dive, one: Scan, hit: Hit | null) => {
    dv.scan = null;
    if (hit) {
      dv.goal = hit;
      dv.land = knot(dv, Math.max(one.k + 1, Math.ceil((Math.log2(dv.opening / hit.span) - CLOSE) / STEP)));
    }
    put(dv, one.k, hit ? hit.at : one.to, Boolean(hit));
  };
  const scan = (dv: Dive, urgent: boolean) => {
    const one = dv.scan!;
    if (!urgent && !spare()) return false;
    let r = one.run.next();
    while (urgent && !r.done) r = one.run.next();
    if (!r.done) return false;
    aimed(dv, one, r.value);
    return true;
  };
  const judge = (dv: Dive, k: number, v: View, probe: Uint8Array) => {
    const list = score(probe, side, side);
    const depth = Math.log2(SPAN / v.span) - ahead;
    const picks = spread(list, plan.picks).map((one) => ({ ...one, at: locate(v, one.x, one.y) }));
    for (const one of picks) {
      if (one.kind !== "island" || resolve(v, one)) continue;
      one.kind = "edge";
      one.at = locate(v, one.ex, one.ey);
    }
    const rim = edge(probe, side, side);
    if (rim) dv.rim = locate(v, rim.x, rim.y);
    const here = dv.stops[k].at;
    let to = here;
    if (dv.goal) to = toward(here, dv.goal.at, v.span);
    else if (!list.length) to = dv.rim ? toward(here, dv.rim, v.span) : here;
    else {
      const [sx, sy] = spans(v);
      const rand = rng((seed ^ Math.imul(dv.j + 1, GOLD) ^ Math.imul(k + 1, MIX)) >>> 0);
      const cos = Math.cos(v.angle);
      const sin = Math.sin(v.angle);
      const offset = (x: number, y: number): Pair => {
        const u = (x / side - 0.5) * sx;
        const w = (y / side - 0.5) * sy;
        return [(u * cos - w * sin) / v.span, (u * sin + w * cos) / v.span];
      };
      const [ox, oy] = prefer(probe, side, side, list, offset, k ? chord(dv, k - 1) : [0, 0], reach, rand)!;
      to = shift(here, ox * v.span, oy * v.span);
    }
    if (!dv.goal && depth >= SOON) dv.scan = { k, to, run: aim(dv, v, to, picks, depth >= deepest - LATE) };
    else put(dv, k, to);
  };
  const issue = (dv: Dive, k: number, urgent: boolean) => {
    dv.k = k + 1;
    const v = { centre: dv.stops[k].at, span: width(dv, k), angle: dv.tilt + spin(knot(dv, k)) };
    const seen = see(v, urgent);
    if (seen.sync) dv.pending = { k, v, sync: seen.sync };
    else judge(dv, k, v, seen.pixels!);
  };
  const schedule = (dv: Dive, t: number) => {
    const want = segment(dv, t) + 2;
    for (;;) {
      const urgent = dv.stops.length <= want;
      if (dv.pending) {
        if (!land(dv, urgent)) return;
      } else if (dv.scan) {
        if (!scan(dv, urgent)) return;
      } else if (dv.k < dv.stops.length && (urgent || knot(dv, dv.k - LEAD) <= t)) issue(dv, dv.k, urgent);
      else return;
    }
  };
  const park = (v: View) => {
    forget(dive!);
    if (next && next !== cur) release(next);
    next = null;
    dive!.scan = null;
    dive!.phase = "rest";
    dive!.frozen = v;
  };
  const shown = (t: number): View => {
    if (dive!.phase === "rest") return dive!.frozen!;
    const v = viewAt(dive!, t);
    return { ...v, angle: v.angle + dive!.tilt + spin(t) };
  };
  const end = (dv: Dive, t: number) => (still ? park(shown(t)) : fadeOut(dv, t));
  const fadeOut = (dv: Dive, t: number) => {
    dv.frozen = viewAt(dv, t);
    dv.scan = null;
    forget(dv);
    dv.phase = "fade";
    dv.fade = t;
    dv.heir = prepare(dv.j + 1);
  };
  const advance = (t: number) => {
    for (;;) {
      const dv = dive!;
      if (dv.phase === "rest") return;
      if (dv.phase === "dive") {
        schedule(dv, Math.min(t, dv.floor, dv.land));
        if (Math.min(dv.floor, dv.land) > t) return;
        if (dv.land < dv.floor) arrive(dv, dv.land, dv.goal!);
        else end(dv, dv.floor);
        continue;
      }
      if (dv.phase === "arrive") {
        const out = dv.arrive!.end + (still ? 0 : HOLD);
        if (out > t) return;
        end(dv, out);
        continue;
      }
      const out = dv.fade + FADE;
      if (out > t) return;
      begin(dv.heir!, out);
    }
  };
  const spare = () => view.fixed || now() < by;
  const work = () => {
    if (!view.fixed) {
      for (const ref of [next, upcoming]) if (ref && !ref.o.done) slice(ref.o, spare);
      return;
    }
    let left = plan.budget;
    if (next && !next.o.done) {
      const before = next.o.length;
      next.o.step(left);
      left -= next.o.length - before + 1;
    }
    if (left > 0 && upcoming && !upcoming.o.done) upcoming.o.step(left);
  };
  const theme = () => {
    ink = view.look();
    shades = tone(ink.accent, ink.paper);
  };
  const trace = (dv: Dive, t: number, v: View) => {
    if (dv.phase === "fade" || dv.phase === "rest") return;
    const end = dv.arrive ? dv.arrive.t + GLIDE : Math.min(knot(dv, known(dv)), dv.floor, dv.land);
    if (!(end > t)) return;
    pen ??= quads(gl, { aA: 2, aB: 2, aShape: 4, aInk: 4 }, 2 * (TRAIL + DOTS));
    nib ??= program(gl, INK, LINE);
    const [W, H] = [view.w, view.h];
    const [sx, sy] = spans(v);
    const cos = Math.cos(v.angle);
    const sin = Math.sin(v.angle);
    const screen = (at: Point): Pair => {
      const [dx, dy] = apart(at, v.centre);
      return [(0.5 + (dx * cos + dy * sin) / sx) * W, (0.5 + (dy * cos - dx * sin) / sy) * H];
    };
    const trail = Array.from({ length: TRAIL }, (_, n) => screen(centreAt(dv, t + ((end - t) * n) / (TRAIL - 1))));
    const inked = (hex: string) => [...unit(hex), 1];
    const accent = inked(ink!.accent);
    const ground = inked(ink!.paper);
    const px = view.dpr || 1;
    let n = 0;
    const add = (a: Pair, b: Pair, r: number, w: number, inside: number, colour: number[]) =>
      pen!.data.set([...a, ...b, r * px, (w * px) / 2, inside, 0, ...colour], n++ * pen!.stride);
    for (const [w, colour] of [
      [PEN + HALO, ground],
      [PEN, accent],
    ] satisfies [number, number[]][])
      for (let i = 1; i < TRAIL; i++) add(trail[i - 1], trail[i], 0, w, 0, colour);
    const live = dv.arrive ? [dv.arrive.to] : dv.stops.slice(segment(dv, t) + 1, known(dv) + 1).map((one) => one.at);
    const rings: Ring[] = [
      [[W / 2, H / 2], HUB, 0],
      ...live.slice(0, DOTS).map((at, i): Ring => [screen(at), MARK, i ? 0 : SHADE]),
    ];
    for (const [at, [r, w], inside] of rings) {
      add(at, at, r, w + HALO, 0, ground);
      add(at, at, r, w, inside, accent);
    }
    nib.set({ uRes: [W, H] });
    pen.draw(nib, n, "alpha");
    blend(gl, null);
  };
  const draw = () => {
    const t = view.t;
    by = now() + BUDGET;
    advance(t);
    work();
    const v = shown(t);
    ensure(v);
    const k = view.fixed ? 1 : Math.min(1, Math.sqrt(plan.pixels / (view.w * view.h)));
    const w = Math.max(1, Math.round(view.w * k));
    const h = Math.max(1, Math.round(view.h * k));
    const small = k < 1;
    if (small) bin = bin ? bin.size(w, h) : target(gl, w, h);
    gl.bindFramebuffer(gl.FRAMEBUFFER, small ? bin!.fb : null);
    gl.viewport(0, 0, w, h);
    blend(gl, null);
    if (view.look() !== ink) theme();
    const frames = t / FRAME;
    const turn = dive!.tilt + spin(t);
    const colour = { ...shades, uFreq: 1, uDrift: t * DRIFT, uTime: frames * CYCLE };
    shade(paint, upload(cur!), v, w, h, { ...colour, uAlpha: 1 });
    if (dive!.phase === "fade") {
      const mix = smooth((t - dive!.fade) / FADE);
      blend(gl, "alpha");
      shade(paint, dive!.heir!.base, { ...homeView(), angle: turn }, w, h, { ...colour, uAlpha: mix });
      blend(gl, null);
    }
    if (small) {
      copy ??= program(gl, COPY);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.viewport(0, 0, view.w, view.h);
      copy.set({ uTex: bin, uRes: [view.w, view.h] });
      fill(gl);
    }
    if (path) trace(dive!, t, v);
  };
  const drop = () => {
    for (const ref of [cur, next, dive?.base, dive?.heir?.base, settled]) {
      if (ref?.tex) gl.deleteTexture(ref.tex);
      if (ref) ref.tex = null;
    }
    forget(dive!);
    gl.deleteBuffer(buffer);
    paint.drop();
    sense.drop();
    sight.drop();
    copy?.drop();
    bin?.drop();
    pen?.drop();
    nib?.drop();
  };
  const fly = (from: View | null = dive!.frozen) => {
    if (dive!.phase !== "rest") return;
    begin(
      { j: dive!.j, c: dive!.c, cf: dive!.cf, crit: dive!.crit, base: dive!.base },
      view.t,
      from,
      from === dive!.frozen ? cur! : dive!.base,
    );
  };
  const halt = () => {
    if (dive!.phase !== "rest") park(shown(view.t));
    return dive!.frozen!;
  };
  const look = (v: View, c: Pair | null = null) => {
    if (julia && c && (c[0] !== dive!.cf![0] || c[1] !== dive!.cf![1])) {
      presets = [c];
      begin(prepare(0), view.t, v);
    }
    park(v);
  };
  if (still) {
    begin(prepare(0), view.t, still);
    park(still);
  } else begin(prepare(0), 0);
  return { draw, theme, drop, fly, halt, look, flying: () => dive!.phase !== "rest", at: () => shown(view.t) };
}
