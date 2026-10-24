import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  arrange,
  daily,
  docked,
  exported,
  gist,
  KEPT,
  rank,
  recents,
  renamed,
  siblings,
  spare,
  today,
  visited,
} from "./launch.js";

const BASE = [
  { name: "Designs", apps: ["arcs", "three"] },
  { name: "Games", apps: ["snake"] },
];

test("a saved layout keeps its order and names, drops ids the site lost or repeats, and returns new ids to their own section", () => {
  expect(arrange(BASE, "")).toEqual(BASE);
  expect(arrange(BASE, "{oops")).toEqual(BASE);
  const saved = JSON.stringify([
    { name: "Mine", apps: ["snake", "gone", "snake"] },
    { name: "Designs", apps: [] },
  ]);
  expect(arrange(BASE, saved)).toEqual([
    { name: "Mine", apps: ["snake"] },
    { name: "Designs", apps: ["arcs", "three"] },
  ]);
  expect(arrange(BASE, JSON.stringify([{ name: "Mine", apps: ["arcs"] }]))).toEqual([
    { name: "Mine", apps: ["arcs"] },
    { name: "Designs", apps: ["three"] },
    { name: "Games", apps: ["snake"] },
  ]);
});

test("the export writes launchpad.json byte for byte", () => {
  const text = readFileSync(join(import.meta.dir, "..", "..", "launchpad.json"), "utf8");
  const { sections, dock } = JSON.parse(text);
  expect(exported(sections, dock)).toBe(text);
});

test("a saved dock keeps its groups, each id once and known, and falls back to the default", () => {
  const known = new Set(["arcs", "three", "snake"]);
  expect(docked([["arcs"], ["snake"]], "", known)).toEqual([["arcs"], ["snake"]]);
  expect(docked([["arcs"], ["snake"]], JSON.stringify([["three", "gone", "three"]]), known)).toEqual([["three"], []]);
});

test("new and renamed sections never take a name twice", () => {
  expect(spare([{ name: "New Section", apps: [] }])).toBe("New Section 2");
  expect(renamed(BASE, "Games", " Play ")[1].name).toBe("Play");
  expect(renamed(BASE, "Games", "designs")).toBe(BASE);
});

test("search ranks a title that starts with the query, then a word that does, then one that holds it, and reads the lead only for longer words", () => {
  const items = [
    { title: "Arcs" },
    { title: "Snake" },
    { title: "Sequences" },
    { title: "Font", more: "the pens of the font" },
    { title: "Big Spin" },
  ];
  expect(rank("s", items).map((one) => one.title)).toEqual(["Sequences", "Snake", "Big Spin", "Arcs"]);
  expect(rank("pens", items).map((one) => one.title)).toEqual(["Font"]);
  expect(rank("sn ke", items).map((one) => one.title)).toEqual(["Snake"]);
  expect(rank("  ", items)).toEqual([]);
});

test("recent apps come newest first, each once, at most the kept count", () => {
  const list = Array.from({ length: KEPT }, (_, n) => ({ id: `a${n}`, url: `/a${n}/`, at: n }));
  const next = visited(list, { id: "a3", url: "/a3/?x=1", at: 99 });
  expect([next.length, next[0].url, next.filter((one) => one.id === "a3").length]).toEqual([KEPT, "/a3/?x=1", 1]);
  expect(gist("/designs/?seed=4&dim=2&code=7")).toBe("dim 2 · code 7");
});

test("a stored recent list that is not an array reads as empty", () => {
  const known = new Set(["arcs"]);
  expect([
    recents("{}", known),
    recents("7", known),
    recents(JSON.stringify([{ id: "arcs", url: "/arcs/" }]), known),
  ]).toEqual([[], [], [{ id: "arcs", url: "/arcs/" }]]);
});

test("the design of the day is one index per date, the same for everyone", () => {
  const zone = process.env.TZ;
  process.env.TZ = "Pacific/Kiritimati";
  try {
    expect([
      today(new Date("2026-10-19T23:30:00-07:00")),
      today(new Date("2026-10-20T08:30:00+14:00")),
      today(new Date("2026-10-19T12:00:00Z")),
    ]).toEqual(["2026-10-20", "2026-10-19", "2026-10-19"]);
  } finally {
    if (zone === undefined) delete process.env.TZ;
    else process.env.TZ = zone;
  }
  expect(daily("2026-10-19", 50)).toBe(daily("2026-10-19", 50));
  expect(
    new Set(Array.from({ length: 30 }, (_, d) => daily(`2026-11-${String(d + 1).padStart(2, "0")}`, 50))).size,
  ).toBeGreaterThan(15);
});

test("an app's siblings are the other apps of its section, in order, and none for an app on no shelf", () => {
  const shelves = [
    { name: "Fractals", apps: ["fractals", "zoom"] },
    { name: "Space", apps: ["planets", "lightspeed", "voyage"] },
  ];
  expect([siblings(shelves, "lightspeed"), siblings(shelves, "zoom"), siblings(shelves, "kit")]).toEqual([
    ["planets", "voyage"],
    ["fractals"],
    [],
  ]);
});
