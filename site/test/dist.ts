import { rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { build, type Built } from "../scripts/build.ts";

export const out = join(tmpdir(), `site-${process.pid}`);

let made: Promise<Built> | null = null;

export const dist = () => {
  if (!made) {
    process.on("exit", () => rmSync(out, { recursive: true, force: true }));
    made = build(out);
  }
  return made;
};
