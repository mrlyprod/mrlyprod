import { button, item, sheet } from "../../net/sdk/pulldown.ts";
import type { Pulldown } from "../../net/shell/design.ts";

/* PULLDOWN */

export const pulldown: Pulldown = { drop: 6, button, sheet: () => sheet("menu"), item };
