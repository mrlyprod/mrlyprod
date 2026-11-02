import type { Bot } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Wishes } from "./app.ts";

/* POLICY */

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

export const bot: Bot<Wishes> = ({ state, me, draw }): ReturnType<Bot> => {
  const open = Object.values(state.wishes).filter((one) => one.withdrawn === null);
  const backable = open.filter((one) => one.by !== me && !one.backers.includes(me));
  if (backable.length && below(draw, 3))
    return { verb: "back", args: { wish: backable[below(draw, backable.length)]!.n } };
  if (open.filter((one) => one.by === me).length >= 2) return null;
  const asked = new Set(open.map((one) => one.text));
  const fresh = ASKS.filter((one) => !asked.has(one));
  return fresh.length ? { verb: "post", args: { text: fresh[below(draw, fresh.length)]! } } : null;
};
