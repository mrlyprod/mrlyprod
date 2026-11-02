import { button, item, sheet } from "../../net/sdk/pulldown.ts";
import type { Pulldown } from "../../net/shell/design.ts";
import { chord, mark } from "./menu.tsx";

/* PULLDOWN */

const menu = () => {
  const one = sheet();
  one.className = "terminal menu frame";
  return one;
};

const row: Pulldown["item"] = (one, mac) => {
  const drawn = item(one, mac);
  if (!one) return drawn;
  drawn.classList.add("item");
  drawn.classList.toggle("off", Boolean(one.off));
  const tick = drawn.querySelector(".tick");
  tick?.classList.replace("tick", "gutter");
  tick?.replaceChildren(mark(one));
  const keys = drawn.querySelector(".chord");
  if (keys && one.keys?.length) keys.replaceChildren(chord(one.keys, mac));
  return drawn;
};

export const pulldown: Pulldown = { drop: 0, button, sheet: menu, item: row };
