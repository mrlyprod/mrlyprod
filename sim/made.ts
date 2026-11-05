import { join } from "node:path";

export type Tree = { commit: string | null; dirty: boolean | null };

export type Made = { at: string; os: Tree; apps: Tree; sim: Tree; resumed?: Made[] };

function git(dir: string, ...args: string[]): string | null {
  const done = Bun.spawnSync(["git", "-C", dir, ...args], { stdout: "pipe", stderr: "pipe" });
  return done.exitCode === 0 ? done.stdout.toString().trim() : null;
}

const treeOf = (dir: string): Tree => {
  const status = git(dir, "status", "--porcelain");
  return { commit: git(dir, "rev-parse", "HEAD"), dirty: status === null ? null : status.length > 0 };
};

export const madeNow = (sim: string): Made => ({
  at: new Date().toISOString(),
  os: treeOf(join(sim, "..", "..", "mrlyprod", "os")),
  apps: treeOf(join(sim, "..", "..", "mrlyprod", "apps")),
  sim: treeOf(sim),
});
