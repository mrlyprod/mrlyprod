import type { Bot, Step } from "../../sdk/system.ts";
import { pick } from "../../sdk/system.ts";
import type { Search } from "./app.ts";

/* WORDS */

export const QUERIES = ["lentils", "plan", "tomatoes tuesdays", "hours", "soup", "test-2", "ship measure"];

/* BOT */

const bot: Bot<Search> = ({ draw }): Step => ({ verb: "find", args: { words: pick(draw, QUERIES) } });

export default bot;
