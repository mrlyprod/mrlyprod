export const PANE = () => false;

export const SOUP = (value) => value.from === 'noise';

export function runs(ctx, types, cols, rows, x, y, cell, high = cell) {
  const snap = cell >= 1 ? Math.round : (n) => n;
  ctx.beginPath();
  for (let r = 0; r < rows; r++) {
    const top = snap(y + r * high);
    const tall = snap(y + (r + 1) * high) - top;
    for (let c = 0; c < cols; c++) {
      if (!types[r * cols + c]) continue;
      const from = c;
      while (c + 1 < cols && types[r * cols + c + 1]) c++;
      const left = snap(x + from * cell);
      ctx.rect(left, top, snap(x + (c + 1) * cell) - left, tall);
    }
  }
  ctx.fill();
}
