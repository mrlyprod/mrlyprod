import { rng } from "../rand.ts";

/* TYPES */

export type Rand = () => number;

export type Kind = "terran" | "desert" | "ice" | "lava" | "gas" | "moon" | "deck";

export type Air = { h: number; hm: number; g: number; top: number; ray: number[]; mie: number[]; dust: number[] };

export type Moon = { r: number; a: number; incl: number; rate: number; tone: number[]; rough: number; phase?: number };

export type Zone = [inner: number, outer: number, tau: number, dust: number, tone: number];

export type Ring = {
  inner: number;
  outer: number;
  zones: Zone[];
  seed: number;
  dark: number[];
  bright: number[];
  halo?: number;
};

export type World = {
  kind: Kind;
  noise: number;
  tilt: number;
  spin: number;
  phase: number;
  freq: number;
  sea?: number;
  bump: number;
  cover: number;
  lights?: number;
  jets?: number;
  colors?: number[][];
  bands?: number[][];
  feature?: number[];
  tone?: number[];
  cloud?: number[];
  drift?: number;
  city?: number[];
  glow?: number[];
  air: Air | null;
  spec?: number;
  rough?: number;
  limb: number;
  flat: number;
  ring: Ring | null;
  moons: Moon[];
  light?: number[];
  name?: string;
};

export type Lit = World & { light: number[] };

type Rolled = Omit<World, "moons"> & { moons?: Moon[] };

type Palette = { dark: string; mid: string; light: string; accent: string; storm: string };

const TAU = Math.PI * 2;
const SALT = 0x7f4a7c15;
const TYPES: Kind[] = ["terran", "desert", "ice", "lava", "gas"];
const WEIGHTS = [0.42, 0.16, 0.14, 0.1, 0.18];
const ALBEDO = 0.8;
const PALETTES: Palette[] = [
  { dark: "#8c6a3e", mid: "#b8935c", light: "#e0d2b0", accent: "#96634a", storm: "#b2603c" },
  { dark: "#4d6270", mid: "#6a9296", light: "#c4d2cf", accent: "#5b7088", storm: "#8a6a6a" },
  { dark: "#8c6a72", mid: "#c19b9c", light: "#e2d8c8", accent: "#6c5c6c", storm: "#c0707a" },
];

/* COLOUR */

export const lin = (hex: string, k = ALBEDO): number[] =>
  [0, 2, 4].map((i) => (Number.parseInt(hex.slice(1 + i, 3 + i), 16) / 255) ** 2.2 * k);

export function hsl(h: number, s: number, l: number, k = ALBEDO): number[] {
  const f = (n: number) => {
    const a = (n + h * 12) % 12;
    return l - s * Math.min(l, 1 - l) * Math.max(-1, Math.min(a - 3, 9 - a, 1));
  };
  return [f(0), f(8), f(4)].map((c) => Math.max(0, c) ** 2.2 * k);
}

export const air = ({ h, hm, g, ray, mie, dust, top }: Air): Air => ({
  h,
  hm,
  g,
  top,
  ray: ray.map((v) => v / h),
  mie: mie.map((v) => v / hm),
  dust: dust.map((v) => v / hm),
});

/* EXO */

function roll(rand: Rand): Kind {
  let r = rand();
  let i = 0;
  while (i < TYPES.length - 1 && r > WEIGHTS[i]) {
    r -= WEIGHTS[i];
    i++;
  }
  return TYPES[i];
}

export function moonOf(rand: Rand): Moon {
  const R = (a: number, b: number) => a + rand() * (b - a);
  const a = R(3.6, 9);
  const icy = rand() < 0.4;
  return {
    r: R(0.025, 0.11),
    a,
    incl: R(-0.3, 0.3),
    rate: (rand() < 0.15 ? -1 : 1) * 0.5 * a ** -1.5,
    tone: icy ? hsl(R(0.5, 0.62), R(0, 0.2), R(0.65, 0.85), 0.7) : hsl(R(0.03, 0.12), R(0.05, 0.35), R(0.3, 0.55), 0.5),
    rough: R(0.2, 0.9),
  };
}

export function ringOf(rand: Rand, seed: number, body: number[] | null): Ring {
  const inner = 1.3 + rand() * 0.3;
  const outer = inner + 0.45 + rand() * 0.75;
  const zones: Zone[] = [];
  let r = inner;
  while (r < outer) {
    const w = 0.03 + rand() * 0.22;
    const gap = rand() < 0.22;
    zones.push([r, Math.min(r + w, outer), gap ? rand() * 0.04 : 0.08 + rand() ** 2 * 2.2, rand() * 0.4, rand()]);
    r += w;
  }
  const hue = rand();
  return {
    inner,
    outer,
    zones,
    seed,
    dark: body ? body.map((c) => c * 0.45) : hsl(hue, 0.15, 0.35),
    bright: body ? body.map((c) => Math.min(c * 1.15, 0.8)) : hsl(hue, 0.2, 0.7),
  };
}

export function exo(seed: number, type = ""): Lit {
  const rand = rng((seed ^ SALT) >>> 0);
  const R = (a: number, b: number) => a + rand() * (b - a);
  const rolled = roll(rand);
  const kind = TYPES.find((one) => one === type) ?? rolled;
  const p: Rolled = {
    kind,
    noise: Math.floor(R(0, 997)),
    tilt: R(-0.5, 0.5),
    spin: R(0.03, 0.07),
    phase: R(0, TAU),
    freq: R(0.95, 1.6),
    sea: 0,
    bump: 0.04,
    cover: 0,
    lights: 0,
    drift: R(0.002, 0.006),
    city: [0, 0, 0],
    glow: [0, 0, 0],
    spec: 0,
    rough: 0.42,
    limb: 0,
    flat: 0,
    air: null,
    ring: null,
  };
  if (kind === "terran") {
    const sea = R(0.5, 0.66);
    Object.assign(p, {
      sea: R(-0.04, 0.12),
      cover: R(0.35, 0.75),
      lights: rand() < 0.5 ? 1 : 0,
      colors: [
        hsl(sea, R(0.55, 0.85), R(0.08, 0.13)),
        hsl(sea + R(-0.04, 0.02), R(0.4, 0.7), R(0.17, 0.27)),
        hsl(R(0.17, 0.38), R(0.25, 0.55), R(0.12, 0.22)),
        hsl(R(0.04, 0.11), R(0.3, 0.6), R(0.45, 0.65)),
        hsl(R(0.04, 0.1), R(0.08, 0.22), R(0.3, 0.45)),
      ],
      cloud: hsl(0.6, 0.05, 0.95, 0.85),
      city: [1, R(0.5, 0.75), R(0.2, 0.4)].map((c) => c * 0.7),
      air: air({
        h: 0.004,
        hm: 0.0012,
        g: 0.76,
        ray: [0.046, 0.108, 0.265].map((v) => v * R(0.6, 1.6)),
        mie: [0.015, 0.015, 0.015],
        dust: [0.017, 0.017, 0.017],
        top: 1.03,
      }),
      spec: 1,
    } satisfies Partial<World>);
  } else if (kind === "desert") {
    const hue = R(0.02, 0.12);
    const bright = hsl(hue, R(0.4, 0.7), R(0.45, 0.6));
    const tau = R(0.03, 0.3);
    Object.assign(p, {
      cover: R(0, 0.15),
      flat: R(0.2, 0.6),
      bump: 0.05,
      colors: [
        bright,
        hsl(hue + R(-0.02, 0.02), R(0.15, 0.35), R(0.17, 0.28)),
        hsl(0.1, 0.1, 0.9),
        hsl(hue + 0.02, R(0.4, 0.6), R(0.55, 0.7)),
        hsl(hue, R(0.2, 0.4), R(0.3, 0.4)),
      ],
      feature: [0, 0, 0, 0],
      cloud: hsl(hue, 0.15, 0.88, 0.8),
      air: air({
        h: 0.004,
        hm: 0.004,
        g: 0.65,
        ray: [0.004, 0.008, 0.02],
        mie: bright.map((c) => tau * (0.4 + c)),
        dust: [tau * 1.3, tau * 1.3, tau * 1.3],
        top: 1.035,
      }),
    } satisfies Partial<World>);
  } else if (kind === "ice") {
    const thin = rand() < 0.5;
    Object.assign(p, {
      cover: thin ? R(0.1, 0.3) : 0,
      flat: 0.3,
      bump: 0.03,
      colors: [
        hsl(R(0.5, 0.62), R(0.05, 0.25), R(0.75, 0.88)),
        hsl(R(0.03, 0.1), R(0.3, 0.6), R(0.35, 0.5)),
        hsl(R(0.55, 0.62), R(0.4, 0.7), R(0.15, 0.3)),
        hsl(R(0.5, 0.6), R(0.1, 0.3), R(0.6, 0.75)),
        hsl(0.58, 0.1, 0.5),
      ],
      cloud: hsl(0.6, 0.05, 0.95, 0.85),
      air: thin
        ? air({
            h: 0.005,
            hm: 0.002,
            g: 0.7,
            ray: [0.02, 0.05, 0.12],
            mie: [0.01, 0.01, 0.01],
            dust: [0.012, 0.012, 0.012],
            top: 1.04,
          })
        : null,
    } satisfies Partial<World>);
  } else if (kind === "lava") {
    Object.assign(p, {
      cover: R(0.1, 0.35),
      bump: 0.05,
      colors: [
        hsl(R(0, 0.08), R(0.05, 0.2), R(0.06, 0.1)),
        hsl(R(0, 0.08), R(0.1, 0.25), R(0.14, 0.22)),
        hsl(0.05, 0.5, 0.3),
        hsl(0.05, 0.3, 0.25),
        hsl(0.05, 0.2, 0.2),
      ],
      cloud: hsl(0.05, 0.1, 0.3, 0.6),
      glow: ((k: number) => [1, R(0.9, 1), R(0.7, 1)].map((c) => c * k))(R(3, 4.5)),
      air: air({
        h: 0.005,
        hm: 0.003,
        g: 0.7,
        ray: [0.01, 0.015, 0.025],
        mie: [0.08, 0.05, 0.03],
        dust: [0.1, 0.1, 0.1],
        top: 1.035,
      }),
    } satisfies Partial<World>);
  } else {
    const palette = PALETTES[Math.floor(rand() * PALETTES.length)];
    const n = 9 + Math.floor(rand() * 5);
    const bands = Array.from({ length: n }, (_, i) => {
      const y = i === 0 ? -1 : i === n - 1 ? 1 : -1 + (2 * (i + R(-0.3, 0.3))) / (n - 1);
      const k = ALBEDO * R(0.9, 1.1);
      if (i > 0 && i < n - 1 && rand() < 0.2) return [y, ...lin(palette.accent, k)];
      const t = rand() < 0.3 ? R(0.25, 0.75) : i % 2 === 0 && i > 0 && i < n - 1 ? R(0.55, 1) : R(0.05, 0.5);
      const [lo, hi] = (t < 0.5 ? [palette.dark, palette.mid] : [palette.mid, palette.light]).map((hex) => lin(hex, k));
      const at = (t * 2) % 1;
      return [y, ...lo.map((c, j) => c + (hi[j] - c) * at)];
    });
    const storm = rand() < 0.6;
    Object.assign(p, {
      spin: R(0.08, 0.13),
      freq: R(0.8, 1.2),
      jets: R(0.0015, 0.003),
      bump: R(0.7, 1),
      bands,
      feature: storm ? [R(-0.5, 0.5), R(0, TAU), R(0.05, 0.12), R(0.5, 0.9)] : [0, 0, 0.01, 0],
      tone: lin(palette.storm),
      cover: R(0.12, 0.3),
      limb: R(0.35, 0.5),
    } satisfies Partial<World>);
  }
  const count = rand() < 0.55 ? 1 : rand() < 0.5 ? 2 : 0;
  p.moons = Array.from({ length: count }, () => moonOf(rand));
  if (rand() < (kind === "gas" ? 0.7 : 0.2))
    p.ring = ringOf(
      rand,
      Math.floor(rand() * 1e6),
      kind === "gas" ? p.bands![Math.floor(p.bands!.length / 2)].slice(1) : null,
    );
  p.light = hsl(R(0.05, 0.6), R(0, 0.25), 0.9, 1).map((c, _i, all) => c / Math.max(...all));
  return p as Lit;
}
