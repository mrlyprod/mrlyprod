import { expect, test } from 'bun:test';
import { SONG, acid, clap, drone, engine, hat, hit, kick, noise, pad, pattern, render, riser, swell, tick, wav } from './audio.js';
import { audio } from './fake.js';

const FLOOR = 0.001;

const VOICES = { kick, hat, clap, acid, pad, riser, drone, hit, swell, tick };

const DRIVE = { name: 'drive', song: { ...SONG, swing: 0 } };

const fixed = (calls) => calls.map(([verb, ...args]) => [verb, ...args.map((v) => +v.toFixed(6))]);

const lone = (voice, args = {}) => {
  const ctx = audio();
  voice(ctx, ctx.destination, 1, args);
  return ctx;
};

const kicks = (ctx) => ctx.find('Oscillator').filter((o) => o.frequency.calls[0]?.[1] === 150).map((o) => +o.frequency.calls[0][2].toFixed(6));

const ends = (node, seen = new Set()) => {
  if (!node?.kind || seen.has(node)) return [];
  seen.add(node);
  const own = node.kind === 'Gain' && node.gain.calls.length ? [Math.max(...node.gain.calls.map((c) => c.at(-1)))] : [];
  return [...own, ...node.outputs.flatMap((to) => ends(to, seen))];
};

const starts = (ctx, shift, until) =>
  ctx.log
    .filter(([verb]) => verb === 'start')
    .map(([, id, when]) => [ctx.nodes[id].kind, Math.round((when - shift) * 1e7)])
    .filter(([, when]) => when < until * 1e7)
    .sort((a, b) => a[1] - b[1] || a[0].localeCompare(b[0]));

test('pattern is pure: one seed, knobs and bar give one bar of 16 steps', () => {
  const bar = pattern(7, SONG, 3);
  expect(bar).toEqual(pattern(7, SONG, 3));
  expect([bar.kick, bar.hat, bar.open, bar.clap, bar.bass].map((row) => row.length)).toEqual([16, 16, 16, 16, 16]);
});

test('another seed gives another groove', () => {
  expect(pattern(8, SONG, 3)).not.toEqual(pattern(7, SONG, 3));
});

test('the groove holds through a section of 8 bars and mutates at the next', () => {
  expect(pattern(7, SONG, 4)).toEqual(pattern(7, SONG, 0));
  expect(pattern(7, SONG, 8)).not.toEqual(pattern(7, SONG, 0));
});

test('wav writes a 16-bit PCM header and the interleaved samples after it', async () => {
  const buffer = audio().createBuffer(2, 3, 8000);
  buffer.getChannelData(0).set([0, 1, -1]);
  buffer.getChannelData(1).set([0.5, -0.5, 2]);
  const blob = wav(buffer);
  const bytes = new DataView(await blob.arrayBuffer());
  const word = (at) => String.fromCharCode(...new Uint8Array(bytes.buffer, at, 4));
  expect([blob.type, blob.size]).toEqual(['audio/wav', 44 + 3 * 2 * 2]);
  const head = [word(0), bytes.getUint32(4, true), word(8), word(12), bytes.getUint32(16, true), bytes.getUint16(20, true), bytes.getUint16(22, true)];
  const format = [bytes.getUint32(24, true), bytes.getUint32(28, true), bytes.getUint16(32, true), bytes.getUint16(34, true), word(36), bytes.getUint32(40, true)];
  expect([...head, ...format]).toEqual(['RIFF', 48, 'WAVE', 'fmt ', 16, 1, 2, 8000, 32000, 4, 16, 'data', 12]);
  expect([...new Int16Array(bytes.buffer, 44)]).toEqual([0, 16384, 32767, -16383, -32767, 32767]);
});

test('the anchor maps t to context time 50 ms ahead and re-anchors after a gap over 80 ms', () => {
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

test('a paused scene books nothing', () => {
  const ctx = audio({ time: 1 });
  const one = engine({ make: () => ctx });
  one.start();
  one.at(1000, DRIVE);
  const made = ctx.nodes.length;
  ctx.currentTime = 3;
  one.at(1000, DRIVE);
  expect(ctx.nodes.length).toBe(made);
});

test('start sets the level at once, and set ramps it only when sound or music changes', () => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  one.set({ sound: 1, music: 1 });
  const master = ctx.find('Gain').find((g) => g.gain.calls.some((c) => c[0] === 'setValueAtTime' && c[1] === 0.9));
  const ramps = () => master.gain.calls.filter((c) => c[0] === 'linearRampToValueAtTime').length;
  const first = ramps();
  one.set({ sound: 0 });
  expect([first, ramps()]).toEqual([0, 1]);
});

test('a muted score books nothing, and music 1 books it again', () => {
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
  const quiet = ctx.find('Oscillator').length;
  one.set({ music: 1 });
  play(3760, 7500);
  expect([quiet, ctx.find('Oscillator').length > 0]).toEqual([0, true]);
});

test('a 48 kHz context gets its reverb impulse at 48 kHz', () => {
  const ctx = audio({ rate: 48000 });
  engine({ make: () => ctx }).start();
  const { buffer } = ctx.find('Convolver')[0];
  expect([buffer.sampleRate, buffer.length]).toEqual([48000, Math.floor(48000 * 3.2)]);
});

test('sound 0 ramps every effect path to 0 in 0.3 s, so a playing drone stops, and sound 1 opens them again', () => {
  const ctx = audio({ time: 1 });
  const one = engine({ make: () => ctx });
  one.start();
  one.cue('drone', 1000, { space: 0.4, echoes: 0.2 });
  ctx.currentTime = 2;
  one.set({ sound: 0 });
  ctx.currentTime = 3;
  one.set({ sound: 1 });
  const shut = (node) => node.kind === 'Gain' && node.gain.calls.some((c) => c[0] === 'linearRampToValueAtTime' && c[1] === 0 && Math.abs(c[2] - 2.3) < 1e-9) && node.gain.calls.some((c) => c[0] === 'linearRampToValueAtTime' && c[1] === 1 && Math.abs(c[2] - 3.3) < 1e-9);
  const leaks = (node, seen = new Set()) => node === ctx.destination || (!shut(node) && !seen.has(node) && (seen.add(node), node.outputs.some((next) => next.kind && leaks(next, seen))));
  const saw = ctx.find('Oscillator').find((o) => o.type === 'sawtooth');
  const kick = ctx.nodes.length;
  one.at(0, { name: 'drive', song: { ...SONG, swing: 0 } });
  const beat = ctx.find('Oscillator').find((o) => o.id >= kick && o.frequency.value === 150);
  expect([leaks(saw), leaks(beat)]).toEqual([false, true]);
});

test('the hum follows the flicker only when its target moves by over 0.01', () => {
  const ctx = audio();
  const one = engine({ make: () => ctx });
  one.start();
  for (const [t, flicker] of [[0, 0.5], [16, 0.5], [32, 0.504], [48, 1]]) one.at(t, { flicker });
  const aims = ctx.log.filter(([verb]) => verb === 'setTargetAtTime').map(([, , , to]) => +to.toFixed(4));
  expect(aims).toEqual([1.175, 1.35]);
});

test('the kick falls from 150 to 45 Hz in 60 ms', () => {
  expect(lone(kick).find('Oscillator').map((o) => fixed(o.frequency.calls))).toEqual([[['setValueAtTime', 150, 1], ['exponentialRampToValueAtTime', 45, 1.06]]]);
});

test('a kick dips the music bus from 1 to 0.3 and back to 1 in 150 ms', () => {
  const ctx = audio();
  const duck = ctx.createGain();
  kick(ctx, ctx.destination, 1, { duck: duck.gain });
  expect(fixed(duck.gain.calls)).toEqual([['cancelScheduledValues', 1], ['setValueAtTime', 1, 1], ['linearRampToValueAtTime', 0.3, 1.01], ['linearRampToValueAtTime', 1, 1.15]]);
});

test('the riser sweeps its noise band from 200 Hz to 5 kHz over 1.8 s', () => {
  const band = lone(riser).find('BiquadFilter').find((f) => f.type === 'bandpass');
  expect(fixed(band.frequency.calls)).toEqual([['setValueAtTime', 200, 1], ['exponentialRampToValueAtTime', 5000, 2.8]]);
});

test('every voice opens and closes its envelopes at the floor and stops no source before its envelope ends', () => {
  for (const [name, voice] of Object.entries(VOICES)) {
    const ctx = lone(voice);
    const shaped = ctx.find('Gain').filter((g) => g.gain.calls.length);
    const edges = shaped.map((g) => [g.gain.calls[0][1] <= FLOOR, g.gain.calls.at(-1)[1] <= FLOOR]);
    expect([name, shaped.length > 0, edges.flat().every(Boolean)]).toEqual([name, true, true]);
    const early = ctx.nodes.filter((one) => one.stopped).filter((one) => ends(one).some((end) => one.stopped[0] < end));
    expect([name, early.map((one) => one.kind)]).toEqual([name, []]);
  }
});

test('noise comes from the seed, so live, offline and recorded runs share it', () => {
  const [a, b] = [audio(), audio()];
  expect(noise(a, 3).getChannelData(0)).toEqual(noise(b, 3).getChannelData(0));
  expect(noise(a, 4).getChannelData(0)).not.toEqual(noise(a, 3).getChannelData(0));
});

test('render books the bar the live engine plays, at the same offsets', async () => {
  const mood = { name: 'drive', song: { ...SONG, bars: 1, swing: 0 } };
  let off = null;
  await render(2, { seed: 5, mood }, (channels, length, rate) => (off = audio({ rate, length })));
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
