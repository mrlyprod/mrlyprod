export type Atom = "carpet" | "net" | "htree" | "vtree" | "void";

export type Sprite = { size: number; cells: Uint8Array };

type Rule = (r: number, c: number) => boolean;

type Path = Pick<CanvasRenderingContext2D, "rect">;

export const ATOMS: readonly Atom[] = ["carpet", "net", "htree", "vtree", "void"];

export const SIZES: readonly number[] = [3, 5, 7, 9];

const RULES: Record<Atom, Rule> = {
  carpet: (r, c) => (r & 1) + (c & 1) <= 1,
  net: (r, c) => (r & 1) + (c & 1) >= 1,
  htree: (r) => (r & 1) === 0,
  vtree: (_r, c) => (c & 1) === 0,
  void: (r, c) => (r & 1) === (c & 1),
};

export function kron(a: Sprite, b: Sprite): Sprite {
  const size = a.size * b.size;
  const cells = new Uint8Array(size * size);
  for (let r = 0; r < size; r++) {
    const up = Math.floor(r / b.size) * a.size;
    const lo = (r % b.size) * b.size;
    for (let c = 0; c < size; c++)
      cells[r * size + c] = a.cells[up + Math.floor(c / b.size)] & b.cells[lo + (c % b.size)];
  }
  return { size, cells };
}

export function sprite(atom: Atom, size: number, level = 1): Sprite {
  const cells = new Uint8Array(size * size);
  for (let r = 0; r < size; r++) for (let c = 0; c < size; c++) cells[r * size + c] = RULES[atom](r, c) ? 1 : 0;
  const base: Sprite = { size, cells };
  let out = base;
  for (let step = 1; step < level; step++) out = kron(out, base);
  return out;
}

export function trace(path: Path, { size, cells }: Sprite, x: number, y: number, side: number) {
  for (let r = 0; r < size; r++) {
    const top = Math.round(y + (r * side) / size);
    const tall = Math.round(y + ((r + 1) * side) / size) - top;
    for (let c = 0; c < size; c++) {
      if (!cells[r * size + c]) continue;
      const left = Math.round(x + (c * side) / size);
      path.rect(left, top, Math.round(x + ((c + 1) * side) / size) - left, tall);
    }
  }
}
