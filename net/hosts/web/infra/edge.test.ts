import { expect, test } from "bun:test";
import crypto from "node:crypto";
import { body, code, HEAD, LOCK, MOVED } from "./edge.ts";

const TABLE = { "/math/": "/mrlymath/", "/julia/": "/fractals/?kind=julia" };
const KEY = "k".repeat(64);
const SALT = "s".repeat(32);
const HASH = crypto.createHash("sha256").update(`${SALT}right horse`).digest("hex");
const STORE: Record<string, string> = { key: KEY, salt: SALT, hash: HASH };

type Answer = {
  uri?: string;
  querystring?: unknown;
  statusCode?: number;
  statusDescription?: string;
  headers: Record<string, { value: string }>;
  cookies?: Record<string, { value: string; attributes: string }>;
};

type Edge = {
  decide: (uri: string) => { uri?: string; redirect?: string };
  handler: (event: unknown) => Promise<Answer>;
};

const text = await Bun.file(new URL("./edge.js", import.meta.url)).text();

const reads: string[] = [];

const kvs = (store: Record<string, string>) => ({
  get: async (name: string) => {
    reads.push(name);
    if (!(name in store)) throw new Error("missing");
    return store[name];
  },
});

const load = (locked: boolean, store = STORE) =>
  new Function("crypto", "kvs", `${body(TABLE, locked)}; return { decide, handler }`)(crypto, kvs(store)) as Edge;

const edge = load(false);

const pass = (uri: string) => expect(edge.decide(uri)).toEqual({ uri });
const shell = (uri: string) => expect(edge.decide(uri)).toEqual({ uri: "/index.html" });

const event = (uri: string, query: Record<string, unknown> = {}, host = "example.com", more: object = {}) => ({
  request: { uri, querystring: query, headers: { host: { value: host } }, cookies: {}, ...more },
});

const ask = (uri: string, query: Record<string, unknown> = {}, host = "example.com") =>
  edge.handler(event(uri, query, host));

test("the file holds each mark once", () => {
  expect([text.split(MOVED).length, text.split(LOCK).length]).toEqual([2, 2]);
});

test("the root answers the shell", () => shell("/"));

test("a bare segment 301s to its slash form", () => {
  expect(edge.decide("/x")).toEqual({ redirect: "/x/" });
});

test("a slash form answers the shell", () => shell("/x/"));

test("the 301 keeps the query string", async () => {
  const out = await ask("/x", { seed: { value: "1" } });
  expect([out.statusCode, out.headers.location!.value]).toEqual([301, "/x/?seed=1"]);
});

test("a move that carries a query joins the asked query after it", async () => {
  expect([
    (await ask("/julia/")).headers.location!.value,
    (await ask("/julia/", { seed: { value: "1" } })).headers.location!.value,
  ]).toEqual(["/fractals/?kind=julia", "/fractals/?kind=julia&seed=1"]);
});

test("the 301 keeps a repeated key", async () => {
  const out = await ask("/x", { a: { value: "1", multiValue: [{ value: "1" }, { value: "2" }] } });
  expect(out.headers.location!.value).toBe("/x/?a=1&a=2");
});

test("a rewrite sets the request uri and leaves the query alone", async () => {
  const out = await ask("/x/", { seed: { value: "1" } });
  expect([out.uri, out.querystring]).toEqual(["/index.html", { seed: { value: "1" } }]);
});

test("raw files pass with or without a dot", () => {
  pass("/raw/LICENSE");
  pass("/raw/a.md");
});

test("a git path answers the shell", () => shell("/git/a/b.ts"));

test("the git root answers the shell", () => shell("/git/"));

test("git.json is a file", () => pass("/git.json"));

test("the shell itself passes", () => pass("/index.html"));

test("root text files pass", () => {
  pass("/robots.txt");
  pass("/sitemap-raw.xml");
  pass("/llms.txt");
});

test("a paper pdf passes", () => pass("/a/paper.pdf"));

test("an old page file passes to the S3 miss", () => pass("/x/index.html"));

test("a moved key 301s to its target", () => {
  expect(edge.decide("/math/")).toEqual({ redirect: "/mrlymath/" });
});

test("a moved key without the slash 301s to its target", () => {
  expect(edge.decide("/math")).toEqual({ redirect: "/mrlymath/" });
});

test("www 301s to the apex with the path and query", async () => {
  const out = await ask("/blog/", { a: { value: "b" } }, "www.example.com");
  expect([out.statusCode, out.statusDescription, out.headers.location!.value]).toEqual([
    301,
    "Moved Permanently",
    "https://example.com/blog/?a=b",
  ]);
});

/* LOCK */

const locked = load(true);

const knock = (password: string, store = STORE) =>
  load(true, store).handler(event("/_unlock", {}, "example.com", { headers: { "x-lock": { value: password } } }));

test("an open site never reads the store", async () => {
  reads.length = 0;
  await ask("/x/");
  expect(reads).toEqual([]);
});

test("a locked site shows the lock page without the cookie and the routed site with the door's cookie", async () => {
  const door = await knock(encodeURIComponent("right horse"));
  const cookies = { lock: { value: door.cookies!.lock!.value } };
  const shut = await locked.handler(event("/x/"));
  const open = await locked.handler(event("/x/", {}, "example.com", { cookies }));
  expect([shut.uri, open.uri]).toEqual(["/_lock/index.html", "/index.html"]);
});

test("the door answers 204, uncached, with an HttpOnly cookie through the cookies object", async () => {
  const door = await knock("right%20horse");
  expect([door.statusCode, door.headers["cache-control"]!.value, door.cookies!.lock!.attributes]).toEqual([
    204,
    "no-store",
    "Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=2592000",
  ]);
});

test("a store that fails leaves every door shut", async () => {
  const door = await knock("right%20horse", { salt: SALT, hash: HASH });
  expect([door.statusCode, door.cookies]).toEqual([401, {}]);
});

test("the locked function opens with the cloudfront imports and fits a function", () => {
  const made = code(TABLE, true);
  expect([made.startsWith(HEAD), made.length <= 10240]).toEqual([true, true]);
});
