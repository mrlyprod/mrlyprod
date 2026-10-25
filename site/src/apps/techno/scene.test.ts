import { expect, test } from "bun:test";
import type { Opts, View } from "../../lib/app/run.ts";
import { defaults } from "../../lib/app/spec.ts";
import { pattern, tempo } from "../../lib/audio/audio.ts";
import { rng } from "../../lib/rand.ts";
import { FEEL, make, SPEC, STYLES, song, stamp, styled } from "./scene.ts";

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

const open = (value: Opts = {}) => {
  const log: Call[] = [];
  const heard: unknown[][] = [];
  const view = {
    rand: rng(value.seed ?? 7),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: false,
    w: 800,
    h: 450,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    seed: 7,
    ...value,
    audio: { at: (...args: unknown[]) => heard.push(args) },
  });
  const frame = (t: number) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { heard, frame };
};

test("each draw hands the bus t and the drive mood with the song of the value", () => {
  const live = open({ bpm: 120, key: "d" });
  live.frame(0);
  live.frame(16);
  const tune = song({ ...defaults(SPEC), seed: 7, bpm: 120, key: "d" });
  expect(live.heard).toEqual([
    [0, { name: "drive", song: tune }],
    [16, { name: "drive", song: tune }],
  ]);
});

test("the grid holds within a step and moves at the next", () => {
  const live = open();
  const time = tempo(song({ ...defaults(SPEC), seed: 7 }));
  const at = (i: number) => time.time(i) + 50;
  expect(live.frame(at(5) + 5)).toEqual(live.frame(at(5) + 60));
  expect(live.frame(at(6) + 5)).not.toEqual(live.frame(at(5) + 5));
});

test("one seed draws one grid; another seed another", () => {
  expect(open({ seed: 7 }).frame(900)).toEqual(open({ seed: 7 }).frame(900));
  expect(open({ seed: 8 }).frame(900)).not.toEqual(open({ seed: 7 }).frame(900));
});

test("every style is a song the knobs keep, the first is the default one, and a value finds its style", () => {
  const kept = STYLES.map((one) => FEEL.every((key) => song(one.song)[key] === one.song[key]));
  expect([
    kept.every(Boolean),
    new Set(STYLES.map((one) => JSON.stringify(one.song))).size,
    FEEL.every((key) => STYLES[0].song[key] === defaults(SPEC)[key]),
  ]).toEqual([true, STYLES.length, true]);
  expect([styled({ ...defaults(SPEC), ...STYLES[3].song }), styled({ ...defaults(SPEC), bpm: 119 })]).toEqual([
    STYLES[3].id,
    "",
  ]);
});

test("a stamp lights one square for each hit of the first bar, four rows of sixteen", () => {
  const log: Call[] = [];
  const p = pattern(7, song({ ...defaults(SPEC), seed: 7 }), 0);
  stamp(pen(log), 56, 56, p, "#008cff");
  const lit = [p.kick, p.hat, p.clap, p.bass].flat().filter(Boolean).length;
  expect([
    log.filter(([key]) => key === "fillRect").length,
    log.filter(([key, v]) => key === "fillStyle" && v === "#008cff").length,
  ]).toEqual([64, lit]);
});
