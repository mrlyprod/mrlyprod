import { expect, test } from "bun:test";
import { createHash, createHmac } from "node:crypto";
import { gate, needs, PAGE, sign } from "./guard.js";

const NOW = 1_800_000_000;
const KEY = "a".repeat(64);
const SALT = "b".repeat(32);

const hmac = (key: string, text: string) => createHmac("sha256", key).update(text).digest("hex");
const sha256 = (text: string) => createHash("sha256").update(text).digest("hex");

const KIT = { key: KEY, salt: SALT, hash: sha256(`${SALT}five plain words`), hmac, sha256 };

const at = (uri: string, cookie = "", password = "") => gate({ uri, cookie, password, now: NOW }, KIT);

const page = async (cookie: string) => (await at("/life/", cookie)).uri;

test("a good cookie passes to the router", async () => {
  expect((await at("/life/", await sign(NOW + 60, KEY, hmac))).pass).toBe(true);
});

test("an expired cookie shows the lock page", async () => {
  expect(await page(await sign(NOW, KEY, hmac))).toBe(PAGE);
});

test("a forged cookie shows the lock page", async () => {
  expect(await page(await sign(NOW + 60, "c".repeat(64), hmac))).toBe(PAGE);
});

test("no cookie shows the lock page", async () => {
  expect(await page("")).toBe(PAGE);
});

test("each open path answers without a cookie and reads no secret", async () => {
  const paths = ["/_lock/", "/_lock/page-1a2b3c4d.js", "/robots.txt"];
  const seen = await Promise.all(paths.map(async (uri) => [(await at(uri)).uri, needs(uri)]));
  expect(seen).toEqual([
    [PAGE, []],
    ["/_lock/page-1a2b3c4d.js", []],
    ["/_lock/robots.txt", []],
  ]);
});

test("a wrong, empty or garbled password answers 401 without a cookie", async () => {
  const said = await Promise.all(["wrong", "", "%E0%A4%A"].map((password) => at("/_unlock", "", password)));
  expect(said.map((one) => [one.status, one.cookie])).toEqual([
    [401, null],
    [401, null],
    [401, null],
  ]);
});

test("the way out expires the cookie and goes home", async () => {
  const out = await at("/_lock/out");
  expect([out.status, out.location, out.cookie?.value, out.cookie?.attributes.endsWith("Max-Age=0")]).toEqual([
    302,
    "/",
    "",
    true,
  ]);
});
