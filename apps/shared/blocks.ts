import type { Knobs } from "@mrlyprod/os/contract/index.ts";
import type { Schema } from "@mrlyprod/os/kernel/index.ts";

export const KINDS = ["h1", "h2", "h3", "p", "li", "quote", "code"] as const;

export type Kind = (typeof KINDS)[number];

export type Block = { kind: Kind; text: string; kids?: Block[] };

export const TREE_KNOBS = {
  blocks: { line: "Blocks one tree holds.", schema: { type: "integer", minimum: 5, maximum: 1000 }, reality: 100 },
  text: {
    line: "Characters one block holds.",
    schema: { type: "integer", minimum: 50, maximum: 10000 },
    reality: 1000,
  },
  depth: { line: "How many levels blocks nest.", schema: { type: "integer", minimum: 1, maximum: 6 }, reality: 3 },
} satisfies Knobs;

export type Limits = { blocks: number; text: number; depth: number };

export const KIND: Schema = { enum: [...KINDS] };

const block = (limits: Limits, depth: number): Schema => ({
  type: "object",
  properties: {
    kind: KIND,
    text: { type: "string", maxLength: limits.text },
    ...(depth > 1 ? { kids: { type: "array", maxItems: limits.blocks, items: block(limits, depth - 1) } } : {}),
  },
  required: ["kind", "text"],
  additionalProperties: false,
});

export const tree = (limits: Limits): Schema => ({
  type: "array",
  maxItems: limits.blocks,
  items: block(limits, limits.depth),
});

export const place = (limits: Limits): Schema => ({
  type: "array",
  items: { type: "integer", minimum: 0, maximum: limits.blocks },
  minItems: 1,
  maxItems: limits.depth,
});

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

const under = (blocks: readonly Block[], path: readonly number[]): readonly Block[] | undefined => {
  if (!path.length) return blocks;
  const one = at(blocks, path);
  return one ? (one.kids ?? []) : undefined;
};

export const fits = (blocks: readonly Block[], path: readonly number[]): boolean => {
  const list = under(blocks, path.slice(0, -1));
  return !!list && path.at(-1)! <= list.length;
};

export const inserted = (blocks: readonly Block[], path: readonly number[], one: Block): Block[] => {
  const [head = 0, ...rest] = path;
  if (!rest.length) return [...blocks.slice(0, head), one, ...blocks.slice(head)];
  return blocks.map((each, i) => (i === head ? { ...each, kids: inserted(each.kids ?? [], rest, one) } : each));
};

export const cut = (blocks: readonly Block[], path: readonly number[]): Block[] => {
  const [head = 0, ...rest] = path;
  if (!rest.length) return blocks.filter((_, i) => i !== head);
  return blocks.map((each, i) => (i === head ? { ...each, kids: cut(each.kids ?? [], rest) } : each));
};

export const checked = (blocks: readonly Block[], most: number, refuse: (note: string) => never): Block[] => {
  if (count(blocks) > most) refuse(`a tree holds at most ${most} blocks`);
  return blocks as Block[];
};
