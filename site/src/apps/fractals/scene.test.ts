import { expect, test } from "bun:test";
import type { Drawn, FakeGl } from "../../../test/fake.ts";
import { gl } from "../../../test/fake.ts";
import type { View } from "../../lib/app/run.ts";
import type { Value } from "../../lib/app/spec.ts";
import { asked, tidy } from "../../lib/app/spec.ts";
import { unit } from "../../lib/fractal/shader.ts";
import { spanOf } from "../../lib/fractal/view.ts";
import { rng } from "../../lib/rand.ts";
import { PLACES } from "./places.ts";
import type { Flight, Given } from "./scene.ts";
import { go, make, SAVERS, SPEC } from "./scene.ts";

const canvas = (fake: FakeGl) =>
  Object.assign({} as HTMLCanvasElement, {
    width: 1,
    height: 1,
    getContext: (kind: string) => (kind === "webgl2" ? fake : null),
  });

const VIEW = {
  rand: rng(7),
  look: () => ({ paper: "#102030", accent: "#ff8000" }),
  still: false,
  w: 1280,
  h: 720,
  dpr: 1,
  t: 0,
} as View;

const LOCK = { ...VIEW, lock: true };

const paints = (fake: FakeGl) => fake.draws().filter((one) => "uAlpha" in one.uniforms && one.fb === null);

const span = (one: Drawn) => +(Math.hypot(...(one.uniforms.uU as number[])) * VIEW.h).toFixed(9);

test("under the lock each kind dives from the start and is still diving a minute in, the seed picking the kind when none is asked", () => {
  const dive = (opts: Given) => {
    const fake = gl();
    const at = { ...LOCK };
    const scene = make(canvas(fake), at, opts);
    scene.draw();
    at.t = 60000;
    scene.draw();
    scene.stop();
    return paints(fake).map((one) => one.uniforms.uJulia);
  };
  expect([
    dive({ seed: 7, kind: "mandelbrot" }),
    dive({ seed: 7, kind: "julia" }),
    dive({ seed: 2 }),
    dive({ seed: 3 }),
  ]).toEqual([
    [0, 0],
    [1, 1],
    [0, 0],
    [1, 1],
  ]);
});

test("under the lock the knobs map: palette empty to classic and seeded to the seeded ramp, path 1 to one more draw, speed 2 to twice the zoom", () => {
  const drawn = (opts: Given, t = 0) => {
    const fake = gl();
    make(canvas(fake), { ...LOCK, t }, { seed: 7, kind: "mandelbrot", ...opts }).draw();
    const [one] = paints(fake);
    return { fake, one, span: span(one) };
  };
  const looks = ["", "seeded"].map((palette) =>
    ["uGround", "uRamp"].map((name) => name in drawn({ palette }).one.uniforms),
  );
  const pens = [0, 1].map(
    (path) =>
      drawn({ path })
        .fake.draws()
        .filter((one) => one.vs.includes("aShape")).length,
  );
  expect([looks, pens, drawn({ speed: 2 }, 2000).span]).toEqual([
    [
      [true, false],
      [false, true],
    ],
    [0, 1],
    drawn({ speed: 1 }, 4000).span,
  ]);
});

test("every saver, on any roll, draws in the site tint: its ground is the paper and its colour the accent", () => {
  const inks = SAVERS.flatMap((saver) =>
    [1, 2, 3].map((seed) => {
      const fake = gl();
      make(canvas(fake), LOCK, { seed, ...saver.pick(rng(seed)) }).draw();
      const [one] = paints(fake);
      return [one.uniforms.uGround, one.uniforms.uAccent];
    }),
  );
  expect(new Set(inks.map((one) => JSON.stringify(one)))).toEqual(
    new Set([JSON.stringify([unit("#102030"), unit("#ff8000")])]),
  );
});

test("the page opens still on the whole set, holds it as time runs, and asks to rest after every still frame", () => {
  const fake = gl();
  const rests: (Value | null)[] = [];
  const at = { ...VIEW };
  const scene = make(canvas(fake), at, { seed: 7, onRest: (place) => rests.push(place) }) as Flight;
  scene.draw();
  at.t = 60000;
  scene.draw();
  const [one, two] = paints(fake);
  expect([span(one), span(two), one.uniforms.uOff, scene.flying(), rests]).toEqual([3, 3, [0, 0], false, [null, null]]);
});

test("Depth caps the zoom: the scene gets the cap, the url keeps the asked zoom, and a flight lands at the cap with its place", () => {
  const value = { depth: "shallow", zoom: 40, jzoom: 60 };
  expect([
    tidy(SPEC, value).zoom,
    tidy(SPEC, value).jzoom,
    asked(SPEC, value).zoom,
    tidy(SPEC, { ...value, depth: "deep" }).zoom,
  ]).toEqual([20, 20, 40, 40]);
  const rests: (Value | null)[] = [];
  const at = { ...VIEW };
  const scene = make(canvas(gl()), at, { seed: 7, onRest: (place) => rests.push(place) }) as Flight;
  scene.fly();
  at.t = 120000;
  scene.draw();
  expect([scene.flying(), +scene.at().span.toExponential(6), rests.at(-1)?.zoom]).toEqual([
    false,
    +spanOf(20).toExponential(6),
    20,
  ]);
});

test("a place picked mid-flight jumps there and stops, a palette change keeps flying from where it is", () => {
  const at = { ...VIEW };
  const start = { seed: 7, ...tidy(SPEC, {}) };
  const scene = make(canvas(gl()), at, start) as Flight;
  scene.fly();
  at.t = 4000;
  scene.draw();
  const before = scene.at().span;
  scene.set({ ...start, palette: "fire" });
  const kept = [scene.flying(), scene.at().span === before];
  const spot = PLACES.find((one) => one.id === "seahorse");
  scene.set({ ...start, ...go(spot!) });
  expect([...kept, scene.flying(), +scene.at().span.toExponential(6)]).toEqual([
    true,
    true,
    false,
    +spanOf(8).toExponential(6),
  ]);
});
