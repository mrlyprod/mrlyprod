// Shared by every example: where the files go, and how pixels become a PNG.
import { mkdirSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { deflateSync } from "node:zlib";
import type { Raster, Svg } from "@mrlyprod/draw";

const REPO = resolve(import.meta.dir, "..", "..", "..");
export const DATA_DIR = join(REPO, "data", relative(REPO, import.meta.dir));

const chunk = (kind: string, body: Uint8Array) => {
  const named = Buffer.concat([Buffer.from(kind, "latin1"), body]);
  const out = Buffer.alloc(named.length + 8);
  out.writeUInt32BE(body.length, 0);
  named.copy(out, 4);
  out.writeUInt32BE(Bun.hash.crc32(named), named.length + 4);
  return out;
};

// A PNG is a signature, a header, deflated rows and an end marker.
export function png({ shape, colors }: { shape: [number, number]; colors: ArrayLike<number> }): Uint8Array {
  const [height, width] = shape;
  const rows = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0, at = 0; y < height; y++) {
    rows[at++] = 0;
    for (let i = y * width * 4, end = i + width * 4; i < end; i++) rows[at++] = colors[i];
  }
  const head = Buffer.alloc(13);
  head.writeUInt32BE(width, 0);
  head.writeUInt32BE(height, 4);
  head.set([8, 6, 0, 0, 0], 8);
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk("IHDR", head),
    chunk("IDAT", deflateSync(rows)),
    chunk("IEND", new Uint8Array()),
  ]);
}

const write = (name: string, bytes: Uint8Array | string) => {
  mkdirSync(DATA_DIR, { recursive: true });
  writeFileSync(join(DATA_DIR, name), bytes);
  console.log(join(relative(REPO, DATA_DIR), name));
};

const stem = (file: string) => file.replace(/\.ts$/, "");

export const save = (file: string, pen: Raster) => write(`${stem(file)}.png`, png(pen.pixels()));

export const saveSvg = async (file: string, pen: Svg) => write(`${stem(file)}.svg`, await pen.text({ png }));
