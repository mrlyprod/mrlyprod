import { expect, test } from "bun:test";
import type { Call, FakeGl } from "../../../test/fake.ts";
import { gl } from "../../../test/fake.ts";
import type { Handle } from "../../lib/app/page.tsx";
import type { View } from "../../lib/app/run.ts";
import { rng } from "../../lib/rand.ts";
import type { Given, Jump } from "./scene.ts";
import { make, PAGE, SAVERS } from "./scene.ts";

type Shell = { paper?: string; accent?: string; webgl?: boolean; bad?: string | null };

type Live = { fake: FakeGl; pen: unknown[][]; view: View; scene: Jump };

const open = (
  opts: Given = {},
  { paper = "#000000", accent = "#008cff", webgl = true, bad = null }: Shell = {},
): Live => {
  const fake = gl({ bad });
  const pen: unknown[][] = [];
  const canvas = Object.assign({} as HTMLCanvasElement, {
    width: 1280,
    height: 720,
    getContext: (kind: string) =>
      kind === "webgl2" ? (webgl ? fake : null) : { fillRect: (...args: unknown[]) => pen.push(args) },
  });
  const view = {
    rand: rng(opts.seed ?? 7),
    look: () => ({ paper, accent }),
    still: false,
    w: 1280,
    h: 720,
    dpr: 1,
    t: 0,
  } as View;
  return { fake, pen, view, scene: make(canvas, view, { seed: 7, ...opts }) };
};

const sent = (call: Call) => {
  const data = call[3];
  const length = call[5];
  if (!(data instanceof Float32Array) || typeof length !== "number") throw new Error("no quads were sent");
  return Array.from(data.slice(0, length));
};

const made = (call: Call | undefined) => {
  const size = call?.[2];
  if (typeof size !== "number") throw new Error("no buffer was made");
  return size;
};

const flying = (scene: Jump) => {
  if (!("stage" in scene)) throw new Error("the scene is the blank one");
  return scene;
};

const play = (live: Live, times: number[]) => {
  for (const t of times) {
    live.view.t = t;
    live.scene.draw();
  }
};

const tape = (opts: Given, times = [0, 600, 1500, 2600]) => {
  const live = open(opts);
  const frames: number[][][] = [];
  live.scene.trigger();
  for (const t of times) {
    live.fake.log.length = 0;
    play(live, [t]);
    frames.push(live.fake.log.filter(([key]) => key === "bufferSubData").map(sent));
  }
  return frames;
};

test("one seed and one set of knobs paint one star field", () => {
  const knobs = { speed: 1.5, wind: 0.5 };
  expect(tape({ seed: 7, ...knobs })).toEqual(tape({ seed: 7, ...knobs }));
  expect(tape({ seed: 7, ...knobs })).not.toEqual(tape({ seed: 8, ...knobs }));
});

test("density multiplies the 6000 stars of a desk", () => {
  const stars = (opts: Given) => made(open(opts).fake.log.find(([key]) => key === "bufferData")) / 32;
  expect([stars({}), stars({ stars: 0.5 }), stars({ stars: 2 })]).toEqual([6000, 3000, 12000]);
});

test("an empty or NaN knob takes its default", () => {
  const plain = tape({});
  expect(tape({ speed: "", idle: NaN, stars: "x", look: "foam", hyper: 7 })).toEqual(plain);
});

test("the ground is black in both themes", () => {
  const night = open({}, { paper: "#000000" });
  const day = open({}, { paper: "#ffffff" });
  play(night, [0, 100]);
  play(day, [0, 100]);
  const clears = day.fake.log.filter(([key]) => key === "clearColor").map((call) => call.slice(1));
  expect([day.fake.log.length === night.fake.log.length, new Set(clears.map(String))]).toEqual([
    true,
    new Set(["0,0,0,1"]),
  ]);
});

test("a trigger while one plays fires its onDone with the first at the white", () => {
  const live = open();
  const seen: (string | number)[][] = [];
  live.scene.trigger(() => seen.push(["first", live.view.t]));
  play(live, [0, 50]);
  live.scene.trigger(() => seen.push(["second", live.view.t]));
  play(live, [1000, 3000]);
  live.view.t = 3400;
  live.scene.exit();
  play(live, [3400, 3800, 4000]);
  expect(seen).toEqual([
    ["first", 3800],
    ["second", 3800],
  ]);
});

test("stop fires every onDone still waiting and lets the context go", () => {
  const live = open();
  let done = 0;
  live.scene.trigger(() => done++);
  live.scene.trigger(() => done++);
  play(live, [0, 50]);
  live.scene.stop();
  expect([done, live.fake.log.some(([key]) => key === "loseContext")]).toEqual([2, true]);
});

test("the page cruises with no hands while every saver jumps on its own", () => {
  const left = (opts: Given) => {
    const live = open(opts);
    for (let t = 0; t <= 31000; t += 500) {
      play(live, [t]);
      if (live.scene.phase() !== "cruise") return true;
    }
    return false;
  };
  expect([left({}), ...SAVERS.map((saver) => left(saver.pick(rng(7))))]).toEqual([false, ...SAVERS.map(() => true)]);
});

test("Enter jumps from cruise and exits once under way", () => {
  const live = open();
  const jump = () => PAGE.actions.jump(live.scene as Handle<Jump>);
  play(live, [0]);
  jump();
  play(live, [1000]);
  const going = live.scene.phase();
  jump();
  play(live, [1050]);
  expect([going, live.scene.phase(), flying(live.scene).stage()]).toEqual(["jump", "exit", "bloom"]);
});

test("without WebGL2 the scene paints black and a trigger lands at once", () => {
  const live = open({}, { webgl: false });
  const seen: string[] = [];
  live.scene.trigger(
    () => seen.push("done"),
    () => seen.push("jump"),
  );
  live.scene.draw();
  expect([seen, live.pen.length, live.scene.phase()]).toEqual([["jump", "done"], 1, "cruise"]);
});

test("a shader that fails to compile leaves a black scene whose trigger lands at once", () => {
  const live = open({}, { bad: "uColor" });
  const seen: string[] = [];
  live.scene.trigger(
    () => seen.push("done"),
    () => seen.push("jump"),
  );
  live.scene.draw();
  expect([seen, live.scene.phase()]).toEqual([["jump", "done"], "cruise"]);
});

const named = (at: unknown) => (at instanceof Object && "name" in at && typeof at.name === "string" ? at.name : "");

const iterable = (one: unknown): one is Iterable<unknown> =>
  one instanceof Object && Symbol.iterator in one && typeof one[Symbol.iterator] === "function";

const vector = (one: unknown) => {
  if (!iterable(one)) throw new Error("the uniform is not a vector");
  return one;
};

const uniform = (live: Live, name: string) =>
  live.fake.log.filter(([key, one]) => key.startsWith("uniform") && named(one) === name).map((call) => vector(call[2]));

const jump = (opts: Given, shell?: Shell) => {
  const live = open(opts, shell);
  live.scene.trigger();
  for (let t = 0; t <= 2400; t += 100) play(live, [t]);
  return live;
};

test("classic lays white streaks over black and builds no cloud", () => {
  const live = jump({ look: "classic" });
  const halos = uniform(live, "uHalo");
  const cloud = (one: Live) => uniform(one, "uHalo").some((halo) => [...halo].some((c) => c !== 1));
  expect([
    halos.length > 0,
    halos.every((halo) => [...halo].every((c) => c === 1)),
    cloud(jump({})),
    live.fake.draws().some(({ fs }) => fs.includes("uMilk")),
  ]).toEqual([true, true, true, false]);
});
