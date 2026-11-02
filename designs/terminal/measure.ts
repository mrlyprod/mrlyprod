import type { Measure } from "../../net/shell/design.ts";

/* MEASURE */

export const CELL = 8;

const cells = (n: number) => n * CELL;

export const MEASURE: Measure = {
  page: cells(92),
  wide: cells(156),
  least: cells(44),
  step: CELL,
  panes: { width: cells(30), least: cells(20), most: cells(50), gap: 0, stage: cells(40) },
  gutter: 16,
  edge: 8,
  thumb: 48,
  dock: 0,
};
