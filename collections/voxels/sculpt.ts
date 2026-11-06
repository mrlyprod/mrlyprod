import { Canvas, type Colors, type Ink, save } from "../pixels/paint.ts";

export class Volume {
  side: number;
  colors: Colors;
  layers: Canvas[];

  constructor(side: number, colors: Colors) {
    this.side = side;
    this.colors = colors;
    this.layers = Array.from({ length: side }, () => new Canvas(side, side, colors));
  }

  inside(x: number, y: number, z: number): boolean {
    return [x, y, z].every((at) => at >= 0 && at < this.side);
  }

  layer(y: number): Canvas {
    const found = this.layers[y];
    if (!found) throw new Error(`no layer ${y} in a cube of ${this.side}`);
    return found;
  }

  get(x: number, y: number, z: number): Ink {
    const [u, v, w] = [Math.round(x), Math.round(y), Math.round(z)];
    return this.inside(u, v, w) ? this.layer(v).get(u, w) : null;
  }

  set(x: number, y: number, z: number, ink: Ink): this {
    const [u, v, w] = [Math.round(x), Math.round(y), Math.round(z)];
    if (this.inside(u, v, w)) this.layer(v).px(u, w, ink);
    return this;
  }

  box(x: number, y: number, z: number, w: number, h: number, d: number, ink: Ink): this {
    const [low, high] = [Math.max(0, Math.round(y)), Math.min(this.side, Math.round(y + h))];
    for (let v = low; v < high; v++)
      this.layer(v).rect(Math.round(x), Math.round(z), Math.round(w), Math.round(d), ink);
    return this;
  }

  ball(cx: number, cy: number, cz: number, rx: number, ry: number, rz: number, ink: Ink): this {
    if (![cx, cy, cz, rx, ry, rz].every(Number.isFinite)) throw new Error("a ball needs finite numbers");
    const span = (c: number, r: number) => [Math.max(0, Math.floor(c - r)), Math.min(this.side - 1, Math.ceil(c + r))];
    const [[x0, x1], [y0, y1], [z0, z1]] = [span(cx, rx), span(cy, ry), span(cz, rz)];
    for (let y = y0!; y <= y1!; y++)
      for (let z = z0!; z <= z1!; z++)
        for (let x = x0!; x <= x1!; x++) {
          const far = ((x - cx) / (rx + 0.25)) ** 2 + ((y - cy) / (ry + 0.25)) ** 2 + ((z - cz) / (rz + 0.25)) ** 2;
          if (far <= 1) this.set(x, y, z, ink);
        }
    return this;
  }

  bare(x: number, y: number, z: number): boolean {
    return [
      [1, 0, 0],
      [-1, 0, 0],
      [0, 1, 0],
      [0, -1, 0],
      [0, 0, 1],
      [0, 0, -1],
    ].some(([dx, dy, dz]) => this.get(x + dx!, y + dy!, z + dz!) === null);
  }

  each(fn: (x: number, y: number, z: number, ink: Ink) => Ink | undefined): this {
    for (let y = 0; y < this.side; y++)
      for (let z = 0; z < this.side; z++)
        for (let x = 0; x < this.side; x++) {
          const ink = fn(x, y, z, this.get(x, y, z));
          if (ink !== undefined) this.set(x, y, z, ink);
        }
    return this;
  }

  save(dir: string, size: string): void {
    save(dir, size, this.layers);
  }
}
