import assert from "node:assert/strict";
import * as core from "@mrlyprod/core";

assert.deepEqual(core.colors.from_hex("#ff3d40"), [255, 61, 64, 255]);
console.log("core node smoke ok");
