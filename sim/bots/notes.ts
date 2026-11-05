import type { Notes } from "@mrlyprod/apps/notes/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const NOTES = [
  "Groceries: milk, eggs, bread",
  "Call the plumber on Monday",
  "Ideas for the weekend: a walk, the museum",
  "Meeting notes: budget approved, next review in May",
  "Books to read: two novels and a history",
  "Gift ideas for the birthday",
  "Passwords go in a password manager, not here",
  "Recipe: lentil soup with lemon",
];

const LINES = ["Done.", "Moved to next week.", "Ask about the price.", "Check the address."];

const bot: Bot<Notes> = ({ state, me, draw }): Step => {
  const notes = state.notes[me] ?? [];
  if (notes.length >= 10) return { verb: "delete", args: { id: notes[0]!.id } };
  if (notes.length && !below(draw, 3)) {
    const note = pick(draw, notes);
    return { verb: "edit", args: { id: note.id, text: `${note.text}\n${pick(draw, LINES)}`.slice(-1000) } };
  }
  if (notes.length && !below(draw, 8)) {
    const note = pick(draw, notes);
    return { verb: "mark", args: { id: note.id, private: !note.private } };
  }
  if (below(draw, 2)) return null;
  return {
    verb: "write",
    args: below(draw, 6) ? { text: pick(draw, NOTES) } : { text: pick(draw, NOTES), private: true },
  };
};

export default bot;
