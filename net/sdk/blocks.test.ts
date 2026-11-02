import { expect, test } from "bun:test";
import { check } from "../kernel/index.ts";
import type { Block } from "./blocks.ts";
import { at, BLOCKS, count, DEPTH, edited, PATH, TREE, text } from "./blocks.ts";

const TREE_OF_THREE: Block[] = [
  { kind: "h1", text: "Soup" },
  { kind: "li", text: "lentils", kids: [{ kind: "li", text: "red ones" }] },
];

test("a tree counts every block, joins every text, finds a block by its path and edits one text in place", () => {
  const changed = edited(TREE_OF_THREE, [1, 0], "green ones");
  expect([count(TREE_OF_THREE), text(TREE_OF_THREE), at(TREE_OF_THREE, [1, 0])?.text, at(TREE_OF_THREE, [2])]).toEqual([
    3,
    "Soup\nlentils\nred ones",
    "red ones",
    undefined,
  ]);
  expect([changed[1]!.kids![0]!.text, TREE_OF_THREE[1]!.kids![0]!.text]).toEqual(["green ones", "red ones"]);
});

test(`the tree schema takes blocks ${DEPTH} deep of the known kinds and a path of 1 to ${DEPTH} indexes`, () => {
  const deep = { kind: "p", text: "", kids: [{ kind: "p", text: "", kids: [{ kind: "p", text: "", kids: [] }] }] };
  expect([
    check(TREE, TREE_OF_THREE),
    check(TREE, [{ kind: "table", text: "" }]),
    check(TREE, [deep]),
    check(PATH, [0, 1]),
    check(PATH, []),
    check(PATH, [BLOCKS]),
  ]).toEqual([
    null,
    `/0/kind must be "h1", "h2", "h3", "p", "li", "quote" or "code"`,
    "/0/kids/0/kids/0 takes no kids; known: kind, text",
    null,
    "the args must hold at least 1 items",
    `/0 must be between 0 and ${BLOCKS - 1}`,
  ]);
});
