import { hide, Lede, show } from './parts.jsx';

export function hub(body, layout = { wide: true, bare: true }) {
  return {
    layout,
    render(row, site, host, context) {
      show(host, <><Lede id={row.meta.id} title={row.title} lead={row.lead} />{body(row, site, context)}</>);
    },
    unmount: hide,
  };
}
