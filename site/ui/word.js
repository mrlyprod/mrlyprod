export function doors(tree) {
  const out = [];
  const walk = (nodes) => {
    for (const node of nodes) {
      if (node.href?.startsWith('/') && node.href.endsWith('/') && node.href !== '/') out.push([node.href, node.name.toLowerCase()]);
      if (node.nodes) walk(node.nodes);
    }
  };
  walk(tree);
  return out.sort((a, b) => b[0].length - a[0].length || (a[0] < b[0] ? -1 : 1));
}

export function word(route, list) {
  if (route === '/') return '';
  const door = list.find(([href]) => route.startsWith(href));
  if (door) return door[1];
  const first = (route.split('/').find(Boolean) ?? '').toLowerCase();
  return first.includes('.') ? '' : first;
}
