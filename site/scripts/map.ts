/* TYPES */

export type Listed = { route: string; kind: string; title: string; source: string | null; lead: string; hidden: boolean; meta: Record<string, unknown> };

export type Line = { name: string; href: string; note: string };

export type Section = { name: string; lines: Line[] };

/* SECTIONS */

const GROUPS: [string, (row: Listed) => boolean][] = [
  ["Blog", (row) => row.meta.was === "post"],
  ["MrlyMath", (row) => row.meta.was === "math"],
  ["About", (row) => row.meta.was === "page"],
  ["Apps", (row) => row.kind === "app" || row.kind === "settings"],
  ["Doors", () => true],
];

export function sections(rows: Listed[], raw: Set<string>): Section[] {
  const left = rows.filter((row) => !row.hidden);
  const line = (row: Listed): Line => ({ name: row.title, note: row.lead, href: row.source && raw.has(row.source) ? `/raw/${row.source}` : row.route });
  return GROUPS.map(([name, pick]) => {
    const taken = left.filter(pick);
    for (const row of taken) left.splice(left.indexOf(row), 1);
    return { name, lines: taken.map(line) };
  }).filter((one) => one.lines.length);
}

/* LLMS */

export function llms(title: string, root: string, words: { about?: string; legend?: string }, rows: Listed[], raw: Set<string>): string {
  const list = (lines: Line[]) => lines.map((one) => `- [${one.name}](${root}${one.href})${one.note ? `: ${one.note}` : ""}`).join("\n");
  const blocks = [`# ${title}`, `> ${root}`, words.about, words.legend, ...sections(rows, raw).map((one) => `## ${one.name}\n\n${list(one.lines)}`)];
  return `${blocks.filter(Boolean).join("\n\n")}\n`;
}
