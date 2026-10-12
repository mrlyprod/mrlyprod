import { hide, show } from '../../ui/parts.jsx';

export const layout = { bare: true };

export function Lost({ site, rows, row = rows.find((one) => one.meta?.id === 'missing') }) {
  const doors = rows.filter((one) => one.meta?.menu === 'read' && !one.hidden).sort((a, b) => a.title.localeCompare(b.title)).map((one) => <a key={one.route} href={one.route}>{one.title}</a>);
  const list = doors.flatMap((one, n) => (n === 0 ? [one] : [n === doors.length - 1 ? ' and ' : ', ', one]));
  return (
    <div className="lede">
      <h1 id="lost">{row.title}</h1>
      <p className="lead">{row.lead} The <a href={site.menu}>Menu</a> holds every door of this site, starting with {list}. A page that moved is on <a href="/redirects/">Redirects</a>.</p>
    </div>
  );
}

export function render(row, site, host, { rows }) {
  show(host, <Lost row={row} site={site} rows={rows} />);
}

export const unmount = hide;
