import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const LEVEL = 5;
const SIDE = 32;

export default function draw(pen: Pen, ink: Ink) {
  const cell = math.two.create(7, 2, LEVEL, 0, 2);
  const network = math.graph.core_graph({ shape: cell.shape, data: cell.types });
  const tally = math.graph.census(network);
  const tags = math.graph.roles(network);
  const tips = tags.filter((r) => r === "Tip").length;
  const paths = tags.filter((r) => r === "Through").length;
  const junctions = tags.filter((r) => r === "Junction").length;
  if (tally.nodes !== 243 || tally.branches !== 242 || tally.components !== 1) throw new Error(`demo-graphs: ${tally.nodes} nodes, ${tally.branches} branches, ${tally.components} components, want 243, 242, 1`);
  if (tips !== 82 || paths !== 81 || junctions !== 80) throw new Error(`demo-graphs: ${tips} tips, ${paths} paths, ${junctions} junctions, want 82, 81, 80`);
  const frame = pen.frame(0.08);
  const unit = frame.w / SIDE;
  const { nodes, branches } = network;
  const at = (index: number): [number, number] => {
    const node = nodes[index];
    return [frame.x + node.position[0] * unit, frame.y + node.position[1] * unit];
  };
  for (const branch of branches) pen.segment(at(branch.parent), at(branch.child), unit * 0.2, ink.fade(ink.blue, 0.55));
  tags.forEach((role, index) => {
    const [x, y] = at(index);
    if (role === "Junction") pen.disc(x, y, unit * 0.34, ink.pink);
    else if (role === "Tip") pen.disc(x, y, unit * 0.3, ink.yellow);
    else pen.disc(x, y, unit * 0.2, ink.blue);
  });
}
