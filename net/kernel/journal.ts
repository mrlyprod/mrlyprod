import type { Config, Kernel, Line, World } from "./kernel.ts";

/* REPLAY */

export function replay(k: Kernel, config: Config, lines: readonly Line[], upto = Number.POSITIVE_INFINITY): World {
  let world = k.boot(config);
  for (const line of lines) {
    if (line.tick > upto) break;
    if (line.tick !== world.tick + 1) throw new Error(`journal: line ${line.tick} follows tick ${world.tick}`);
    for (let i = 0; i < (line.n ?? 1); i++)
      world = k.call(world, { verb: line.verb, args: line.args, who: line.who, at: line.at }).world;
  }
  return world;
}

/* LINES */

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
