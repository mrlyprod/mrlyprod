import type { Bot, Step } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Sim } from "./app.ts";

/* BOT */

const bot: Bot<Sim> = ({ draw }): Step => (below(draw, 4) === 0 ? { verb: "snap", args: {} } : null);

export default bot;
