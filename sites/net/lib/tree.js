import site from '../../ui/site.json';
import manifest from '../pages.json';

const ROUTE = { demos: '/demos/' };

export function tree(lists = {}) {
  const demos = manifest.shelves
    .map((shelf) => ({
      name: shelf.title,
      nodes: manifest.pages.filter((page) => page.category === shelf.key).map((page) => ({ name: page.title, href: `/demos/${page.name}/` })),
    }))
    .filter((shelf) => shelf.nodes.length);
  const filled = { demos, ...lists };
  return site.tree.map(({ fill, ...node }) => {
    const key = fill ?? node.name.toLowerCase();
    const href = node.href ?? ROUTE[key];
    const nodes = filled[key];
    return nodes?.length ? { ...node, href, nodes } : { ...node, href };
  });
}
