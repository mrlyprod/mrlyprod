import type { Measure } from "../../shell/design.ts";

/* MEASURE */

export const STUD = 24;

const studs = (n: number) => n * STUD;

export const MEASURE: Measure = {
  page: studs(25),
  wide: studs(41),
  least: studs(15),
  step: STUD,
  panes: { width: studs(11), least: studs(9), most: studs(18), gap: 8, stage: studs(15) },
  thumb: studs(2),
};
