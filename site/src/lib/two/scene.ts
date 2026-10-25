import { paints } from "../color.ts";

export const SIZES = [512, 1024, 2048, 4096];

const SECOND = ["orange", "cyan"];

export function second(canvas: Element | null | undefined, tint: string, accent: string): string {
  const [one, other] = SECOND;
  const ink = paints(canvas, tint === one ? other : one).accent;
  return ink === accent ? paints(canvas, other).accent : ink;
}
