import assert from "node:assert/strict";
import * as tile from "@mrlyprod/tile";

assert.deepEqual(tile.background(1, 2, 2).shape, [18, 18]);
console.log("tile node smoke ok");
