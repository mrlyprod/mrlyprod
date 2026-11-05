import { COOKIE, gate, HEADER } from "./guard.js";

export type Secrets = { LOCK_KEY?: string; LOCK_SALT?: string; LOCK_HASH?: string };

export type Assets = { fetch: (request: Request) => Promise<Response> };

const bytes = new TextEncoder();

const hex = (data: ArrayBuffer) => [...new Uint8Array(data)].map((one) => one.toString(16).padStart(2, "0")).join("");

export async function hmac(key: string, text: string) {
  const sealed = await crypto.subtle.importKey("raw", bytes.encode(key), { name: "HMAC", hash: "SHA-256" }, false, [
    "sign",
  ]);
  return hex(await crypto.subtle.sign("HMAC", sealed, bytes.encode(text)));
}

export const sha256 = async (text: string) => hex(await crypto.subtle.digest("SHA-256", bytes.encode(text)));

const crumb = (header: string | null) =>
  (header ?? "")
    .split(/;\s*/)
    .find((one) => one.startsWith(`${COOKIE}=`))
    ?.slice(COOKIE.length + 1) ?? "";

export async function guarded(
  request: Request,
  env: Secrets & { ASSETS: Assets },
  next: (request: Request) => Promise<Response>,
): Promise<Response> {
  const url = new URL(request.url);
  const kit = { key: env.LOCK_KEY ?? "", salt: env.LOCK_SALT ?? "", hash: env.LOCK_HASH ?? "", hmac, sha256 };
  const ask = {
    uri: url.pathname,
    cookie: crumb(request.headers.get("cookie")),
    password: request.headers.get(HEADER) ?? "",
    now: Math.floor(Date.now() / 1000),
  };
  const shut = await gate(ask, kit);
  if (shut.pass) return next(request);
  if (!shut.status) {
    url.pathname = shut.uri.replace(/index\.html$/, "");
    return env.ASSETS.fetch(new Request(url, request));
  }
  const headers = new Headers({ "cache-control": "no-store" });
  if (shut.location) headers.set("location", shut.location);
  if (shut.cookie) headers.append("set-cookie", `${COOKIE}=${shut.cookie.value}; ${shut.cookie.attributes}`);
  return new Response(null, { status: shut.status, headers });
}
