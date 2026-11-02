import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import type { Who } from "../../kernel/index.ts";
import { kernel } from "../../kernel/index.ts";
import type { Port } from "../../sdk/world.ts";
import { plug } from "../../sdk/world.ts";
import wallet from "../wallet/app.ts";
import { Give } from "./index.tsx";

const K = kernel([wallet]);

const CONFIG = {
  name: "give",
  apps: [{ id: "wallet", config: { causes: { commons: "The commons" } } }],
  actors: ["test-1", "test-2"],
  rules: { gifts: "off", grant: 500 },
};

function port(who: Who, actor: boolean): Port {
  let world = K.boot(CONFIG);
  world = K.call(world, {
    verb: "wallet.give",
    args: { to: "commons", amount: 40 },
    who: { id: "test-2", kind: "human" },
  }).world;
  return {
    subscribe: () => () => {},
    read: (path, shape) => K.read(world, path, shape),
    list: () => K.list(world),
    call: () => ({ ok: true, note: null, envelope: null }),
    who: () => who,
    actor: () => actor,
    at: () => world.at,
  };
}

test("give reads the wallet: the causes with what they raised, the public gifts, and the give form only for a test user", () => {
  const drawn = (who: Who, actor: boolean) => {
    plug(port(who, actor));
    const html = renderToStaticMarkup(<Give />);
    plug(null);
    return html;
  };
  const signed = drawn({ id: "test-1", kind: "human" }, true);
  const guest = drawn({ id: "guest", kind: "human" }, false);
  expect([
    signed.includes("Signed in as Test 1, holding 500</p>"),
    signed.includes("The commons"),
    signed.includes('<dt>raised</dt><dd><span class="leaf">40</span></dd>'),
    [...signed.matchAll(/<button[^>]*>(\w+)<\/button>/g)].map(([, verb]) => verb),
    guest.includes("Watching as Guest"),
    guest.includes("<button"),
  ]).toEqual([true, true, true, ["give"], true, false]);
});
