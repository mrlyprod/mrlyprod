import { design } from "site:design";
import type { Measure } from "./design.ts";

/* MEASURE */

export const measure: Measure & { dock: number } = { dock: Number.POSITIVE_INFINITY, ...design.measure };
