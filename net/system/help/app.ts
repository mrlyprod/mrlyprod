import type { Json, Manual } from "../../sdk/system.ts";
import { app, manuals, NAV, walk } from "../../sdk/system.ts";

/* TYPES */

export type Help = Record<string, never>;

export const KERNEL: Manual = {
  id: NAV,
  title: "Nav",
  line: "The kernel's own verb, in every world.",
  verbs: { [`${NAV}.open`]: "Bring an app to the front." },
};

/* THE BOOK */

export const book = (): Record<string, Manual> =>
  Object.fromEntries([...manuals(), KERNEL].map((one) => [one.id, one]));

/* APP */

export default app<Help>({
  id: "help",
  title: "Help",
  line: "Every installed app's manual: its title, its line and what each verb does; read-only.",
  boot: () => ({}),
  verbs: {},
  read: (_state, path) => walk(book() as Json, path),
});
