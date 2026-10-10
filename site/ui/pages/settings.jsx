import { hide, Lede, Settings, show } from '../parts.jsx';

export function render(row, site, host) {
  const savers = site.apps.filter((one) => one.kind === 'saver');
  show(host, <><Lede id="settings" title={row.title} lead={row.lead} /><Settings savers={savers} /></>);
}

export const unmount = hide;
