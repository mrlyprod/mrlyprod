import assert from "node:assert/strict";
import * as life from "@mrlyprod/life";

assert.deepEqual(Array.from(life.history([0, 1, 0], 30, 1, true).shape), [2, 3]);
console.log("life node smoke ok");
