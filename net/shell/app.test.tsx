import { expect, mock, test } from "bun:test";
import type { Row } from "../sdk/rows.ts";
import type { Own } from "./router.ts";

mock.module("site:apps", () => ({
  apps: { settings: async () => "settings" },
  own: new Set(["screensavers", "settings"]),
}));

const { page } = await import("./app.tsx?page");

const row = (id: string, more: Row["meta"] = {}): Row => ({
  route: `/${id}/`,
  kind: "app",
  title: id,
  source: null,
  date: null,
  lead: `${id} line`,
  figure: id,
  hidden: false,
  meta: { id, ...more },
});

test("a scene app draws in the app window, any other island in a wide one, and an own app is its chunk", async () => {
  const scene = (page(row("life", { scene: true })) as Own).layout?.window;
  const island = (page(row("kit")) as Own).layout?.window;
  const settings: unknown = await page(row("settings"));
  expect([scene, island, settings]).toEqual(["app", "wide", "settings"]);
});
