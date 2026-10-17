import { Lost, hide, show } from '../../shell/parts.jsx';

export const layout = { bare: true };

export function render(row, site, host, { rows }) {
  show(host, <Lost row={row} site={site} rows={rows} />);
}

export const unmount = hide;
