import { expect, test } from "bun:test";
import { look, stamp } from "./prefs.ts";

test("an icon style resolves like Apple: Auto follows the theme, Always keeps the dark look, Default never changes", () => {
  expect(look("", false, true)).toBe("");
  expect(look("", true, false)).toBe("");
  expect(look("dark", false, false)).toBe("");
  expect(look("dark", false, true)).toBe("dark");
  expect(look("dark", true, false)).toBe("dark");
  expect(look("clear", false, false)).toBe("clear-light");
  expect(look("clear", false, true)).toBe("clear-dark");
  expect(look("tinted", true, false)).toBe("tinted-dark");
});

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
