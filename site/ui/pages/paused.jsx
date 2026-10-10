import { Figure, hide, Lede, show } from '../parts.jsx';

export function render(row, site, host) {
  show(
    host,
    <>
      <Figure className="opener" name={row.figure} label={row.title} />
      <Lede id={row.title.toLowerCase()} title={row.title} lead="This section is being rebuilt." />
      <p><a href="/">Back to the home page</a>.</p>
    </>,
  );
}

export const unmount = hide;
