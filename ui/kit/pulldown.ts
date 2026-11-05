import type { IconName } from "./controls.tsx";
import { ICONS, VIEWBOX } from "./icons.gen.js";
import type { Item } from "./menus.ts";
import { label } from "./menus.ts";

const SVG = "http://www.w3.org/2000/svg";

export function glyph(name: IconName) {
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("viewBox", VIEWBOX);
  svg.setAttribute("aria-hidden", "true");
  const path = document.createElementNS(SVG, "path");
  path.setAttribute("d", ICONS[name]);
  svg.append(path);
  return svg;
}

export function chord(keys: string[], mac: boolean) {
  const out = document.createElement("span");
  out.className = "chord";
  const parts = keys.map((one) =>
    one === "mod"
      ? mac
        ? glyph("keyboard_command_key")
        : "Ctrl"
      : one === "shift"
        ? mac
          ? glyph("shift")
          : "Shift"
        : label(one),
  );
  parts.forEach((part, n) => {
    if (n && !mac) out.append("+");
    out.append(part);
  });
  out.setAttribute(
    "aria-label",
    keys
      .map((one) => (one === "mod" ? (mac ? "Command" : "Control") : one === "shift" ? "Shift" : label(one)))
      .join(" "),
  );
  return out;
}

export function item(row: Item | null, mac: boolean) {
  if (!row) {
    const rule = document.createElement("hr");
    rule.setAttribute("role", "separator");
    return rule;
  }
  const one = document.createElement(row.href ? "a" : "button");
  if (row.href) (one as HTMLAnchorElement).href = row.href;
  else (one as HTMLButtonElement).type = "button";
  one.setAttribute("role", row.radio ? "menuitemradio" : "on" in row ? "menuitemcheckbox" : "menuitem");
  if ("on" in row) one.setAttribute("aria-checked", String(Boolean(row.on)));
  if (row.off) one.setAttribute("aria-disabled", "true");
  one.tabIndex = -1;
  if (row.act) one.dataset.act = row.act;
  const tick = document.createElement("span");
  tick.className = "tick";
  if (row.on) tick.append(glyph("check"));
  const text = document.createElement("span");
  text.className = "label";
  text.textContent = String(row.label);
  one.append(tick, text);
  if (row.keys?.length) one.append(chord(row.keys, mac));
  return one;
}

export function button(name: string) {
  const one = document.createElement("button");
  one.type = "button";
  one.textContent = name;
  return one;
}

export function sheet(lens?: string) {
  const one = document.createElement("div");
  one.className = "pulldown";
  if (lens) one.dataset.lens = lens;
  return one;
}
