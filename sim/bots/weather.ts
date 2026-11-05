import type { Json } from "@mrlyprod/os/contract/index.ts";
import { below, pick, walk } from "@mrlyprod/os/contract/index.ts";
import type { Saved, Weather } from "../apps/weather/app.ts";
import { inside } from "../apps/weather/app.ts";
import type { Bot, Step } from "./types.ts";

const NAMES = ["Home", "Work", "The coast", "The hills", "Grandma's", "The market"];

type Pinned = { q: number; r: number; name: string };

const pinned = (apps: Record<string, Json>): Pinned[] => {
  const maps = apps.maps;
  const places = maps && typeof maps === "object" && !Array.isArray(maps) ? maps.places : null;
  return places && typeof places === "object" && !Array.isArray(places) ? (Object.values(places) as Pinned[]) : [];
};

const bot: Bot<Weather> = ({ state, me, draw, world }): Step => {
  const mine = state.places[me] ?? {};
  const names = Object.keys(mine);
  if (names.length >= 4 || (names.length && !below(draw, 4)))
    return { verb: "forget", args: { name: pick(draw, names) } };
  const here = walk(world.apps, ["maps", "where", me]) as Saved | undefined;
  if (here && !("Here" in mine) && !below(draw, 4))
    return { verb: "save", args: { q: here.q, r: here.r, name: "Here" } };
  if (below(draw, 2)) return null;
  const spots = pinned(world.apps).filter((one) => !(one.name in mine));
  if (spots.length && below(draw, 2)) {
    const spot = pick(draw, spots);
    return { verb: "save", args: { q: spot.q, r: spot.r, name: spot.name } };
  }
  const span = 2 * state.radius + 1;
  const fresh = NAMES.filter((name) => !(name in mine));
  for (let tries = 0; tries < 8 && fresh.length; tries++) {
    const q = below(draw, span) - state.radius;
    const r = below(draw, span) - state.radius;
    if (inside(q, r, state.radius)) return { verb: "save", args: { q, r, name: pick(draw, fresh) } };
  }
  return null;
};

export default bot;
