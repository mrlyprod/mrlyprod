import type { CSSProperties } from "react";
import type { Bind, Clock } from "../../shell/design.ts";

/* BLOCKS */

export const BLOCKS = { "data-design": "blocks" } as const;

export const none: Bind = () => ({});

export const href = (bind: Bind) => String(bind().href ?? "");

/* COLOUR */

export const TOYS = ["red", "yellow", "blue", "green"] as const;

export type Colour = (typeof TOYS)[number] | "white" | "black";

export const hash = (id: string): Colour => {
  let sum = 0;
  for (const c of id) sum = (sum * 31 + c.charCodeAt(0)) >>> 0;
  return TOYS[sum % TOYS.length]!;
};

const LIGHT: Colour[] = ["yellow", "white"];

export const paint = (colour: Colour) =>
  ({
    "--brick": `var(--${colour})`,
    "--lip": `var(--${colour}-lip)`,
    "--print": LIGHT.includes(colour) ? "var(--bk-ink)" : "var(--bk-paper)",
  }) as CSSProperties;

/* CLOCK */

export const time = ({ now, hours }: Clock) =>
  hours === null
    ? ""
    : now.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", hourCycle: hours === 12 ? "h12" : "h23" });

export const day = ({ now, hours }: Clock) =>
  hours === null ? "" : now.toLocaleDateString("en-GB", { weekday: "short", day: "numeric", month: "short" });
