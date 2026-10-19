import assert from "node:assert/strict";
import init, { path } from "@mrlyprod/mrly/font";
import * as mrly from "@mrlyprod/mrly";

await init();
assert.ok(path("M").length > 0);
await mrly.init();
assert.equal(mrly.num.factor.gcd("1071", "462"), "21");
assert.deepEqual(mrly.math.two.carpet(3, 1).shape, [3, 3]);
console.log("node smoke ok");
