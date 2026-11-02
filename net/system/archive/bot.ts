import type { Bot, Json, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Archive } from "./app.ts";
import { KEEP, SIZE } from "./app.ts";

/* POLICY */

const keys = (value: Json | undefined) =>
  value && typeof value === "object" && !Array.isArray(value) ? Object.keys(value) : [];

export const bot: Bot<Archive> = ({ state, me, draw, world }): Step => {
  const mine = Object.values(state.pages).filter((one) => one.by === me);
  if (mine.length >= KEEP - 2) return { verb: "delete", args: { id: mine[0]!.id } };
  const apps = Object.keys(world.apps).filter((id) => id !== "archive");
  if (!apps.length || below(draw, 3)) return null;
  let path = pick(draw, apps);
  let page: Json | undefined = world.apps[path];
  for (let depth = 0; depth < 3; depth++) {
    const inside = keys(page);
    if (!inside.length || (JSON.stringify(page).length <= SIZE && below(draw, 2))) break;
    const key = pick(draw, inside);
    path = `${path}/${key}`;
    page = (page as Record<string, Json>)[key];
  }
  return page !== undefined && JSON.stringify(page).length <= SIZE ? { verb: "save", args: { path } } : null;
};
