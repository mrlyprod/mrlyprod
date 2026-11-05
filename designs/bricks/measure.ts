import type { Measure } from "@mrlyprod/ui/shell/design.ts";

export const STUD = 24;

const studs = (n: number) => n * STUD;

export const MEASURE: Measure = {
  page: studs(38),
  wide: studs(41),
  least: studs(15),
  step: STUD,
  panes: { width: studs(11), least: studs(9), most: studs(18), gap: 8, stage: studs(15) },
  gutter: 12,
  edge: 8,
  thumb: studs(2),
};
