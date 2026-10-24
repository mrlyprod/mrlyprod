import { BITS } from "./consts.js";
import { shift } from "./orbit.js";

export const WHOLE = 3;
const MORE = 5;
const POWER = 400;
const WORD = BigInt(BITS);
const HALF = 1n << (WORD - 1n);
const DECIMAL = /^([+-]?)(\d*)(?:\.(\d*))?(?:e([+-]?\d+))?$/i;

/* DECIMALS */

export function parse(text) {
  const found = DECIMAL.exec(String(text ?? "").trim());
  if (!found || !(found[2] || found[3])) return null;
  const [, sign, whole = "", part = "", power = "0"] = found;
  const e = Number(power) - part.length;
  if (Math.abs(e) > POWER) return null;
  const digits = BigInt(`${whole}${part}` || "0");
  const ten = 10n ** BigInt(Math.abs(e));
  const n = e >= 0 ? (digits * ten) << WORD : ((digits << WORD) + ten / 2n) / ten;
  return sign === "-" ? -n : n;
}

export function print(v, span) {
  const places = Math.max(1, Math.ceil(-Math.log10(span)) + MORE);
  const neg = v < 0n;
  const n = ((neg ? -v : v) * 10n ** BigInt(places) + HALF) >> WORD;
  const text = n.toString().padStart(places + 1, "0");
  const tail = text.slice(-places).replace(/0+$/, "");
  return `${neg && n ? "-" : ""}${text.slice(0, -places)}${tail ? `.${tail}` : ""}`;
}

/* VIEW */

export const spanOf = (zoom) => WHOLE * 2 ** -zoom;

export const zoomOf = (span) => Math.log2(WHOLE / span);

export function offset(v, [u, w]) {
  const cos = Math.cos(v.angle);
  const sin = Math.sin(v.angle);
  return [(u * cos - w * sin) * v.span, (u * sin + w * cos) * v.span];
}

export function pan(v, by) {
  const [dx, dy] = offset(v, by);
  return { ...v, centre: shift(v.centre, -dx, -dy) };
}

export function zoomAt(v, at, k) {
  const [dx, dy] = offset(v, at);
  return { ...v, centre: shift(v.centre, dx * (1 - 1 / k), dy * (1 - 1 / k)), span: v.span / k };
}
