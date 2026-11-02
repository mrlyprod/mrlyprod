import type { Ctx, Json, Schema } from "../../sdk/system.ts";
import { app, below, sliced, stream } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Terrain = "water" | "sand" | "grass" | "forest" | "hill" | "mountain";

export type Place = { id: number; q: number; r: number; name: string; by: string; at: number };

export type Maps = { radius: number; cells: Record<string, Terrain>; next: number; places: Record<string, Place> };

export const RADIUS = 8;

export const NAME = 40;

export const OWN = 20;

const LAND: Terrain[] = [
  "mountain",
  "hill",
  "hill",
  "forest",
  "forest",
  "forest",
  "grass",
  "grass",
  "grass",
  "grass",
  "sand",
];

/* CELLS */

export const key = (q: number, r: number) => `${q},${r}`;

export const inside = (q: number, r: number, radius: number) =>
  Math.abs(q) <= radius && Math.abs(r) <= radius && Math.abs(q + r) <= radius;

export function cells(seed: number, radius: number): Record<string, Terrain> {
  const { draw } = stream(seed);
  const out: Record<string, Terrain> = {};
  for (let q = -radius; q <= radius; q++)
    for (let r = -radius; r <= radius; r++) {
      if (!inside(q, r, radius)) continue;
      const far = Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
      const roll = below(draw, 10) + Math.floor((far * 4) / radius);
      out[key(q, r)] = LAND[roll] ?? "water";
    }
  return out;
}

const placeOf = (state: Maps, id: number, me: string, refuse: Ctx["refuse"]): Place => {
  const one = state.places[id];
  if (!one) return refuse(`no place ${id}; it was unpinned`);
  if (one.by !== me) return refuse(`place ${id} is ${one.by}'s; only they change it`);
  return one;
};

const at = (state: Maps, q: number, r: number) =>
  Object.values(state.places).filter((one) => one.q === q && one.r === r);

const cell = (state: Maps, path: string[]): Json | undefined => {
  const q = Number(path[1]);
  const r = Number(path[2]);
  if (!Number.isInteger(q) || !Number.isInteger(r) || !inside(q, r, state.radius)) return undefined;
  return { q, r, terrain: state.cells[key(q, r)]!, places: at(state, q, r) };
};

const mine = (state: Maps, id: string) => ({
  places: Object.values(state.places)
    .filter((one) => one.by === id)
    .map((one) => ({ ...one, terrain: state.cells[key(one.q, one.r)]! })),
});

/* SCHEMAS */

const coordinate = (state: Maps): Schema => ({ type: "integer", minimum: -state.radius, maximum: state.radius });

const name: Schema = { type: "string", minLength: 1, maxLength: NAME };

const pinning = (state: Maps): Schema => ({
  type: "object",
  properties: { q: coordinate(state), r: coordinate(state), name },
  required: ["q", "r", "name"],
  additionalProperties: false,
});

const naming = (state: Maps, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { place: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) }, ...more },
  required: ["place", ...Object.keys(more)],
  additionalProperties: false,
});

/* APP */

export default app<Maps>({
  id: "maps",
  title: "Maps",
  line: `A hex map of the world, ${RADIUS} cells from the centre: a terrain per cell, and the places people pin and name.`,
  boot: ({ seed }) => ({ radius: RADIUS, cells: cells(seed, RADIUS), next: 1, places: {} }),
  verbs: {
    pin: {
      line: `Pin a place on a cell (q, r) and name it, 1 to ${NAME} characters; you pin at most ${OWN}.`,
      args: pinning,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const q = Number(args.q);
        const r = Number(args.r);
        const text = String(args.name).trim();
        if (!inside(q, r, state.radius))
          return ctx.refuse(`(${q}, ${r}) is off the map; q + r stays within ${state.radius} of the centre`);
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        if (mine(state, me).places.length >= OWN) return ctx.refuse(`you pinned ${OWN} places; unpin one first`);
        const id = state.next;
        return {
          ...state,
          next: id + 1,
          places: { ...state.places, [id]: { id, q, r, name: text, by: me, at: ctx.at } },
        };
      },
    },
    rename: {
      line: "Rename a place of yours.",
      args: (state) => naming(state, { name }),
      run: (state, args, ctx) => {
        const one = placeOf(state, Number(args.place), ctx.who.id, ctx.refuse);
        const text = String(args.name).trim();
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        return { ...state, places: { ...state.places, [one.id]: { ...one, name: text } } };
      },
    },
    unpin: {
      line: "Take a place of yours off the map.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const one = placeOf(state, Number(args.place), ctx.who.id, ctx.refuse);
        const places = { ...state.places };
        delete places[one.id];
        return { ...state, places };
      },
    },
  },
  moves: (state) => (Object.keys(state.places).length ? ["pin", "rename", "unpin"] : ["pin"]),
  read: (state, path, who) => (path[0] === "cell" ? cell(state, path) : sliced(mine)(state, path, who)),
  bot,
});
