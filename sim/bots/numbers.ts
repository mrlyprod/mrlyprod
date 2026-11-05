import type { Sheet, Sheets } from "@mrlyprod/apps/numbers/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const TITLES = ["Budget", "Pantry", "Hours", "Seeds"];

const LABELS = ["rent", "food", "travel", "total", "lentils", "onions", "hours"];

const NUMBERS = ["900", "300", "120", "3.50", "12", "0.25", "48"];

const FORMULAS = ["=A1+A2", "=SUM(A1:A5)", "=B2*2", "=(A1-A2)/4", "=SUM(A1:B3)-C1", "=-A3", "=B1/3"];

const owning = (sheet: Sheet, id: string, draw: () => number, others: string[]): Step => {
  const fresh = others.filter((one) => !sheet.editors.includes(one) && !sheet.readers.includes(one));
  const sharers = [...sheet.editors, ...sheet.readers];
  const turn = below(draw, 12);
  if (turn === 0 && fresh.length)
    return { verb: "share", args: { sheet: id, with: pick(draw, fresh), role: pick(draw, ["editor", "reader"]) } };
  if (turn === 1 && sharers.length > 1) return { verb: "unshare", args: { sheet: id, with: pick(draw, sharers) } };
  if (turn === 2 && Object.keys(sheet.cells).length) return { verb: "publish", args: { sheet: id, on: !sheet.open } };
  if (turn === 3) return { verb: "rename", args: { sheet: id, title: pick(draw, TITLES) } };
  return null;
};

const bot: Bot<Sheets> = ({ state, me, actors, draw }): Step => {
  const ids = Object.keys(state.sheets);
  const own = ids.filter((id) => state.sheets[id]!.owner === me);
  const editing = ids.filter((id) => state.sheets[id]!.editors.includes(me));
  const owners = Object.values(state.sheets).map((one) => one.owner);
  const others = [...new Set([...actors, ...owners])].filter((one) => one !== me);
  if (!own.length || below(draw, 10) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  if (own.length > 5 && below(draw, 4) === 0) return { verb: "delete", args: { sheet: pick(draw, own) } };
  const id = pick(draw, [...own, ...editing]);
  const sheet = state.sheets[id]!;
  const act = own.includes(id) ? owning(sheet, id, draw, others) : null;
  if (act) return act;
  const { cols, rows } = state.grid;
  const col = cols[below(draw, Math.min(3, cols.length))]!;
  const row = 1 + below(draw, Math.min(6, rows));
  if (below(draw, 5) === 0) {
    const cells = Array.from({ length: 1 + below(draw, 3) }, () => pick(draw, NUMBERS));
    return {
      verb: "set",
      args: { sheet: id, col: cols[0]!, row, rows: below(draw, 2) ? [cells] : cells.map((one) => [one]) },
    };
  }
  const kind = below(draw, 4);
  const text = kind === 0 ? pick(draw, LABELS) : kind === 3 ? pick(draw, FORMULAS) : pick(draw, NUMBERS);
  return { verb: "set", args: { sheet: id, col, row, text } };
};

export default bot;
