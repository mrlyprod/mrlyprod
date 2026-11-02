import { expect, test } from "bun:test";
import type { Who } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import chat from "../messages/app.ts";
import help, { KERNEL } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "help", apps: ["messages", "help"], actors: ACTORS };

const K = kernel([chat, help]);

const as = (id: string): Who => ({ id, kind: "human" });

test("help has no verb: nothing to call, nothing a watcher could break, and the goose skips it", () => {
  const world = K.boot(CONFIG);
  expect(K.list(world).moves.filter((move) => move.verb.startsWith("help."))).toEqual([]);
  expect(K.list(world).apps.find((one) => one.id === "help")!.verbs).toEqual([]);
  expect(() => K.call(world, { verb: "help.open", args: {}, who: as("test-1") })).toThrow("unknown-verb");
  expect(digest(K.boot(CONFIG).apps.help)).toBe(digest(K.boot({ ...CONFIG, seed: 7 }).apps.help));
});

test("help reads every installed app's manual, one verb a line, and the kernel's own nav.open", () => {
  const world = K.boot(CONFIG);
  const book = K.read(world, "help").state as Record<string, { title: string; verbs: Record<string, string> }>;
  expect([book.messages!.title, Object.keys(book.messages!.verbs), book.nav]).toEqual([
    "Messages",
    ["messages.send", "messages.read"],
    KERNEL,
  ]);
  expect(K.read(world, "help/messages/verbs/messages.send").state).toContain("Write");
  expect(K.read(world, "help/nowhere").state).toBe(null);
});

test("10,000 seeded worlds of messages and help played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 4);
  expect([played.same, played.acts]).toEqual([10_000, 40_000]);
}, 30_000);
