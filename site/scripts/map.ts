/* TYPES */

export type Listed = { route: string; kind: string; title: string; source: string | null; date?: string | null; lead: string; hidden: boolean; meta: Record<string, unknown> };

export type Line = { name: string; href: string; note: string };

export type Section = { name: string; lines: Line[] };

export type Topic = { slug: string; title: string };

/* SECTIONS */

const GROUPS: [string, (row: Listed) => boolean][] = [
  ["Play", (row) => row.meta.menu === "play"],
  ["Read", (row) => row.meta.menu === "read"],
  ["Tools", (row) => row.meta.menu === "tool"],
  ["Blog", (row) => row.meta.prose === "post"],
  ["Doors", () => true],
];

export function sections(rows: Listed[], raw: Set<string>, topics: Topic[] = []): Section[] {
  const left = rows.filter((row) => !row.hidden);
  const line = (row: Listed): Line => ({ name: row.title, note: row.lead, href: row.source && raw.has(row.source) ? `/raw/${row.source}` : row.route });
  const found = GROUPS.map(([name, pick]) => {
    const taken = left.filter(pick).sort((a, b) => (b.date ?? "").localeCompare(a.date ?? "") || a.title.localeCompare(b.title));
    for (const row of taken) left.splice(left.indexOf(row), 1);
    return { name, lines: taken.map(line) };
  });
  const research = { name: "Research", lines: topics.length ? [{ name: "Research", note: "", href: "/raw/research/README.md" }, ...topics.map((one) => ({ name: one.title, note: "", href: `/raw/research/${one.slug}/README.md` }))] : [] };
  return [...found, research].filter((one) => one.lines.length);
}

/* LLMS */

export function llms(title: string, root: string, words: { about?: string; legend?: string }, rows: Listed[], raw: Set<string>, topics: Topic[] = []): string {
  const list = (lines: Line[]) => lines.map((one) => `- [${one.name}](${root}${one.href})${one.note ? `: ${one.note}` : ""}`).join("\n");
  const blocks = [`# ${title}`, `> ${root}`, words.about, words.legend, ...sections(rows, raw, topics).map((one) => `## ${one.name}\n\n${list(one.lines)}`)];
  return `${blocks.filter(Boolean).join("\n\n")}\n`;
}
