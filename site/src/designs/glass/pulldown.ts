import { button, item, sheet } from "../../lib/app/pulldown.ts";
import type { Pulldown } from "../../shell/design.ts";

/* PULLDOWN */

export const pulldown: Pulldown = { drop: 6, button, sheet: () => sheet("menu"), item };
