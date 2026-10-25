import { expect, test } from "bun:test";
import { audio } from "../../../test/fake.ts";
import type { View } from "../../lib/app/run.ts";
import type { Value } from "../../lib/app/spec.ts";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Knobs } from "./scene.ts";
import { analyse, band, facts, lasts, make, NAMES, PRESETS, preset, SHAPE, SPEC, seconds } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

const pen = (log: Call[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, value: unknown) => {
      log.push([key, value]);
      return true;
    },
  });

const sine = (hertz: number, level: number, length = 22050, rate = 44100) => {
  const buffer = audio().createBuffer(2, length, rate);
  for (let c = 0; c < 2; c++)
    buffer
      .getChannelData(c)
      .set(Float32Array.from({ length }, (_, i) => level * Math.sin((2 * Math.PI * hertz * i) / rate)));
  return buffer;
};

const open = (value: Value = {}) => {
  const log: Call[] = [];
  const heard: unknown[][] = [];
  const asked: unknown[][] = [];
  const view = {
    rand: rng(7),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: false,
    w: 640,
    h: 360,
    dpr: 1,
    t: 0,
  } as View;
  const bus = {
    at: (...args: unknown[]) => heard.push(["at", ...args]),
    cue: (...args: unknown[]) => heard.push(["cue", ...args]),
  };
  const render = (...args: unknown[]) => {
    asked.push(args);
    return Promise.resolve(sine(440, 0.5));
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    seed: 7,
    ...value,
    audio: bus,
    render,
  });
  const frame = (t: number) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { scene, view, heard, asked, frame };
};

test("play cues the effect on the bus at t with the shape of the value", () => {
  const live = open({ effect: "exit", pitch: -3 });
  live.frame(1200);
  live.scene.cue();
  expect(live.heard).toEqual([
    ["at", 1200],
    ["cue", "exit", 1200, { pitch: -3, length: 1, bright: 0.5, drive: 0.2, space: 0.4 }],
  ]);
});

test("the scene renders its effect once for its seconds and draws the waveform it gets", async () => {
  const live = open({ effect: "tunnel", length: 2 });
  const blank = live.frame(0);
  await new Promise((done) => setTimeout(done, 0));
  expect(live.asked).toEqual([
    [
      seconds({ effect: "tunnel", length: 2 }),
      { seed: 7, cues: [["tunnel", 0, { pitch: 0, length: 2, bright: 0.5, drive: 0.2, space: 0.4 }]] },
    ],
  ]);
  expect(live.frame(0).filter(([key]) => key === "rect").length).toBeGreaterThan(
    blank.filter(([key]) => key === "rect").length + 100,
  );
});

test("the playhead moves only while the effect plays", () => {
  const live = open({ effect: "hit" });
  const long = seconds({ effect: "hit", length: 1 }) * 1000;
  expect(live.frame(500)).toEqual(live.frame(100));
  live.scene.cue();
  expect(live.frame(600)).not.toEqual(live.frame(900));
  expect(live.frame(500 + long + 10)).toEqual(live.frame(500 + long + 400));
});

test("analyse reads a sine at its level and in its band", () => {
  const seen = analyse(sine(1000, 0.5), 8, 48);
  expect([Math.max(...seen.peaks).toFixed(2), Math.min(...seen.peaks).toFixed(2)]).toEqual(["0.50", "-0.50"]);
  const column = Array.from(seen.heat.slice(4 * 48, 5 * 48));
  expect(column.indexOf(Math.max(...column))).toBe(band(1000));
});

test("every preset is a sound the knobs keep, a value finds its preset, and an effect says how long it lasts", () => {
  const kept = PRESETS.map(
    (one) =>
      NAMES.includes(one.set.effect) &&
      ["effect" as const, ...SHAPE].every((key) => tidy(SPEC, one.set)[key] === one.set[key]),
  );
  expect([kept.every(Boolean), new Set(PRESETS.map((one) => one.id)).size]).toEqual([true, PRESETS.length]);
  expect([
    preset({ ...defaults(SPEC), ...PRESETS[2].set }),
    preset({ ...defaults(SPEC), ...PRESETS[2].set, pitch: 0 }),
    lasts("tick"),
    lasts("hit"),
    lasts("jump"),
  ]).toEqual([PRESETS[2].id, "", "20 ms", "0.8 s", "2.6 s"]);
});

test("the status names the effect, its length at the knob and its preset", () => {
  expect([
    facts({ ...defaults(SPEC), ...PRESETS[2].set }),
    facts({ ...(defaults(SPEC) as Knobs), effect: "tick", length: 2 })[1],
  ]).toEqual([
    [
      ["effect", "Hit"],
      ["length", lasts("hit", 2)],
      ["preset", "Boom"],
    ],
    ["length", lasts("tick", 2)],
  ]);
});
