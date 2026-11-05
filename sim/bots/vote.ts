import type { Ballot, Votes } from "@mrlyprod/apps/vote/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("vote");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

type Asked = { wishes?: Record<string, { n: number; text: string; status: string; backers: string[] }> };

export const RIPE = 2;

const QUESTIONS = [
  "Should the market open on rest days?",
  "Should the bridge toll be dropped?",
  "Should the wiki be kept in plain words?",
  "Should news keep older posts?",
];

type Org = { n: number; dissolved: number | null; members: Record<string, string> };

const eligible = (state: Votes, one: Ballot) => one.members ?? state.people;

const counted = (state: Votes, one: Ballot) => !!eligible(state, one)?.every((id) => Object.hasOwn(one.votes, id));

const clubsOf = (world: Parameters<Bot>[0]["world"], me: string) =>
  Object.values((world.apps.people as { orgs?: Record<string, Org> } | undefined)?.orgs ?? {}).filter(
    (one) => one.dissolved === null && Object.hasOwn(one.members, me),
  );

const seconds = (draw: () => number, least: number) => Math.max(least, 20 + below(draw, 41));

const asking = (
  state: Votes,
  world: Parameters<Bot>[0]["world"],
  me: string,
  draw: () => number,
  least: number,
): Step => {
  if (state.decide === "vote") {
    const decided = new Set(Object.values(state.ballots).map((one) => one.wish));
    const ripe = Object.values((world.apps.wish as Asked | undefined)?.wishes ?? {})
      .filter((one) => one.status === "open" && one.backers.length >= RIPE && !decided.has(one.n))
      .sort((a, b) => b.backers.length - a.backers.length || a.n - b.n);
    const top = ripe[0];
    if (top) return { verb: "open", args: { question: top.text, wish: top.n, seconds: seconds(draw, least) } };
  }
  if (below(draw, 4)) return null;
  const clubs = clubsOf(world, me);
  const args = { question: pick(draw, QUESTIONS), seconds: seconds(draw, least) };
  return { verb: "open", args: clubs.length && below(draw, 2) ? { ...args, org: pick(draw, clubs).n } : args };
};

const bot: Bot<Votes> = ({ state, me, at, draw, world }): Step => {
  const ballots = Object.values(state.ballots);
  const shortest = knob(world.rules, "shortest");
  const ready = ballots.find(
    (one) => one.result === null && (at >= one.closes || (counted(state, one) && at >= one.opened + shortest * 1000)),
  );
  if (ready) return { verb: "close", args: { ballot: ready.n } };
  const open = ballots.filter(
    (one) =>
      one.result === null &&
      at < one.closes &&
      !Object.hasOwn(one.votes, me) &&
      (eligible(state, one)?.includes(me) ?? true),
  );
  if (open.length && below(draw, 4))
    return { verb: "cast", args: { ballot: pick(draw, open).n, choice: below(draw, 5) < 3 ? "yes" : "no" } };
  const mine = ballots.filter((one) => one.by === me && one.result === null);
  const silent = mine.filter((one) => !Object.keys(one.votes).length);
  if (silent.length && !below(draw, 8)) return { verb: "cancel", args: { ballot: pick(draw, silent).n } };
  if (mine.length >= knob(world.rules, "opened") || below(draw, 2)) return null;
  return asking(state, world, me, draw, Math.max(1, shortest));
};

export default bot;
