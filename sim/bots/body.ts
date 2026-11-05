import type { Held } from "@mrlyprod/apps/market/goods.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Body } from "../apps/body/needs.ts";
import { LOW, levelsOf } from "../apps/body/needs.ts";
import type { Bot, Step } from "./types.ts";

type Shop = { own: Record<string, Record<string, Held>> };

const DISHES = ["Stew", "Fried rice", "Omelette", "Hash", "Curry", "Soup of the day"];

const OUTINGS = ["a walk", "chess", "a picnic", "football", "dancing", "a swim", "cards", "a film"];

const NIGHT = 8;

const NAP = 4;

const FUN = 10;

const POT = 4;

const going = (own: [string, Held][], others: string[], draw: () => number): Step => {
  const toys = own.filter(([, one]) => Number(one.traits.fun ?? 0) >= FUN);
  const what = toys.length && below(draw, 2) ? pick(draw, toys)[0] : pick(draw, OUTINGS);
  return { verb: "play", args: { what, ...(others.length && below(draw, 2) ? { with: pick(draw, others) } : {}) } };
};

const bot: Bot<Body> = ({ state, me, actors, at, draw, world }): Step => {
  const { hunger, energy, fun } = levelsOf(state, me, at);
  const own = Object.entries((world.apps.market as unknown as Shop | undefined)?.own[me] ?? {});
  const food = own.filter(([, one]) => Number(one.traits.food ?? 0) > 0);
  if (hunger >= LOW && food.length && below(draw, 2)) return { verb: "eat", args: { item: pick(draw, food)[0] } };
  if (food.length && !below(draw, 6)) return { verb: "feast", args: { item: pick(draw, food)[0] } };
  if (food.length >= 2 && !below(draw, 3)) {
    const first = below(draw, food.length);
    const second = (first + 1 + below(draw, food.length - 1)) % food.length;
    const servings = 1 + below(draw, Math.min(POT, state.dials.servings));
    return { verb: "cook", args: { items: [food[first]![0], food[second]![0]], name: pick(draw, DISHES), servings } };
  }
  if (hunger >= LOW && food.length) return { verb: "eat", args: { item: pick(draw, food)[0] } };
  if (energy <= LOW)
    return { verb: "sleep", args: { hours: Math.min(state.dials.night, NAP + below(draw, NIGHT - NAP + 1)) } };
  if (fun <= LOW || !below(draw, 3))
    return going(
      own,
      actors.filter((id) => id !== me),
      draw,
    );
  return null;
};

export default bot;
