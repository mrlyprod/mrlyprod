import { rows, site } from 'site:routes';
import { island } from '../../lib/island.jsx';
import { Widget } from './widget.jsx';

let drawn = null;

const draw = () => Promise.all([import('../../lib/figure.js'), import('site:figures')]).then(([one, table]) => (one.start(table), (drawn = one)));

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return {
    node: <Widget rows={rows} site={site} draw={draw} />,
    close: () => {
      drawn?.drop();
      if (still) host.after(still);
    },
  };
});
