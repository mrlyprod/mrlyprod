import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Feedback } from "./app.ts";
import { KEEP, SEVERITIES, verbsOf } from "./app.ts";

/* WORDS */

export const HAPPENED = [
  "The window opened empty",
  "My act was refused with no note",
  "The list shows yesterday's state",
  "The icon is blurry on the dock",
  "The verb took ten seconds to answer",
  "The read came back as null",
  "The form lost my text on a refusal",
  "Two items show the same id",
];

export const EXPECTED = [
  "The window shows the state",
  "A note naming why",
  "The state as of now",
  "A sharp icon",
  "An answer within a second",
  "The item I asked for",
  "My text kept in the form",
  "One id an item",
];

/* BOT */

export const bot: Bot<Feedback> = ({ state, me, draw, world }): Step => {
  const items = Object.values(state.items);
  const others = items.filter((one) => one.by !== me && !one.same.includes(me));
  if (others.length && !below(draw, 10)) return { verb: "same", args: { id: pick(draw, others).id } };
  if (below(draw, 20)) return null;
  if (items.filter((one) => one.by === me).length >= KEEP) return null;
  const app = pick(draw, Object.keys(world.apps));
  const verbs = verbsOf(app) ?? [];
  return {
    verb: "send",
    args: {
      app,
      happened: pick(draw, HAPPENED),
      severity: pick(draw, SEVERITIES),
      ...(verbs.length && below(draw, 2) ? { verb: pick(draw, verbs) } : {}),
      ...(below(draw, 2) ? { expected: pick(draw, EXPECTED) } : {}),
    },
  };
};
