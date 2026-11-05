import type { Config, Kernel, Line, World } from "./kernel.ts";

export function replay(k: Kernel, config: Config, lines: readonly Line[], upto = Number.POSITIVE_INFINITY): World {
  let world = k.boot(config);
  for (const line of lines) {
    if (line.tick > upto) break;
    if (line.tick !== world.tick + 1) throw new Error(`journal: line ${line.tick} follows tick ${world.tick}`);
    world = k.call(world, line).world;
  }
  return world;
}

export const jsonl = (lines: readonly Line[]) => lines.map((line) => `${JSON.stringify(line)}\n`).join("");

export function parse(text: string): Line[] {
  return text
    .split("\n")
    .filter((one) => one.trim())
    .map((one, i) => {
      try {
        return JSON.parse(one) as Line;
      } catch {
        throw new Error(`journal: line ${i + 1} is not JSON`);
      }
    });
}
