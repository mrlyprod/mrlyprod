const MIME = { png: 'image/png', webp: 'image/webp', jpeg: 'image/jpeg', jpg: 'image/jpeg', webm: 'video/webm', mp4: 'video/mp4', wav: 'audio/wav', svg: 'image/svg+xml', json: 'application/json', csv: 'text/csv', obj: 'model/obj', txt: 'text/plain' };

const EXT = { 'image/png': 'png', 'image/webp': 'webp', 'image/jpeg': 'jpg', 'video/webm': 'webm', 'video/mp4': 'mp4', 'audio/wav': 'wav' };

const LATER = 1000;

const ext = (type) => EXT[type] ?? 'png';

export function download(blob, file) {
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = file;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), LATER);
  return file;
}

export function text(string, name, type = 'txt') {
  return download(new Blob([string], { type: MIME[type] ?? MIME.txt }), `${name}.${type}`);
}

const MOST = 2 ** 24;

export function grown(w, h, scale) {
  const k = Math.min(scale, Math.sqrt(MOST / (w * h)));
  return k > 1 ? [Math.floor(w * k), Math.floor(h * k)] : null;
}

export async function save(canvas, { name = 'mrly', kind = 'png', draw, scale = 1, fix } = {}) {
  const frame = typeof fix === 'function' ? grown(canvas.width, canvas.height, scale) : null;
  if (frame) fix({ frame, dpr: frame[0] / (canvas.clientWidth || canvas.width) });
  let blob;
  try {
    draw?.();
    blob = await new Promise((done) => canvas.toBlob(done, MIME[kind] ?? MIME.png));
  } finally {
    if (frame) fix();
  }
  if (!blob) throw new Error('the canvas gave no picture');
  return download(blob, `${name}.${ext(blob.type)}`);
}
