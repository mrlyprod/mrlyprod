const MIME = { png: 'image/png', webp: 'image/webp', jpeg: 'image/jpeg', jpg: 'image/jpeg' };

const EXT = { 'image/png': 'png', 'image/webp': 'webp', 'image/jpeg': 'jpg' };

const LATER = 1000;

const ext = (type) => EXT[type] ?? 'png';

export async function save(canvas, { name = 'mrly', kind = 'png', draw } = {}) {
  draw?.();
  const blob = await new Promise((done) => canvas.toBlob(done, MIME[kind] ?? MIME.png));
  if (!blob) throw new Error('the canvas gave no picture');
  const file = `${name}.${ext(blob.type)}`;
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = file;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), LATER);
  return file;
}
