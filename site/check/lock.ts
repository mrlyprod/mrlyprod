export type Rows = Record<string, unknown>;

export function law(names: string[], rows: Rows, press: Record<string, string> | null): string[] {
  const bad: string[] = [];
  const made = new Set(names);
  for (const name of names) if (typeof rows[name] !== 'string') bad.push(`figures/${name}.ts has no pressed row in site/figures.lock`);
  for (const [name, row] of Object.entries(rows)) if (typeof row === 'string' && !made.has(name)) bad.push(`${name} is a pressed row of site/figures.lock with no figures/${name}.ts`);
  const behind = names.filter((name) => typeof rows[name] === 'string' && press?.[name] && press[name] !== rows[name]);
  if (behind.length) bad.push(`the lock is behind the press on ${behind.length} figures (${behind.slice(0, 3).join(', ')}); run the figures console`);
  return bad;
}
