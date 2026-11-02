import assert from "node:assert/strict";
import * as font from "@mrlyprod/font";

assert.ok(font.path("M").length > 0);
console.log("font node smoke ok");
