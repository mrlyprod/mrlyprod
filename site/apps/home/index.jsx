import { hide, show } from '../../ui/parts.jsx';

export const layout = { wide: true, bare: true };

export function render(row, site, host) {
  show(host, <h1 className="visually-hidden">{site.title}</h1>);
}

export const unmount = hide;
