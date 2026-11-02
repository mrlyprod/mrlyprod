import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Ad = { n: number; by: string; org: string; title: string; text: string; posted: number };

export type Ads = { ads: Record<string, Ad>; n: number };

export const ORG = 40;

export const TITLE = 60;

export const TEXT = 280;

export const LIVE = 3;

/* ADS */

const postedBy = (state: Ads, id: string) => Object.values(state.ads).filter((one) => one.by === id);

/* APP */

export default app<Ads>({
  id: "ads",
  title: "Ads",
  line: "The one place ads live: an organisation's ad, posted by one of its people, seen only by who opens this app.",
  boot: () => ({ ads: {}, n: 0 }),
  verbs: {
    post: {
      line: `Post an ad for an organisation: a title of 1 to ${TITLE} characters and a text of 1 to ${TEXT}; up to ${LIVE} live each.`,
      args: {
        type: "object",
        properties: {
          org: { type: "string", minLength: 1, maxLength: ORG },
          title: { type: "string", minLength: 1, maxLength: TITLE },
          text: { type: "string", minLength: 1, maxLength: TEXT },
        },
        required: ["org", "title", "text"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const [org, title, text] = [args.org, args.title, args.text].map((one) => String(one).trim());
        if (!org || !title || !text) return ctx.refuse("an ad needs an organisation, a title and a text, not spaces");
        if (postedBy(state, ctx.who.id).length >= LIVE) return ctx.refuse(`you have ${LIVE} ads live; pull one first`);
        const n = state.n + 1;
        return {
          n,
          ads: { ...state.ads, [n]: { n, by: ctx.who.id, org: org!, title: title!, text: text!, posted: ctx.at } },
        };
      },
    },
    pull: {
      line: "Take down an ad you posted.",
      args: (state) => ({
        type: "object",
        properties: { ad: { type: "integer", minimum: 1, maximum: Math.max(1, state.n) } },
        required: ["ad"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const key = String(args.ad);
        const one = state.ads[key];
        if (!one) return ctx.refuse(`ad ${key} was already pulled`);
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by} pulls ad ${one.n}`);
        return { ...state, ads: Object.fromEntries(Object.entries(state.ads).filter(([n]) => n !== key)) };
      },
    },
  },
  moves: (state) => (state.n ? ["post", "pull"] : ["post"]),
  read: sliced((state, id) => postedBy(state, id)),
  bot,
});
