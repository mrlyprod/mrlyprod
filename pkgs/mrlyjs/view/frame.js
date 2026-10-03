// FRAME

class Frame {
  constructor(x, y, w, h) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }

  inset(px) {
    return new Frame(this.x + px, this.y + px, this.w - 2 * px, this.h - 2 * px);
  }

  cell(n) {
    return this.w / n;
  }

  at(u, v) {
    return [this.x + u * this.w, this.y + v * this.h];
  }

  center() {
    return this.at(0.5, 0.5);
  }

  square() {
    const side = Math.min(this.w, this.h);
    return new Frame(this.x + (this.w - side) / 2, this.y + (this.h - side) / 2, side, side);
  }

  radius() {
    return Math.min(this.w, this.h) / 2;
  }

  cols(n) {
    const w = this.w / n;
    return Array.from({ length: n }, (_, i) => new Frame(this.x + i * w, this.y, w, this.h));
  }

  rows(n) {
    const h = this.h / n;
    return Array.from({ length: n }, (_, i) => new Frame(this.x, this.y + i * h, this.w, h));
  }
}

export function frame(x, y, w, h) {
  return new Frame(x, y, w, h);
}

// BOARD

export const CLEAR = Object.freeze([0, 0, 0, 0]);

export function board(width, height) {
  return {
    width,
    height,
    frame(margin) {
      const side = Math.min(width, height) * (1 - 2 * margin);
      return new Frame((width - side) / 2, (height - side) / 2, side, side);
    },
    area(margin) {
      return new Frame(0, 0, width, height).inset(Math.min(width, height) * margin);
    },
  };
}
