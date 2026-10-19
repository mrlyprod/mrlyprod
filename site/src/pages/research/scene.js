export const ROOT = '/research/';

export function slugOf(path) {
  if (!path.startsWith(ROOT)) return '';
  try {
    return decodeURIComponent(path.slice(ROOT.length)).replace(/\/$/, '');
  } catch {
    return path.slice(ROOT.length).replace(/\/$/, '');
  }
}

export const hrefOf = (slug) => `${ROOT}${encodeURIComponent(slug)}/`;

export const sourceOf = (slug) => `research/${slug}/README.md`;

export const codeOf = (slug) => `/git/${sourceOf(slug)}`;

export const cards = (topics) => topics.map((one) => ({ name: one.title, href: hrefOf(one.slug), figure: one.figure }));

export function shelf(topics, query = '') {
  const want = query.trim().toLowerCase();
  const hit = (one) => !want || `${one.title}\n${one.lead ?? ''}`.toLowerCase().includes(want);
  return topics.filter(hit).sort((a, b) => b.date.localeCompare(a.date) || a.title.localeCompare(b.title));
}

export const many = (n, word) => `${n} ${word}${n === 1 ? '' : 's'}`;
