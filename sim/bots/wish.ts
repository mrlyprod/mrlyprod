import type { Wish, Wishes } from "@mrlyprod/apps/wish/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("wish");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const ASKS = [
  "Build a maps app",
  "Build a weather app",
  "Build a news app",
  "Build a books app",
  "Build a music app",
  "Build a podcasts app",
  "Build a wiki",
  "Build a sheets app",
  "Build a slides app",
  "Build a video app",
  "Let chat threads hold more than two people",
  "Keep more than 50 messages in a chat thread",
];

const COMMENTS = ["I would use this every day.", "What would it cost?", "Who builds it?", "Needed, and soon."];

const HELD = 2;

const decided = (draw: () => number, mine: Wish[]): Step => ({
  verb: below(draw, 4) ? "grant" : "decline",
  args: { wish: pick(draw, mine).n },
});

const talked = (draw: () => number, open: Wish[], me: string): Step => {
  const turn = below(draw, 8);
  if (turn < 2 && open.length)
    return { verb: "comment", args: { wish: pick(draw, open).n, text: pick(draw, COMMENTS) } };
  const backed = open.filter((one) => one.backers.includes(me));
  if (turn === 2 && backed.length) return { verb: "unback", args: { wish: pick(draw, backed).n } };
  const held = open.filter((one) => one.by === me);
  return turn === 3 && held.length ? { verb: "withdraw", args: { wish: pick(draw, held).n } } : null;
};

const bot: Bot<Wishes> = ({ state, me, draw, world }): Step => {
  const open = Object.values(state.wishes).filter((one) => one.status === "open");
  const mine = open.filter((one) => one.decider === me);
  if (mine.length && below(draw, 2)) return decided(draw, mine);
  const backable = open.filter((one) => one.by !== me && !one.backers.includes(me));
  if (backable.length && below(draw, 3)) return { verb: "back", args: { wish: pick(draw, backable).n } };
  const talk = talked(draw, open, me);
  if (talk) return talk;
  if (open.filter((one) => one.by === me).length >= Math.min(HELD, knob(world.rules, "open"))) return null;
  const asked = new Set(open.map((one) => one.text));
  const fresh = ASKS.filter((one) => !asked.has(one));
  return fresh.length ? { verb: "post", args: { text: pick(draw, fresh) } } : null;
};

export default bot;
