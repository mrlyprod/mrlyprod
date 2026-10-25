import * as math from "@mrlyprod/mrly/math";
import type { Pen, Point } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

/* TYPES */

type Drawn = { side: number; positions: Float64Array; pairs: Uint32Array };

const CODE = 239;
const NUMBER = 3;
const LEVEL = 2;
const BASE = 3;
const DOT = 0.3;
const LINE = 0.14;
const WANT = { nodes: 49, branches: 54, tips: 8, junctions: 18, pieces: 1 };

let plan: Drawn | null = null;

function study(): Drawn {
  if (plan) return plan;
  const cell = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const net = math.graph.core_graph({ shape: cell.shape, data: cell.types });
  const census = math.graph.census(net);
  const read = {
    nodes: census.nodes,
    branches: census.branches,
    tips: census.tips,
    junctions: census.junctions,
    pieces: census.components,
  };
  const got = Object.values(read).join();
  if (got !== Object.values(WANT).join())
    throw new Error(`graphs: the core graph of ${CODE} reads ${got}, want ${Object.values(WANT).join()}`);
  const positions = new Float64Array(net.nodes.length * 2);
  net.nodes.forEach((node, i) => {
    positions.set(node.position, i * 2);
  });
  const pairs = new Uint32Array(net.branches.length * 2);
  net.branches.forEach((branch, j) => {
    pairs.set([branch.parent, branch.child], 2 * j);
  });
  net.free();
  plan = { side: cell.shape[0], positions, pairs };
  return plan;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.16);
  const { side, positions, pairs } = study();
  const k = box.w / side;
  const at = (i: number): Point => [box.x + positions[2 * i] * k, box.y + positions[2 * i + 1] * k];
  for (let j = 0; j < pairs.length; j += 2) pen.segment(at(pairs[j]), at(pairs[j + 1]), LINE * k, paint.light);
  for (let i = 0; i < positions.length / 2; i++) {
    const [x, y] = at(i);
    pen.disc(x, y, DOT * k, paint.white);
  }
}
