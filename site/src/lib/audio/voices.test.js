import { expect, test } from 'bun:test';
import { VOICES, arp, bass, kick, noise, riser } from './voices.js';
import { audio } from '../../../test/fake.js';

const FLOOR = 0.001;

const fixed = (calls) => calls.map(([verb, ...args]) => [verb, ...args.map((v) => +v.toFixed(6))]);

const lone = (voice, args = {}) => {
  const ctx = audio();
  voice(ctx, ctx.destination, 1, args);
  return ctx;
};

const ends = (node, seen = new Set()) => {
  if (!node?.kind || seen.has(node)) return [];
  seen.add(node);
  const own = node.kind === 'Gain' && node.gain.calls.length ? [Math.max(...node.gain.calls.map((c) => c.at(-1)))] : [];
  return [...own, ...node.outputs.flatMap((to) => ends(to, seen))];
};

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

test('the arp is two saws at -7 and +7 cents through a lowpass of Q 2 that closes to 0.3 of its cutoff in 0.25 s', () => {
  const ctx = lone(arp, { note: 69, cutoff: 2000 });
  const [low] = ctx.find('BiquadFilter');
  const amp = ctx.find('Gain').find((g) => g.gain.calls.length);
  expect(ctx.find('Oscillator').map((o) => [o.type, o.detune.value, o.frequency.value])).toEqual([['sawtooth', -7, 440], ['sawtooth', 7, 440]]);
  expect([low.type, low.Q.value, fixed(low.frequency.calls)]).toEqual(['lowpass', 2, [['setValueAtTime', 2000, 1], ['exponentialRampToValueAtTime', 600, 1.25]]]);
  expect(fixed(amp.gain.calls)).toEqual([['setValueAtTime', 0.0001, 1], ['linearRampToValueAtTime', 0.16, 1.003], ['setValueAtTime', 0.16, 1.023], ['exponentialRampToValueAtTime', 0.0001, 1.243]]);
});

test('the arp sends 0.35 to the reverb and 0.3 to the delay', () => {
  const ctx = audio();
  const [verb, echo] = [ctx.createConvolver(), ctx.createGain()];
  arp(ctx, ctx.destination, 1, { verb, echo });
  const sent = ctx.find('Gain').filter((g) => g.outputs[0] === verb || g.outputs[0] === echo);
  expect(sent.map((g) => [g.gain.value, g.outputs[0] === verb])).toEqual([[0.35, true], [0.3, false]]);
});

test('the bass is a saw and a quiet square an octave down through two lowpasses of Q 3 that close from 2.5 to 0.6 of the cutoff over its length', () => {
  const ctx = lone(bass, { note: 45, cutoff: 400, dur: 0.4 });
  const lows = ctx.find('BiquadFilter');
  const amp = ctx.find('Gain').find((g) => g.gain.calls.length);
  expect(ctx.find('Oscillator').map((o) => [o.type, o.frequency.value])).toEqual([['sawtooth', 110], ['square', 55]]);
  expect(lows.map((f) => [f.Q.value, fixed(f.frequency.calls)])).toEqual(Array(2).fill([3, [['setValueAtTime', 1000, 1], ['exponentialRampToValueAtTime', 240, 1.4]]]));
  expect(fixed(amp.gain.calls)).toEqual([['setValueAtTime', 0.0001, 1], ['linearRampToValueAtTime', 0.4, 1.004], ['setValueAtTime', 0.4, 1.204], ['exponentialRampToValueAtTime', 0.0001, 1.404]]);
});
