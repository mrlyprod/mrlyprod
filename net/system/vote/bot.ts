import type { Bot } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Votes } from "./app.ts";
import { OPENED } from "./app.ts";

/* POLICY */

type Asked = { wishes?: Record<string, { n: number; text: string; withdrawn: number | null; backers: string[] }> };

export const RIPE = 2;

export const bot: Bot<Votes> = ({ state, me, at, draw, world }): ReturnType<Bot> => {
  const ballots = Object.values(state.ballots);
  const due = ballots.find((one) => one.result === null && at >= one.closes);
  if (due) return { verb: "close", args: { ballot: due.n } };
  const open = ballots.filter((one) => at < one.closes && !(me in one.votes));
  if (open.length && below(draw, 4)) {
    const one = open[below(draw, open.length)]!;
    return { verb: "cast", args: { ballot: one.n, choice: below(draw, 5) < 3 ? "yes" : "no" } };
  }
  if (ballots.filter((one) => one.by === me && one.result === null).length >= OPENED || below(draw, 2)) return null;
  const decided = new Set(ballots.map((one) => one.wish));
  const ripe = Object.values((world.apps.wish as Asked | undefined)?.wishes ?? {})
    .filter((one) => one.withdrawn === null && one.backers.length >= RIPE && !decided.has(one.n))
    .sort((a, b) => b.backers.length - a.backers.length || a.n - b.n);
  const top = ripe[0];
  return top ? { verb: "open", args: { question: top.text, wish: top.n, seconds: 20 + below(draw, 41) } } : null;
};
