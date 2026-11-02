import type { Clock } from "../../net/shell/design.ts";

/* CLOCK */

const two = (n: number) => String(n).padStart(2, "0");

export function hhmm({ now, hours }: Clock) {
  const h = now.getHours();
  if (hours === 12) return `${h % 12 || 12}:${two(now.getMinutes())} ${h < 12 ? "AM" : "PM"}`;
  return `${two(h)}:${two(now.getMinutes())}`;
}

export function spot(now: Date) {
  const minute = Math.floor(now.getTime() / 60000);
  const hash = Math.imul(minute ^ 0x9e3779b9, 0x85ebca6b) >>> 0;
  return { left: `${10 + (hash % 61)}%`, top: `${10 + ((hash >>> 8) % 71)}%` };
}
