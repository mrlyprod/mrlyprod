import assert from "node:assert/strict";
import * as math from "@mrlyprod/math";

assert.deepEqual(math.two.carpet(3, 1).shape, [3, 3]);
console.log("math node smoke ok");
