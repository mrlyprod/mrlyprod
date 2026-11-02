/* BIND */

export const href = (bind: () => Record<string, unknown>) => String(bind().href ?? "");

/* CLOCK */

export function spot(now: Date) {
  const minute = Math.floor(now.getTime() / 60000);
  const hash = Math.imul(minute ^ 0x9e3779b9, 0x85ebca6b) >>> 0;
  return { left: `${10 + (hash % 61)}%`, top: `${10 + ((hash >>> 8) % 71)}%` };
}
