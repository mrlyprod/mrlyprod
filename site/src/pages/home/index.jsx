import { hide, show } from "../../shell/parts.jsx";

export const layout = { wide: true, bare: true, window: "none" };

export function render(_row, site, host) {
  show(host, <h1 className="visually-hidden">{site.title}</h1>);
}

export const unmount = hide;
