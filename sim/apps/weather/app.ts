import type { Json, Knob, Knobs, Peek, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream } from "@mrlyprod/os/contract/index.ts";

export type Sky = "clear" | "cloudy" | "rain" | "storm" | "snow" | "fog";

export type Report = { q: number; r: number; day: number; sky: Sky; high: number; low: number; wind: number };

export type Saved = { q: number; r: number };

export type Skies = { climate: string; seasons: boolean; poles: number; gusts: number; ahead: number };

export type Weather = {
  seed: number;
  radius: number;
  skies: Skies;
  places: Record<string, Record<string, Saved>>;
};

const KNOBS = {
  climate: {
    line: "The climate of the whole world.",
    schema: { enum: ["temperate", "tropical", "arctic", "desert", "rainy"] },
    reality: "temperate",
  },
  seasons: {
    line: "Whether the year turns from cold to hot and back.",
    schema: { type: "boolean" },
    reality: true,
  },
  poles: {
    line: "Degrees colder at the map's far north and south than at its centre.",
    schema: { type: "integer", minimum: 0, maximum: 60 },
    reality: 8,
  },
  gusts: {
    line: "The strongest everyday wind, before a storm adds to it.",
    schema: { type: "integer", minimum: 1, maximum: 200 },
    reality: 40,
  },
  ahead: {
    line: "Days ahead a forecast shows.",
    schema: { type: "integer", minimum: 0, maximum: 30 },
    reality: 3,
  },
  saves: {
    line: "Places one person saves.",
    schema: { type: "integer", minimum: 1, maximum: 200 },
    reality: 10,
  },
  name: {
    line: "Characters in a saved place's name.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 40,
  },
} satisfies Knobs;

export const DAY = 86_400_000;

const HOUR = 3_600_000;

const RADIUS = 8;

const COOLEST = 5;

const WARMING = 10;

const AVERAGE = 13;

const SEASON = [2, 4, 8, 12, 17, 21, 24, 23, 19, 13, 7, 3];

const CLIMATES: Record<string, { warm: number; odds: number[] }> = {
  temperate: { warm: 0, odds: [35, 60, 78, 85] },
  tropical: { warm: 14, odds: [30, 55, 80, 92] },
  arctic: { warm: -22, odds: [40, 70, 85, 90] },
  desert: { warm: 18, odds: [80, 92, 96, 98] },
  rainy: { warm: 2, odds: [12, 35, 75, 90] },
};

const SKIES: Sky[] = ["clear", "cloudy", "rain", "storm"];

export const inside = (q: number, r: number, radius: number) =>
  Math.abs(q) <= radius && Math.abs(r) <= radius && Math.abs(q + r) <= radius;

export const dayOf = (at: number) => Math.floor(at / DAY);

export function report(state: Weather, q: number, r: number, day: number): Report {
  const { climate, seasons, poles, gusts } = state.skies;
  const { warm, odds } = CLIMATES[climate]!;
  const mixed =
    (state.seed ^ Math.imul(q + 1000, 73856093) ^ Math.imul(r + 1000, 19349663) ^ Math.imul(day + 1, 83492791)) >>> 0;
  const { draw } = stream(mixed);
  draw();
  draw();
  const season = seasons ? SEASON[Math.floor((((day % 360) + 360) % 360) / 30)]! : AVERAGE;
  const base = season + warm - Math.floor((Math.abs(r) * poles) / state.radius);
  const high = base + below(draw, 7) - 3;
  const low = high - 4 - below(draw, 6);
  const roll = below(draw, 100);
  const wet = SKIES[odds.findIndex((odd) => roll < odd)] ?? "fog";
  const sky: Sky = (wet === "rain" || wet === "storm") && high <= 2 ? "snow" : wet;
  const wind = below(draw, gusts) + (sky === "storm" ? 30 : 0);
  return { q, r, day, sky, high, low, wind };
}

const warmth = (one: Report, at: number) => {
  const since = (Math.floor(((at % DAY) + DAY) / HOUR) + 24 - COOLEST) % 24;
  const part = since <= WARMING ? since / WARMING : (24 - since) / (24 - WARMING);
  return Math.round(one.low + (one.high - one.low) * part);
};

const looking = (state: Weather, q: number, r: number, at: number) => {
  const day = dayOf(at);
  const today = report(state, q, r, day);
  return {
    q,
    r,
    day,
    sky: today.sky,
    temperature: warmth(today, at),
    high: today.high,
    low: today.low,
    wind: today.wind,
    ahead: Array.from({ length: state.skies.ahead }, (_, i) => {
      const one = report(state, q, r, day + 1 + i);
      return { day: one.day, sky: one.sky, high: one.high, low: one.low, wind: one.wind };
    }),
  };
};

const atCell = (state: Weather, path: string[], at: number): Json | undefined => {
  const q = Number(path[1]);
  const r = Number(path[2]);
  if (!Number.isInteger(q) || !Number.isInteger(r) || !inside(q, r, state.radius)) return undefined;
  if (path[3] === undefined) return looking(state, q, r, at);
  const day = Number(path[3]);
  return Number.isInteger(day) ? report(state, q, r, day) : undefined;
};

const mine = (state: Weather, id: string, _who: Who | null, at: number, peek: Peek) => {
  const here = peek(`maps/where/${id}`) as Saved | null;
  return {
    at,
    day: dayOf(at),
    here: here ? looking(state, here.q, here.r, at) : null,
    places: Object.fromEntries(
      Object.entries(state.places[id] ?? {}).map(([name, { q, r }]) => [name, looking(state, q, r, at)]),
    ),
  };
};

const open = (state: Weather, who: Who | null, at: number, peek: Peek): Json =>
  who ? mine(state, who.id, who, at, peek) : { at, day: dayOf(at), climate: state.skies.climate, radius: state.radius };

const looked = (state: Weather, path: string[], who: Who | null, at: number, peek: Peek): Json | undefined =>
  path[0] === "at" || path[0] === "cell" ? atCell(state, path, at) : sliced(mine, open)(state, path, who, at, peek);

const radiusOf = (knob: Knob) => knob<number | null>("maps.radius") ?? RADIUS;

const coordinate = (knob: Knob): Schema => ({ type: "integer", minimum: -radiusOf(knob), maximum: radiusOf(knob) });

const naming = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("name") });

const saving = (knob: Knob): Schema => ({
  type: "object",
  properties: { q: coordinate(knob), r: coordinate(knob), name: naming(knob) },
  required: ["q", "r", "name"],
  additionalProperties: false,
});

const forgetting = (knob: Knob): Schema => ({
  type: "object",
  properties: { name: naming(knob) },
  required: ["name"],
  additionalProperties: false,
});

export default app<Weather>({
  id: "weather",
  title: "Weather",
  line: "The sky over every map cell, live in world time: temperature, wind and a forecast for where you stand and the places you save. Any other cell: weather/at/q/r, or weather/at/q/r/day, where day is a day number as weather shows it.",
  boot: ({ seed, knob }) => ({
    seed,
    radius: radiusOf(knob),
    skies: {
      climate: knob("climate"),
      seasons: knob("seasons"),
      poles: knob("poles"),
      gusts: knob("gusts"),
      ahead: knob("ahead"),
    },
    places: {},
  }),
  knobs: KNOBS,
  verbs: {
    save: {
      line: (knob) =>
        `Save a map cell (q, r) under a name of 1 to ${knob("name")} characters to see its weather; you save at most ${knob("saves")}.`,
      args: saving,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const q = Number(args.q);
        const r = Number(args.r);
        const text = String(args.name).trim();
        const places = state.places[me] ?? {};
        const most = ctx.knob("saves");
        if (!inside(q, r, state.radius))
          return ctx.refuse(`(${q}, ${r}) is off the map; q, r and q + r stay within ${state.radius} of the centre`);
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        if (Object.hasOwn(places, text)) return ctx.refuse(`you saved a place called ${text} already; forget it first`);
        if (Object.keys(places).length >= most) return ctx.refuse(`you saved ${most} places; forget one first`);
        const { sky, temperature } = looking(state, q, r, ctx.at);
        ctx.say({ note: `Saved ${text} at (${q}, ${r}): ${sky}, ${temperature} degrees`, name: text, q, r });
        return { ...state, places: { ...state.places, [me]: { ...places, [text]: { q, r } } } };
      },
    },
    forget: {
      line: "Forget a saved place of yours by its name.",
      args: forgetting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.name).trim();
        const places = { ...(state.places[me] ?? {}) };
        if (!Object.hasOwn(places, text)) return ctx.refuse(`you saved no place called ${text}`);
        delete places[text];
        ctx.say({ note: `Forgot ${text}`, name: text });
        return { ...state, places: { ...state.places, [me]: places } };
      },
    },
  },
  moves: (state) =>
    Object.values(state.places).some((one) => Object.keys(one).length) ? ["save", "forget"] : ["save"],
  read: looked,
});
