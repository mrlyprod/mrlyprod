import { expect, test } from "bun:test";
import { kind } from "./types.ts";

test("an extension maps to its type in any case, an unknown or inherited one to octet-stream", () => {
  expect(["a/clip.MP4", "Logo.SVG", "x.woff2", "dir.d/README", "x.constructor", "x.toString"].map(kind)).toEqual([
    "video/mp4",
    "image/svg+xml",
    "font/woff2",
    "application/octet-stream",
    "application/octet-stream",
    "application/octet-stream",
  ]);
});
