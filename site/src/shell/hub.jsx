import { mount as search, unmount as unsearch } from './search.js';
import { hide, Lede, show } from './parts.jsx';

export function hub(body) {
  return {
    layout: { wide: true, bare: true },
    render(row, site, host, context) {
      show(host, <><Lede id={row.meta.id} title={row.title} lead={row.lead} />{body(row, site, context)}</>);
    },
    mount(host, { rows }) {
      const menu = host.querySelector('.menu');
      if (menu) search(menu, rows);
    },
    unmount(host) {
      const menu = host.querySelector('.menu');
      if (menu) unsearch(menu);
      hide(host);
    },
  };
}
