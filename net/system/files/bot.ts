import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Files } from "./app.ts";

/* WORDS */

const FOLDERS = ["/Documents", "/Work", "/Recipes"];

const NAMES = ["todo.txt", "plan.md", "notes.txt", "budget.csv", "ideas.md", "letter.txt"];

const TEXTS = [
  "First draft.",
  "rent,900\nfood,300\ntravel,120",
  "- call the bank\n- book the dentist",
  "Lentil soup: lentils, onion, lemon.",
  "Plan for the quarter: ship, measure, repeat.",
];

/* BOT */

const bot: Bot<Files> = ({ state, me, draw }): Step => {
  const store = state.stores[me] ?? {};
  const paths = Object.keys(store);
  const folders = paths.filter((path) => store[path]!.kind === "folder");
  const files = paths.filter((path) => store[path]!.kind === "file");
  const missing = FOLDERS.filter((path) => !store[path]);
  if (missing.length && (!folders.length || !below(draw, 6)))
    return { verb: "folder", args: { path: pick(draw, missing) } };
  if (files.length > 15) return { verb: "delete", args: { path: pick(draw, files) } };
  const free = folders.flatMap((folder) => NAMES.map((name) => `${folder}/${name}`)).filter((path) => !store[path]);
  if (files.length && free.length && !below(draw, 8))
    return { verb: "move", args: { from: pick(draw, files), to: pick(draw, free) } };
  if (files.length && !below(draw, 3)) {
    const path = pick(draw, files);
    return { verb: "write", args: { path, text: `${store[path]!.text}\n${pick(draw, TEXTS)}`.slice(-1000) } };
  }
  if (!free.length || below(draw, 3) === 0) return null;
  return { verb: "write", args: { path: pick(draw, free), text: pick(draw, TEXTS) } };
};

export default bot;
