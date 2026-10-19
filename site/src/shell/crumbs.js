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

export function paint(steps) {
  const list = document.querySelector('.title .crumbs ol');
  if (!list) return;
  list.replaceChildren(
    ...steps.map((step, n) => {
      const item = document.createElement('li');
      const a = document.createElement('a');
      a.href = step.href;
      a.textContent = step.name;
      if (n === steps.length - 1) a.setAttribute('aria-current', 'page');
      item.append(a);
      return item;
    }),
  );
}
