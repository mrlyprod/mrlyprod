import { design } from "site:design";
import type { Measure } from "./design.ts";

/* MEASURE */

export const measure: Required<Measure> = { edge: 8, dock: Number.POSITIVE_INFINITY, thumb: 56, ...design.measure };
