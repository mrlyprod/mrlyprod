import type { Knob, Knobs, Schema } from "@mrlyprod/os/contract/index.ts";
import { app } from "@mrlyprod/os/contract/index.ts";
import type { Base, Shared } from "../docs/share.ts";
import { SHARING, shape, sharing } from "../docs/share.ts";
import type { Cells, Grid } from "./formula.ts";
import { cellOf, checked, LETTERS, values } from "./formula.ts";

export type Sheet = Shared & { cells: Cells };

export type Sheets = Base & { grid: Grid; sheets: Record<string, Sheet> };

const KNOBS = {
  ...SHARING,
  cell: { line: "Characters one cell holds.", schema: { type: "integer", minimum: 10, maximum: 1000 }, reality: 100 },
  cols: { line: "Columns one sheet has.", schema: { type: "integer", minimum: 1, maximum: 26 }, reality: 8 },
  rows: { line: "Rows one sheet has.", schema: { type: "integer", minimum: 1, maximum: 200 }, reality: 20 },
} satisfies Knobs;

const SAID = 12;

const gridOf = (knob: Knob): Grid => ({ cols: LETTERS.slice(0, knob("cols")), rows: knob("rows") });

const span = (knob: Knob) => {
  const { cols, rows } = gridOf(knob);
  return `A1 to ${cols.at(-1)}${rows}`;
};

const put = (state: Sheets, id: string, sheet: Sheet | null): Sheets => {
  const sheets = { ...state.sheets };
  if (sheet) sheets[id] = sheet;
  else delete sheets[id];
  return { ...state, sheets };
};

const { pointer, writable, changed, verbs, read } = sharing<Sheets, Sheet>({
  id: "numbers",
  noun: "sheet",
  arg: "sheet",
  prefix: "sheet",
  many: "sheets",
  items: (state) => state.sheets,
  put,
  blank: () => ({ cells: {} }),
  brief: (one) => ({ cells: Object.keys(one.cells).length }),
  full: (state, _id, one) => ({ cells: one.cells, values: values(one.cells, state.grid) }),
});

export default app<Sheets>({
  id: "numbers",
  title: "Numbers",
  line: "Grids of exact numbers, labels and formulas with + - * / and SUM: private until shared with editors or readers, or made public. Read numbers/sheets/<id> for one in full.",
  boot: ({ actors, knob }) => ({
    people: actors,
    next: 1,
    listed: knob("listed"),
    grid: gridOf(knob),
    sheets: {},
  }),
  verbs: {
    ...verbs,
    set: {
      line: (knob) =>
        `Write a cell from ${span(knob)}: a number like 12 or 3.50, a label, or a formula like =A1*2+SUM(B1:B5); an empty text clears it. For a row or a range in one act, give rows instead of text: a list of rows, each a list of texts, filling right and down from col and row, all or nothing.`,
      args: (knob) => {
        const text: Schema = { type: "string", maxLength: knob("cell") };
        const grid = gridOf(knob);
        return shape(
          {
            sheet: pointer,
            col: { enum: [...grid.cols] },
            row: { type: "integer", minimum: 1, maximum: grid.rows },
            text,
            rows: {
              type: "array",
              minItems: 1,
              maxItems: grid.rows,
              items: { type: "array", minItems: 1, maxItems: grid.cols.length, items: text },
            },
          },
          ["text", "rows"],
        );
      },
      run: (state, args, ctx) => {
        const id = String(args.sheet);
        const was = writable(state, id, ctx);
        if ((args.text === undefined) === (args.rows === undefined))
          return ctx.refuse("give text for one cell, or rows for many; one of them");
        const { cols, rows: last } = state.grid;
        const left = cols.indexOf(String(args.col));
        const top = Number(args.row);
        const rows = (args.rows ?? [[args.text]]) as string[][];
        const cells = { ...was.cells };
        const touched: string[] = [];
        for (const [down, row] of rows.entries())
          for (const [right, raw] of row.entries()) {
            const col = cols[left + right];
            if (!col || top + down > last)
              return ctx.refuse(`those rows run past the sheet; it spans ${span(ctx.knob)}`);
            const cell = cellOf(col, top + down);
            const text = raw.trim();
            const note = checked(text, state.grid);
            if (note) return ctx.refuse(`${cell}: ${note}`);
            if (text) cells[cell] = text;
            else delete cells[cell];
            touched.push(cell);
          }
        const first = touched[0];
        if (!first) return ctx.refuse("those rows are empty; give at least one text");
        const final = touched.at(-1)!;
        const worked = values(cells, state.grid);
        ctx.say({
          note: `Set ${first === final ? first : `${first}:${final}`} on ${id}`,
          id,
          cells: touched.length,
          ...(touched.length <= SAID
            ? { values: Object.fromEntries(touched.filter((one) => one in worked).map((one) => [one, worked[one]!])) }
            : {}),
        });
        return put(state, id, { ...changed(id, was, ctx, "changed"), cells });
      },
    },
  },
  moves: (state) =>
    Object.keys(state.sheets).length
      ? ["create", "rename", "set", "share", "unshare", "publish", "delete"]
      : ["create"],
  read,
  knobs: KNOBS,
});
