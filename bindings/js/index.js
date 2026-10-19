import * as core from "./core.js";
import * as font from "./font.js";
import * as life from "./life.js";
import * as math from "./math.js";
import * as num from "./num.js";
import * as tile from "./tile.js";

export { core, font, life, math, num, tile };

export async function init() {
    await Promise.all([core.default(), font.default(), life.default(), math.default(), num.default(), tile.default()]);
}

export default init;
