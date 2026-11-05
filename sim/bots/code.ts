import type { Code } from "@mrlyprod/apps/code/app.ts";
import { FILES } from "@mrlyprod/apps/code/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const NAMES = ["/main.ts", "/notes.md", "/todo.txt", "/plan.md"];

const LINES = [
  "const soup = ['lentils', 'onion'];",
  "export const hours = 48;",
  "- water the tomatoes",
  "# Plan",
  "ship, measure, repeat",
  "return soup.length;",
];

type Entry = { kind: "file" | "folder"; text: string };

const bot: Bot<Code> = ({ state, me, draw, world }): Step => {
  const files =
    (world.apps[FILES] as { stores?: Record<string, Record<string, Entry>> } | undefined)?.stores?.[me] ?? {};
  const mine = state.sessions[me];
  if (!mine?.open) {
    const have = Object.keys(files).filter((path) => files[path]!.kind === "file" && !path.slice(1).includes("/"));
    const fresh = NAMES.filter((path) => !files[path]);
    if (have.length && (!fresh.length || below(draw, 2) === 0))
      return { verb: "open", args: { path: pick(draw, have) } };
    if (fresh.length) return { verb: "new", args: { path: pick(draw, fresh) } };
    return null;
  }
  if (mine.dirty && (mine.buffer.length > 5 || below(draw, 3) === 0)) return { verb: "save", args: {} };
  if (!mine.dirty && below(draw, 4) === 0) return { verb: "close", args: {} };
  if (below(draw, 5) === 0) return { verb: "cursor", args: { line: below(draw, mine.buffer.length), col: 0 } };
  const line = below(draw, mine.buffer.length + 1);
  return { verb: "edit", args: { line, text: pick(draw, LINES) } };
};

export default bot;
