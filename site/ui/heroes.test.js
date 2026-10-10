import { expect, test } from "bun:test";
import { art, fit, HALF, inside, TILINGS, WHOLE } from "./heroes.js";

const area = (poly) => Math.abs(poly.reduce((sum, [x, y], i) => sum + x * poly[(i + 1) % poly.length][1] - poly[(i + 1) % poly.length][0] * y, 0) / 2);

test("every tiling covers the art once, each point in one cell, and the cells of one tiling are the same size", () => {
  for (const [name, tiling] of Object.entries(TILINGS)) {
    for (const shape of [WHOLE, HALF]) {
      const cells = tiling.lay(shape.cols, shape.rows);
      let off = 0;
      for (let y = 0.0137; y < shape.rows; y += 0.2) for (let x = 0.0291; x < shape.cols; x += 0.2) if (cells.filter((one) => inside(one.poly, x, y)).length !== 1) off++;
      const areas = new Set(cells.map((one) => Math.round(area(one.poly) * 1e6)));
      expect([name, shape.kind, off, areas.size, cells.filter((one) => one.whole).length > 0]).toEqual([name, shape.kind, 0, 1, true]);
    }
  }
});

test("squares are the 16 by 9 whole cells; hexes and triangles overhang the edge and are clipped by the svg", () => {
  const square = TILINGS.square.lay(16, 9);
  expect([square.length, square.every((one) => one.whole)]).toEqual([144, true]);
  const hex = TILINGS.hex.lay(16, 9);
  expect([hex.length > 100, hex.some((one) => !one.whole), hex[0].poly.length]).toEqual([true, true, 6]);
  const tri = TILINGS.tri.lay(16, 9);
  expect([tri.length > 90, tri.some((one) => !one.whole), tri[0].poly.length]).toEqual([true, true, 3]);
});

test("fit finds the largest rectangle inside a mask that is at least the size asked, or none", () => {
  const blob = new Set(["0,0", "1,0", "2,0", "0,1", "1,1", "2,1", "1,2"]);
  const on = (x, y) => blob.has(`${x},${y}`);
  expect(fit(on, 3, 3, [3, 2])).toEqual([0, 0, 3, 2]);
  expect(fit(on, 3, 3, [1, 3])).toEqual([1, 0, 1, 3]);
  expect(fit(on, 3, 3, [2, 3])).toBeNull();
});

test("the art is one seeded svg: a shape per cell kind, the base cells, a plate and its near ring per orientation, and the text box inside the plate", () => {
  for (const avoid of ["", "square", "hex", "tri"]) {
    const one = art("Door", ["red", "blue", "accent"], 7, WHOLE, avoid);
    expect([avoid, one.tiling !== avoid, Object.hasOwn(TILINGS, one.tiling)]).toEqual([avoid, true, true]);
    expect([avoid, one.art]).toEqual([avoid, art("Door", ["red", "blue", "accent"], 7, WHOLE, avoid).art]);
    expect([avoid, one.paint.filter((hue) => hue === "a").length >= 8]).toEqual([avoid, true]);
    expect([avoid, one.art.match(/<svg /g)?.length, one.art.match(/<path id="door\d" d="[^"]+" vector-effect="non-scaling-stroke"\/>/g)?.length, one.art.match(/<g class="base">/g)?.length]).toEqual([avoid, 1, TILINGS[one.tiling].shapes.length, 1]);
    expect([avoid, one.art.match(/<use href="#door\d" x="[^"]+" y="[^"]+" class="[a-z]" style="--n:\d+"\/>/g)?.length]).toEqual([avoid, one.cells]);
    expect([avoid, one.art.match(/<g class="plate">/g)?.length, one.art.match(/<g class="near">/g)?.length, one.art.match(/<g class="plate tall">/g)?.length, one.art.match(/<g class="near tall">/g)?.length]).toEqual([avoid, 1, 1, 1, 1]);
    const vars = Object.fromEntries(one.style.split(";").map((pair) => pair.split(":")).map(([key, value]) => [key, Number(value)]));
    expect([avoid, vars["--cx"] >= 6, vars["--cy"] >= 2, vars["--qcx"] >= 7, vars["--qcy"] >= 3]).toEqual([avoid, true, true, true, true]);
    expect([avoid, vars["--x"] + vars["--cx"] <= 16, vars["--y"] + vars["--cy"] <= 9, vars["--qx"] + vars["--qcx"] <= 9, vars["--qy"] + vars["--qcy"] <= 16]).toEqual([avoid, true, true, true, true]);
  }
  expect(art("Door", ["red"], 7, WHOLE).art).not.toBe(art("Door", ["red"], 8, WHOLE).art);
});

test("a hue site.json does not know stops the build and names the choices", () => {
  expect(() => art("Door", ["plaid"], 1, WHOLE)).toThrow(/the hue plaid, and the hues are red/);
  expect(() => art("Door", [], 1, WHOLE)).toThrow(/no hues/);
});
