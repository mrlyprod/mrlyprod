import { expect, test } from "bun:test";
import { lacks, OPTIONAL, REQUIRED } from "./design.ts";

test("a design fails for each required part it lacks, and never for an optional one", () => {
  const none = () => null;
  const all = Object.fromEntries(REQUIRED.map((part) => [part, none]));
  const { dialog: _, tile: __, ...short } = all;
  expect([lacks({ parts: all }), lacks({ parts: short }), lacks({ parts: { ...short, dialog: "x" } })]).toEqual([
    [],
    ["dialog", "tile"],
    ["dialog", "tile"],
  ]);
  expect(OPTIONAL.filter((part) => (REQUIRED as readonly string[]).includes(part))).toEqual([]);
});
