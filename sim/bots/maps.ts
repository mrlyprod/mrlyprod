import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Maps, Spot } from "../apps/maps/app.ts";
import { inside, key } from "../apps/maps/app.ts";
import type { Bot, Step } from "./types.ts";

export const NAMES = ["Home", "Market", "Well", "Mill", "Bakery", "School", "Harbour", "Bridge", "Orchard", "Workshop"];

const LINES = [
  "Anyone around?",
  "Good to see you here.",
  "Lovely spot, this.",
  "Shall we walk to the market?",
  "What brings you here?",
  "I could stay here all day.",
];

const POSTS = [
  "Fresh bread at the bakery this morning.",
  "Looking for someone to share a walk to the harbour.",
  "Meet here at dusk if you want company.",
  "Lost a blue scarf near the well; please say if found.",
  "Anyone have a job going? I can carry, cook or write.",
  "Reading a good book; ask me which.",
];

const STEPS: Spot[] = [
  { q: 1, r: 0 },
  { q: -1, r: 0 },
  { q: 0, r: 1 },
  { q: 0, r: -1 },
  { q: 1, r: -1 },
  { q: -1, r: 1 },
];

const wander = (state: Maps, spot: Spot, draw: () => number): Step => {
  const step = pick(draw, STEPS);
  const q = spot.q + step.q;
  const r = spot.r + step.r;
  return inside(q, r, state.radius) && state.cells[key(q, r)] !== "water" ? { verb: "go", args: { q, r } } : null;
};

const boarding = (state: Maps, me: string, spot: Spot, draw: () => number): Step => {
  if (!state.keeps) return null;
  const open = Object.values(state.places).filter(
    (one) => one.q === spot.q && one.r === spot.r && (one.by === me || one.shared || state.own === "shared"),
  );
  return open.length ? { verb: "post", args: { place: pick(draw, open).id, text: pick(draw, POSTS) } } : null;
};

const pinning = (state: Maps, me: string, draw: () => number, squares: boolean): Step => {
  const mine = Object.values(state.places).filter((one) => one.by === me);
  if (mine.length >= 6 || (mine.length && !below(draw, 3))) {
    const one = pick(draw, mine);
    return below(draw, 2) || mine.length >= 6
      ? { verb: "unpin", args: { place: one.id } }
      : { verb: "rename", args: { place: one.id, name: `${pick(draw, NAMES)} of ${me}` } };
  }
  if (state.own === "none") return null;
  const shared = squares && !below(draw, 2);
  if (below(draw, 2)) return { verb: "pin", args: { name: pick(draw, NAMES), ...(shared ? { shared } : {}) } };
  const span = 2 * state.radius + 1;
  for (let tries = 0; tries < 8; tries++) {
    const q = below(draw, span) - state.radius;
    const r = below(draw, span) - state.radius;
    if (inside(q, r, state.radius) && state.cells[key(q, r)] !== "water")
      return { verb: "pin", args: { q, r, name: pick(draw, NAMES), ...(shared ? { shared } : {}) } };
  }
  return null;
};

const bot: Bot<Maps> = ({ state, me, actors, at, draw, world }): Step => {
  const spot = state.where[me];
  if (!spot) return null;
  const others = actors.filter((one) => one !== me);
  const walking = (state.walking[me] ?? 0) > at;
  const here = others.filter((one) => state.where[one]?.q === spot.q && state.where[one]?.r === spot.r);
  const places = Object.values(state.places);
  const roll = below(draw, 10);
  if (!walking && roll < 2) {
    const board = boarding(state, me, spot, draw);
    if (board) return board;
  }
  if (!walking && here.length && roll < 4) return { verb: "say", args: { text: pick(draw, LINES) } };
  if (!walking && places.length && roll < 2) return { verb: "go", args: { place: pick(draw, places).id } };
  if (!walking && roll < 5) return wander(state, spot, draw);
  if (others.length && roll < 6) {
    const invited = { person: pick(draw, others) };
    return { verb: "invite", args: places.length ? { ...invited, place: pick(draw, places).id } : invited };
  }
  if (!walking && others.length && roll < 7) {
    const friend = state.where[pick(draw, others)];
    return friend && (friend.q !== spot.q || friend.r !== spot.r)
      ? { verb: "go", args: { q: friend.q, r: friend.r } }
      : null;
  }
  return pinning(state, me, draw, reader(world.rules, "maps")<boolean>("squares"));
};

export default bot;
