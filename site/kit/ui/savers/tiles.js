export const DESIGNS = ['carpet', 'net', 'htree', 'vtree', 'void'];

export const NUMBERS = [3, 5, 7, 9];

export const LEVELS = [1, 2];

/* RULES */

export const RULES = {
  carpet: (r, c) => (r & 1) + (c & 1) <= 1,
  net: (r, c) => (r & 1) + (c & 1) >= 1,
  htree: (r) => (r & 1) === 0,
  vtree: (r, c) => (c & 1) === 0,
  void: (r, c) => (r & 1) === (c & 1),
};

export function seed(design, n) {
  const rule = RULES[design] ?? RULES.carpet;
  const cells = new Uint8Array(n * n);
  for (let r = 0; r < n; r++) for (let c = 0; c < n; c++) cells[r * n + c] = rule(r, c) ? 1 : 0;
  return { size: n, cells };
}

/* GROWTH */

export function kron(base, level = 2) {
  let out = base;
  const p = base.size;
  for (let step = 1; step < level; step++) {
    const size = out.size * p;
    const cells = new Uint8Array(size * size);
    for (let r = 0; r < size; r++) {
      const up = ((r / p) | 0) * out.size;
      const lo = (r % p) * p;
      for (let c = 0; c < size; c++) cells[r * size + c] = out.cells[up + ((c / p) | 0)] & base.cells[lo + (c % p)];
    }
    out = { size, cells };
  }
  return out;
}

export function tile(design, n = NUMBERS[0], level = 1) {
  return kron(seed(design, n), level);
}
