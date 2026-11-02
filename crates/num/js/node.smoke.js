import assert from "node:assert/strict";
import * as num from "@mrlyprod/num";

assert.equal(num.factor.gcd("1071", "462"), "21");
console.log("num node smoke ok");
