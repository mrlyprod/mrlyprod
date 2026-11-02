import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Papers, Section } from "./app.ts";

/* WORDS */

export const TITLES = ["On lentils", "A week of hours", "Notes on the garden", "Why ship twice"];

const SECTIONS: Section[][] = [
  [
    { heading: "Abstract", text: "We cooked lentils for a week and wrote down what we learned." },
    { heading: "Method", text: "One pot, one onion, one bay leaf." },
  ],
  [
    { heading: "Introduction", text: "Forty-eight hours a week is a number." },
    { heading: "Results", text: "The number held." },
    { heading: "Discussion", text: "Measure again next quarter." },
  ],
];

const WORKS: [string, string][] = [
  ["The Soup Book", "A. Cook"],
  ["Hours and Days", "B. Clock"],
  ["Gardens of the North", "C. Green"],
  ["Shipping Twice", "D. Boat"],
];

/* BOT */

const bot: Bot<Papers> = ({ state, me, draw }): Step => {
  const own = Object.keys(state.papers).filter((id) => state.papers[id]!.owner === me);
  const drafts = own.filter((id) => state.papers[id]!.status === "draft");
  if (!drafts.length || (own.length < 6 && below(draw, 12) === 0))
    return { verb: "create", args: { title: pick(draw, TITLES) } };
  const id = pick(draw, drafts);
  const paper = state.papers[id]!;
  const unread = WORKS.filter(([title]) => !paper.references.some((ref) => ref.title === title));
  if (paper.sections.length && paper.references.length > 1 && below(draw, 5) === 0)
    return { verb: "publish", args: { paper: id } };
  if (unread.length && below(draw, 2) === 0) {
    const [title, by] = pick(draw, unread);
    return { verb: "cite", args: { paper: id, title, by } };
  }
  return { verb: "write", args: { paper: id, sections: pick(draw, SECTIONS) } };
};

export default bot;
