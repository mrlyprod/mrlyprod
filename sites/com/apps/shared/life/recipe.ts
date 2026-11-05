import type { recipe } from "@mrlyprod/life";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Patch, Scalar, Value } from "@mrlyprod/ui/kit/spec.ts";
import type { Door } from "./engine.ts";
import { codeOf, codesOf, grammar, parse, roll, rolled } from "./engine.ts";
import { sprite } from "./scene.ts";

export type Recipe = recipe.Recipe;

export type Mode = recipe.Mode;

export type Stop = recipe.Stop;

export type Run = recipe.Run;

export type Tokens = { of: Map<number, string>; code: Map<string, number> };

export const FIXED = "board";

export const DESIGN = "design";

export const THUMB = 64;

const TOKENS = new WeakMap<Door, Tokens>();

const CODE = "code";

export const fixed = (value: Value) => String(value.mode ?? FIXED) === FIXED;

export const modes = (life: Door | undefined): [string, string][] => [
  [FIXED, "Board"],
  ...(life ? grammar(life).modes.map((mode): [string, string] => [mode, mode[0].toUpperCase() + mode.slice(1)]) : []),
];

export function modeOf(life: Door, value: Value): Mode | null {
  const at = grammar(life).modes.indexOf(String(value.mode));
  return at < 0 ? null : (life.recipe.Mode.all()[at] ?? null);
}

export const island = (life: Door, value: Value) => modeOf(life, value) === "Island";

export function tokens(life: Door, math: Mrly): Tokens {
  let made = TOKENS.get(life);
  if (made) return made;
  const named = new Map<string, number>();
  for (const code of codesOf(2).map(Number)) {
    const name = math.name.of(code, 2, 2);
    if (name) named.set(name, code);
  }
  const of = new Map<number, string>();
  const code = new Map<string, number>();
  for (const token of grammar(life).shapes) {
    const c = token.startsWith(CODE) ? Number(token.slice(CODE.length)) : named.get(token);
    if (c === undefined || !Number.isInteger(c)) continue;
    code.set(token, c);
    if (!of.has(c)) of.set(c, token);
  }
  made = { of, code };
  TOKENS.set(life, made);
  return made;
}

export const tokenOf = (made: Tokens, code: number) => made.of.get(code) ?? `${CODE}${code}`;

export const codeOfToken = (made: Tokens, token: string) =>
  made.code.get(token) ?? (token.startsWith(CODE) ? Number(token.slice(CODE.length)) : Number.NaN);

function side(life: Door, text: Scalar, label: string): string {
  const drawn = rolled(life, text);
  if (drawn) return roll(life, drawn);
  const counts = parse(text);
  if (!counts.length) throw new Error(`${label} names no count and no source`);
  return counts.join(",");
}

export function recipeOf(life: Door, math: Mrly, value: Value): Recipe {
  const mode = modeOf(life, value);
  if (!mode) throw new Error(`${value.mode} is no recipe mode`);
  const alone = mode === "Island";
  const mask = String(value.mask);
  return {
    seed: String(value.shape),
    side: Number(value.side),
    level: Number(value.depth),
    tiling: alone ? Number(value.tiling) : 1,
    mode,
    mask: grammar(life).copies.includes(mask)
      ? mask
      : `${tokenOf(tokens(life, math), codeOf(value.code))}.${Number(value.number)}.${Number(value.level)}`,
    birth: side(life, value.born, "born"),
    survive: side(life, value.stay, "stay"),
    x: Number(value.x),
    cap: alone ? Number(value.cap) : 0,
  };
}

export function knobsOf(life: Door, math: Mrly, recipe: Recipe): Patch {
  const made = grammar(life);
  const alone = recipe.mode === "Island";
  const design = made.copies.includes(recipe.mask) ? null : recipe.mask.split(".");
  return {
    mode: made.modes[life.recipe.Mode.all().indexOf(recipe.mode)],
    dim: 2,
    shape: recipe.seed,
    side: recipe.side,
    depth: recipe.level,
    tiling: recipe.tiling,
    x: recipe.x,
    ...(alone ? { cap: recipe.cap } : {}),
    mask: design ? DESIGN : recipe.mask,
    ...(design
      ? {
          code: String(codeOfToken(tokens(life, math), design[0])),
          number: Number(design[1]),
          level: Number(design[2]),
        }
      : {}),
    born: recipe.birth,
    stay: recipe.survive,
  };
}

export function nameOf(life: Door, math: Mrly, value: Value): string {
  try {
    return fixed(value) ? "" : life.recipe.name(recipeOf(life, math, value));
  } catch {
    return "";
  }
}

export function said(stop: Stop): string {
  const parts = [`${stop.kind} after ${stop.generation}`];
  if (stop.period > 1) parts.push(`period ${stop.period}`);
  if (stop.dr || stop.dc) parts.push(`by ${stop.dr},${stop.dc}`);
  if (stop.wall_by) parts.push(`wall by ${stop.wall_by}`);
  return parts.join(", ");
}

export function thumb(
  ctx: CanvasRenderingContext2D,
  life: Door,
  key: string,
  px: number,
  ink: string,
  paper: string,
  cap = THUMB,
): void {
  let run: Run | null = null;
  try {
    run = new life.recipe.Run(life.recipe.parse(key));
    for (let i = 0; i < cap; i++) if (run.step()) break;
    const frame = run.frame();
    const [rows, cols] = frame.shape;
    const full = run.background() === 1;
    sprite(ctx, frame.types, cols, rows, px, full ? paper : ink, full ? ink : undefined);
  } catch {
    ctx.clearRect(0, 0, px, px);
  } finally {
    run?.free();
  }
}
