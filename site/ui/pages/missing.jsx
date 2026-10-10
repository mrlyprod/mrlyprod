import { hide, show } from '../parts.jsx';

export function render(row, site, host) {
  const doors = site.heroes.map((one) => <a key={one.href} href={one.href}>{one.name}</a>);
  const list = doors.flatMap((one, n) => (n === 0 ? [one] : [n === doors.length - 1 ? ' and ' : ', ', one]));
  show(
    host,
    <div className="lede">
      <h1 id="lost">{row.title}</h1>
      <p className="lead">{row.lead} The <a href={site.menu}>Menu</a> holds every door of this site, starting with {list}. A page that moved is on <a href="/redirects/">Redirects</a>.</p>
    </div>,
  );
}

export const unmount = hide;
