import type { Measure } from "@mrlyprod/ui/shell/design.ts";

export const DOT = 3;

export const CELL = 8 * DOT;

const cells = (n: number) => n * CELL;

export const MEASURE: Measure = {
  page: cells(40),
  wide: cells(54),
  least: cells(16),
  step: CELL,
  panes: { width: cells(12), least: cells(9), most: cells(18), gap: DOT * 2, stage: cells(16) },
  gutter: CELL,
  edge: DOT * 3,
  thumb: cells(2),
  dock: cells(3),
};
