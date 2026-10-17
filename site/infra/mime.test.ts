import { expect, test } from "bun:test";
import { kind, rawKind } from "./mime.ts";

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

test("raw bytes are plain text when text or svg, keep a passive type, else octet-stream", () => {
  expect([rawKind("a.rs", true), rawKind("logo.svg", false), rawKind("logo.png", false), rawKind("page.html", false), rawKind("x.constructor", false)]).toEqual([
    "text/plain; charset=utf-8",
    "text/plain; charset=utf-8",
    "image/png",
    "application/octet-stream",
    "application/octet-stream",
  ]);
});
