import { expect, test } from "bun:test";
import { fits, MIN } from "./gate.ts";

test("the shell boots on a screen at least the site's min, 1280x720 by default, and shows only the logo below", () => {
  expect(MIN).toEqual([1280, 720]);
  expect([
    fits([1280, 720]),
    fits([1440, 900]),
    fits([1279, 900]),
    fits([1440, 719]),
    fits([1000, 600]),
    fits([1000, 600], [800, 600]),
    fits([1280, 720], [1440, 900]),
  ]).toEqual([true, true, false, false, false, true, false]);
});
