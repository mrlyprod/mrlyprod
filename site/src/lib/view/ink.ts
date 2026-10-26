import { CLEAR } from "./frame.ts";

/* TYPES */

export type Color = [number, number, number, number];

export type Swatch = string | ArrayLike<number> | { r: number; g: number; b: number; a?: number };

export const NAMES = [
  "ground",
  "panel",
  "line",
  "fg",
  "dim",
  "red",
  "orange",
  "yellow",
  "green",
  "mint",
  "teal",
  "cyan",
  "blue",
  "indigo",
  "purple",
  "pink",
  "brown",
  "gray",
] as const;

export type Name = (typeof NAMES)[number];

export type Palette = Record<Name, Swatch>;

export type Ramps = {
  new (stops: Color[]): Ramp;
  heat(): Ramp;
  fire(): Ramp;
  diverge(): Ramp;
  tone(a: Color, b: Color): Ramp;
};

export type Ink = Readonly<Record<Name, Color>> & {
  readonly inks: readonly Color[];
  mix(a: Color, b: Color, t: number): Color;
  fade(c: Color, alpha: number): Color;
  Ramp: Ramps;
};

/* COLOR */

const byte = (v: number) => {
  const r = Math.round(v);
  return r > 0 ? Math.min(r, 255) : 0;
};
export const clamp = (v: number) => (v < 0 ? 0 : Math.min(v, 1));

function color(value: Swatch, name: string): Color {
  if (typeof value === "string" && /^#?([0-9a-f]{6}|[0-9a-f]{8})$/i.test(value)) {
    const hex = value.replace("#", "");
    const n = (i: number) => parseInt(hex.slice(i, i + 2), 16);
    return Object.freeze<Color>([n(0), n(2), n(4), hex.length === 8 ? n(6) : 255]) as Color;
  }
  if (value && typeof value === "object" && "r" in value)
    return Object.freeze<Color>([value.r, value.g, value.b, value.a ?? 255]) as Color;
  if (value && typeof value !== "string" && value.length >= 3)
    return Object.freeze([value[0], value[1], value[2], value[3] ?? 255]) as Color;
  throw new Error(`ink: ${name} is not a color: ${value}`);
}

/* MIXING */

export function mix(a: Color, b: Color, t: number): Color {
  const s = clamp(t);
  const lerp = (x: number, y: number) => byte(x + (y - x) * s);
  return [lerp(a[0], b[0]), lerp(a[1], b[1]), lerp(a[2], b[2]), lerp(a[3], b[3])];
}

export function fade(c: Color, alpha: number): Color {
  return [c[0], c[1], c[2], byte(255 * clamp(alpha))];
}

/* RAMP */

export class Ramp {
  stops: Color[];
  ground: Color;

  constructor(stops: Color[], ground: Color = CLEAR) {
    this.stops = stops;
    this.ground = ground;
  }

  at(t: number): Color {
    const n = this.stops.length;
    if (n === 0) return this.ground;
    if (n === 1) return this.stops[0];
    const s = clamp(t) * (n - 1);
    const i = Math.min(Math.floor(s) || 0, n - 2);
    return mix(this.stops[i], this.stops[i + 1], s - i);
  }
}

/* INK */

export function ink(palette: Palette): Ink {
  const out = {} as Record<Name, Color>;
  for (const name of NAMES) {
    if (palette[name] === undefined) throw new Error(`ink: the palette has no ${name}`);
    out[name] = color(palette[name], name);
  }
  const { ground, fg, blue, orange, yellow, green, pink, indigo } = out;
  class Bound extends Ramp {
    constructor(stops: Color[]) {
      super(stops, ground);
    }

    static heat() {
      return new Bound([ground, blue, yellow, fg]);
    }

    static fire() {
      return new Bound([ground, orange, yellow, fg]);
    }

    static diverge() {
      return new Bound([blue, ground, orange]);
    }

    static tone(a: Color, b: Color) {
      return new Bound([a, b]);
    }
  }
  return Object.freeze({
    ...out,
    inks: Object.freeze([blue, orange, yellow, green, pink, indigo]),
    mix,
    fade,
    Ramp: Bound,
  });
}
