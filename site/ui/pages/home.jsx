import { hide, show } from '../parts.jsx';

export function render(row, site, host) {
  show(host, <h1 className="visually-hidden">{site.title}</h1>);
}

export const unmount = hide;
