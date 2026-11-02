import type { Bot } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Wallet } from "./app.ts";

/* POLICY */

export const SHARE = 20;

export const bot: Bot<Wallet> = ({ state, me, actors, draw, world }): ReturnType<Bot> => {
  const held = state.balances[me] ?? 0;
  if (held < 1 || below(draw, 3)) return null;
  const amount = 1 + below(draw, Math.max(1, Math.floor(held / SHARE)));
  const others = actors.filter((id) => id !== me);
  const causes = Object.keys(state.causes);
  const verbs = [...(world.rules.payments === "on" ? ["send"] : []), ...(world.rules.gifts === "on" ? ["give"] : [])];
  if (!verbs.length || !others.length || !below(draw, 4))
    return causes.length ? { verb: "give", args: { to: pick(draw, causes), amount } } : null;
  return { verb: pick(draw, verbs), args: { to: pick(draw, others), amount } };
};
