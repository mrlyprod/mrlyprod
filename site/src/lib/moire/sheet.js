const FIT = 0.94;

export function fit(w, h, fill = FIT) {
  const side = Math.max(1, Math.floor(Math.min(w, h) * fill));
  return { x: Math.floor((w - side) / 2), y: Math.floor((h - side) / 2), side };
}

export function shell(canvas, view) {
  const ctx = canvas.getContext('2d');
  const sheets = new Map();
  const sheet = (size) => {
    if (!sheets.has(size)) sheets.set(size, new OffscreenCanvas(size, size));
    return sheets.get(size);
  };
  const put = ({ colors }, size) => {
    sheet(size).getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(colors.buffer, colors.byteOffset, colors.length), size, size), 0, 0);
  };
  const clear = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
  };
  const blit = (size, box, smooth) => {
    ctx.imageSmoothingEnabled = smooth;
    ctx.drawImage(sheet(size), box.x, box.y, box.side, box.side);
  };
  return { ctx, put, clear, blit };
}

export function stamp(ctx, { colors }, size) {
  const image = new ImageData(new Uint8ClampedArray(colors.buffer, colors.byteOffset, colors.length), size, size);
  const side = ctx.canvas.width;
  if (side === size) {
    ctx.putImageData(image, 0, 0);
    return;
  }
  const tile = new OffscreenCanvas(size, size);
  tile.getContext('2d').putImageData(image, 0, 0);
  ctx.imageSmoothingEnabled = true;
  ctx.drawImage(tile, 0, 0, side, side);
}
