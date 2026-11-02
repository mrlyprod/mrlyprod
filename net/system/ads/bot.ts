import type { Bot } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Ads } from "./app.ts";
import { LIVE } from "./app.ts";

/* POLICY */

type Orgs = {
  orgs?: Record<
    string,
    { n: number; name: string; open: boolean; dissolved: number | null; members: Record<string, string> }
  >;
};

export const bot: Bot<Ads> = ({ state, me, draw, world }): ReturnType<Bot> => {
  const live = Object.values(state.ads).filter((one) => one.by === me);
  if (live.length >= LIVE) return below(draw, 3) ? null : { verb: "pull", args: { ad: live[0]!.n } };
  const shown = new Set(live.map((one) => one.org));
  const led = Object.values((world.apps.people as Orgs | undefined)?.orgs ?? {}).filter(
    (one) => one.dissolved === null && ["owner", "admin"].includes(one.members[me] ?? "") && !shown.has(one.name),
  );
  if (!led.length || below(draw, 2)) return null;
  const org = led[below(draw, led.length)]!;
  return {
    verb: "post",
    args: { org: org.name, title: org.name, text: org.open ? "Open to new members" : "Ask to join" },
  };
};
