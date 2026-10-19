import * as core from "./core.node.js";
import * as font from "./font.node.js";
import * as life from "./life.node.js";
import * as math from "./math.node.js";
import * as num from "./num.node.js";
import * as tile from "./tile.node.js";

export { core, font, life, math, num, tile };

export async function init() {
    await Promise.all([core.default(), font.default(), life.default(), math.default(), num.default(), tile.default()]);
}

export default init;
