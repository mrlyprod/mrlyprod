import { expect, test } from "bun:test";
import * as num from "@mrlyprod/mrly/num";
import type { ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Patch, Raw } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import type { FactsOf } from "./engine.ts";
import { designOf, study, warm } from "./engine.ts";
import { SPEC, tidied } from "./scene.ts";
import { Designs, ROWS, Widget } from "./widget.tsx";

const units = { num };

const value = (over: Raw = {}) => tidied({ ...defaults(SPEC), seed: 1, ...over });

const facts = <K extends "plaid" | "battery" | "arcs">(over: Raw) => study(num, value(over)).facts as FactsOf<K>;

const library = (over: Raw = {}) =>
  renderToStaticMarkup(<Designs units={units} value={value(over)} tick={0} onChange={() => {}} />);

test("the page renders nothing before the unit lands", () => {
  expect(
    renderToStaticMarkup(
      <Widget
        value={{ ...value(), seed: 1 }}
        onChange={() => {}}
        onReady={() => {}}
        onStatus={() => {}}
        onExport={() => {}}
      />,
    ),
  ).toBe("");
});

test("the plaid reads out the stream, N, the bias, the row and the near-heads", () => {
  expect(ROWS.plaid(facts<"plaid">({}))).toEqual([
    ["stream", "2:1, Thue-Morse along n^2"],
    ["N", "2^12 = 4096"],
    ["bias", "S(N) = -260, z -4.06"],
    ["row", "64 wide, 64 rows"],
    ["plus", "1918 of 4096"],
    ["near-heads", "(2, 0), (4, 0), (8, 0), kernel 2 exact, reading +1, +1, +1, +1"],
  ]);
});

test("the battery reads out the verdict and the largest z of each test, the near-head named", () => {
  const read = Object.fromEntries(ROWS.battery(facts<"battery">({ view: "battery" })));
  expect(read).toMatchObject({
    verdict: "fails: the dilation test at (2, 0), z 64.00 lights",
    shift: "z 3.66 at h = 15",
    dilation: "z 64.00 at (2, 0), an exact near-head",
    kernel: "z 64.00 at b = 2, an exact near-head",
    square: "z 64.00",
  });
  expect(String(read.lit).startsWith("(2, 0) 64.00, (4, 0) 64.00")).toBe(true);
  const free = Object.fromEntries(
    ROWS.battery(facts<"battery">({ view: "battery", rudin: 1, reading: "Primes", power: 10 })),
  );
  expect([free.verdict, free["near-heads"]]).toEqual(["passes: every z inside 3", "none: a free reading"]);
});

test("the arcs read out the residual, the bias and the prediction once the model is warm, and say why without one", () => {
  warm(num, designOf(value()));
  const read = Object.fromEntries(ROWS.arcs(facts<"arcs">({ view: "arcs" })));
  expect(read).toMatchObject({
    residual: "13.3, z 0.21",
    bias: "S(N) = -260, z -4.06",
    "major arcs": "-273.3, z -4.27",
    arcs: "1/3 at eta 0.2075, 2/3 at eta 0.2075",
  });
  const cube = Object.fromEntries(ROWS.arcs(facts<"arcs">({ view: "arcs", reading: "Cube" })));
  expect(cube.residual).toBe("the major arcs predict the squares; along this reading the bias stands alone");
  const cold = Object.fromEntries(ROWS.arcs(facts<"arcs">({ view: "arcs", base: 6, digits: "135" })));
  expect(cold.residual).toBe("summing the arcs");
});

test("the library shelves the featured designs with thumbs and the five readings, the ones on the stage lit, and a pick writes its patch", () => {
  const html = library();
  expect(html.match(/<h3>(\w+)<\/h3>/g)).toEqual(["<h3>Designs</h3>", "<h3>Readings</h3>"]);
  expect([...html.matchAll(/class="label">([^<]*)</g)].map((hit) => hit[1])).toEqual([
    "Thue-Morse",
    "Rudin-Shapiro",
    "Ternary parity",
    "4:13",
    "6:135",
    "6:5",
    "6:12345",
    "5:14",
    "5:34",
    "n",
    "n²",
    "n² + n",
    "n³",
    "primes",
  ]);
  expect((html.match(/<canvas /g) ?? []).length).toBe(9);
  const lit = (over: Raw) =>
    [...library(over).matchAll(/aria-current="true"[\s\S]*?class="label">([^<]*)</g)].map((hit) => hit[1]);
  expect([lit({}), lit({ rudin: 1, reading: "Primes" }), lit({ base: 6, digits: "531", reading: "N" })]).toEqual([
    ["Thue-Morse", "n²"],
    ["Rudin-Shapiro", "primes"],
    ["6:135", "n"],
  ]);
  const wrote: Patch[] = [];
  const shelves = Designs({ units, value: value(), tick: 0, onChange: (patch) => wrote.push(patch) }) as ReactElement<{
    children: ReactElement<{ children: ReactElement<{ onPick: () => void }>[] }>[];
  }>;
  const [designs, readings] = shelves.props.children;
  designs.props.children[4].props.onPick();
  readings.props.children[2].props.onPick();
  expect(wrote).toEqual([{ rudin: 0, base: 6, digits: "135" }, { reading: "SquarePlusN" }]);
});
