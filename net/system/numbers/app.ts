import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";
import type { Cells } from "./formula.ts";
import { COLS, cellOf, checked, ROWS, USAGE, values } from "./formula.ts";

/* TYPES */

export type Sheet = { owner: string; title: string; editors: string[]; cells: Cells; at: number };

export type Sheets = { people: string[] | null; next: number; sheets: Record<string, Sheet> };

export const TITLE = 100;

export const CELL = 100;

export const SHEETS = 50;

const ID = 40;

export { COLS, ROWS, USAGE };

/* SHEETS */

const held = (state: Sheets, id: string, ctx: Ctx, own: boolean): Sheet => {
  const sheet = state.sheets[id]!;
  const me = ctx.who.id;
  if (sheet.owner === me) return sheet;
  if (!own && sheet.editors.includes(me)) return sheet;
  return ctx.refuse(
    own ? `${id} is ${sheet.owner}'s; only its owner may` : `${id} is ${sheet.owner}'s; ask to be an editor`,
  );
};

const put = (state: Sheets, id: string, sheet: Sheet): Sheets => ({
  ...state,
  sheets: { ...state.sheets, [id]: sheet },
});

const shownSheet = (sheet: Sheet) => ({ ...sheet, values: values(sheet.cells) });

const mine = (state: Sheets, me: string) => ({
  sheets: Object.fromEntries(
    Object.entries(state.sheets)
      .filter(([, sheet]) => sheet.owner === me || sheet.editors.includes(me))
      .map(([id, sheet]) => [id, shownSheet(sheet)]),
  ),
});

const whole = (state: Sheets) => ({
  ...state,
  sheets: Object.fromEntries(Object.entries(state.sheets).map(([id, sheet]) => [id, shownSheet(sheet)])),
});

/* SCHEMAS */

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const title: Schema = { type: "string", minLength: 1, maxLength: TITLE };

const sheet = (state: Sheets): Schema => ({ enum: Object.keys(state.sheets) });

const person = (state: Sheets): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

/* APP */

export default app<Sheets>({
  id: "numbers",
  title: "Numbers",
  line: `Grids of ${COLS[0]}1 to ${COLS.at(-1)}${ROWS}: exact numbers, labels and formulas with + - * / and SUM, shared with editors.`,
  boot: ({ actors }) => ({ people: actors, next: 1, sheets: {} }),
  verbs: {
    create: {
      line: `Start a sheet of yours, titled 1 to ${TITLE} characters; you hold at most ${SHEETS}.`,
      args: shape({ title }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(state.sheets).filter((one) => one.owner === me).length >= SHEETS)
          return ctx.refuse(`you hold ${SHEETS} sheets already; delete one first`);
        const id = `sheet-${state.next}`;
        return put({ ...state, next: state.next + 1 }, id, {
          owner: me,
          title: name,
          editors: [],
          cells: {},
          at: ctx.at,
        });
      },
    },
    set: {
      line: `Write a cell: a number like 12 or 3.50, a label, or a formula like =A1*2+SUM(B1:B5); an empty text clears it.`,
      args: (state) =>
        shape({
          sheet: sheet(state),
          col: { enum: [...COLS] },
          row: { type: "integer", minimum: 1, maximum: ROWS },
          text: { type: "string", maxLength: CELL },
        }),
      run: (state, args, ctx) => {
        const id = String(args.sheet);
        const was = held(state, id, ctx, false);
        const cell = cellOf(String(args.col), Number(args.row));
        const text = String(args.text).trim();
        const note = checked(text);
        if (note) return ctx.refuse(note === USAGE ? USAGE : note);
        const { [cell]: _, ...rest } = was.cells;
        return put(state, id, { ...was, cells: text ? { ...rest, [cell]: text } : rest, at: ctx.at });
      },
    },
    share: {
      line: "Let someone edit a sheet of yours.",
      args: (state) => shape({ sheet: sheet(state), with: person(state) }),
      run: (state, args, ctx) => {
        const id = String(args.sheet);
        const was = held(state, id, ctx, true);
        const who = String(args.with);
        if (who === ctx.who.id) return ctx.refuse("you own it already; share with someone else");
        if (was.editors.includes(who)) return ctx.refuse(`${who} edits ${id} already`);
        return put(state, id, { ...was, editors: [...was.editors, who] });
      },
    },
    delete: {
      line: "Delete a sheet of yours.",
      args: (state) => shape({ sheet: sheet(state) }),
      run: (state, args, ctx) => {
        const id = String(args.sheet);
        held(state, id, ctx, true);
        const { [id]: _, ...rest } = state.sheets;
        return { ...state, sheets: rest };
      },
    },
  },
  moves: (state) => (Object.keys(state.sheets).length ? ["create", "set", "share", "delete"] : ["create"]),
  read: sliced(mine, whole),
  bot,
});
