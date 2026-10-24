import { expect, test } from "bun:test";
import { pen } from "../../../test/pen.js";
import data from "./fixture.json";
import { ago, draw, facts, scale, series, size, views } from "./stats.js";

const LOOK = { paper: "#fff", accent: "#06f", ink: "#111", dim: "#777", line: "#ddd", font: "sans-serif" };

test("ago counts seconds, minutes, hours and days, and reads an iso time", () => {
  expect([
    ago(940, 1000),
    ago(1000 - 180, 1000),
    ago(1000 - 7200, 1000),
    ago(1000 - 3 * 86400, 1000),
    ago(0, 1000),
    ago("1970-01-01T00:16:00Z", 1000),
  ]).toEqual(["60 s ago", "3 min ago", "2.0 h ago", "3 d ago", "never", "40 s ago"]);
});

test("size writes kB, MB and GB", () => {
  expect([size(45372), size(45372436), size(4537243600)]).toEqual(["45 kB", "45.4 MB", "4.54 GB"]);
});

test("views lists the cdn, each lambda, the bucket and the errors with a headline", () => {
  expect(views(data).map((one) => [one.id, one.head])).toEqual([
    ["cdn", "4,989 requests"],
    ["mrlynet-site", "598 invocations"],
    ["mrlynet-stats", "24 invocations"],
    ["bucket", "3,749 objects"],
    ["errors", "2 errors"],
  ]);
});

test("series gives a point an hour for the cdn and a bar a kind for the bucket, largest first", () => {
  const cdn = series(data, "cdn");
  const bucket = series(data, "bucket");
  expect([
    cdn.kind,
    cdn.points.length,
    cdn.points[6].value,
    bucket.kind,
    bucket.bars.length,
    bucket.bars[0].label,
    bucket.bars.at(-1).value,
  ]).toEqual(["line", 24, 279, "bars", 10, "Site", 1]);
});

test("series counts the errors as a note and says no data without data", () => {
  expect([series(data, "errors").text, series(null, "cdn").text]).toEqual(["2 errors in the window", "No data yet."]);
});

test("facts give the view readouts for the status", () => {
  expect([facts(data, "cdn"), facts(data, "mrlynet-site"), facts(data, "bucket")]).toEqual([
    ["4,989 requests", "45.4 MB", "17.0% 4xx", "0.0% 5xx"],
    ["598 invocations", "2 errors", "0 throttles", "42 ms avg"],
    ["site 124.3 MB"],
  ]);
});

test("scale keeps 3 or 4 grid lines on round steps", () => {
  expect([scale(279), scale(34), scale(1)].map(({ step, n }) => [step, n + 1])).toEqual([
    [100, 4],
    [20, 3],
    [1, 3],
  ]);
});

test("draw strokes one vertex a point for the cdn series", () => {
  const log = [];
  draw(pen(log), 800, 600, 1, series(data, "cdn"), LOOK);
  const strokes = [];
  let path = 0;
  let style = "";
  for (const [key, value] of log) {
    if (key === "beginPath") path = 0;
    if (key === "moveTo" || key === "lineTo") path++;
    if (key === "strokeStyle") style = value;
    if (key === "stroke") strokes.push([style, path]);
  }
  expect(strokes).toEqual([[LOOK.accent, 24]]);
});
