import { expect, test } from "bun:test";
import * as num from "./index.js";

const bytes = await Bun.file(new URL("./pkg/mrly_num_js_bg.wasm", import.meta.url)).arrayBuffer();
num.initSync({ module: bytes });

test("u128 crosses both ways as a decimal string", () => {
  expect(num.factor.gcd("1071", "462")).toBe("21");
  expect(num.factor.gcd(1071, 462n)).toBe("21");
  expect(num.factor.gcd("340282366920938463463374607431768211454", "2")).toBe("2");
  expect(() => num.factor.gcd("x", 2)).toThrow("a u128 wants a decimal string, a whole number or a bigint.");
});
