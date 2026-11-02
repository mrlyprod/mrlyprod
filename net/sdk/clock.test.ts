import { expect, test } from "bun:test";
import { stamp } from "./clock.ts";

test("the clock reads 12 or 24 hours when asked, and the bare saver clock drops AM and PM", () => {
  const now = new Date(2026, 0, 1, 13, 5);
  expect(stamp(now, "24")).toContain("13");
  expect(stamp(now, "12")).not.toContain("13");
  expect(stamp(now, "12")).toContain("05");
  expect(stamp(now, "12", true)).toBe(
    stamp(now, "12", true)
      .replace(/[^\d:.\s]/g, "")
      .trim(),
  );
  expect(stamp(now, "")).toBe(new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit" }).format(now));
});
