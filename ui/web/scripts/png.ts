import { crc32, deflateSync } from "node:zlib";

export type Pixels = { shape: [number, number]; colors: Uint8Array | Uint8ClampedArray };

const SIGNATURE = Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10]);

function chunk(type: string, data: Uint8Array) {
  const out = new Uint8Array(data.length + 12);
  const view = new DataView(out.buffer);
  view.setUint32(0, data.length);
  out.set(new TextEncoder().encode(type), 4);
  out.set(data, 8);
  view.setUint32(data.length + 8, crc32(out.subarray(4, data.length + 8)));
  return out;
}

export function png({ shape: [height, width], colors }: Pixels) {
  const line = width * 4 + 1;
  const rows = new Uint8Array(line * height);
  for (let y = 0; y < height; y++) rows.set(colors.subarray(y * width * 4, (y + 1) * width * 4), y * line + 1);
  const head = new Uint8Array(13);
  const view = new DataView(head.buffer);
  view.setUint32(0, width);
  view.setUint32(4, height);
  head.set([8, 6], 8);
  return Buffer.concat([
    SIGNATURE,
    chunk("IHDR", head),
    chunk("IDAT", deflateSync(rows)),
    chunk("IEND", new Uint8Array()),
  ]);
}

export function straight({ shape, colors }: Pixels): Pixels {
  const out = Uint8ClampedArray.from(colors);
  for (let i = 0; i < out.length; i += 4) {
    const a = out[i + 3]!;
    if (a > 0 && a < 255) for (let k = i; k < i + 3; k++) out[k] = (out[k]! * 255) / a;
  }
  return { shape, colors: out };
}
