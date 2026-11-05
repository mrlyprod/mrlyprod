import { button, item, sheet } from "@mrlyprod/ui/kit/pulldown.ts";
import type { Pulldown } from "@mrlyprod/ui/shell/design.ts";

export const pulldown: Pulldown = { drop: 6, button, sheet: () => sheet("menu"), item };
