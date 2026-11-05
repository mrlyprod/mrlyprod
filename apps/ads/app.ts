import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { pay } from "../wallet/app.ts";

export type Ad = { n: number; by: string; org: string; title: string; text: string; posted: number };

export type Ads = { ads: Record<string, Ad>; n: number };

const KNOBS = {
  cost: {
    line: "Coins an ad costs to post, paid to the ads account; 0 is free.",
    schema: { type: "integer", minimum: 0, maximum: 10000 },
    reality: 0,
  },
  live: {
    line: "Ads one person may have live.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 3,
  },
  title: {
    line: "Longest ad title, in characters.",
    schema: { type: "integer", minimum: 10, maximum: 200 },
    reality: 60,
  },
  text: {
    line: "Longest ad text, in characters.",
    schema: { type: "integer", minimum: 40, maximum: 2000 },
    reality: 280,
  },
} satisfies Knobs;

export const ORG = 40;

const SINK = "ads";

const TOP = 20;

const OPENING = ["post"];

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

function orgOf(ctx: Ctx, org: string): Org {
  const me = ctx.who.id;
  const orgs = Object.values((ctx.read("people/orgs") ?? {}) as Record<string, Org>);
  const named = orgs.filter((one) => one.dissolved === null && (one.name === org || String(one.n) === org));
  const mine = named.find((one) => Object.hasOwn(one.members, me));
  return mine ?? ctx.refuse(`you do not belong to an organisation called ${org}; people/orgs lists them`);
}

const listed = (state: Ads) => Object.values(state.ads);

const postedBy = (state: Ads, id: string) => listed(state).filter((one) => one.by === id);

const board = (state: Ads) => ({
  count: listed(state).length,
  latest: listed(state)
    .sort((a, b) => b.n - a.n)
    .slice(0, TOP),
});

const slice = (state: Ads, id: string, who: Who | null): Json =>
  who && who.id !== id ? { id, posted: postedBy(state, id) } : { ...board(state), posted: postedBy(state, id) };

const whole = (state: Ads, who: Who | null): Json => (who ? slice(state, who.id, who) : board(state));

const view = sliced(slice, whole, true);

const posting = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    org: { type: "string", minLength: 1, maxLength: ORG },
    title: { type: "string", minLength: 1, maxLength: knob("title") },
    text: { type: "string", minLength: 1, maxLength: knob("text") },
  },
  required: ["org", "title", "text"],
  additionalProperties: false,
});

const priced = (knob: Knob) => {
  const cost = knob("cost");
  return cost ? `, for ${cost} coins` : "";
};

export default app<Ads>({
  id: "ads",
  title: "Ads",
  line: "The one place ads live: an organisation's ad, posted by one of its people, seen only by who opens this app. Read ads for the latest, ads/ads/3 for one, ads/@bot-2 for someone's.",
  boot: () => ({ ads: {}, n: 0 }),
  knobs: KNOBS,
  verbs: {
    post: {
      line: (knob) =>
        `Post an ad for an organisation you belong to, by its name or number: a title of 1 to ${knob("title")} characters and a text of 1 to ${knob("text")}${priced(knob)}; up to ${knob("live")} live each.`,
      args: posting,
      run: (state, args, ctx) => {
        const [org, title, text] = [args.org, args.title, args.text].map((one) => String(one).trim());
        const most = ctx.knob("live");
        if (ctx.knob("cost")) pay(ctx, SINK, ctx.knob("cost"));
        if (!org || !title || !text) return ctx.refuse("an ad needs an organisation, a title and a text, not spaces");
        if (postedBy(state, ctx.who.id).length >= most) return ctx.refuse(`you have ${most} ads live; pull one first`);
        const { name } = orgOf(ctx, org!);
        const n = state.n + 1;
        ctx.say({ note: `Posted ad ${n} for ${name}`, id: n });
        return {
          n,
          ads: { ...state.ads, [n]: { n, by: ctx.who.id, org: name, title: title!, text: text!, posted: ctx.at } },
        };
      },
    },
    pull: {
      line: "Take down an ad you posted.",
      args: {
        type: "object",
        properties: { ad: { type: "integer", minimum: 1 } },
        required: ["ad"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const key = String(args.ad);
        const one = state.ads[key];
        if (!one) return ctx.refuse(`ad ${key} was already pulled`);
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by} pulls ad ${one.n}`);
        ctx.say({ note: `Pulled ad ${one.n}`, id: one.n });
        return { ...state, ads: Object.fromEntries(Object.entries(state.ads).filter(([n]) => n !== key)) };
      },
    },
  },
  moves: (state) => (state.n ? [...OPENING, "pull"] : OPENING),
  read: (state, path, who, at) => (path[0] === "ads" ? walk(state.ads, path.slice(1)) : view(state, path, who, at)),
});
