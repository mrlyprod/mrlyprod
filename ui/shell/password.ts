import { PREFIX, read, root, write } from "./prefs.ts";

const ROUNDS = 600_000;
const SALT = 16;
const BITS = 256;
const OPEN = `${PREFIX}unlocked`;

export const STORE = `${PREFIX}password`;

const hex = (data: ArrayBuffer | Uint8Array) =>
  [...new Uint8Array(data)].map((one) => one.toString(16).padStart(2, "0")).join("");

const unhex = (text: string) => Uint8Array.from(text.match(/../g) ?? [], (one) => Number.parseInt(one, 16));

async function stretch(password: string, salt: Uint8Array<ArrayBuffer>) {
  const key = await crypto.subtle.importKey("raw", new TextEncoder().encode(password), "PBKDF2", false, ["deriveBits"]);
  return hex(await crypto.subtle.deriveBits({ name: "PBKDF2", hash: "SHA-256", salt, iterations: ROUNDS }, key, BITS));
}

const remember = (on: boolean) => {
  try {
    if (on) sessionStorage.setItem(OPEN, "1");
    else sessionStorage.removeItem(OPEN);
  } catch {}
};

const remembered = () => {
  try {
    return sessionStorage.getItem(OPEN) === "1";
  } catch {
    return false;
  }
};

export const held = () => Boolean(read(STORE));

export const sealed = () => held() && !remembered();

export function admit() {
  remember(true);
  delete root().dataset.sealed;
}

export function seal() {
  remember(false);
  root().dataset.sealed = "";
}

export async function matches(password: string) {
  const [salt, hash] = read(STORE).split(".");
  return Boolean(salt && hash) && (await stretch(password, unhex(salt!))) === hash;
}

export async function choose(password: string) {
  const salt = crypto.getRandomValues(new Uint8Array(SALT));
  write(STORE, `${hex(salt)}.${await stretch(password, salt)}`);
  admit();
}

export function forget() {
  write(STORE, "");
  remember(false);
}
