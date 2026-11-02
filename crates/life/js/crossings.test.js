import { expect, test } from "bun:test";
import * as life from "./index.js";

const bytes = await Bun.file(new URL("./pkg/mrly_life_js_bg.wasm", import.meta.url)).arrayBuffer();
life.initSync({ module: bytes });

test("a class field is set and crosses as plain data", () => {
  const carpet = { shape: [3, 3], types: Uint8Array.from([1, 1, 1, 1, 0, 1, 1, 1, 1]) };
  const config = new life.Config(carpet, life.Counts.list([3]), life.Counts.list([2, 3]));
  expect(config.boundary).toBe("Constant");
  config.boundary = "Wrap";
  expect(config.boundary).toBe("Wrap");
  expect(config.toJSON().boundary).toBe("Wrap");
  expect(() => {
    config.boundary = "Nope";
  }).toThrow("unknown variant");
});
