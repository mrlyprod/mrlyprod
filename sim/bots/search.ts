import type { Search } from "@mrlyprod/apps/search/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const QUERIES = ["lentils", "plan", "tomatoes tuesdays", "hours", "soup", "ship measure", "pay request"];

const bot: Bot<Search> = ({ me, actors, draw }): Step => {
  const others = actors.filter((one) => one !== me);
  return {
    verb: "find",
    args: { words: others.length && below(draw, 3) === 0 ? pick(draw, others) : pick(draw, QUERIES) },
  };
};

export default bot;
