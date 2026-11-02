import type { Frame } from "./run.ts";

/* TYPES */

export type Fixer = (next?: { frame: Frame; dpr: number }) => void;

export type Saving = { name?: string; kind?: string; draw?: () => void; scale?: number; fix?: Fixer };

const MIME: Partial<Record<string, string>> = {
  png: "image/png",
  webp: "image/webp",
  jpeg: "image/jpeg",
  jpg: "image/jpeg",
  webm: "video/webm",
  mp4: "video/mp4",
  wav: "audio/wav",
  svg: "image/svg+xml",
  json: "application/json",
  csv: "text/csv",
  obj: "model/obj",
  txt: "text/plain",
};

const EXT: Partial<Record<string, string>> = {
  "image/png": "png",
  "image/webp": "webp",
  "image/jpeg": "jpg",
  "video/webm": "webm",
  "video/mp4": "mp4",
  "audio/wav": "wav",
};

const LATER = 1000;

const ext = (type: string) => EXT[type] ?? "png";

export function download(blob: Blob, file: string): string {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = file;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), LATER);
  return file;
}

export function text(string: string, name: string, type = "txt"): string {
  return download(new Blob([string], { type: MIME[type] ?? MIME.txt }), `${name}.${type}`);
}

const MOST = 2 ** 24;

export function grown(w: number, h: number, scale: number): Frame | null {
  const k = Math.min(scale, Math.sqrt(MOST / (w * h)));
  return k > 1 ? [Math.floor(w * k), Math.floor(h * k)] : null;
}

export async function save(
  canvas: HTMLCanvasElement,
  { name = "mrly", kind = "png", draw, scale = 1, fix }: Saving = {},
): Promise<string> {
  const frame = typeof fix === "function" ? grown(canvas.width, canvas.height, scale) : null;
  if (frame) fix!({ frame, dpr: frame[0] / (canvas.clientWidth || canvas.width) });
  let blob: Blob | null | undefined;
  try {
    draw?.();
    blob = await new Promise<Blob | null>((done) => canvas.toBlob(done, MIME[kind] ?? MIME.png));
  } finally {
    if (frame) fix!();
  }
  if (!blob) throw new Error("the canvas gave no picture");
  return download(blob, `${name}.${ext(blob.type)}`);
}

export function wav(buffer: AudioBuffer): Blob {
  const channels = buffer.numberOfChannels;
  const frames = buffer.length;
  const rate = buffer.sampleRate;
  const size = frames * channels * 2;
  const bytes = new ArrayBuffer(44 + size);
  const view = new DataView(bytes);
  const word = (at: number, text: string) => {
    for (const [i, c] of [...text].entries()) view.setUint8(at + i, c.charCodeAt(0));
  };
  word(0, "RIFF");
  view.setUint32(4, 36 + size, true);
  word(8, "WAVE");
  word(12, "fmt ");
  view.setUint32(16, 16, true);
  view.setUint16(20, 1, true);
  view.setUint16(22, channels, true);
  view.setUint32(24, rate, true);
  view.setUint32(28, rate * channels * 2, true);
  view.setUint16(32, channels * 2, true);
  view.setUint16(34, 16, true);
  word(36, "data");
  view.setUint32(40, size, true);
  const pcm = new Int16Array(bytes, 44);
  const data = Array.from({ length: channels }, (_, c) => buffer.getChannelData(c));
  for (let i = 0, at = 0; i < frames; i++) {
    for (let c = 0; c < channels; c++) pcm[at++] = Math.max(-32768, Math.min(32767, Math.round(data[c][i] * 32767)));
  }
  return new Blob([bytes], { type: "audio/wav" });
}
