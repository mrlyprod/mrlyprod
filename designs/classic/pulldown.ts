import { button, item, sheet } from "../../net/sdk/pulldown.ts";
import type { Pulldown } from "../../net/shell/design.ts";
import { pixel } from "./glyphs.tsx";

/* PULLDOWN */

const menu = () => {
  const one = sheet();
  one.className = "classic menu raised";
  return one;
};

const row: Pulldown["item"] = (one, mac) => {
  const drawn = item(one, mac);
  if (!one) return drawn;
  drawn.classList.add("item");
  drawn.classList.toggle("off", Boolean(one.off));
  const tick = drawn.querySelector(".tick");
  tick?.classList.replace("tick", "gutter");
  tick?.replaceChildren(...(one.on ? [pixel(one.radio ? "dot" : "tick")] : []));
  const marks = (one.keys ?? []).filter((key) => key === "mod" || key === "shift");
  drawn.querySelectorAll(".chord svg").forEach((svg, n) => {
    svg.replaceWith(pixel(marks[n] === "shift" ? "shift" : "cmd"));
  });
  return drawn;
};

export const pulldown: Pulldown = { drop: 0, button, sheet: menu, item: row };
