import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pin, wasmHash } from "./wasm.ts";

test("the pin reads the lock, and site/wasm's hash only once it holds a file", () => {
  const root = mkdtempSync(join(tmpdir(), "pin-"));
  writeFileSync(join(root, "wasm.lock"), "abc\n");
  expect(pin(root)).toEqual({ lock: "abc", local: null });
  mkdirSync(join(root, "wasm", "core"), { recursive: true });
  writeFileSync(join(root, "wasm", "core", "a.wasm"), "x");
  expect(pin(root)).toEqual({ lock: "abc", local: wasmHash(join(root, "wasm")) });
  expect(wasmHash(join(root, "wasm"))).toMatch(/^[0-9a-f]{64}$/);
});
