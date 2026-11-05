import type { Item } from "@mrlyprod/ui/kit/menus.ts";
import type { Pulldown } from "@mrlyprod/ui/shell/design.ts";
import { chord } from "./menu.tsx";
import { sprite } from "./sprite.tsx";

const button = (name: string) => {
  const one = document.createElement("button");
  one.type = "button";
  one.textContent = name.toUpperCase();
  one.setAttribute("aria-label", name);
  return one;
};

const sheet = () => {
  const one = document.createElement("div");
  one.className = "pixel menu pulldown";
  return one;
};

const item = (row: Item | null, mac: boolean) => {
  if (!row) {
    const rule = document.createElement("hr");
    rule.setAttribute("role", "separator");
    return rule;
  }
  const one = document.createElement(row.href ? "a" : "button");
  if (row.href) (one as HTMLAnchorElement).href = row.href;
  else (one as HTMLButtonElement).type = "button";
  one.className = row.off ? "item off" : "item";
  one.setAttribute("role", row.radio ? "menuitemradio" : "on" in row ? "menuitemcheckbox" : "menuitem");
  if ("on" in row) one.setAttribute("aria-checked", String(Boolean(row.on)));
  if (row.off) one.setAttribute("aria-disabled", "true");
  one.tabIndex = -1;
  if (row.act) one.dataset.act = row.act;
  const pointer = document.createElement("span");
  pointer.className = "pointer";
  pointer.append(sprite("pac"));
  const tick = document.createElement("span");
  tick.className = "tick";
  if (row.on) tick.append(sprite(row.radio ? "pellet" : "check"));
  const text = document.createElement("span");
  text.className = "label";
  text.textContent = String(row.label);
  one.append(pointer, tick, text);
  if (row.keys?.length) {
    const keys = document.createElement("span");
    keys.className = "chord";
    keys.textContent = chord(row.keys, mac);
    one.append(keys);
  }
  return one;
};

export const pulldown: Pulldown = { drop: 3, button, sheet, item };
