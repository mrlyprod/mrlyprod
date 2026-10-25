import { expect, test } from "bun:test";
import { sample } from "./widget.tsx";

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, v) => {
      log.push([key, v]);
      return true;
    },
  });

test("a piece sample draws its design once, and the random piece draws four designs in the corners", () => {
  const fills = (design: string) => {
    const log: unknown[][] = [];
    sample(pen(log), design, 100, "#008cff");
    return log.filter(([verb]) => verb === "fill").length;
  };
  expect([fills("carpet"), fills("")]).toEqual([1, 4]);
});
