import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const LEVEL = 3;
const SIDE = 8;

export default function draw(pen: Pen, ink: Ink) {
  const cell = math.two.create(7, 2, LEVEL, 0, 2);
  const network = math.graph.core_graph({ shape: cell.shape, data: cell.types });
  const tally = math.graph.census(network);
  const tags = math.graph.roles(network);
  const tips = tags.filter((r) => r === "Tip").length;
  const through = tags.filter((r) => r === "Through").length;
  const junctions = tags.filter((r) => r === "Junction").length;
  if (tally.nodes !== 27 || tally.branches !== 26 || tally.components !== 1) throw new Error(`wiki-graphs: ${tally.nodes} nodes, ${tally.branches} branches, ${tally.components} components, want 27, 26, 1`);
  if (tips !== 10 || through !== 9 || junctions !== 8) throw new Error(`wiki-graphs: ${tips} tips, ${through} through, ${junctions} junctions, want 10, 9, 8`);
  const frame = pen.frame(0.08);
  const unit = frame.w / SIDE;
  const { nodes, branches } = network;
  const at = (index: number): Point => {
    const node = nodes[index];
    return [frame.x + node.position[0] * unit, frame.y + node.position[1] * unit];
  };
  for (const branch of branches) pen.segment(at(branch.parent), at(branch.child), unit * 0.16, ink.fade(ink.blue, 0.6));
  tags.forEach((role, index) => {
    const [x, y] = at(index);
    if (role === "Junction") pen.disc(x, y, unit * 0.3, ink.pink);
    else if (role === "Tip") pen.disc(x, y, unit * 0.26, ink.yellow);
    else pen.disc(x, y, unit * 0.21, ink.blue);
  });
}
