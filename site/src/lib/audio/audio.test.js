import { expect, test } from "bun:test";
import { audio } from "../../../test/fake.js";
import { engine, pattern, render, SONG } from "./audio.js";

const STRAIGHT = { ...SONG, swing: 0 };

const DRIVE = { name: "drive", song: STRAIGHT };

const fixed = (calls) => calls.map(([verb, ...args]) => [verb, ...args.map((v) => +v.toFixed(6))]);

const kicks = (ctx) =>
  ctx
    .find("Oscillator")
    .filter((o) => o.frequency.calls[0]?.[1] === 150)
    .map((o) => +o.frequency.calls[0][2].toFixed(6));

const hz = (note) => 440 * 2 ** ((note - 69) / 12);

const film = (arrange, until) => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  for (let t = 0; t <= until; t += 10) {
    ctx.currentTime = t / 1000;
    one.at(t, { name: "film", song: STRAIGHT, arrange });
  }
  return ctx;
};

const heard = (ctx, detune) =>
  ctx
    .find("Oscillator")
    .filter((o) => o.detune.value === detune)
    .map((o) => +o.started[0].toFixed(6));

const starts = (ctx, shift, until) =>
  ctx.log
    .filter(([verb]) => verb === "start")
    .map(([, id, when]) => [ctx.nodes[id].kind, Math.round((when - shift) * 1e7)])
    .filter(([, when]) => when < until * 1e7)
    .sort((a, b) => a[1] - b[1] || a[0].localeCompare(b[0]));

test("pattern is pure: one seed, knobs and bar give one bar of 16 steps", () => {
  const bar = pattern(7, SONG, 3);
  expect(bar).toEqual(pattern(7, SONG, 3));
  expect([bar.kick, bar.hat, bar.open, bar.clap, bar.bass].map((row) => row.length)).toEqual([16, 16, 16, 16, 16]);
});

test("the drums, bass and chord of a seed stay as they were, so the techno loop is the same", () => {
  const row = (r) => r.map((v) => (v ? 1 : 0)).join("");
  const line = (r) => r.map((b) => (b ? `${b.note}${b.accent ? "a" : ""}${b.slide ? "s" : ""}` : "-")).join(" ");
  const frozen = (bar) => {
    const p = pattern(7, SONG, bar);
    return {
      kick: row(p.kick),
      hat: row(p.hat),
      open: row(p.open),
      clap: row(p.clap),
      bass: line(p.bass),
      chord: p.chord,
    };
  };
  expect(frozen(3)).toEqual({
    kick: "1000100010001000",
    hat: "1110001101111111",
    open: "0010001000100010",
    clap: "0000100000001000",
    bass: "- - 52s 52s - 48 40 40 - 52a 52a 40 - 40 40 -",
    chord: [64, 67, 71, 74],
  });
  expect(frozen(8)).toEqual({
    kick: "1000100010001000",
    hat: "1111001101111111",
    open: "0010001000100010",
    clap: "0000100000001000",
    bass: "- - 57s 53 - 53 45 45 - 57a 57a 45 - 45 48a -",
    chord: [57, 60, 64, 67],
  });
});

test("pattern adds 16 arp picks, skips and octave lifts from a stream of its own", () => {
  const bar = pattern(7, SONG, 3);
  expect([bar.arp.length, bar.skip.length, bar.octave.length]).toEqual([16, 16, 16]);
  expect([
    bar.arp.every((n) => n >= 0 && n < 4),
    bar.skip.every((v) => typeof v === "boolean"),
    bar.octave.every((v) => v === 0 || v === 12),
  ]).toEqual([true, true, true]);
  expect(pattern(8, SONG, 3).arp).not.toEqual(bar.arp);
});

test("another seed gives another groove", () => {
  expect(pattern(8, SONG, 3)).not.toEqual(pattern(7, SONG, 3));
});

test("the groove holds through a section of 8 bars and mutates at the next", () => {
  expect(pattern(7, SONG, 4)).toEqual(pattern(7, SONG, 0));
  expect(pattern(7, SONG, 8)).not.toEqual(pattern(7, SONG, 0));
});

test("the anchor maps t to context time 50 ms ahead and re-anchors after a gap over 80 ms", () => {
  const ctx = audio({ time: 10 });
  const one = engine({ seed: 3, make: () => ctx });
  one.start();
  one.at(0, DRIVE);
  ctx.currentTime = 10.4;
  one.at(350, DRIVE);
  ctx.currentTime = 12;
  one.at(400, DRIVE);
  ctx.currentTime = 12.5;
  one.at(900, DRIVE);
  expect(kicks(ctx)).toEqual([10.05, 10.51875, 12.5875]);
});

test("a paused scene books nothing", () => {
  const ctx = audio({ time: 1 });
  const one = engine({ make: () => ctx });
  one.start();
  one.at(1000, DRIVE);
  const made = ctx.nodes.length;
  ctx.currentTime = 3;
  one.at(1000, DRIVE);
  expect(ctx.nodes.length).toBe(made);
});

test("start sets the level at once, and set ramps it only when sound or music changes", () => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  one.set({ sound: 1, music: 1 });
  const master = ctx.find("Gain").find((g) => g.gain.calls.some((c) => c[0] === "setValueAtTime" && c[1] === 0.9));
  const ramps = () => master.gain.calls.filter((c) => c[0] === "linearRampToValueAtTime").length;
  const first = ramps();
  one.set({ sound: 0 });
  expect([first, ramps()]).toEqual([0, 1]);
});

test("a muted score books nothing, and music 1 books it again", () => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  one.set({ sound: 1, music: 0 });
  const play = (from, to) => {
    for (let t = from; t <= to; t += 10) {
      ctx.currentTime = t / 1000;
      one.at(t, DRIVE);
    }
  };
  play(0, 3750);
  const quiet = ctx.find("Oscillator").length;
  one.set({ music: 1 });
  play(3760, 7500);
  expect([quiet, ctx.find("Oscillator").length > 0]).toEqual([0, true]);
});

test("a 48 kHz context gets its reverb impulse at 48 kHz", () => {
  const ctx = audio({ rate: 48000 });
  engine({ make: () => ctx }).start();
  const { buffer } = ctx.find("Convolver")[0];
  expect([buffer.sampleRate, buffer.length]).toEqual([48000, Math.floor(48000 * 3.2)]);
});

test("sound 0 ramps every effect path to 0 in 0.3 s, so a playing drone stops, and sound 1 opens them again", () => {
  const ctx = audio({ time: 1 });
  const one = engine({ make: () => ctx });
  one.start();
  one.cue("drone", 1000, { space: 0.4, echoes: 0.2 });
  ctx.currentTime = 2;
  one.set({ sound: 0 });
  ctx.currentTime = 3;
  one.set({ sound: 1 });
  const shut = (node) =>
    node.kind === "Gain" &&
    node.gain.calls.some((c) => c[0] === "linearRampToValueAtTime" && c[1] === 0 && Math.abs(c[2] - 2.3) < 1e-9) &&
    node.gain.calls.some((c) => c[0] === "linearRampToValueAtTime" && c[1] === 1 && Math.abs(c[2] - 3.3) < 1e-9);
  const leaks = (node, seen = new Set()) => {
    if (node === ctx.destination) return true;
    if (shut(node) || seen.has(node)) return false;
    seen.add(node);
    return node.outputs.some((next) => next.kind && leaks(next, seen));
  };
  const saw = ctx.find("Oscillator").find((o) => o.type === "sawtooth");
  const kick = ctx.nodes.length;
  one.at(0, { name: "drive", song: { ...SONG, swing: 0 } });
  const beat = ctx.find("Oscillator").find((o) => o.id >= kick && o.frequency.value === 150);
  expect([leaks(saw), leaks(beat)]).toEqual([false, true]);
});

test("the hum follows the flicker only when its target moves by over 0.01", () => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  for (const [t, flicker] of [
    [0, 0.5],
    [16, 0.5],
    [32, 0.504],
    [48, 1],
  ])
    one.at(t, { flicker });
  const aims = ctx.log.filter(([verb]) => verb === "setTargetAtTime").map(([, , , to]) => +to.toFixed(4));
  expect(aims).toEqual([1.175, 1.35]);
});

test("render books the bar the live engine plays, at the same offsets", async () => {
  const mood = { name: "drive", song: { ...SONG, bars: 1, swing: 0 } };
  let off = null;
  await render(2, { seed: 5, mood }, (_channels, length, rate) => (off = audio({ rate, length })));
  const ctx = audio();
  const one = engine({ seed: 5, make: () => ctx });
  one.start();
  for (let t = 0; t <= 2000; t += 10) {
    ctx.currentTime = t / 1000;
    one.at(t, mood);
  }
  const bar = 1.875;
  expect(starts(ctx, 0.05, bar)).toEqual(starts(off, 0, bar));
  expect(starts(off, 0, bar).length).toBeGreaterThan(16);
});

test("the intro books the progression pad once a bar: the chord, then its root an octave up", () => {
  const ctx = film(() => ({ name: "intro", swell: 0 }), 3000);
  const { chord } = pattern(SONG.seed, STRAIGHT, 0);
  const notes = [chord[0], chord[1], chord[2], chord[0] + 12].map(hz);
  const pads = ctx.find("Oscillator").filter((o) => o.detune.value === -9 || o.detune.value === 9);
  expect(pads.map((o) => o.frequency.value).slice(0, 8)).toEqual(notes.flatMap((n) => [n, n]));
  expect(pads.length).toBe(16);
});

test("a boost books a kick on step 0", () => {
  const ctx = film(() => ({ name: "boost", swell: 0 }), 100);
  expect(kicks(ctx)[0]).toBe(0.05);
});

test("a section reached at 1300 ms is heard from the first step after it and never before", () => {
  const ctx = film((ms) => ({ name: ms < 1300 ? "intro" : "boost", swell: 0 }), 2000);
  expect(heard(ctx, 7).slice(0, 2)).toEqual([1.45625, 1.690625]);
});

test("a cut books nothing from 100 to 550 ms and the gate ramps to 0 in 10 ms there and back to 1 by its end", () => {
  const ctx = film((ms) => ({ name: ms >= 100 && ms < 550 ? "cut" : "boost", swell: 0 }), 1000);
  const gate = ctx.find("Gain").find((g) => g.gain.calls.some((c) => c[0] === "linearRampToValueAtTime" && c[1] === 0));
  const booked = ctx.log.filter(([verb, , when]) => verb === "start" && when > 0.1501 && when < 0.5999);
  expect([booked.length, fixed(gate.gain.calls).slice(2)]).toEqual([
    0,
    [
      ["setValueAtTime", 1, 0.15],
      ["linearRampToValueAtTime", 0, 0.16],
      ["setValueAtTime", 0, 0.59],
      ["linearRampToValueAtTime", 1, 0.6],
    ],
  ]);
});

test("the swell opens the arp filter to 500 + 3200 swell and the bass to half of it", () => {
  const ctx = film(() => ({ name: "hyper", swell: 0.5 }), 400);
  const opened = (q) => ctx.find("BiquadFilter").find((f) => f.Q.value === q).frequency.calls[0][1];
  expect([opened(2), opened(3) / 2.5]).toEqual([2100, 1050]);
});

test("the outro plays the kick and the arp only while their flags are up", () => {
  const played = (flags) => film(() => ({ name: "outro", swell: 0.3, ...flags }), 1000);
  expect([
    kicks(played({ kick: false })).length,
    kicks(played({ kick: true })).length > 0,
    heard(played({}), 7).length,
    heard(played({ arp: true }), 7).length > 0,
  ]).toEqual([0, true, 0, true]);
});
