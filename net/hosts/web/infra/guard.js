export const COOKIE = "lock";
export const HEADER = "x-lock";
export const DOOR = "/_unlock";
export const HALL = "/_lock/";
export const PAGE = "/_lock/index.html";
export const OUT = "/_lock/out";
export const ROBOTS = "/_lock/robots.txt";
export const DAYS = 30;

const DAY = 86400;
const KEEP = "Path=/; Secure; HttpOnly; SameSite=Lax";

const answer = (pass, uri, status, location, value, age) => ({
  pass: pass,
  uri: uri,
  status: status,
  location: location,
  cookie: age < 0 ? null : { value: value, attributes: `${KEEP}; Max-Age=${age}` },
});

const show = (uri) => answer(false, uri, 0, "", "", -1);

const say = (status) => answer(false, "", status, "", "", -1);

export function needs(uri) {
  if (uri === DOOR) return ["key", "salt", "hash"];
  if (uri === "/robots.txt" || uri.startsWith(HALL)) return [];
  return ["key"];
}

export function same(one, two) {
  if (typeof one !== "string" || typeof two !== "string" || one.length !== two.length) return false;
  let diff = 0;
  for (let i = 0; i < one.length; i++) diff |= one.charCodeAt(i) ^ two.charCodeAt(i);
  return diff === 0;
}

export async function sign(expiry, key, hmac) {
  return `${expiry}.${await hmac(key, String(expiry))}`;
}

export async function valid(value, now, key, hmac) {
  const dot = value ? value.indexOf(".") : -1;
  if (!key || dot < 1) return false;
  const expiry = Number(value.slice(0, dot));
  if (!Number.isInteger(expiry) || expiry <= now) return false;
  return same(value, await sign(expiry, key, hmac));
}

export function spoken(text) {
  try {
    return typeof text === "string" ? decodeURIComponent(text) : "";
  } catch (_e) {
    return "";
  }
}

export async function gate(ask, kit) {
  const uri = ask.uri;
  if (uri === "/robots.txt") return show(ROBOTS);
  if (uri === OUT) return answer(false, "", 302, "/", "", 0);
  if (uri === HALL) return show(PAGE);
  if (uri.startsWith(HALL)) return show(uri);
  if (uri === DOOR) {
    const password = spoken(ask.password);
    if (!password || !kit.key || !kit.salt || !kit.hash) return say(401);
    if (!same(await kit.sha256(kit.salt + password), kit.hash)) return say(401);
    return answer(false, "", 204, "", await sign(ask.now + DAYS * DAY, kit.key, kit.hmac), DAYS * DAY);
  }
  if (await valid(ask.cookie, ask.now, kit.key, kit.hmac)) return answer(true, "", 0, "", "", -1);
  return show(PAGE);
}
