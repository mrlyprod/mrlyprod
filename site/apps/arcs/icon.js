import * as math from "@mrlyprod/mrly/math";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = 495;
const BASE = 3;
const LEVEL = 2;
const THICK = 0.34;
const GAP = 0.08;
const GHOST = 48;
const QUARTER = Math.PI / 4;
const WANT = { side: 9, loops: 3, strands: 18, arcs: 32, box: [2, 7] };

let plan = null;

function study() {
  if (plan) return plan;
  const drawn = math.arcs.draw(CODE, BASE, LEVEL);
  const law = math.arcs.law(CODE, BASE, LEVEL);
  if (drawn.side !== WANT.side || drawn.loops !== WANT.loops || drawn.strands !== WANT.strands || law?.loops !== drawn.loops) throw new Error(`arcs: side ${drawn.side}, ${drawn.loops} loops, ${drawn.strands} strands, the law gives ${law?.loops}, want ${WANT.side}, ${WANT.loops}, ${WANT.strands}`);
  const side = drawn.side;
  const loop = math.arcs.curves(CODE, BASE, LEVEL).filter((curve) => curve.closed).sort((a, b) => b.arcs.length - a.arcs.length)[0];
  const corners = loop.arcs.map((a) => {
    const i = a >> 1;
    const which = a & 1;
    const x = i % side;
    const y = side - 1 - (i - x) / side;
    const filled = drawn.cells[i] & 1;
    const cx = which ? (filled ? x + 1 : x) : filled ? x : x + 1;
    const cy = which ? y : y + 1;
    return [cx, cy, Math.atan2(cy === y ? 1 : -1, cx === x ? 1 : -1)];
  });
  const lo = Math.min(...corners.flatMap(([x, y]) => [x, y]));
  const hi = Math.max(...corners.flatMap(([x, y]) => [x, y]));
  if (loop.arcs.length !== WANT.arcs || lo !== WANT.box[0] || hi !== WANT.box[1]) throw new Error(`arcs: the longest loop has ${loop.arcs.length} arcs over corners ${lo} to ${hi}, want ${WANT.arcs} over ${WANT.box.join(" to ")}`);
  const cells = [];
  for (let y = lo - 1; y <= hi; y++) for (let x = lo - 1; x <= hi; x++) if (drawn.cells[(side - 1 - y) * side + x] & 1) cells.push([x - lo + 1, y - lo + 1]);
  plan = { span: hi - lo + 2, cells, arcs: corners.map(([x, y, mid]) => [x - lo + 1, y - lo + 1, mid]) };
  return plan;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "teal", 0.15);
  const { span, cells, arcs } = study();
  const unit = box.w / span;
  const gap = unit * GAP;
  const ghost = [...paint.white.slice(0, 3), GHOST];
  for (const [x, y] of cells) pen.rect(box.x + x * unit + gap, box.y + y * unit + gap, unit - 2 * gap, unit - 2 * gap, ghost);
  for (const [x, y, mid] of arcs) pen.arc([box.x + x * unit, box.y + y * unit], unit / 2, [mid - QUARTER, mid + QUARTER], THICK * unit, paint.white);
}
