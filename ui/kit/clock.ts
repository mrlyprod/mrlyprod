const CYCLES: Partial<Record<string, "h12" | "h23">> = { 12: "h12", 24: "h23" };

export function stamp(now: Date, mode: string, bare?: boolean) {
  const cycle = CYCLES[mode];
  return new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit", ...(cycle && { hourCycle: cycle }) })
    .formatToParts(now)
    .filter((part) => !bare || part.type !== "dayPeriod")
    .map((part) => part.value)
    .join("")
    .trim();
}
