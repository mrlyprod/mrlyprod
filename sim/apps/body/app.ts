import type { Held } from "@mrlyprod/apps/market/goods.ts";
import { NAME, placed, slugOf } from "@mrlyprod/apps/market/goods.ts";
import { among, blockedBy, notice, reach, someone } from "@mrlyprod/apps/shared/people.ts";
import type { Ctx, Json, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import type { Body, Levels, Need, Person } from "./needs.ts";
import { badOf, dialed, FULL, HOUR, KNOBS, LOW, levelsOf, personOf, spent, warnedOf, watched } from "./needs.ts";

type Spot = { q: number; r: number };

const ID = "body";

const ITEM = 120;

const PERSON = 40;

const POWER = 1000;

const VERY = 70;

const LITTLE = 20;

const foodOf = (one: Held) => Number(one.traits.food ?? 0);

const funOf = (one: Held) => Number(one.traits.fun ?? 0);

const rawFoodOf = (one: Held) => (one.traits.cooked === true ? 0 : foodOf(one));

const WARNINGS: Record<string, string> = {
  hunger: "You are very hungry: body.eat what you own, or market.buy food.",
  energy: "You are very tired: body.sleep.",
};

const hungerWords = (n: number) => {
  if (n >= FULL) return "You are starving.";
  if (n >= VERY) return "You are very hungry.";
  return n >= LOW ? "You are hungry." : "You are not hungry.";
};

const energyWords = (n: number) => {
  if (n <= 0) return "You are exhausted.";
  if (n <= LITTLE) return "You are very tired.";
  return n <= LOW ? "You are tired." : "You have energy.";
};

const funWords = (n: number) => {
  if (n <= 0) return "You are miserable.";
  if (n <= LITTLE) return "You are very bored.";
  return n <= LOW ? "You are bored." : "You are in good spirits.";
};

const warning = (state: Body, levels: Levels) => {
  if (!spent(levels) || state.dials.empty === "nothing") return [];
  return [
    state.dials.empty === "refuse" ? "You cannot go out until you eat and rest." : "Going out now, you would faint.",
  ];
};

const couldOf = (state: Body, levels: Levels) => {
  const needs = [
    ...(levels.hunger >= LOW ? ["Eat: body.eat what you own (market/@me), market.buy food, or body.cook a meal."] : []),
    ...(levels.energy <= LOW ? ["Rest: body.sleep."] : []),
    ...(levels.fun <= LOW ? ["Go out: body.play, worth more with people."] : []),
    ...warning(state, levels),
  ];
  return needs.length ? needs : ["Nothing is pressing: cook something, share a meal with body.feast, or go out."];
};

const numbers = ({ hunger, energy, fun }: Levels) => ({
  hunger: Math.round(hunger),
  energy: Math.round(energy),
  fun: Math.round(fun),
});

const mine = (state: Body, id: string, _who: Who | null, at: number) => {
  const levels = levelsOf(state, id, at);
  const { last, faints } = personOf(state, id, at);
  return {
    ...numbers(levels),
    feel: [hungerWords(levels.hunger), energyWords(levels.energy), funWords(levels.fun)].join(" "),
    could: couldOf(state, levels),
    last,
    faints,
  };
};

const everyone = (state: Body, at: number) =>
  Object.fromEntries(watched(state).map((id) => [id, numbers(levelsOf(state, id, at))]));

const whole = (state: Body, who: Who | null, at: number): Json =>
  who ? mine(state, who.id, who, at) : everyone(state, at);

function settled(state: Body, id: string, at: number): Person {
  const one = personOf(state, id, at);
  const levels = levelsOf(state, id, at);
  return {
    ...one,
    hunger: { level: levels.hunger, at },
    energy: { level: levels.energy, at },
    fun: { level: levels.fun, at },
  };
}

const kept = (state: Body, people: Record<string, Person>, at: number): Body => ({
  ...state,
  clock: state.clock ?? at,
  persons: { ...state.persons, ...people },
});

const fed = (one: Person, food: number, fun: number, at: number): Person => ({
  ...one,
  hunger: { level: Math.max(0, one.hunger.level - food), at },
  fun: { level: Math.min(FULL, one.fun.level + fun), at },
});

const tired = (one: Person, cost: number, at: number): Person => ({
  ...one,
  energy: { level: Math.max(0, one.energy.level - cost), at },
});

const pantry = (ctx: Ctx) => (ctx.read(`market/own/${ctx.who.id}`) ?? {}) as Record<string, Held>;

const inPantry = (ctx: Ctx, key: string) => {
  const mine = pantry(ctx);
  return Object.hasOwn(mine, key) ? mine[key]! : null;
};

const late = (state: Body, id: string, at: number) =>
  badOf(state, levelsOf(state, id, at)).join() !== warnedOf(state, id).join();

function foodOwned(ctx: Ctx, item: string, n: number): Held {
  if (ctx.read("market/own") === null) return ctx.refuse("this world has no market, so you own nothing to eat");
  const one = inPantry(ctx, slugOf(item));
  if (!one) return ctx.refuse(`you own no ${item}; buy food with market.buy, market/@me lists what you own`);
  if (one.n < n) return ctx.refuse(`you own ${one.n} ${one.name}, not ${n}; buy more with market.buy`);
  if (foodOf(one) <= 0) return ctx.refuse(`${one.name} is not food; market/@me lists what you own`);
  return one;
}

const tallied = (items: string[]) => {
  const counts = new Map<string, { item: string; n: number }>();
  for (const item of items) {
    const key = slugOf(item);
    counts.set(key, { item, n: (counts.get(key)?.n ?? 0) + 1 });
  }
  return [...counts.entries()];
};

function around(ctx: Ctx, me: string, other: string | null) {
  const where = (ctx.read("maps/where") ?? {}) as Record<string, Spot>;
  const here = Object.hasOwn(where, me) ? where[me] : undefined;
  const same = (id: string) => where[id]!.q === here!.q && where[id]!.r === here!.r;
  const near = here ? Object.keys(where).filter((id) => id !== me && same(id)) : [];
  const apart = !!(other && here && Object.hasOwn(where, other) && !same(other));
  return { apart, near: other && !apart && !near.includes(other) ? [...near, other] : near };
}

const fainted = (state: Body, one: Person, at: number): Person => ({
  ...one,
  energy: { level: state.dials.revive, at },
  fun: { level: 0, at },
  faints: one.faints + 1,
  last: "You fainted instead of going out, and woke with little energy and no fun. Eat and rest.",
});

const wentOut = (label: string, near: string[], other: string | null, apart: boolean) => {
  const company = near.length ? ` with ${near.length} ${near.length === 1 ? "other" : "others"}` : "";
  const missed = apart ? ` ${other} is elsewhere, so it did not count; meet them with maps.go or maps.invite.` : "";
  return `You went out: ${label}${company}.${missed}`;
};

const invited = (me: string, other: string, label: string, apart: boolean) =>
  notice(
    other,
    apart
      ? `${me} wanted ${label} with you, but you are in different places; meet with maps.go`
      : `${me} enjoyed ${label} with you`,
  );

const person: Schema = { type: "string", minLength: 1, maxLength: PERSON };

const itemName: Schema = { type: "string", minLength: 1, maxLength: ITEM };

export default app<Body>({
  id: ID,
  title: "Body",
  line: "Your needs and daily life: hunger rises toward 100, energy and fun fall toward 0, with world time. Eat, cook, sleep and go out.",
  knobs: KNOBS,
  boot: ({ seed, start, knob, actors }) => ({
    seed,
    clock: start || null,
    people: actors,
    dials: dialed(knob),
    persons: {},
    warned: {},
  }),
  verbs: {
    eat: {
      line: "Eat one of something you own: its food lowers your hunger and its fun lifts you.",
      args: {
        type: "object",
        properties: { item: itemName },
        required: ["item"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = foodOwned(ctx, String(args.item), 1);
        const now = settled(state, me, ctx.at);
        if (now.hunger.level <= 0) return ctx.refuse("you are full; keep it for later");
        ctx.call("market.use", { item: slugOf(one.name), n: 1 });
        const after = fed(now, foodOf(one), funOf(one), ctx.at);
        ctx.say({
          note: `You ate ${one.name}; hunger ${Math.round(after.hunger.level)}`,
          id: slugOf(one.name),
          hunger: Math.round(after.hunger.level),
        });
        return kept(state, { [me]: { ...after, last: `You ate ${one.name}.` } }, ctx.at);
      },
    },
    cook: {
      line: (knob) =>
        `Cook 2 to ${knob("recipe")} ingredients you own into a named meal worth ${knob("meal")}% of their raw food. servings splits the pot into up to ${knob("servings")} equal portions. Eat with body.eat, share with market.give, sell with market.sell. Name an ingredient twice to use two.`,
      args: (knob) => ({
        type: "object",
        properties: {
          items: { type: "array", items: itemName, minItems: 2, maxItems: knob("recipe") },
          name: { type: "string", minLength: 1, maxLength: NAME },
          servings: { type: "integer", minimum: 1, maximum: knob("servings") },
        },
        required: ["items", "name"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.name).trim();
        if (!slugOf(name)) return ctx.refuse("a meal's name needs letters or digits");
        const parts = tallied((args.items as string[]).map(String)).map(([key, { item, n }]) => ({
          key,
          n,
          one: foodOwned(ctx, item, n),
        }));
        const servings = Number(args.servings ?? 1);
        const sum = (of: (one: Held) => number) => parts.reduce((total, { n, one }) => total + of(one) * n, 0);
        const raw = sum(rawFoodOf);
        const pot = Math.min(POWER, Math.round((raw * state.dials.meal) / 100) + sum(foodOf) - raw);
        const food = Math.floor(pot / servings);
        const fun = Math.floor(Math.min(POWER, sum(funOf) + (raw ? state.dials.relish : 0)) / servings);
        if (food < 1) return ctx.refuse(`${pot} food will not split into ${servings} servings; make fewer`);
        const traits = { food, fun, lasts: false, cooked: true };
        const left = { ...pantry(ctx) };
        for (const { key, n, one } of parts) if (one.n <= n) delete left[key];
        const { key: id } = placed(left, { name, traits });
        for (const { key, n } of parts) ctx.call("market.use", { item: key, n });
        ctx.call("market.make", { name, traits, n: servings });
        const now = tired(settled(state, me, ctx.at), state.dials.cooking, ctx.at);
        const made = servings > 1 ? `${servings} portions of ${name}` : name;
        const last = `You cooked ${made}: food ${food}, fun ${fun}${servings > 1 ? " each" : ""}.`;
        ctx.say({ note: `${last} The item id is ${id}.`, id, n: servings, food, fun });
        return kept(state, { [me]: { ...now, last } }, ctx.at);
      },
    },
    sleep: {
      line: (knob) =>
        `Nap up to ${knob("night")} hours in your own time: each restores ${knob("rest")} energy at once and adds the hunger those hours would; the world's clock does not wait for you. Everyone sleeps through the night anyway, at dawn.`,
      args: (knob) => ({
        type: "object",
        properties: { hours: { type: "integer", minimum: 1, maximum: knob("night") } },
        required: ["hours"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const hours = Number(args.hours);
        const now = settled(state, me, ctx.at);
        const { rest, hunger, asleep } = state.dials;
        if (now.energy.level >= FULL) return ctx.refuse("you are wide awake; eat, cook or go out instead");
        const one: Person = {
          ...now,
          energy: { level: Math.min(FULL, now.energy.level + hours * rest), at: ctx.at },
          hunger: { level: Math.min(FULL, now.hunger.level + (hours * hunger * asleep) / 100), at: ctx.at },
          last: `You slept ${hours} ${hours === 1 ? "hour" : "hours"}.`,
        };
        ctx.say({
          note: `You slept ${hours} ${hours === 1 ? "hour" : "hours"} in your own time; the world moved on at its own pace. Energy ${Math.round(one.energy.level)}, hunger ${Math.round(one.hunger.level)}.`,
          energy: Math.round(one.energy.level),
          hunger: Math.round(one.hunger.level),
        });
        return kept(state, { [me]: one }, ctx.at);
      },
    },
    play: {
      line: (knob) =>
        `Go out and have fun: say what you are doing, or name something you own to use it. Worth ${knob("play")} fun alone and ${knob("together")} more per person at your place or with you; with tells someone you played with them, and lifts them too.`,
      args: (_knob) => ({
        type: "object",
        properties: { what: { type: "string", minLength: 1, maxLength: NAME }, with: person },
        required: ["what"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const what = String(args.what).trim();
        const other = typeof args.with === "string" ? someone(state.people, args.with, ctx) : null;
        const { dials } = state;
        if (!what) return ctx.refuse("say what you are doing: a walk, chess, a film");
        if (other === me) return ctx.refuse("name someone else, or leave with out to go alone");
        if (other) reach(ctx, other);
        const now = settled(state, me, ctx.at);
        if (spent(levelsOf(state, me, ctx.at))) {
          if (dials.empty === "refuse")
            return ctx.refuse("you are too hungry or too tired to go out; eat or sleep first");
          if (dials.empty === "faint") {
            const out = fainted(state, now, ctx.at);
            ctx.say({ note: out.last! });
            return kept(state, { [me]: out }, ctx.at);
          }
        }
        const { apart, near } = around(ctx, me, other);
        const key = slugOf(what);
        const thing = inPantry(ctx, key);
        const using = thing && funOf(thing) > 0 ? thing : null;
        if (using && using.traits.lasts !== true) ctx.call("market.use", { item: key, n: 1 });
        const gain = dials.play + dials.together * Math.min(dials.crowd, near.length) + (using ? funOf(using) : 0);
        const after = tired(fed(now, 0, gain, ctx.at), dials.effort, ctx.at);
        const label = using?.name ?? what;
        const out = wentOut(label, near, other, apart);
        const people: Record<string, Person> = { [me]: { ...after, last: out } };
        ctx.say({ note: `${out} Fun ${Math.round(after.fun.level)}.`, fun: Math.round(after.fun.level) });
        if (other) ctx.wish(invited(me, other, label, apart));
        if (other && !apart)
          people[other] = {
            ...fed(settled(state, other, ctx.at), 0, dials.together, ctx.at),
            last: `${me} enjoyed ${label} with you.`,
          };
        return kept(state, people, ctx.at);
      },
    },
    feast: {
      line: (knob) =>
        `Share a meal you own with everyone at your place: its food and fun split among up to ${knob("table")} people, you included, and each gains ${knob("feast")} more fun for the company.`,
      args: {
        type: "object",
        properties: { item: itemName },
        required: ["item"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = foodOwned(ctx, String(args.item), 1);
        const { dials } = state;
        const guests = around(ctx, me, null)
          .near.filter((id) => !blockedBy(ctx, id, me) && !blockedBy(ctx, me, id))
          .slice(0, dials.table - 1);
        if (!guests.length)
          return ctx.refuse("nobody is at your place to share with; meet people with maps.go, or body.eat it alone");
        ctx.call("market.use", { item: slugOf(one.name), n: 1 });
        const diners = [me, ...guests];
        const food = Math.floor(foodOf(one) / diners.length);
        const fun = Math.floor(funOf(one) / diners.length) + dials.feast;
        const people: Record<string, Person> = {};
        for (const id of diners)
          people[id] = {
            ...fed(settled(state, id, ctx.at), food, fun, ctx.at),
            last: id === me ? `You shared ${one.name} with ${among(guests)}.` : `${me} shared ${one.name} with you.`,
          };
        for (const id of guests) ctx.wish(notice(id, `${me} shared ${one.name} with you: food ${food}, fun ${fun}`));
        ctx.say({ note: `You shared ${one.name} with ${among(guests)}: food ${food}, fun ${fun} each.`, fed: guests });
        return kept(state, people, ctx.at);
      },
    },
    night: {
      inner: true,
      line: "Everyone sleeps through the night; it happens by itself at dawn.",
      run: (state, _args, ctx) => {
        const { night, rest, energy, hunger, asleep, baseline } = state.dials;
        const within = (n: number, high: number) => Math.min(high, Math.max(0, n));
        const hoursOf = (need: Need) => Math.max(0, ctx.at - need.at) / HOUR;
        const sleptOf = (need: Need) => Math.min(night, hoursOf(need));
        const people: Record<string, Person> = {};
        for (const id of watched(state)) {
          const was = personOf(state, id, ctx.at);
          const up = (need: Need) => hoursOf(need) - sleptOf(need);
          const hungry = within(was.hunger.level + up(was.hunger) * hunger, baseline);
          const tired = within(was.energy.level - up(was.energy) * energy, FULL);
          const slept = sleptOf(was.energy);
          people[id] = {
            ...settled(state, id, ctx.at),
            energy: { level: within(tired + slept * rest, FULL), at: ctx.at },
            hunger: {
              level: within(hungry + (sleptOf(was.hunger) * hunger * asleep) / FULL, baseline),
              at: ctx.at,
            },
            last: `You slept ${Math.round(slept)} hours through the night.`,
          };
        }
        return kept(state, people, ctx.at);
      },
    },
    tick: {
      inner: true,
      line: "Tell everyone whose hunger or energy just turned bad; it happens by itself whenever anyone acts.",
      run: (state, _args, ctx) => {
        const warned = { ...state.warned };
        for (const id of watched(state)) {
          const bad = badOf(state, levelsOf(state, id, ctx.at));
          const had = warnedOf(state, id);
          for (const need of bad) if (!had.includes(need)) ctx.wish(notice(id, WARNINGS[need]!, true));
          if (bad.length) warned[id] = bad;
          else delete warned[id];
        }
        return { ...state, warned };
      },
    },
  },
  read: sliced(mine, whole),
  wake: (state, at) => (watched(state).some((id) => late(state, id, at)) ? "tick" : null),
  after: ({ line, envelope }) => (line.verb === "dawn.rise" && envelope.last.ok ? { verb: `${ID}.night` } : null),
});
