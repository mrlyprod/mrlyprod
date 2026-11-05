import type { Papers, Section } from "@mrlyprod/apps/papers/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const REMARKS = [
  "The method is thin.",
  "Have you tried it with salt?",
  "Good result; I would like the data.",
  "Cite Cook.",
];

const bot: Bot<Papers> = ({ state, me, actors, draw }): Step => {
  const ids = Object.keys(state.papers);
  const own = ids.filter((id) => state.papers[id]!.owner === me);
  const drafts = own.filter((id) => state.papers[id]!.status === "draft");
  const open = ids.filter((id) => {
    const one = state.papers[id]!;
    return one.owner !== me && (one.status === "published" || one.readers.includes(me));
  });
  if (open.length && !below(draw, 5))
    return { verb: "comment", args: { paper: pick(draw, open), text: pick(draw, REMARKS) } };
  if (!drafts.length || (own.length < 6 && below(draw, 12) === 0))
    return { verb: "create", args: { title: pick(draw, TITLES) } };
  const id = pick(draw, drafts);
  const paper = state.papers[id]!;
  const owners = Object.values(state.papers).map((one) => one.owner);
  const others = [...new Set([...actors, ...owners])].filter((one) => one !== me && !paper.readers.includes(one));
  if (others.length && !below(draw, 8)) return { verb: "share", args: { paper: id, with: pick(draw, others) } };
  if (paper.readers.length && !below(draw, 16))
    return { verb: "unshare", args: { paper: id, with: pick(draw, paper.readers) } };
  if (drafts.length > 3 && !below(draw, 10)) return { verb: "delete", args: { paper: id } };
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
