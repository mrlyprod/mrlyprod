import type { Feedback } from "@mrlyprod/apps/feedback/app.ts";
import { SEVERITIES } from "@mrlyprod/apps/feedback/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("feedback");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const PLACES = ["read", "door", "world"];

const WISHED = ["schedule", "undo", "search", "mute", "forward"];

const bot: Bot<Feedback> = ({ state, me, draw, world }): Step => {
  const items = Object.values(state.items);
  const others = items.filter((one) => one.by !== me && !one.same.includes(me));
  if (others.length && !below(draw, 10)) return { verb: "same", args: { id: pick(draw, others).id } };
  if (below(draw, 20)) return null;
  if (items.filter((one) => one.by === me).length >= knob(world.rules, "keep")) return null;
  const app = below(draw, 4) ? pick(draw, Object.keys(world.apps)) : pick(draw, PLACES);
  const verbs = WISHED;
  return {
    verb: "send",
    args: {
      app,
      happened: pick(draw, HAPPENED),
      severity: pick(draw, SEVERITIES),
      ...(below(draw, 2) ? { verb: pick(draw, verbs) } : {}),
      ...(below(draw, 2) ? { expected: pick(draw, EXPECTED) } : {}),
    },
  };
};

export default bot;
