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

export async function save(canvas, { name = 'mrly', kind = 'png', draw } = {}) {
  draw?.();
  const blob = await new Promise((done) => canvas.toBlob(done, MIME[kind] ?? MIME.png));
  if (!blob) throw new Error('the canvas gave no picture');
  return download(blob, `${name}.${ext(blob.type)}`);
}
