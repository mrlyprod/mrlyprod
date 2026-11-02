import type { Kind, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Card = { handle: string; kind: Kind; created: number };

export type Iden = { cards: Record<string, Card> };

export const HANDLE = 20;

export const HANDLES = /^[a-z][a-z0-9-]*$/;

/* SCHEMAS */

const naming: Schema = {
  type: "object",
  properties: { handle: { type: "string", minLength: 3, maxLength: HANDLE } },
  required: ["handle"],
  additionalProperties: false,
};

/* APP */

export default app<Iden>({
  id: "identity",
  title: "Identity",
  line: "Each user's identity card: a handle, a kind and the tick it was created; no passwords yet.",
  boot: ({ actors }) => ({
    cards: Object.fromEntries((actors ?? []).map((id) => [id, { handle: id, kind: "human" as Kind, created: 0 }])),
  }),
  verbs: {
    handle: {
      line: `Choose your handle: 3 to ${HANDLE} lowercase letters, digits and hyphens, one nobody else holds.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const handle = String(args.handle);
        if (!HANDLES.test(handle))
          return ctx.refuse(
            `a handle starts with a letter and holds lowercase letters, digits and hyphens, not ${handle}`,
          );
        const held = Object.keys(state.cards).find((id) => id !== me && state.cards[id]!.handle === handle);
        if (held) return ctx.refuse(`${held} holds the handle ${handle}; pick another`);
        const card = state.cards[me] ?? { handle: me, kind: ctx.who.kind, created: ctx.tick };
        return { cards: { ...state.cards, [me]: { ...card, handle } } };
      },
    },
  },
  read: sliced((state: Iden, me: string) => state.cards[me]),
  bot,
});
