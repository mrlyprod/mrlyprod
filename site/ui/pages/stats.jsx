import { mount as watch, unmount as unwatch } from '../stats.js';
import { hide, Lede, show } from '../parts.jsx';

export function render(row, site, host) {
  show(
    host,
    <>
      <Lede id="stats" title={row.title}>
        <p className="lead">{row.lead} Raw: <a href="/stats/stats.json">stats.json</a>.</p>
      </Lede>
      <div>
        <section><h2 id="cloud">Cloud</h2><div data-stats="cloud"><p className="fine">Loading</p></div></section>
        <section><h2 id="errors">Errors</h2><div data-stats="errors"><p className="fine">Loading</p></div></section>
      </div>
    </>,
  );
}

export function mount(host) {
  watch(host);
}

export function unmount(host) {
  unwatch(host);
  hide(host);
}
