import { expect, test } from "bun:test";
import crypto from "node:crypto";
import { body } from "./edge.ts";
import { guarded } from "./worker.ts";

const SALT = "s".repeat(32);
const SECRETS = {
  LOCK_KEY: "k".repeat(64),
  LOCK_SALT: SALT,
  LOCK_HASH: crypto.createHash("sha256").update(`${SALT}open sesame`).digest("hex"),
};

const served: string[] = [];

const ASSETS = {
  fetch: async (request: Request) => {
    served.push(new URL(request.url).pathname);
    return new Response("page");
  },
};

const site = async () => new Response("site");

const visit = (path: string, headers: Record<string, string> = {}) =>
  guarded(new Request(`https://example.com${path}`, { headers }), { ...SECRETS, ASSETS }, site);

test("the worker serves the lock page from its assets to a visitor without the cookie", async () => {
  served.length = 0;
  const out = await visit("/life/");
  expect([await out.text(), served]).toEqual(["page", ["/_lock/"]]);
});

test("a cookie from the worker's door opens the cloudfront function", async () => {
  const door = await visit("/_unlock", { "x-lock": "open%20sesame" });
  const value = door.headers.get("set-cookie")!.split(";")[0]!.slice("lock=".length);
  const store = { get: async (name: string) => ({ key: SECRETS.LOCK_KEY })[name] };
  const edge = new Function("crypto", "kvs", `${body({}, true)}; return { handler }`)(crypto, store) as {
    handler: (event: unknown) => Promise<{ uri: string }>;
  };
  const request = {
    uri: "/life/",
    querystring: {},
    headers: { host: { value: "example.com" } },
    cookies: { lock: { value } },
  };
  expect([door.status, (await edge.handler({ request })).uri]).toEqual([204, "/index.html"]);
});
