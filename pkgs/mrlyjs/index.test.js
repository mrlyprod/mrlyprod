import { expect, test } from "bun:test";
import { hello } from "./index.js";

test("hello", () => {
  expect(hello()).toBe("Hello, World!");
});
