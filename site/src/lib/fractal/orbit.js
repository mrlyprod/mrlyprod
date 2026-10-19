import { BITS, ESCAPE, SHARP, WIDE } from './consts.js';

const NEWTON = 64;
const ZERO = 1e-8;
const CHUNK = 32;

/* NUMBERS */

export function fixed(x, bits = BITS) {
  if (!x || !Number.isFinite(x)) return 0n;
  let e = Math.floor(Math.log2(Math.abs(x))) - 52;
  if (Math.abs(x) / 2 ** e < 2 ** 52) e -= 1;
  const m = BigInt(Math.round(x / 2 ** e));
  const s = bits + e;
  return s >= 0 ? m << BigInt(s) : m >> BigInt(-s);
}

export const float = (v, bits = BITS) => Number(v) * 2 ** -bits;

export const shift = (v, dx, dy) => [v[0] + fixed(dx), v[1] + fixed(dy)];

export const apart = (a, b) => [float(a[0] - b[0]), float(a[1] - b[1])];

export const WORD = BigInt(BITS);

export const LIMIT = BigInt(ESCAPE) << WORD;

/* ORBIT */

export function orbit({ c, z0 = null, bits = BITS, max = 8192 }) {
  const b = BigInt(bits);
  const rows = Math.max(1, Math.ceil(max / WIDE));
  const data = new Float32Array(rows * WIDE * 2);
  const wide = new Float64Array(max * 2);
  const limit = BigInt(ESCAPE) << b;
  const [cr, ci] = c;
  let [zr, zi] = z0 ?? [0n, 0n];
  let length = 0;
  let done = false;
  const put = () => {
    const x = float(zr, bits);
    const y = float(zi, bits);
    wide[2 * length] = x;
    wide[2 * length + 1] = y;
    data[2 * length] = x;
    data[2 * length + 1] = y;
    length += 1;
  };
  put();
  const self = { data, wide, rows, max, julia: Boolean(z0), length, done, crit: null };
  self.crit = self;
  self.step = (n = Infinity) => {
    for (let k = 0; k < n && !done; k++) {
      const rr = (zr * zr) >> b;
      const ii = (zi * zi) >> b;
      if (length >= max || rr + ii > limit) {
        done = true;
        break;
      }
      const ri = (zr * zi) >> (b - 1n);
      zr = rr - ii + cr;
      zi = ri + ci;
      put();
    }
    self.length = length;
    self.done = done;
    return done;
  };
  return self;
}

export function slice(o, spare) {
  do o.step(CHUNK);
  while (!o.done && spare());
}

const walker = (ref, dc) => {
  const julia = ref.julia;
  const er = julia ? 0 : dc[0];
  const ei = julia ? 0 : dc[1];
  let o = ref;
  let m = 0;
  let dr = julia ? dc[0] : 0;
  let di = julia ? dc[1] : 0;
  return () => {
    const tr = 2 * o.wide[2 * m] + dr;
    const ti = 2 * o.wide[2 * m + 1] + di;
    const nr = tr * dr - ti * di + er;
    di = tr * di + ti * dr + ei;
    dr = nr;
    m += 1;
    const zr = o.wide[2 * m] + dr;
    const zi = o.wide[2 * m + 1] + di;
    if (Math.hypot(zr, zi) < Math.hypot(dr, di) || m >= o.length - 1) {
      o = o.crit;
      m = 0;
      dr = zr - o.wide[0];
      di = zi - o.wide[1];
    }
    return [zr, zi];
  };
};

const start = (ref, dc) => (ref.julia ? [ref.wide[0] + dc[0], ref.wide[1] + dc[1]] : [0, 0]);

export function period(ref, dc, max = ref.max) {
  const next = walker(ref, dc);
  let low = Infinity;
  let p = 0;
  for (let n = 1; n <= max; n++) {
    const [zr, zi] = next();
    const r = zr * zr + zi * zi;
    if (r > ESCAPE) break;
    if (r < low) {
      if (!p || n % p || r < SHARP * low) p = n;
      low = r;
    }
  }
  return p;
}

export function exact(ref, dc, p) {
  const next = walker(ref, dc);
  const tiny = [];
  for (let n = 1; n <= p; n++) {
    const [zr, zi] = next();
    tiny.push(Math.hypot(zr, zi) < ZERO);
  }
  for (let q = 1; q <= p; q++) if (p % q === 0 && tiny[q - 1]) return q;
  return 0;
}

export function nucleus(ref, dc, p) {
  if (!(p >= 1)) return null;
  let [x, y] = dc;
  let tol = 0;
  let settled = 0;
  for (let it = 0; it < NEWTON; it++) {
    const next = walker(ref, [x, y]);
    let [zr, zi] = start(ref, [x, y]);
    let gr = ref.julia ? 1 : 0;
    let gi = 0;
    for (let n = 1; n <= p; n++) {
      const nr = 2 * (zr * gr - zi * gi) + (ref.julia ? 0 : 1);
      gi = 2 * (zr * gi + zi * gr);
      gr = nr;
      [zr, zi] = next();
    }
    const q = gr * gr + gi * gi;
    if (!(q > 0) || !Number.isFinite(zr + zi)) return null;
    const sr = (zr * gr + zi * gi) / q;
    const si = (zi * gr - zr * gi) / q;
    x -= sr;
    y -= si;
    const step = Math.hypot(sr, si);
    if (!Number.isFinite(x + y)) return null;
    if (!it) tol = 1e-12 * Math.max(step, Math.hypot(...dc));
    if (step <= tol) settled += 1;
    if (settled > 1) return [x, y];
  }
  return null;
}

export function ball(ref, dc, radius, max = ref.max) {
  const next = walker(ref, dc);
  let [zr, zi] = start(ref, dc);
  let r = ref.julia ? radius : 0;
  for (let n = 1; n <= max; n++) {
    r = (2 * Math.hypot(zr, zi) + r) * r + (ref.julia ? 0 : radius);
    [zr, zi] = next();
    const a = Math.hypot(zr, zi);
    if (a < r) return n;
    if (a * a > ESCAPE || !Number.isFinite(r)) return 0;
  }
  return 0;
}

export function size(ref, dc, p) {
  const next = walker(ref, dc);
  if (ref.julia) {
    let [zr, zi] = start(ref, dc);
    let lr = 1;
    let li = 0;
    for (let n = 0; n < p; n++) {
      const nr = 2 * (zr * lr - zi * li);
      li = 2 * (zr * li + zi * lr);
      lr = nr;
      if (n < p - 1) [zr, zi] = next();
    }
    const q = lr * lr + li * li;
    return [lr / q, -li / q];
  }
  let lr = 1;
  let li = 0;
  let br = 1;
  let bi = 0;
  for (let n = 1; n < p; n++) {
    const [zr, zi] = next();
    const nr = 2 * (zr * lr - zi * li);
    li = 2 * (zr * li + zi * lr);
    lr = nr;
    const q = lr * lr + li * li;
    br += lr / q;
    bi -= li / q;
  }
  const l2r = lr * lr - li * li;
  const l2i = 2 * lr * li;
  const dr = br * l2r - bi * l2i;
  const di = br * l2i + bi * l2r;
  const q = dr * dr + di * di;
  return [dr / q, -di / q];
}
