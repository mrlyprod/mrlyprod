import type { CSSProperties } from "react";
import type { Bind, Clock, World } from "../../shell/design.ts";

/* MRLY */

export const MRLY = { "data-design": "mrly" } as const;

export const none: Bind = () => ({});

export const href = (bind: Bind) => String(bind().href ?? "");

/* HUE */

export const HUES = [
  "red",
  "orange",
  "yellow",
  "green",
  "mint",
  "teal",
  "cyan",
  "blue",
  "indigo",
  "purple",
  "pink",
  "brown",
  "gray",
] as const;

export type Hue = (typeof HUES)[number];

export const hash = (id: string): Hue => {
  let sum = 0;
  for (const c of id) sum = (sum * 31 + c.charCodeAt(0)) >>> 0;
  return HUES[sum % HUES.length]!;
};

export const tint = (hue: Hue) => ({ "--hue": `var(--${hue})` }) as CSSProperties;

export function hued({ root }: World) {
  const box = root.querySelector<HTMLElement>('.window[data-design="mrly"]');
  if (!box) return () => {};
  const paint = () => {
    const hue = box.dataset.app ? hash(box.dataset.app) : "gray";
    box.dataset.hue = hue;
    box.style.setProperty("--hue", `var(--${hue})`);
  };
  paint();
  const eye = new MutationObserver(paint);
  eye.observe(box, { attributeFilter: ["data-app"] });
  return () => eye.disconnect();
}

/* CLOCK */

export const time = ({ now, hours }: Clock) =>
  hours === null
    ? ""
    : now.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", hourCycle: hours === 12 ? "h12" : "h23" });

export const day = ({ now, hours }: Clock) =>
  hours === null ? "" : now.toLocaleDateString("en-GB", { weekday: "short", day: "numeric", month: "short" });
