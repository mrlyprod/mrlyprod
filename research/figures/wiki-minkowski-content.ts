import { frame, plot, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-minkowski-content.json" with { type: "json" };

const ROWS = 7;
const WIDEST = 0.1;
const THREAD = 5;
const FROM = 2;
const TO = 6;
const SAMPLES = 4000;

export const units = {};

type Span = [number, number];

function check(name: string, spans: Span[], least: number) {
  if (spans.length < least) throw new Error(`wiki-minkowski-content: ${name} holds ${spans.length} spans, want ${least} or more`);
  if (spans.some(([a, b]) => !(b > a))) throw new Error(`wiki-minkowski-content: ${name} holds a span that does not rise`);
}

export default function draw(pen: Pen, ink: Ink) {
  const { tubes, thread, ys, low, high } = census as { tubes: Span[][]; thread: Span[]; ys: number[]; low: number; high: number };
  if (tubes.length !== ROWS) throw new Error(`wiki-minkowski-content: ${tubes.length} tubes, want ${ROWS}`);
  tubes.forEach((tube, row) => check(`tube ${row}`, tube, 1));
  if (thread.length !== 1 << THREAD) throw new Error(`wiki-minkowski-content: ${thread.length} thread spans, want ${1 << THREAD}`);
  check("thread", thread, 1);
  if (ys.length !== SAMPLES + 1) throw new Error(`wiki-minkowski-content: ${ys.length} readings, want ${SAMPLES + 1}`);
  if (!(high > low)) throw new Error("wiki-minkowski-content: the reading ceiling is not above its floor");

  const area = pen.frame(0.08);
  const upper = frame(area.x, area.y, area.w, area.h * 0.58);
  const lower = frame(area.x, area.y + area.h * 0.66, area.w, area.h * 0.34);

  const span = 1 + 2 * WIDEST;
  const px = (u: number) => upper.x + (upper.w * (u + WIDEST)) / span;
  const pitch = upper.h / ROWS;
  const thick = pitch * 0.62;
  const fine = pitch * 0.16;
  tubes.forEach((tube, row) => {
    const top = upper.y + row * pitch + (pitch - thick) / 2;
    for (const [a, b] of tube) pen.rect(px(a), top, px(b) - px(a), thick, ink.blue);
    const mid = top + (thick - fine) / 2;
    for (const [a, b] of thread) pen.rect(px(a), mid, px(b) - px(a), fine, ink.yellow);
  });

  const pad = (high - low) * 0.12;
  const bottom = low - pad;
  const ceiling = high + pad;
  const hair = Math.max(lower.h / 256, 1);
  for (let k = FROM + 1; k < TO; k++) {
    const x = lower.x + (lower.w * (k - FROM)) / (TO - FROM);
    pen.segment([x, lower.y], [x, lower.y + lower.h], hair, ink.dim);
  }
  const pts: Point[] = ys.map((y, i) => {
    const x = FROM + ((TO - FROM) * i) / SAMPLES;
    return [lower.x + (lower.w * (x - FROM)) / (TO - FROM), lower.y + lower.h * (1 - (y - bottom) / (ceiling - bottom))];
  });
  pen.polyline(pts, Math.max(lower.h / 64, 2), ink.yellow);
  plot.axis(pen, lower, ink.line);
}
