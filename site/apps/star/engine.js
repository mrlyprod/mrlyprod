import { odd } from '../moire/engine.js';

export const CARPET = '23';
export const SIZES = [128, 192, 256, 320, 384];
export const LEAST = 4;
export const MOST = 128;
const RIBBON = 12;
const DIAGONAL = [1, 1, 1];
const MIDDLE = 0.5;

export const deepest = (layers) => 2 * layers - 1;

export const more = (layers) => (layers + 2 > MOST ? LEAST : layers + 2);

/* CUTS */

export function cutter(math) {
  const spec = math.moire.Spec.new(CARPET, 2, 3);
  const frame = math.moire.frame(DIAGONAL, MIDDLE);
  const held = new Map();
  let inside = null;
  let side = 0;
  const cut = (n, size) => {
    if (size !== side) {
      held.clear();
      inside = null;
      side = size;
    }
    if (!held.has(n)) {
      const volume = math.moire.volume(spec, [n], 'Sum', 1, n);
      const [ink, mask] = volume.plane(frame, size);
      volume.free();
      held.set(n, Uint8Array.from(ink));
      inside ??= mask;
    }
    return held.get(n);
  };
  return { cut, mask: () => inside, count: () => held.size, free: () => held.clear() };
}

export function field(math, cuts, layers, size) {
  const data = new Float32Array(size * size);
  for (const n of odd(deepest(layers))) {
    const one = cuts.cut(n, size);
    for (let i = 0; i < data.length; i++) data[i] += one[i];
  }
  const inside = cuts.mask();
  let low = Infinity;
  let high = -Infinity;
  for (let i = 0; i < data.length; i++) {
    if (!inside[i]) {
      data[i] = NaN;
      continue;
    }
    data[i] /= layers;
    low = Math.min(low, data[i]);
    high = Math.max(high, data[i]);
  }
  return { field: math.moire.Field.from_data(data, size), data, size, low, high };
}

/* LAW */

const share = (one) => {
  const [numer, denom] = one.reduced();
  const out = { text: `${numer}/${denom}`, value: one.value() };
  one.free();
  return out;
};

export function rungs(layers) {
  const out = [layers];
  while (out.at(-1) % 4 === 0 && out.at(-1) / 2 >= LEAST) out.push(out.at(-1) / 2);
  return out.reverse();
}

export function law(math, star, layers, half) {
  const s = math.six.star;
  const excesses = star.excesses(layers, half);
  const read = s.decay(excesses, layers);
  const branch = s.Branch.of(layers);
  const ribbon = [];
  let exact = 0;
  let arm = null;
  for (const n of odd(deepest(layers))) {
    arm = share(star.arm(n, half));
    const rule = share(s.arm_law(n));
    exact += arm.text === rule.text ? 1 : 0;
    if (ribbon.length < RIBBON) ribbon.push({ n, arm: arm.text, law: rule.text, exact: arm.text === rule.text });
  }
  const n = deepest(layers);
  const hexagon = share(star.hexagon(n));
  return {
    layers,
    half,
    deepest: n,
    exact,
    ribbon,
    chi: Number(s.chi8(n)),
    arm: arm.value,
    hexagon: hexagon.value,
    excess: excesses[layers - 1],
    scaled: read.scaled,
    logged: read.logged,
    constant: s.constant(),
    slope: read.slope ?? null,
    target: s.width_law(half),
    branch: s.Branch.name(branch),
    residual: read.residual,
    predicted: s.Branch.residual(branch) ?? null,
    ladder: rungs(layers).map((count) => {
      const rung = s.decay(excesses, count);
      return { count, scaled: rung.scaled, logged: rung.logged };
    }),
  };
}
