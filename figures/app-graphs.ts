import * as math from "mrlyjs/math";
import type { Color, Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const CODE = 239;
const NUMBER = 3;
const LEVEL = 3;
const BASE = 3;
const MARGIN = 0.08;
const DOT = 0.2;
const LINE = 0.08;
const FADES = [0.72, 0.48, 0.22, 0];
const ROLES = ["Alone", "Tip", "Through", "Junction"];
const WANT = { nodes: 343, branches: 402, tips: 44, junctions: 156, pieces: 1 };

type Plan = { side: number; positions: Float64Array; pairs: Uint32Array; roles: Uint8Array };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const cell = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const net = math.graph.core_graph({ shape: cell.shape, data: cell.types });
  const census = math.graph.census(net);
  const read = { nodes: census.nodes, branches: census.branches, tips: census.tips, junctions: census.junctions, pieces: census.components };
  const got = Object.values(read).join();
  if (got !== Object.values(WANT).join()) throw new Error(`app-graphs: the core graph of ${CODE} reads ${got}, want ${Object.values(WANT).join()}`);
  const nodes = net.nodes;
  const positions = new Float64Array(nodes.length * 2);
  nodes.forEach((node, i) => positions.set(node.position, i * 2));
  const branches = net.branches;
  const pairs = new Uint32Array(branches.length * 2);
  branches.forEach((branch, j) => {
    pairs[2 * j] = branch.parent;
    pairs[2 * j + 1] = branch.child;
  });
  const roles = Uint8Array.from(math.graph.roles(net), (role) => ROLES.indexOf(role));
  net.free();
  plan = { side: cell.shape[0], positions, pairs, roles };
  return plan;
}

export default function draw(pen: Pen, ink: Ink) {
  const { side, positions, pairs, roles } = study();
  const box = pen.frame(MARGIN);
  const k = box.w / side;
  const at = (i: number): Point => [box.x + positions[2 * i] * k, box.y + positions[2 * i + 1] * k];
  const shades: Color[] = FADES.map((t) => ink.mix(ink.blue, ink.ground, t));
  for (let s = 0; s < shades.length; s++) {
    for (let j = 0; j < pairs.length; j += 2) {
      const a = pairs[j];
      const b = pairs[j + 1];
      if (Math.min(roles[a], roles[b]) !== s) continue;
      pen.segment(at(a), at(b), LINE * k, shades[s]);
    }
  }
  for (let i = 0; i < roles.length; i++) {
    const [x, y] = at(i);
    pen.disc(x, y, DOT * k, shades[roles[i]]);
  }
}
