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

export const cards = (topics) => topics.map((one) => ({ name: one.title, href: hrefOf(one.slug), figure: one.figure }));
