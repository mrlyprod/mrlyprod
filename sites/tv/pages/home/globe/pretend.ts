import { type Rng, rng } from "./rand.ts";

export type Trip = { from: number; to: number; start: number; dur: number };

export type Route = (rand: Rng) => Trip;

export type Stream = {
  at: (time: number) => Trip[];
  fade: number;
};

const GLOW = 1.6;

export function stream(seed: number, rate: number, route: Route, longest: number, fade: number): Stream {
  const trip = (k: number): Trip => {
    const made = route(rng(seed, k));
    return { ...made, start: k / rate };
  };
  return {
    fade,
    at(time) {
      const out: Trip[] = [];
      const first = Math.max(0, Math.ceil((time - longest - fade) * rate));
      const last = Math.floor(time * rate);
      for (let k = first; k <= last; k++) {
        const one = trip(k);
        if (one.start <= time && time < one.start + one.dur + fade) out.push(one);
      }
      return out;
    },
  };
}

const spark = (age: number) => (age < 0 ? 0 : Math.max(0, 1 - age / GLOW) ** 2);

export function glow(trips: Trip[], time: number, out: Float32Array): Float32Array {
  out.fill(0);
  for (const { from, to, start, dur } of trips) {
    out[from] = Math.max(out[from], spark(time - start));
    out[to] = Math.max(out[to], spark(time - start - dur));
  }
  return out;
}
