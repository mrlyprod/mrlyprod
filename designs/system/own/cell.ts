import type { Bind, Clock } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";

export const none: Bind = () => ({});

const HUES = [
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

type Hue = (typeof HUES)[number];

export const hash = (id: string): Hue => {
  let sum = 0;
  for (const c of id) sum = (sum * 31 + c.charCodeAt(0)) >>> 0;
  return HUES[sum % HUES.length]!;
};

export const tint = (hue: Hue) => ({ "--hue": `var(--${hue})` }) as CSSProperties;

export const time = ({ now, hours }: Clock) =>
  now.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", hourCycle: hours === 12 ? "h12" : "h23" });

export const day = ({ now }: Clock) =>
  now.toLocaleDateString("en-GB", { weekday: "short", day: "numeric", month: "short" });
