import { CLEAR } from "./frame.js";

const NAMES = ["ground", "panel", "line", "fg", "dim", "red", "orange", "yellow", "green", "mint", "teal", "cyan", "blue", "indigo", "purple", "pink", "brown", "gray"];

// COLOR

const byte = (v) => {
  const r = Math.round(v);
  return r > 255 ? 255 : r > 0 ? r : 0;
};
const clamp = (v) => (v < 0 ? 0 : v > 1 ? 1 : v);

function color(value, name) {
  if (typeof value === "string" && /^#?([0-9a-f]{6}|[0-9a-f]{8})$/i.test(value)) {
    const hex = value.replace("#", "");
    const n = (i) => parseInt(hex.slice(i, i + 2), 16);
    return Object.freeze([n(0), n(2), n(4), hex.length === 8 ? n(6) : 255]);
  }
  if (value && typeof value === "object" && "r" in value) return Object.freeze([value.r, value.g, value.b, value.a ?? 255]);
  if (value && value.length >= 3) return Object.freeze([value[0], value[1], value[2], value[3] ?? 255]);
  throw new Error(`ink: ${name} is not a color: ${value}`);
}

// MIXING

export function mix(a, b, t) {
  const s = clamp(t);
  const lerp = (x, y) => byte(x + (y - x) * s);
  return [lerp(a[0], b[0]), lerp(a[1], b[1]), lerp(a[2], b[2]), lerp(a[3], b[3])];
}

export function fade(c, alpha) {
  return [c[0], c[1], c[2], byte(255 * clamp(alpha))];
}

// RAMP

export class Ramp {
  constructor(stops, ground = CLEAR) {
    this.stops = stops;
    this.ground = ground;
  }

  at(t) {
    const n = this.stops.length;
    if (n === 0) return this.ground;
    if (n === 1) return this.stops[0];
    const s = clamp(t) * (n - 1);
    const i = Math.min(Math.floor(s) || 0, n - 2);
    return mix(this.stops[i], this.stops[i + 1], s - i);
  }
}

// INK

export function ink(palette) {
  const out = {};
  for (const name of NAMES) {
    if (palette[name] === undefined) throw new Error(`ink: the palette has no ${name}`);
    out[name] = color(palette[name], name);
  }
  const { ground, fg, blue, orange, yellow, green, pink, indigo } = out;
  class Bound extends Ramp {
    constructor(stops) {
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

    static tone(a, b) {
      return new Bound([a, b]);
    }
  }
  return Object.freeze({ ...out, inks: Object.freeze([blue, orange, yellow, green, pink, indigo]), mix, fade, Ramp: Bound });
}
