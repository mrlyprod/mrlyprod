import * as math from "mrlyjs/math";
import { field, frame } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const LIMIT = 9;
const LAYERS = 5;
const PANEL = 420;
const GUTTER = 20;

let memo: Float32Array[] | undefined;

function sheets() {
  if (memo) return memo;
  const recipes = math.moire.all(LIMIT);
  if (recipes.length !== 4) throw new Error(`demo-moire: ${recipes.length} presets, want 4`);
  for (const recipe of recipes) {
    const numbers = recipe.numbers;
    if (numbers.join() !== "1,3,5,7,9") throw new Error(`demo-moire: ${recipe.name} stacks ${numbers}, want 1,3,5,7,9`);
    if (numbers.length !== LAYERS) throw new Error(`demo-moire: ${recipe.name} stacks ${numbers.length} layers, want ${LAYERS}`);
  }
  const table = math.moire.sample.membership(recipes[3].spec.code, 3, 2);
  const lit = table.filter((on) => on).length;
  if (lit !== 8) throw new Error(`demo-moire: the truth table lights ${lit}, want 8`);
  if (table[math.moire.sample.pack([1, 1], 3)]) throw new Error("demo-moire: the centre residue is lit");
  const fields = recipes.map((recipe) => {
    const data = recipe.field(PANEL).data;
    if (data.length !== PANEL * PANEL) throw new Error(`demo-moire: ${recipe.name} holds ${data.length} samples, want ${PANEL * PANEL}`);
    return data;
  });
  if (!fields[1].every((v) => v === 0 || v === 1)) throw new Error("demo-moire: the weave is not zero and one");
  for (const counted of [fields[0], fields[2], fields[3]]) {
    const high = counted.reduce((a, v) => Math.max(a, v), -Number.MAX_VALUE);
    if (high !== LAYERS) throw new Error(`demo-moire: a stack peaks at ${high}, want ${LAYERS}`);
  }
  memo = fields;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const ramp = new ink.Ramp([ink.mix(ink.ground, ink.blue, 0.38), ink.blue, ink.yellow]);
  const side = 2 * PANEL + GUTTER;
  const edge = (pen.width - side) / 2;
  sheets().forEach((sheet, index) => {
    const x = edge + (index % 2) * (PANEL + GUTTER);
    const y = edge + Math.floor(index / 2) * (PANEL + GUTTER);
    field.draw(pen, frame(x, y, PANEL, PANEL), PANEL, PANEL, sheet, ramp);
  });
}
