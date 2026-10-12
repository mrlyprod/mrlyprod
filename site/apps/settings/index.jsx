import { hide, Lede, Settings, show } from '../../ui/parts.jsx';

export const layout = {};

export function render(row, site, host) {
  show(host, <><Lede id="settings" title={row.title} lead={row.lead} /><Settings savers={site.savers} /></>);
}

export const unmount = hide;
