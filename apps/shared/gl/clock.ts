/* TYPES */

export type Phase = readonly [name: string, ms: number];

export type Span = { name: string; from: number; ms: number };

export type At = { name: string | null; k: number; since: number; left: number };

export type Clock = {
  at: (t: number) => At;
  start: (t: number) => void;
  end: (t: number) => void;
  spans: () => Span[];
};

export type Curve = readonly [from: number, to: number, power: number];

export type Speeds = Partial<Record<string, number | Curve>>;

/* CLOCK */

export function clock(phases: readonly Phase[]): Clock {
  const names = phases.map(([name]) => name);
  const given = phases.map(([, ms]) => (Number.isFinite(ms) ? Math.max(0, ms) : Infinity));
  let lengths = [...given];
  let origin: number | null = null;
  const start = (t: number) => {
    origin = t;
    lengths = [...given];
  };
  const end = (t: number) => {
    if (origin === null) return;
    let from = origin;
    for (let i = 0; i < lengths.length; i++) {
      if (lengths[i] === Infinity) {
        lengths[i] = Math.max(0, t - from);
        return;
      }
      from += lengths[i];
    }
  };
  const spans = () => {
    if (origin === null) return [];
    let from = origin;
    return names.map((name, i) => {
      const span = { name, from, ms: lengths[i] };
      from += lengths[i];
      return span;
    });
  };
  const at = (t: number): At => {
    if (origin === null || t < origin) return { name: null, k: 0, since: 0, left: Infinity };
    let from = origin;
    for (let i = 0; i < names.length; i++) {
      const ms = lengths[i];
      if (t < from + ms) {
        const since = t - from;
        return { name: names[i], k: ms === Infinity ? 0 : since / ms, since, left: ms - since };
      }
      from += lengths[i];
    }
    return { name: null, k: 1, since: t - from, left: 0 };
  };
  return { at, start, end, spans };
}

const curve = (speed: number | Curve | undefined): Curve =>
  typeof speed === "number" ? [speed, speed, 1] : (speed ?? [0, 0, 1]);

export function flown(clock: Clock, speeds: Speeds, t: number) {
  let s = 0;
  for (const { name, from, ms } of clock.spans()) {
    if (t <= from) break;
    const dur = Math.min(t - from, ms);
    if (dur <= 0) continue;
    const [a, b, n] = curve(speeds[name]);
    const k = ms === Infinity ? 0 : dur / ms;
    s += (dur / 1000) * (a + ((b - a) * k ** n) / (n + 1));
  }
  return s;
}
