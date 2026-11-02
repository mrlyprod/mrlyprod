import { expect, test } from "bun:test";
import { daily, today } from "./desk.tsx";

test("the design of the day is one index per date, the same for everyone", () => {
  const zone = process.env.TZ;
  process.env.TZ = "Pacific/Kiritimati";
  try {
    expect([
      today(new Date("2026-10-19T23:30:00-07:00")),
      today(new Date("2026-10-20T08:30:00+14:00")),
      today(new Date("2026-10-19T12:00:00Z")),
    ]).toEqual(["2026-10-20", "2026-10-19", "2026-10-19"]);
  } finally {
    if (zone === undefined) delete process.env.TZ;
    else process.env.TZ = zone;
  }
  expect(daily("2026-10-19", 50)).toBe(daily("2026-10-19", 50));
  expect(
    new Set(Array.from({ length: 30 }, (_, d) => daily(`2026-11-${String(d + 1).padStart(2, "0")}`, 50))).size,
  ).toBeGreaterThan(15);
});
