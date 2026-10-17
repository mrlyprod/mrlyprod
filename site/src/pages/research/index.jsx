import { rows, site } from 'site:routes';
import { island } from '../../lib/app/island.jsx';
import { Widget } from './widget.jsx';
import { drawn, figures } from '../../shell/figures.js';

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return {
    node: <Widget rows={rows} site={site} draw={figures} />,
    close: () => {
      drawn?.drop();
      if (still) host.after(still);
    },
  };
});
