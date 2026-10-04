const plain = (part) => {
  try {
    return decodeURIComponent(part);
  } catch {
    return part;
  }
};

export function crumbs(path) {
  const parts = path.split('/').filter(Boolean);
  const folder = path.endsWith('/');
  return parts.map((part, n) => ({ name: plain(part), href: `/${parts.slice(0, n + 1).join('/')}${folder || n < parts.length - 1 ? '/' : ''}` }));
}
