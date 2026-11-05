import type { Ctx } from "@mrlyprod/os/contract/index.ts";

const RESERVED = [
  "admin",
  "administrator",
  "official",
  "root",
  "staff",
  "support",
  "system",
  "moderator",
  "mod",
  "owner",
  "verified",
  "security",
  "helpdesk",
];

const DECOR = ["official", "real", "original", "true", "verified", "new", "the", "genuine"];

const HIDDEN = /[\p{Cc}\p{Cf}\p{Zl}\p{Zp}\p{Co}\p{Cn}]/u;

const LOOKALIKES: Record<string, string> = Object.fromEntries(
  "аa еe оo рp сc ѕs хx уy іi јj һh ԁd ɡg кk мm нh тt вb гr ԝw οo αa νv ιi κk ρp τt υu εe ηn łl ıi ӏl ǀl ｌl 0o"
    .split(" ")
    .map((pair) => [...pair] as [string, string]),
);

const plain = (name: string) => name.normalize("NFKD").replace(/[\p{M}\p{Cf}]/gu, "");

export const hidden = (name: string) => HIDDEN.test(name);

export const skeleton = (name: string): string => {
  const mapped = [...plain(name).toLowerCase()].map((one) => LOOKALIKES[one] ?? one).join("");
  const folded = mapped
    .replace(/[i1|!]/g, "l")
    .replace(/rn/g, "m")
    .replace(/vv/g, "w");
  return folded.replace(/[^\p{L}\p{N}]/gu, "") || mapped;
};

const words = (name: string) =>
  plain(name)
    .split(/[^\p{L}\p{N}]+/u)
    .filter(Boolean);

const BARRED = new Set(RESERVED.map(skeleton));

const CROWD = new Set(DECOR.map(skeleton));

export const reserved = (name: string) =>
  skeleton(name).includes("mrly") || words(name).some((one) => BARRED.has(skeleton(one.replace(/\d+$/, ""))));

export const core = (name: string) => {
  const kept = words(name)
    .map(skeleton)
    .filter((one) => !CROWD.has(one));
  return kept.length ? kept.join("") : skeleton(name);
};

const reverse = (text: string) => [...text].reverse().join("");

export const alike = (one: string, other: string) => {
  const a = skeleton(one);
  const b = skeleton(other);
  if (!a || !b) return false;
  if (a === b || a === reverse(b)) return true;
  const c = core(one);
  const d = core(other);
  return c === d || c === reverse(d);
};

export function clean(name: string, what: string, ctx: Ctx): string {
  if (hidden(name)) return ctx.refuse(`${what} cannot hold control, zero-width or direction characters`);
  if (reserved(name)) return ctx.refuse(`${name} uses a reserved word (admin, official, support, mrly); pick another`);
  return name;
}
