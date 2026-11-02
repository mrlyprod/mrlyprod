import { expect, test } from "bun:test";
import { table } from "./build.ts";

test("the loader table imports only the named units that have a loader", () => {
  expect(table(["core", "math"]).match(/"[a-z]+": /g)).toEqual(['"math": ']);
});
