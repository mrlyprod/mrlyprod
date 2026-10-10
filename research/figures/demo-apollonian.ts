import * as num from "mrlyjs/num";
import { field, frame, type Color, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const TOP = 2048;
const DEEP = 32;
const CIRCLES = 2448;
const RESTING = 323;
const BRIGHT = 528n;
const SIDE = 640;
const BAND = 232;
const OVER = 26;
const TICK = 3;
const RULE = 2;

const { apollonian, lattice } = num;

type Round = { centre: Point; k: number; ford: boolean };
type Stack = [number, number, number];

let memo: { rings: Round[]; stack: Stack[]; peak: number } | undefined;

// FACTS

function facts() {
  if (memo) return memo;
  const p = apollonian.grow("strip", TOP);
  const marks = apollonian.touches(p);
  const read = apollonian.shadow(p, DEEP);
  const stack: Stack[] = lattice
    .farey(DEEP)
    .filter((n) => n.num > 0 && n.num < n.den)
    .map((n) => [n.num, n.den, n.brightness]);

  if (p.circles.length !== CIRCLES) throw new Error(`demo-apollonian: ${p.circles.length} circles, want ${CIRCLES}`);
  if (p.broken !== 0 || p.strayed !== 0) throw new Error(`demo-apollonian: ${p.broken} broken and ${p.strayed} strayed, want none`);
  if (!p.strip) throw new Error("demo-apollonian: the packing is not a strip");
  if (marks.length !== RESTING) throw new Error(`demo-apollonian: ${marks.length} tangency points, want ${RESTING}`);
  if (!read.covered) throw new Error("demo-apollonian: the packing does not carry the stack");
  if (read.nodes !== RESTING || read.touched !== RESTING || read.missed !== 0) {
    throw new Error(`demo-apollonian: stack reads ${read.nodes} nodes, ${read.touched} touched, ${read.missed} missed, want ${RESTING}, ${RESTING}, 0`);
  }
  if (read.bright !== BRIGHT || read.want !== BRIGHT) throw new Error(`demo-apollonian: brightness ${read.bright} of ${read.want}, want ${BRIGHT}`);
  if (stack.length !== RESTING) throw new Error(`demo-apollonian: ${stack.length} stack nodes, want ${RESTING}`);

  const rings: Round[] = [];
  [...p.circles, ...p.root].forEach((data, i) => {
    const c = apollonian.Circle.from(data);
    const ford = apollonian.is_ford(c);
    if (i < p.circles.length && apollonian.on_line(c) && !ford) throw new Error(`demo-apollonian: line-tangent circle ${i} is not a Ford circle`);
    if (!c.is_line()) {
      if (!(data.k > 0)) throw new Error(`demo-apollonian: circle of curvature ${data.k}`);
      rings.push({ centre: c.centre() ?? [0, 0], k: data.k, ford });
    }
    c.free();
  });

  let peak = 0;
  stack.forEach(([n, d, bright], i) => {
    const mark = marks[i];
    if (n !== mark.num || d !== mark.den) throw new Error(`demo-apollonian: node ${n}/${d} against tangency ${mark.num}/${mark.den}`);
    if (mark.k !== 2 * mark.den * mark.den) throw new Error(`demo-apollonian: tangency ${mark.num}/${mark.den} carries curvature ${mark.k}`);
    if (bright !== Math.floor(DEEP / mark.den)) throw new Error(`demo-apollonian: node ${n}/${d} is ${bright} bright, want ${Math.floor(DEEP / mark.den)}`);
    peak = Math.max(peak, bright);
  });
  if (peak !== DEEP / 2) throw new Error(`demo-apollonian: peak brightness ${peak}, want ${DEEP / 2}`);
  memo = { rings, stack, peak };
  return memo;
}

// RING

function octave(k: number) {
  return (31 - Math.clz32(k)) % 6;
}

function ring(pen: Pen, at: Point, r: number, thick: number, span: Point, color: Color) {
  const half = thick / 2;
  const reach = r + half + 1;
  const x0 = Math.floor(Math.max(at[0] - reach, span[0] - 1, 0));
  const x1 = Math.min(Math.ceil(Math.max(Math.min(at[0] + reach, span[1] + 1), 0)), pen.width);
  const y0 = Math.floor(Math.max(at[1] - reach, 0));
  const y1 = Math.min(Math.ceil(Math.max(at[1] + reach, 0)), pen.height);
  if (x1 <= x0 || y1 <= y0) return;
  const stroke = field.patch(x0, y0, x1 - x0, y1 - y0);
  for (let py = y0; py < y1; py++) {
    for (let px = x0; px < x1; px++) {
      const fx = px + 0.5;
      const fy = py + 0.5;
      const dx = fx - at[0];
      const dy = fy - at[1];
      const edge = Math.abs(Math.sqrt(dx * dx + dy * dy) - r) - half;
      const cut = Math.max(span[0] - fx, fx - span[1]);
      const cover = 0.5 - Math.max(edge, cut);
      if (cover > 0) stroke.blend(px, py, color, cover);
    }
  }
  stroke.paint(pen);
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { rings, stack, peak } = facts();
  const left = (pen.width - SIDE) / 2;
  const head = (pen.height - (SIDE + BAND)) / 2;
  const box = frame(left, head, SIDE, SIDE);
  const span: Point = [box.x, box.x + box.w];
  const foot = box.y + box.h;

  for (const rule of [box.y, foot]) pen.segment([span[0] - OVER, rule], [span[1] + OVER, rule], RULE, ink.dim);
  for (const c of rings) {
    const at: Point = [box.x + box.w * c.centre[0], foot - box.h * c.centre[1]];
    const r = box.w / c.k;
    const hair = Math.min(Math.max(r / 12, 0.65), 1.8);
    ring(pen, at, r, c.ford ? hair * 1.5 : hair, span, ink.inks[octave(c.k)]);
  }
  for (const [n, d, bright] of stack) {
    const at = box.x + (box.w * n) / d;
    const lit = bright / peak;
    pen.segment([at, foot - TICK], [at, foot + BAND * lit], 0.75 + 2 * lit, ink.fade(ink.blue, 0.45 + 0.55 * lit));
  }
}
