import type { Json, Schema } from "../../sdk/system.ts";
import { app, below, sliced, stream } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Sky = "clear" | "cloudy" | "rain" | "storm" | "snow" | "fog";

export type Report = { q: number; r: number; day: number; sky: Sky; high: number; low: number; wind: number };

export type Saved = { q: number; r: number };

export type Weather = { seed: number; radius: number; day: number; places: Record<string, Record<string, Saved>> };

export const DAY = 86_400_000;

export const RADIUS = 8;

export const NAME = 40;

export const OWN = 10;

export const AHEAD = 3;

const SEASON = [2, 4, 8, 12, 17, 21, 24, 23, 19, 13, 7, 3];

/* SKIES */

export const inside = (q: number, r: number, radius: number) =>
  Math.abs(q) <= radius && Math.abs(r) <= radius && Math.abs(q + r) <= radius;

export const dayOf = (at: number) => Math.floor(at / DAY);

export function report(seed: number, q: number, r: number, day: number): Report {
  const mixed =
    (seed ^ Math.imul(q + 1000, 73856093) ^ Math.imul(r + 1000, 19349663) ^ Math.imul(day + 1, 83492791)) >>> 0;
  const { draw } = stream(mixed);
  draw();
  draw();
  const base = SEASON[Math.floor((((day % 360) + 360) % 360) / 30)]! - Math.abs(r);
  const high = base + below(draw, 7) - 3;
  const low = high - 4 - below(draw, 6);
  const roll = below(draw, 100);
  const wet = roll < 35 ? "clear" : roll < 60 ? "cloudy" : roll < 78 ? "rain" : roll < 85 ? "storm" : "fog";
  const sky: Sky = (wet === "rain" || wet === "storm") && high <= 2 ? "snow" : wet;
  const wind = below(draw, 40) + (sky === "storm" ? 30 : 0);
  return { q, r, day, sky, high, low, wind };
}

const ahead = (state: Weather, q: number, r: number) =>
  Array.from({ length: AHEAD + 1 }, (_, i) => report(state.seed, q, r, state.day + i));

const at = (state: Weather, path: string[]): Json | undefined => {
  const q = Number(path[1]);
  const r = Number(path[2]);
  const day = path[3] === undefined ? state.day : Number(path[3]);
  if (!Number.isInteger(q) || !Number.isInteger(r) || !Number.isInteger(day) || !inside(q, r, state.radius))
    return undefined;
  return report(state.seed, q, r, day);
};

const mine = (state: Weather, id: string) => ({
  day: state.day,
  places: Object.entries(state.places[id] ?? {}).map(([name, { q, r }]) => ({
    name,
    q,
    r,
    ahead: ahead(state, q, r),
  })),
});

/* SCHEMAS */

const coordinate = (state: Weather): Schema => ({ type: "integer", minimum: -state.radius, maximum: state.radius });

const name: Schema = { type: "string", minLength: 1, maxLength: NAME };

const saving = (state: Weather): Schema => ({
  type: "object",
  properties: { q: coordinate(state), r: coordinate(state), name },
  required: ["q", "r", "name"],
  additionalProperties: false,
});

const forgetting: Schema = { type: "object", properties: { name }, required: ["name"], additionalProperties: false };

/* APP */

export default app<Weather>({
  id: "weather",
  title: "Weather",
  line: "The sky over every map cell, a world day at a time, drawn from the seed; and the places you save to watch.",
  boot: ({ seed }) => ({ seed, radius: RADIUS, day: 0, places: {} }),
  verbs: {
    save: {
      line: `Save a map cell (q, r) under a name of 1 to ${NAME} characters to see its weather; you save at most ${OWN}.`,
      args: saving,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const q = Number(args.q);
        const r = Number(args.r);
        const text = String(args.name).trim();
        const places = state.places[me] ?? {};
        if (!inside(q, r, state.radius))
          return ctx.refuse(`(${q}, ${r}) is off the map; q + r stays within ${state.radius} of the centre`);
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        if (text in places) return ctx.refuse(`you saved a place called ${text} already; forget it first`);
        if (Object.keys(places).length >= OWN) return ctx.refuse(`you saved ${OWN} places; forget one first`);
        return { ...state, day: dayOf(ctx.at), places: { ...state.places, [me]: { ...places, [text]: { q, r } } } };
      },
    },
    forget: {
      line: "Forget a saved place of yours by its name.",
      args: forgetting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.name).trim();
        const places = { ...(state.places[me] ?? {}) };
        if (!(text in places)) return ctx.refuse(`you saved no place called ${text}`);
        delete places[text];
        return { ...state, day: dayOf(ctx.at), places: { ...state.places, [me]: places } };
      },
    },
  },
  moves: (state) =>
    Object.values(state.places).some((one) => Object.keys(one).length) ? ["save", "forget"] : ["save"],
  read: (state, path, who) => (path[0] === "at" ? at(state, path) : sliced(mine)(state, path, who)),
  bot,
});
