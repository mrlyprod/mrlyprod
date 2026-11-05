import type { Wallet } from "@mrlyprod/apps/wallet/app.ts";
import { allowed, claimable, DAY } from "@mrlyprod/apps/wallet/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Bot, Step } from "./types.ts";

export const SHARE = 20;

const claiming = (state: Wallet, me: string, at: number) =>
  state.claims && state.claims[me] !== Math.floor(at / DAY) && claimable(state, me) > 0;

const bot: Bot<Wallet> = ({ state, me, actors, at, draw, world }): Step => {
  const held = state.balances[me] ?? 0;
  if (claiming(state, me, at) && below(draw, 2)) return { verb: "claim", args: {} };
  const live = Object.values(state.bills).filter((one) => one.status === "open");
  const owed = live.filter((one) => one.to === me);
  if (owed.length && below(draw, 2)) {
    const one = pick(draw, owed);
    return { verb: one.amount <= held && below(draw, 4) ? "pay" : "decline", args: { request: one.n } };
  }
  const asked = live.filter((one) => one.by === me);
  if (asked.length && !below(draw, 8)) return { verb: "cancel", args: { request: pick(draw, asked).n } };
  const others = actors.filter((id) => id !== me);
  const paying = allowed(reader(world.rules, "wallet"), "payments");
  if (paying && others.length && state.supply && !below(draw, 6)) {
    const amount = 1 + below(draw, Math.min(50, state.supply));
    return { verb: "request", args: { to: pick(draw, others), amount, note: "for the work" } };
  }
  if (held < 1 || below(draw, 3)) return null;
  const amount = 1 + below(draw, Math.max(1, Math.floor(held / SHARE)));
  const causes = Object.keys(state.causes);
  const verbs = [...(paying ? ["send"] : []), ...(allowed(reader(world.rules, "wallet"), "gifts") ? ["give"] : [])];
  if (!verbs.length || !others.length || !below(draw, 4))
    return causes.length ? { verb: "give", args: { to: pick(draw, causes), amount } } : null;
  return { verb: pick(draw, verbs), args: { to: pick(draw, others), amount } };
};

export default bot;
