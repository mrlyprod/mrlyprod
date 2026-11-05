import type { Files } from "@mrlyprod/apps/files/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const FOLDERS = ["/Documents", "/Work", "/Recipes"];

const NAMES = ["todo.txt", "plan.md", "notes.txt", "budget.csv", "ideas.md", "letter.txt"];

const TEXTS = [
  "First draft.",
  "rent,900\nfood,300\ntravel,120",
  "- call the bank\n- book the dentist",
  "Lentil soup: lentils, onion, lemon.",
  "Plan for the quarter: ship, measure, repeat.",
];

const bot: Bot<Files> = ({ state, me, actors, draw }): Step => {
  const store = state.stores[me] ?? {};
  const paths = Object.keys(store);
  const folders = paths.filter((path) => store[path]!.kind === "folder");
  const files = paths.filter((path) => store[path]!.kind === "file");
  const missing = FOLDERS.filter((path) => !store[path]);
  if (missing.length && (!folders.length || !below(draw, 6)))
    return { verb: "folder", args: { path: pick(draw, missing) } };
  if (files.length > 15) return { verb: "delete", args: { path: pick(draw, files) } };
  const others = [...new Set([...actors, ...Object.keys(state.stores)])].filter((one) => one !== me);
  if (files.length && others.length && !below(draw, 8)) {
    const path = pick(draw, files);
    const held = store[path]!.with ?? [];
    const fresh = others.filter((one) => !held.includes(one));
    if (held.length && !below(draw, 3)) return { verb: "unshare", args: { path, with: pick(draw, held) } };
    if (fresh.length) return { verb: "share", args: { path, with: pick(draw, fresh) } };
  }
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
