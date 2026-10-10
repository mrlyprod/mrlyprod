import { hide, Lede, show } from '../parts.jsx';

export function render(row, site, host) {
  show(host, <><Lede id="cart" title={row.title} lead={row.lead} /><p>mrly.net has no shop yet.</p><p><a href="/">Back to the home page</a>.</p></>);
}

export const unmount = hide;
