import type { Schema } from "../kernel/index.ts";

/* TYPES */

export const KINDS = ["h1", "h2", "h3", "p", "li", "quote", "code"] as const;

export type Kind = (typeof KINDS)[number];

export type Block = { kind: Kind; text: string; kids?: Block[] };

export const TEXT = 1000;

export const BLOCKS = 100;

export const DEPTH = 3;

/* SCHEMAS */

const block = (depth: number): Schema => ({
  type: "object",
  properties: {
    kind: { enum: [...KINDS] },
    text: { type: "string", maxLength: TEXT },
    ...(depth > 1 ? { kids: { type: "array", maxItems: BLOCKS, items: block(depth - 1) } } : {}),
  },
  required: ["kind", "text"],
  additionalProperties: false,
});

export const TREE: Schema = { type: "array", maxItems: BLOCKS, items: block(DEPTH) };

export const PATH: Schema = {
  type: "array",
  items: { type: "integer", minimum: 0, maximum: BLOCKS - 1 },
  minItems: 1,
  maxItems: DEPTH,
};

/* TREES */

export const count = (blocks: readonly Block[]): number => blocks.reduce((n, one) => n + 1 + count(one.kids ?? []), 0);

export const text = (blocks: readonly Block[]): string =>
  blocks
    .flatMap((one) => [one.text, text(one.kids ?? [])])
    .filter(Boolean)
    .join("\n");

export function at(blocks: readonly Block[], path: readonly number[]): Block | undefined {
  let list = blocks;
  let found: Block | undefined;
  for (const i of path) {
    found = list[i];
    if (!found) return undefined;
    list = found.kids ?? [];
  }
  return found;
}

export const edited = (blocks: readonly Block[], path: readonly number[], next: string): Block[] =>
  blocks.map((one, i) =>
    i !== path[0]
      ? one
      : path.length === 1
        ? { ...one, text: next }
        : { ...one, kids: edited(one.kids ?? [], path.slice(1), next) },
  );

export const checked = (blocks: readonly Block[], refuse: (note: string) => never): Block[] => {
  if (count(blocks) > BLOCKS) refuse(`a tree holds at most ${BLOCKS} blocks`);
  return blocks as Block[];
};
