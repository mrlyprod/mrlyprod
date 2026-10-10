import { hide, Lede, show } from '../parts.jsx';

export function render(row, site, host) {
  const moves = Object.entries(site.redirects).sort(([a, x], [b, y]) => y.since.localeCompare(x.since) || a.localeCompare(b));
  show(
    host,
    <>
      <Lede id="redirects" title={row.title} lead={row.lead} />
      {moves.length ? (
        <div className="scroll">
          <table>
            <thead><tr><th>From</th><th>To</th><th>Since</th></tr></thead>
            <tbody>
              {moves.map(([from, one]) => (
                <tr key={from}><td><code>{from}</code></td><td><a href={one.to}>{one.to}</a></td><td>{one.since}</td></tr>
              ))}
            </tbody>
          </table>
        </div>
      ) : (
        <p>No page has moved.</p>
      )}
    </>,
  );
}

export const unmount = hide;
