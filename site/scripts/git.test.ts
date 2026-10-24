import { afterAll, expect, test } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { config, forest, tree } from "./git.ts";

/* TREE */

const home = join(tmpdir(), `mrlygit-${process.pid}`);

mkdirSync(join(home, "src"), { recursive: true });
writeFileSync(join(home, "README.md"), "# demo\n\nA tree.\n");
writeFileSync(join(home, "LICENSE"), "MIT\n");
writeFileSync(join(home, ".gitignore"), "dist\n");
writeFileSync(join(home, "src", "a.rs"), "fn main() {}\n");

afterAll(() => rmSync(home, { recursive: true, force: true }));

const repo = config(home, { root: ".", slug: "mrlyprod/mrlyprod" })!;

test("no git block means no repo", () => {
  expect(config(home)).toBeNull();
});

test("outside a git checkout the tree is every file but the skipped folders", () => {
  mkdirSync(join(home, "node_modules", "x"), { recursive: true });
  writeFileSync(join(home, "node_modules", "x", "i.js"), "");
  expect(tree(home)).toEqual([".gitignore", "LICENSE", "README.md", "src/a.rs"]);
});

test("the tree data names the repo and gives every file its icon and its size, folders first", () => {
  expect(forest(repo, [".gitignore", "LICENSE", "README.md", "src/a.rs"])).toEqual({
    base: "/git/",
    name: "mrlyprod",
    slug: "mrlyprod/mrlyprod",
    branch: "main",
    c: [
      { n: "src", k: "d", c: [{ n: "a.rs", k: "f", i: "rust", s: 13 }] },
      { n: ".gitignore", k: "f", i: "git", s: 5 },
      { n: "LICENSE", k: "f", i: "license", s: 4 },
      { n: "README.md", k: "f", i: "markdown", s: 16 },
    ],
  });
});
