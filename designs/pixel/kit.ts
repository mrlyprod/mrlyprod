import type { Bind, Clock } from "@mrlyprod/ui/shell/design.ts";

export const cx = (...names: (string | false | null | undefined)[]) => names.filter(Boolean).join(" ") || undefined;

export const none: Bind = () => ({});

export const href = (bind: Bind) => String(bind().href ?? "");

export const caps = (text: string) => text.toUpperCase();

const two = (n: number) => String(n).padStart(2, "0");

export function time({ now, hours }: Clock) {
  const h = now.getHours();
  if (hours === 12) return `${two(h % 12 || 12)}:${two(now.getMinutes())}${h < 12 ? "A" : "P"}`;
  return `${two(h)}:${two(now.getMinutes())}`;
}

export function day({ now }: Clock) {
  return now.toLocaleDateString("en-GB", { weekday: "short", day: "numeric", month: "short" }).toUpperCase();
}

export function spot(now: Date) {
  const minute = Math.floor(now.getTime() / 60000);
  const hash = Math.imul(minute ^ 0x9e3779b9, 0x85ebca6b) >>> 0;
  return { left: `${10 + (hash % 61)}%`, top: `${12 + ((hash >>> 8) % 66)}%` };
}

export const dark = () => {
  const set = document.documentElement.dataset.theme;
  return set ? set === "dark" : matchMedia("(prefers-color-scheme: dark)").matches;
};

export function tone(value: string, night = dark()) {
  const m = /light-dark\(\s*([^,]+?)\s*,\s*([^)]+?)\s*\)/.exec(value);
  return (m ? m[night ? 2 : 1]! : value).trim();
}

export function ink(name: string, or: string) {
  return tone(getComputedStyle(document.documentElement).getPropertyValue(name)) || or;
}
